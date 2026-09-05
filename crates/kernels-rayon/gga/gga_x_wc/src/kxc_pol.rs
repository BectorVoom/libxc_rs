//! GGA_X_WC kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_wc.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_wc_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v2rhosigma_0 = V_ZERO;
        let mut acc_v2rhosigma_1 = V_ZERO;
        let mut acc_v2rhosigma_2 = V_ZERO;
        let mut acc_v2rhosigma_3 = V_ZERO;
        let mut acc_v2rhosigma_4 = V_ZERO;
        let mut acc_v2rhosigma_5 = V_ZERO;
        let mut acc_v2sigma2_0 = V_ZERO;
        let mut acc_v2sigma2_1 = V_ZERO;
        let mut acc_v2sigma2_2 = V_ZERO;
        let mut acc_v2sigma2_3 = V_ZERO;
        let mut acc_v2sigma2_4 = V_ZERO;
        let mut acc_v2sigma2_5 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v3rho2sigma_0 = V_ZERO;
        let mut acc_v3rho2sigma_1 = V_ZERO;
        let mut acc_v3rho2sigma_2 = V_ZERO;
        let mut acc_v3rho2sigma_3 = V_ZERO;
        let mut acc_v3rho2sigma_4 = V_ZERO;
        let mut acc_v3rho2sigma_5 = V_ZERO;
        let mut acc_v3rho2sigma_6 = V_ZERO;
        let mut acc_v3rho2sigma_7 = V_ZERO;
        let mut acc_v3rho2sigma_8 = V_ZERO;
        let mut acc_v3rhosigma2_0 = V_ZERO;
        let mut acc_v3rhosigma2_1 = V_ZERO;
        let mut acc_v3rhosigma2_2 = V_ZERO;
        let mut acc_v3rhosigma2_3 = V_ZERO;
        let mut acc_v3rhosigma2_4 = V_ZERO;
        let mut acc_v3rhosigma2_5 = V_ZERO;
        let mut acc_v3rhosigma2_6 = V_ZERO;
        let mut acc_v3rhosigma2_7 = V_ZERO;
        let mut acc_v3rhosigma2_8 = V_ZERO;
        let mut acc_v3rhosigma2_9 = V_ZERO;
        let mut acc_v3rhosigma2_10 = V_ZERO;
        let mut acc_v3rhosigma2_11 = V_ZERO;
        let mut acc_v3sigma3_0 = V_ZERO;
        let mut acc_v3sigma3_1 = V_ZERO;
        let mut acc_v3sigma3_2 = V_ZERO;
        let mut acc_v3sigma3_3 = V_ZERO;
        let mut acc_v3sigma3_4 = V_ZERO;
        let mut acc_v3sigma3_5 = V_ZERO;
        let mut acc_v3sigma3_6 = V_ZERO;
        let mut acc_v3sigma3_7 = V_ZERO;
        let mut acc_v3sigma3_8 = V_ZERO;
        let mut acc_v3sigma3_9 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t28 * t32;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t39 = v_sigma0 * t38;
            let t40 = t33 * t39;
            let t43 = (simd::exp(-t40 / f64x8::splat(24.0)));
            let t47 = t28 * t28;
            let t49 = f64x8::splat(1.0) / t30 / t29;
            let t50 = t47 * t49;
            let t51 = v_sigma0 * v_sigma0;
            let t52 = t34 * t34;
            let t53 = t52 * v_rho0;
            let t55 = f64x8::splat(1.0) / t35 / t53;
            let t59 = f64x8::splat(1.0) + f64x8::splat(1.3780328706878157e-05) * t50 * t51 * t55;
            let t60 = (simd::ln(t59));
            let t61 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t40 + f64x8::splat(0.004002424276710846) * t33 * t39 * t43 + t60;
            let t64 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t61;
            let t68 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t25 * t26 * t64));
            let t69 = (v_rho1).simd_le(dens_threshold);
            let t70 = -t16;
            let t72 = ((t14).select(t11, (t10).select(t15, t70 * t7)));
            let t73 = f64x8::splat(1.0) + t72;
            let t74 = (t73).simd_le(zeta_threshold);
            let t75 = (simd::cbrt(t73));
            let t77 = ((t74).select(t22, t75 * t73));
            let t79 = v_rho1 * v_rho1;
            let t80 = (simd::cbrt(v_rho1));
            let t81 = t80 * t80;
            let t83 = f64x8::splat(1.0) / t81 / t79;
            let t84 = v_sigma2 * t83;
            let t85 = t33 * t84;
            let t88 = (simd::exp(-t85 / f64x8::splat(24.0)));
            let t92 = v_sigma2 * v_sigma2;
            let t93 = t79 * t79;
            let t94 = t93 * v_rho1;
            let t96 = f64x8::splat(1.0) / t80 / t94;
            let t100 = f64x8::splat(1.0) + f64x8::splat(1.3780328706878157e-05) * t50 * t92 * t96;
            let t101 = (simd::ln(t100));
            let t102 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t85 + f64x8::splat(0.004002424276710846) * t33 * t84 * t88 + t101;
            let t105 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t102;
            let t109 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t77 * t26 * t105));
            let tzk0 = t68 + t109;
            acc_zk = tzk0;
            let t110 = t6 * t6;
            let t111 = f64x8::splat(1.0) / t110;
            let t112 = t16 * t111;
            let t114 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t112)));
            let t117 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t114));
            let t122 = t26 * t26;
            let t123 = f64x8::splat(1.0) / t122;
            let t127 = t5 * t25 * t123 * t64 / f64x8::splat(8.0);
            let t128 = t2 * t25;
            let t129 = t61 * t61;
            let t130 = f64x8::splat(1.0) / t129;
            let t131 = t26 * t130;
            let t132 = t34 * v_rho0;
            let t134 = f64x8::splat(1.0) / t36 / t132;
            let t135 = v_sigma0 * t134;
            let t141 = t52 * t34;
            let t143 = f64x8::splat(1.0) / t35 / t141;
            let t144 = t51 * t143;
            let t148 = f64x8::splat(1.0) / t59;
            let t152 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t33 * t135 - f64x8::splat(0.010673131404562256) * t33 * t135 * t43 + f64x8::splat(0.00044471380852342736) * t50 * t144 * t43 - f64x8::splat(7.349508643668351e-05) * t50 * t144 * t148;
            let t153 = t131 * t152;
            let t157 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t117 * t26 * t64 - t127 - f64x8::splat(0.1655109536374632) * t128 * t153));
            let t158 = t70 * t111;
            let t160 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t158)));
            let t163 = ((t74).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t75 * t160));
            let t171 = t5 * t77 * t123 * t105 / f64x8::splat(8.0);
            let t173 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t163 * t26 * t105 - t171));
            let tvrho0 = t68 + t109 + t6 * (t157 + t173);
            acc_vrho_0 = tvrho0;
            let t177 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t112)));
            let t180 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t177));
            let t186 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t180 * t26 * t64 - t127));
            let t188 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t158)));
            let t191 = ((t74).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t75 * t188));
            let t196 = t2 * t77;
            let t197 = t102 * t102;
            let t198 = f64x8::splat(1.0) / t197;
            let t199 = t26 * t198;
            let t200 = t79 * v_rho1;
            let t202 = f64x8::splat(1.0) / t81 / t200;
            let t203 = v_sigma2 * t202;
            let t209 = t93 * t79;
            let t211 = f64x8::splat(1.0) / t80 / t209;
            let t212 = t92 * t211;
            let t216 = f64x8::splat(1.0) / t100;
            let t220 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t33 * t203 - f64x8::splat(0.010673131404562256) * t33 * t203 * t88 + f64x8::splat(0.00044471380852342736) * t50 * t212 * t88 - f64x8::splat(7.349508643668351e-05) * t50 * t212 * t216;
            let t221 = t199 * t220;
            let t225 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t191 * t26 * t105 - t171 - f64x8::splat(0.1655109536374632) * t196 * t221));
            let tvrho1 = t68 + t109 + t6 * (t186 + t225);
            acc_vrho_1 = tvrho1;
            let t233 = v_sigma0 * t55;
            let t240 = f64x8::splat(5.0) / f64x8::splat(972.0) * t33 * t38 + f64x8::splat(0.004002424276710846) * t33 * t38 * t43 - f64x8::splat(0.00016676767819628525) * t50 * t233 * t43 + f64x8::splat(2.7560657413756314e-05) * t50 * t233 * t148;
            let t241 = t131 * t240;
            let t244 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t128 * t241));
            let tvsigma0 = t6 * t244;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t250 = v_sigma2 * t96;
            let t257 = f64x8::splat(5.0) / f64x8::splat(972.0) * t33 * t83 + f64x8::splat(0.004002424276710846) * t33 * t83 * t88 - f64x8::splat(0.00016676767819628525) * t50 * t250 * t88 + f64x8::splat(2.7560657413756314e-05) * t50 * t250 * t216;
            let t258 = t199 * t257;
            let t261 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t196 * t258));
            let tvsigma2 = t6 * t261;
            acc_vsigma_2 = tvsigma2;
            let t264 = t23 * t23;
            let t265 = f64x8::splat(1.0) / t264;
            let t266 = t114 * t114;
            let t269 = t110 * t6;
            let t270 = f64x8::splat(1.0) / t269;
            let t271 = t16 * t270;
            let t274 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t111 + f64x8::splat(2.0) * t271)));
            let t278 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t265 * t266 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t274));
            let t285 = t5 * t117 * t123 * t64;
            let t287 = t2 * t117;
            let t291 = f64x8::splat(1.0) / t122 / t6;
            let t295 = t5 * t25 * t291 * t64 / f64x8::splat(12.0);
            let t296 = t123 * t130;
            let t297 = t296 * t152;
            let t298 = t128 * t297;
            let t301 = f64x8::splat(1.0) / t129 / t61;
            let t302 = t26 * t301;
            let t303 = t152 * t152;
            let t304 = t302 * t303;
            let t308 = f64x8::splat(1.0) / t36 / t52;
            let t309 = v_sigma0 * t308;
            let t315 = t52 * t132;
            let t317 = f64x8::splat(1.0) / t35 / t315;
            let t318 = t51 * t317;
            let t322 = t29 * t29;
            let t323 = f64x8::splat(1.0) / t322;
            let t324 = t51 * v_sigma0;
            let t325 = t323 * t324;
            let t326 = t52 * t52;
            let t327 = t326 * t34;
            let t328 = f64x8::splat(1.0) / t327;
            let t337 = t28 / t31 / t322;
            let t338 = t51 * t51;
            let t339 = t326 * t52;
            let t341 = f64x8::splat(1.0) / t36 / t339;
            let t343 = t59 * t59;
            let t344 = f64x8::splat(1.0) / t343;
            let t348 = f64x8::splat(110.0) / f64x8::splat(2187.0) * t33 * t309 + f64x8::splat(0.039134815150061605) * t33 * t309 * t43 - f64x8::splat(0.004002424276710846) * t50 * t318 * t43 + f64x8::splat(0.0002964758723489516) * t325 * t328 * t43 + f64x8::splat(0.0004654688807656622) * t50 * t318 * t148 - f64x8::splat(3.240916638201348e-08) * t337 * t338 * t341 * t344;
            let t349 = t131 * t348;
            let t353 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t278 * t26 * t64 - t285 / f64x8::splat(4.0) - f64x8::splat(0.3310219072749264) * t287 * t153 + t295 - f64x8::splat(0.1103406357583088) * t298 + f64x8::splat(0.3310219072749264) * t128 * t304 - f64x8::splat(0.1655109536374632) * t128 * t349));
            let t354 = t75 * t75;
            let t355 = f64x8::splat(1.0) / t354;
            let t356 = t160 * t160;
            let t359 = t70 * t270;
            let t362 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t111 + f64x8::splat(2.0) * t359)));
            let t366 = ((t74).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t355 * t356 + f64x8::splat(4.0) / f64x8::splat(3.0) * t75 * t362));
            let t373 = t5 * t163 * t123 * t105;
            let t378 = t5 * t77 * t291 * t105 / f64x8::splat(12.0);
            let t380 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t366 * t26 * t105 - t373 / f64x8::splat(4.0) + t378));
            let tv2rho20 = f64x8::splat(2.0) * t157 + f64x8::splat(2.0) * t173 + t6 * (t353 + t380);
            acc_v2rho2_0 = tv2rho20;
            let t383 = t265 * t177;
            let t387 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t271)));
            let t391 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t383 * t114 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t387));
            let t398 = t5 * t180 * t123 * t64;
            let t400 = t2 * t180;
            let t406 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t391 * t26 * t64 - t398 / f64x8::splat(8.0) - f64x8::splat(0.1655109536374632) * t400 * t153 - t285 / f64x8::splat(8.0) + t295 - f64x8::splat(0.0551703178791544) * t298));
            let t407 = t355 * t188;
            let t411 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t359)));
            let t415 = ((t74).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t407 * t160 + f64x8::splat(4.0) / f64x8::splat(3.0) * t75 * t411));
            let t422 = t5 * t191 * t123 * t105;
            let t425 = t2 * t163;
            let t428 = t123 * t198;
            let t429 = t428 * t220;
            let t430 = t196 * t429;
            let t433 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t415 * t26 * t105 - t422 / f64x8::splat(8.0) - t373 / f64x8::splat(8.0) + t378 - f64x8::splat(0.1655109536374632) * t425 * t221 - f64x8::splat(0.0551703178791544) * t430));
            let tv2rho21 = t157 + t173 + t186 + t225 + t6 * (t406 + t433);
            acc_v2rho2_1 = tv2rho21;
            let t438 = t177 * t177;
            let t443 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t111 + f64x8::splat(2.0) * t271)));
            let t447 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t265 * t438 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t443));
            let t454 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t447 * t26 * t64 - t398 / f64x8::splat(4.0) + t295));
            let t455 = t188 * t188;
            let t460 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t111 + f64x8::splat(2.0) * t359)));
            let t464 = ((t74).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t355 * t455 + f64x8::splat(4.0) / f64x8::splat(3.0) * t75 * t460));
            let t470 = t2 * t191;
            let t475 = f64x8::splat(1.0) / t197 / t102;
            let t476 = t26 * t475;
            let t477 = t220 * t220;
            let t478 = t476 * t477;
            let t482 = f64x8::splat(1.0) / t81 / t93;
            let t483 = v_sigma2 * t482;
            let t489 = t93 * t200;
            let t491 = f64x8::splat(1.0) / t80 / t489;
            let t492 = t92 * t491;
            let t496 = t92 * v_sigma2;
            let t497 = t323 * t496;
            let t498 = t93 * t93;
            let t499 = t498 * t79;
            let t500 = f64x8::splat(1.0) / t499;
            let t507 = t92 * t92;
            let t508 = t498 * t93;
            let t510 = f64x8::splat(1.0) / t81 / t508;
            let t512 = t100 * t100;
            let t513 = f64x8::splat(1.0) / t512;
            let t517 = f64x8::splat(110.0) / f64x8::splat(2187.0) * t33 * t483 + f64x8::splat(0.039134815150061605) * t33 * t483 * t88 - f64x8::splat(0.004002424276710846) * t50 * t492 * t88 + f64x8::splat(0.0002964758723489516) * t497 * t500 * t88 + f64x8::splat(0.0004654688807656622) * t50 * t492 * t216 - f64x8::splat(3.240916638201348e-08) * t337 * t507 * t510 * t513;
            let t518 = t199 * t517;
            let t522 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t464 * t26 * t105 - t422 / f64x8::splat(4.0) - f64x8::splat(0.3310219072749264) * t470 * t221 + t378 - f64x8::splat(0.1103406357583088) * t430 + f64x8::splat(0.3310219072749264) * t196 * t478 - f64x8::splat(0.1655109536374632) * t196 * t518));
            let tv2rho22 = f64x8::splat(2.0) * t186 + f64x8::splat(2.0) * t225 + t6 * (t454 + t522);
            acc_v2rho2_2 = tv2rho22;
            let t527 = t296 * t240;
            let t529 = f64x8::splat(0.0551703178791544) * t128 * t527;
            let t530 = t128 * t26;
            let t531 = t301 * t240;
            let t532 = t531 * t152;
            let t540 = t143 * v_sigma0;
            let t544 = t323 * t51;
            let t545 = t326 * v_rho0;
            let t546 = f64x8::splat(1.0) / t545;
            let t553 = t326 * t132;
            let t555 = f64x8::splat(1.0) / t36 / t553;
            let t560 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t33 * t134 - f64x8::splat(0.010673131404562256) * t33 * t134 * t43 + f64x8::splat(0.001334141425570282) * t50 * t540 * t43 - f64x8::splat(0.00011117845213085684) * t544 * t546 * t43 - f64x8::splat(0.00014699017287336702) * t50 * t540 * t148 + f64x8::splat(1.2153437393255055e-08) * t337 * t324 * t555 * t344;
            let t561 = t131 * t560;
            let t565 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t287 * t241 - t529 + f64x8::splat(0.3310219072749264) * t530 * t532 - f64x8::splat(0.1655109536374632) * t128 * t561));
            let tv2rhosigma0 = t6 * t565 + t244;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t569 = t428 * t257;
            let t571 = f64x8::splat(0.0551703178791544) * t196 * t569;
            let t573 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t425 * t258 - t571));
            let tv2rhosigma2 = t6 * t573 + t261;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t578 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t400 * t241 - t529));
            let tv2rhosigma3 = t6 * t578 + t244;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t582 = t196 * t26;
            let t583 = t475 * t257;
            let t584 = t583 * t220;
            let t592 = t211 * v_sigma2;
            let t596 = t323 * t92;
            let t597 = t498 * v_rho1;
            let t598 = f64x8::splat(1.0) / t597;
            let t605 = t498 * t200;
            let t607 = f64x8::splat(1.0) / t81 / t605;
            let t612 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t33 * t202 - f64x8::splat(0.010673131404562256) * t33 * t202 * t88 + f64x8::splat(0.001334141425570282) * t50 * t592 * t88 - f64x8::splat(0.00011117845213085684) * t596 * t598 * t88 - f64x8::splat(0.00014699017287336702) * t50 * t592 * t216 + f64x8::splat(1.2153437393255055e-08) * t337 * t496 * t607 * t513;
            let t613 = t199 * t612;
            let t617 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t470 * t258 - t571 + f64x8::splat(0.3310219072749264) * t582 * t584 - f64x8::splat(0.1655109536374632) * t196 * t613));
            let tv2rhosigma5 = t6 * t617 + t261;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t619 = t240 * t240;
            let t620 = t302 * t619;
            let t626 = t323 * v_sigma0;
            let t627 = f64x8::splat(1.0) / t326;
            let t635 = f64x8::splat(1.0) / t36 / t327;
            let t640 = -f64x8::splat(0.0003335353563925705) * t50 * t55 * t43 + f64x8::splat(4.169191954907131e-05) * t626 * t627 * t43 + f64x8::splat(2.7560657413756314e-05) * t50 * t55 * t148 - f64x8::splat(4.5575390224706455e-09) * t337 * t51 * t635 * t344;
            let t641 = t131 * t640;
            let t645 = ((t1).select(f64x8::splat(0.0), f64x8::splat(0.3310219072749264) * t128 * t620 - f64x8::splat(0.1655109536374632) * t128 * t641));
            let tv2sigma20 = t6 * t645;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t646 = t257 * t257;
            let t647 = t476 * t646;
            let t653 = t323 * v_sigma2;
            let t654 = f64x8::splat(1.0) / t498;
            let t662 = f64x8::splat(1.0) / t81 / t499;
            let t667 = -f64x8::splat(0.0003335353563925705) * t50 * t96 * t88 + f64x8::splat(4.169191954907131e-05) * t653 * t654 * t88 + f64x8::splat(2.7560657413756314e-05) * t50 * t96 * t216 - f64x8::splat(4.5575390224706455e-09) * t337 * t92 * t662 * t513;
            let t668 = t199 * t667;
            let t672 = ((t69).select(f64x8::splat(0.0), f64x8::splat(0.3310219072749264) * t196 * t647 - f64x8::splat(0.1655109536374632) * t196 * t668));
            let tv2sigma25 = t6 * t672;
            acc_v2sigma2_5 = tv2sigma25;
            let t676 = f64x8::splat(1.0) / t36 / t53;
            let t677 = v_sigma0 * t676;
            let t684 = f64x8::splat(1.0) / t35 / t326;
            let t685 = t51 * t684;
            let t689 = f64x8::splat(1.0) / t553;
            let t693 = t323 * t338;
            let t694 = t326 * t53;
            let t696 = f64x8::splat(1.0) / t36 / t694;
            let t698 = t33 * t43;
            let t708 = t338 * t51;
            let t709 = t326 * t326;
            let t711 = f64x8::splat(1.0) / t709 / t132;
            let t714 = f64x8::splat(1.0) / t343 / t59;
            let t717 = -f64x8::splat(1540.0) / f64x8::splat(6561.0) * t33 * t677 - f64x8::splat(0.18262913736695416) * t33 * t677 * t43 + f64x8::splat(0.0336994241569975) * t50 * t685 * t43 - f64x8::splat(0.00563304157463008) * t325 * t689 * t43 + f64x8::splat(3.2941763594327954e-05) * t693 * t696 * t698 - f64x8::splat(0.0034134384589481898) * t50 * t685 * t148 + f64x8::splat(6.157741612582561e-07) * t337 * t338 * t696 * t344 - f64x8::splat(3.012370804988963e-15) * t708 * t711 * t714;
            let t718 = t131 * t717;
            let t721 = t291 * t130;
            let t722 = t721 * t152;
            let t723 = t128 * t722;
            let t725 = t296 * t348;
            let t726 = t128 * t725;
            let t728 = t2 * t278;
            let t731 = t287 * t297;
            let t735 = t129 * t129;
            let t736 = f64x8::splat(1.0) / t735;
            let t737 = t26 * t736;
            let t738 = t303 * t152;
            let t739 = t737 * t738;
            let t743 = t301 * t152 * t348;
            let t747 = f64x8::splat(1.0) / t122 / t110;
            let t751 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t25 * t747 * t64;
            let t752 = t123 * t301;
            let t753 = t752 * t303;
            let t754 = t128 * t753;
            let t758 = t5 * t278 * t123 * t64;
            let t762 = t5 * t117 * t291 * t64;
            let t767 = f64x8::splat(1.0) / t264 / t19;
            let t768 = t266 * t114;
            let t771 = t265 * t114;
            let t774 = t110 * t110;
            let t775 = f64x8::splat(1.0) / t774;
            let t776 = t16 * t775;
            let t779 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t270 - f64x8::splat(6.0) * t776)));
            let t783 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t767 * t768 + f64x8::splat(4.0) / f64x8::splat(3.0) * t771 * t274 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t779));
            let t788 = -f64x8::splat(0.1655109536374632) * t128 * t718 + f64x8::splat(0.1103406357583088) * t723 - f64x8::splat(0.1655109536374632) * t726 - f64x8::splat(0.49653286091238963) * t728 * t153 - f64x8::splat(0.3310219072749264) * t731 - f64x8::splat(0.49653286091238963) * t287 * t349 - f64x8::splat(0.9930657218247793) * t128 * t739 + f64x8::splat(0.9930657218247793) * t530 * t743 - t751 + f64x8::splat(0.3310219072749264) * t754 - f64x8::splat(3.0) / f64x8::splat(8.0) * t758 + t762 / f64x8::splat(4.0) + f64x8::splat(0.9930657218247793) * t287 * t304 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t783 * t26 * t64;
            let t789 = ((t1).select(f64x8::splat(0.0), t788));
            let t791 = f64x8::splat(1.0) / t354 / t73;
            let t792 = t356 * t160;
            let t795 = t355 * t160;
            let t798 = t70 * t775;
            let t801 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t270 - f64x8::splat(6.0) * t798)));
            let t805 = ((t74).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t791 * t792 + f64x8::splat(4.0) / f64x8::splat(3.0) * t795 * t362 + f64x8::splat(4.0) / f64x8::splat(3.0) * t75 * t801));
            let t812 = t5 * t366 * t123 * t105;
            let t816 = t5 * t163 * t291 * t105;
            let t821 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t77 * t747 * t105;
            let t823 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t805 * t26 * t105 - f64x8::splat(3.0) / f64x8::splat(8.0) * t812 + t816 / f64x8::splat(4.0) - t821));
            let tv3rho30 = f64x8::splat(3.0) * t353 + f64x8::splat(3.0) * t380 + t6 * (t789 + t823);
            acc_v3rho3_0 = tv3rho30;
            let t826 = f64x8::splat(2.0) * t406;
            let t827 = f64x8::splat(2.0) * t433;
            let t828 = t767 * t177;
            let t831 = t265 * t387;
            let t836 = f64x8::splat(2.0) * t270;
            let t837 = f64x8::splat(6.0) * t776;
            let t839 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t836 - t837)));
            let t843 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t828 * t266 + f64x8::splat(8.0) / f64x8::splat(9.0) * t831 * t114 + f64x8::splat(4.0) / f64x8::splat(9.0) * t383 * t274 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t839));
            let t851 = t5 * t391 * t123 * t64 / f64x8::splat(4.0);
            let t852 = t2 * t391;
            let t857 = t5 * t180 * t291 * t64;
            let t860 = f64x8::splat(0.1103406357583088) * t400 * t297;
            let t871 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t843 * t26 * t64 - t851 - f64x8::splat(0.3310219072749264) * t852 * t153 + t857 / f64x8::splat(12.0) - t860 + f64x8::splat(0.3310219072749264) * t400 * t304 - f64x8::splat(0.1655109536374632) * t400 * t349 - t758 / f64x8::splat(8.0) + t762 / f64x8::splat(6.0) - f64x8::splat(0.1103406357583088) * t731 - t751 + f64x8::splat(0.07356042383887254) * t723 + f64x8::splat(0.1103406357583088) * t754 - f64x8::splat(0.0551703178791544) * t726;
            let t872 = ((t1).select(f64x8::splat(0.0), t871));
            let t873 = t791 * t188;
            let t876 = t355 * t411;
            let t881 = f64x8::splat(6.0) * t798;
            let t883 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t836 - t881)));
            let t887 = ((t74).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t873 * t356 + f64x8::splat(8.0) / f64x8::splat(9.0) * t876 * t160 + f64x8::splat(4.0) / f64x8::splat(9.0) * t407 * t362 + f64x8::splat(4.0) / f64x8::splat(3.0) * t75 * t883));
            let t895 = t5 * t415 * t123 * t105 / f64x8::splat(4.0);
            let t898 = t5 * t191 * t291 * t105;
            let t902 = t2 * t366;
            let t906 = f64x8::splat(0.1103406357583088) * t425 * t429;
            let t907 = t291 * t198;
            let t908 = t907 * t220;
            let t909 = t196 * t908;
            let t912 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t887 * t26 * t105 - t895 + t898 / f64x8::splat(12.0) - t812 / f64x8::splat(8.0) + t816 / f64x8::splat(6.0) - t821 - f64x8::splat(0.1655109536374632) * t902 * t221 - t906 + f64x8::splat(0.03678021191943627) * t909));
            let tv3rho31 = t353 + t380 + t826 + t827 + t6 * (t872 + t912);
            acc_v3rho3_1 = tv3rho31;
            let t915 = t767 * t438;
            let t920 = t265 * t443;
            let t924 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t836 - t837)));
            let t928 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t915 * t114 + f64x8::splat(8.0) / f64x8::splat(9.0) * t383 * t387 + f64x8::splat(4.0) / f64x8::splat(9.0) * t920 * t114 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t924));
            let t935 = t5 * t447 * t123 * t64;
            let t937 = t2 * t447;
            let t944 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t928 * t26 * t64 - t935 / f64x8::splat(8.0) - f64x8::splat(0.1655109536374632) * t937 * t153 - t851 + t857 / f64x8::splat(6.0) - t860 + t762 / f64x8::splat(12.0) - t751 + f64x8::splat(0.03678021191943627) * t723));
            let t945 = t791 * t455;
            let t950 = t355 * t460;
            let t954 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t836 - t881)));
            let t958 = ((t74).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t945 * t160 + f64x8::splat(8.0) / f64x8::splat(9.0) * t407 * t411 + f64x8::splat(4.0) / f64x8::splat(9.0) * t950 * t160 + f64x8::splat(4.0) / f64x8::splat(3.0) * t75 * t954));
            let t965 = t5 * t464 * t123 * t105;
            let t968 = t2 * t415;
            let t971 = t470 * t429;
            let t977 = t123 * t475;
            let t978 = t977 * t477;
            let t979 = t196 * t978;
            let t983 = t428 * t517;
            let t984 = t196 * t983;
            let t986 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t958 * t26 * t105 - t965 / f64x8::splat(8.0) - t895 + t898 / f64x8::splat(6.0) - f64x8::splat(0.3310219072749264) * t968 * t221 - f64x8::splat(0.1103406357583088) * t971 + t816 / f64x8::splat(12.0) - t821 - t906 + f64x8::splat(0.07356042383887254) * t909 + f64x8::splat(0.3310219072749264) * t425 * t478 + f64x8::splat(0.1103406357583088) * t979 - f64x8::splat(0.1655109536374632) * t425 * t518 - f64x8::splat(0.0551703178791544) * t984;
            let t987 = ((t69).select(f64x8::splat(0.0), t986));
            let tv3rho32 = t826 + t827 + t454 + t522 + t6 * (t944 + t987);
            acc_v3rho3_2 = tv3rho32;
            let t992 = t438 * t177;
            let t999 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t270 - f64x8::splat(6.0) * t776)));
            let t1003 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t767 * t992 + f64x8::splat(4.0) / f64x8::splat(3.0) * t383 * t443 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t999));
            let t1011 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1003 * t26 * t64 - f64x8::splat(3.0) / f64x8::splat(8.0) * t935 + t857 / f64x8::splat(4.0) - t751));
            let t1013 = f64x8::splat(1.0) / t81 / t94;
            let t1014 = v_sigma2 * t1013;
            let t1021 = f64x8::splat(1.0) / t80 / t498;
            let t1022 = t92 * t1021;
            let t1026 = f64x8::splat(1.0) / t605;
            let t1030 = t323 * t507;
            let t1031 = t498 * t94;
            let t1033 = f64x8::splat(1.0) / t81 / t1031;
            let t1035 = t33 * t88;
            let t1045 = t507 * t92;
            let t1046 = t498 * t498;
            let t1048 = f64x8::splat(1.0) / t1046 / t200;
            let t1051 = f64x8::splat(1.0) / t512 / t100;
            let t1054 = -f64x8::splat(1540.0) / f64x8::splat(6561.0) * t33 * t1014 - f64x8::splat(0.18262913736695416) * t33 * t1014 * t88 + f64x8::splat(0.0336994241569975) * t50 * t1022 * t88 - f64x8::splat(0.00563304157463008) * t497 * t1026 * t88 + f64x8::splat(3.2941763594327954e-05) * t1030 * t1033 * t1035 - f64x8::splat(0.0034134384589481898) * t50 * t1022 * t216 + f64x8::splat(6.157741612582561e-07) * t337 * t507 * t1033 * t513 - f64x8::splat(3.012370804988963e-15) * t1045 * t1048 * t1051;
            let t1055 = t199 * t1054;
            let t1058 = t2 * t464;
            let t1063 = t455 * t188;
            let t1070 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t270 - f64x8::splat(6.0) * t798)));
            let t1074 = ((t74).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t791 * t1063 + f64x8::splat(4.0) / f64x8::splat(3.0) * t407 * t460 + f64x8::splat(4.0) / f64x8::splat(3.0) * t75 * t1070));
            let t1079 = t197 * t197;
            let t1080 = f64x8::splat(1.0) / t1079;
            let t1081 = t26 * t1080;
            let t1082 = t477 * t220;
            let t1083 = t1081 * t1082;
            let t1087 = t475 * t220 * t517;
            let t1098 = -f64x8::splat(0.1655109536374632) * t196 * t1055 - f64x8::splat(0.49653286091238963) * t1058 * t221 - f64x8::splat(0.49653286091238963) * t470 * t518 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1074 * t26 * t105 - f64x8::splat(0.9930657218247793) * t196 * t1083 + f64x8::splat(0.9930657218247793) * t582 * t1087 + f64x8::splat(0.9930657218247793) * t470 * t478 - f64x8::splat(0.3310219072749264) * t971 + f64x8::splat(0.3310219072749264) * t979 - f64x8::splat(0.1655109536374632) * t984 - f64x8::splat(3.0) / f64x8::splat(8.0) * t965 + t898 / f64x8::splat(4.0) + f64x8::splat(0.1103406357583088) * t909 - t821;
            let t1099 = ((t69).select(f64x8::splat(0.0), t1098));
            let tv3rho33 = f64x8::splat(3.0) * t454 + f64x8::splat(3.0) * t522 + t6 * (t1011 + t1099);
            acc_v3rho3_3 = tv3rho33;
            let t1105 = t287 * t527;
            let t1107 = t287 * t26;
            let t1112 = t721 * t240;
            let t1114 = f64x8::splat(0.03678021191943627) * t128 * t1112;
            let t1115 = t128 * t123;
            let t1116 = t1115 * t532;
            let t1118 = t296 * t560;
            let t1119 = t128 * t1118;
            let t1121 = t736 * t240;
            let t1122 = t1121 * t303;
            let t1125 = t301 * t560;
            let t1126 = t1125 * t152;
            let t1129 = t531 * t348;
            let t1137 = t317 * v_sigma0;
            let t1141 = t323 * t328;
            let t1142 = t51 * t43;
            let t1155 = t338 * v_sigma0;
            let t1157 = f64x8::splat(1.0) / t709 / t34;
            let t1161 = f64x8::splat(110.0) / f64x8::splat(2187.0) * t33 * t308 + f64x8::splat(0.039134815150061605) * t33 * t308 * t43 - f64x8::splat(0.009635465851340926) * t50 * t1137 * t43 + f64x8::splat(0.0018900336862245663) * t1141 * t1142 - f64x8::splat(1.2353161347872983e-05) * t325 * t341 * t698 + f64x8::splat(0.0009309377615313244) * t50 * t1137 * t148 - f64x8::splat(2.0660843568533593e-07) * t337 * t324 * t341 * t344 + f64x8::splat(1.129639051870861e-15) * t1155 * t1157 * t714;
            let t1162 = t131 * t1161;
            let t1165 = -f64x8::splat(0.1655109536374632) * t728 * t241 - f64x8::splat(0.1103406357583088) * t1105 + f64x8::splat(0.6620438145498528) * t1107 * t532 - f64x8::splat(0.3310219072749264) * t287 * t561 + t1114 + f64x8::splat(0.2206812715166176) * t1116 - f64x8::splat(0.1103406357583088) * t1119 - f64x8::splat(0.9930657218247793) * t530 * t1122 + f64x8::splat(0.6620438145498528) * t530 * t1126 + f64x8::splat(0.3310219072749264) * t530 * t1129 - f64x8::splat(0.1655109536374632) * t128 * t1162;
            let t1166 = ((t1).select(f64x8::splat(0.0), t1165));
            let tv3rho2sigma0 = t6 * t1166 + f64x8::splat(2.0) * t565;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t1171 = t425 * t569;
            let t1173 = t907 * t257;
            let t1175 = f64x8::splat(0.03678021191943627) * t196 * t1173;
            let t1177 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t902 * t258 - f64x8::splat(0.1103406357583088) * t1171 + t1175));
            let tv3rho2sigma2 = t6 * t1177 + f64x8::splat(2.0) * t573;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t1181 = t400 * t527;
            let t1183 = t400 * t26;
            let t1192 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t852 * t241 - f64x8::splat(0.0551703178791544) * t1181 + f64x8::splat(0.3310219072749264) * t1183 * t532 - f64x8::splat(0.1655109536374632) * t400 * t561 - f64x8::splat(0.0551703178791544) * t1105 + t1114 + f64x8::splat(0.1103406357583088) * t1116 - f64x8::splat(0.0551703178791544) * t1119));
            let tv3rho2sigma3 = t6 * t1192 + t565 + t578;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1196 = t470 * t569;
            let t1199 = t425 * t26;
            let t1202 = t196 * t123;
            let t1203 = t1202 * t584;
            let t1207 = t428 * t612;
            let t1208 = t196 * t1207;
            let t1211 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t968 * t258 - f64x8::splat(0.0551703178791544) * t1196 - f64x8::splat(0.0551703178791544) * t1171 + t1175 + f64x8::splat(0.3310219072749264) * t1199 * t584 + f64x8::splat(0.1103406357583088) * t1203 - f64x8::splat(0.1655109536374632) * t425 * t613 - f64x8::splat(0.0551703178791544) * t1208));
            let tv3rho2sigma5 = t6 * t1211 + t573 + t617;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1218 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t937 * t241 - f64x8::splat(0.1103406357583088) * t1181 + t1114));
            let tv3rho2sigma6 = t6 * t1218 + f64x8::splat(2.0) * t578;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1224 = t470 * t26;
            let t1231 = t1080 * t257;
            let t1232 = t1231 * t477;
            let t1235 = t475 * t612;
            let t1236 = t1235 * t220;
            let t1239 = t583 * t517;
            let t1247 = t491 * v_sigma2;
            let t1251 = t323 * t500;
            let t1252 = t92 * t88;
            let t1265 = t507 * v_sigma2;
            let t1267 = f64x8::splat(1.0) / t1046 / t79;
            let t1271 = f64x8::splat(110.0) / f64x8::splat(2187.0) * t33 * t482 + f64x8::splat(0.039134815150061605) * t33 * t482 * t88 - f64x8::splat(0.009635465851340926) * t50 * t1247 * t88 + f64x8::splat(0.0018900336862245663) * t1251 * t1252 - f64x8::splat(1.2353161347872983e-05) * t497 * t510 * t1035 + f64x8::splat(0.0009309377615313244) * t50 * t1247 * t216 - f64x8::splat(2.0660843568533593e-07) * t337 * t496 * t510 * t513 + f64x8::splat(1.129639051870861e-15) * t1265 * t1267 * t1051;
            let t1272 = t199 * t1271;
            let t1275 = -f64x8::splat(0.1655109536374632) * t1058 * t258 - f64x8::splat(0.1103406357583088) * t1196 + f64x8::splat(0.6620438145498528) * t1224 * t584 - f64x8::splat(0.3310219072749264) * t470 * t613 + t1175 + f64x8::splat(0.2206812715166176) * t1203 - f64x8::splat(0.1103406357583088) * t1208 - f64x8::splat(0.9930657218247793) * t582 * t1232 + f64x8::splat(0.6620438145498528) * t582 * t1236 + f64x8::splat(0.3310219072749264) * t582 * t1239 - f64x8::splat(0.1655109536374632) * t196 * t1272;
            let t1276 = ((t69).select(f64x8::splat(0.0), t1275));
            let tv3rho2sigma8 = t6 * t1276 + f64x8::splat(2.0) * t617;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1280 = t752 * t619;
            let t1282 = f64x8::splat(0.1103406357583088) * t128 * t1280;
            let t1283 = t736 * t619;
            let t1284 = t1283 * t152;
            let t1287 = t531 * t560;
            let t1292 = t296 * t640;
            let t1294 = f64x8::splat(0.0551703178791544) * t128 * t1292;
            let t1295 = t301 * t640;
            let t1296 = t1295 * t152;
            let t1302 = t323 * t546;
            let t1303 = v_sigma0 * t43;
            let t1312 = t555 * t344;
            let t1316 = t709 * v_rho0;
            let t1317 = f64x8::splat(1.0) / t1316;
            let t1321 = f64x8::splat(0.0017788552340937095) * t50 * t143 * t43 - f64x8::splat(0.0005558922606542842) * t1302 * t1303 + f64x8::splat(4.632435505452368e-06) * t544 * t555 * t698 - f64x8::splat(0.00014699017287336702) * t50 * t143 * t148 + f64x8::splat(6.076718696627527e-08) * t337 * t1312 * t51 - f64x8::splat(4.236146444515729e-16) * t338 * t1317 * t714;
            let t1322 = t131 * t1321;
            let t1326 = ((t1).select(f64x8::splat(0.0), f64x8::splat(0.3310219072749264) * t287 * t620 + t1282 - f64x8::splat(0.9930657218247793) * t530 * t1284 + f64x8::splat(0.6620438145498528) * t530 * t1287 - f64x8::splat(0.1655109536374632) * t287 * t641 - t1294 + f64x8::splat(0.3310219072749264) * t530 * t1296 - f64x8::splat(0.1655109536374632) * t128 * t1322));
            let tv3rhosigma20 = t6 * t1326 + t645;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1330 = t977 * t646;
            let t1332 = f64x8::splat(0.1103406357583088) * t196 * t1330;
            let t1335 = t428 * t667;
            let t1337 = f64x8::splat(0.0551703178791544) * t196 * t1335;
            let t1339 = ((t69).select(f64x8::splat(0.0), f64x8::splat(0.3310219072749264) * t425 * t647 + t1332 - f64x8::splat(0.1655109536374632) * t425 * t668 - t1337));
            let tv3rhosigma25 = t6 * t1339 + t672;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1346 = ((t1).select(f64x8::splat(0.0), f64x8::splat(0.3310219072749264) * t400 * t620 + t1282 - f64x8::splat(0.1655109536374632) * t400 * t641 - t1294));
            let tv3rhosigma26 = t6 * t1346 + t645;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1350 = t1080 * t646;
            let t1351 = t1350 * t220;
            let t1354 = t583 * t612;
            let t1359 = t475 * t667;
            let t1360 = t1359 * t220;
            let t1366 = t323 * t598;
            let t1367 = v_sigma2 * t88;
            let t1376 = t607 * t513;
            let t1380 = t1046 * v_rho1;
            let t1381 = f64x8::splat(1.0) / t1380;
            let t1385 = f64x8::splat(0.0017788552340937095) * t50 * t211 * t88 - f64x8::splat(0.0005558922606542842) * t1366 * t1367 + f64x8::splat(4.632435505452368e-06) * t596 * t607 * t1035 - f64x8::splat(0.00014699017287336702) * t50 * t211 * t216 + f64x8::splat(6.076718696627527e-08) * t337 * t1376 * t92 - f64x8::splat(4.236146444515729e-16) * t507 * t1381 * t1051;
            let t1386 = t199 * t1385;
            let t1390 = ((t69).select(f64x8::splat(0.0), f64x8::splat(0.3310219072749264) * t470 * t647 + t1332 - f64x8::splat(0.9930657218247793) * t582 * t1351 + f64x8::splat(0.6620438145498528) * t582 * t1354 - f64x8::splat(0.1655109536374632) * t470 * t668 - t1337 + f64x8::splat(0.3310219072749264) * t582 * t1360 - f64x8::splat(0.1655109536374632) * t196 * t1386));
            let tv3rhosigma211 = t6 * t1390 + t672;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1392 = t619 * t240;
            let t1393 = t737 * t1392;
            let t1396 = t531 * t640;
            let t1405 = t635 * t344;
            let t1409 = f64x8::splat(1.0) / t709;
            let t1413 = f64x8::splat(0.00012507575864721394) * t323 * t627 * t43 - f64x8::splat(1.7371633145446381e-06) * t626 * t635 * t698 - f64x8::splat(1.3672617067411937e-08) * t337 * t1405 * v_sigma0 + f64x8::splat(1.5885549166933983e-16) * t324 * t1409 * t714;
            let t1414 = t131 * t1413;
            let t1418 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.9930657218247793) * t128 * t1393 + f64x8::splat(0.9930657218247793) * t530 * t1396 - f64x8::splat(0.1655109536374632) * t128 * t1414));
            let tv3sigma30 = t6 * t1418;
            acc_v3sigma3_0 = tv3sigma30;
            let tv3sigma31 = f64x8::splat(0.0);
            acc_v3sigma3_1 = tv3sigma31;
            let tv3sigma32 = f64x8::splat(0.0);
            acc_v3sigma3_2 = tv3sigma32;
            let tv3sigma33 = f64x8::splat(0.0);
            acc_v3sigma3_3 = tv3sigma33;
            let tv3sigma34 = f64x8::splat(0.0);
            acc_v3sigma3_4 = tv3sigma34;
            let tv3sigma35 = f64x8::splat(0.0);
            acc_v3sigma3_5 = tv3sigma35;
            let tv3sigma36 = f64x8::splat(0.0);
            acc_v3sigma3_6 = tv3sigma36;
            let tv3sigma37 = f64x8::splat(0.0);
            acc_v3sigma3_7 = tv3sigma37;
            let tv3sigma38 = f64x8::splat(0.0);
            acc_v3sigma3_8 = tv3sigma38;
            let t1419 = t646 * t257;
            let t1420 = t1081 * t1419;
            let t1423 = t583 * t667;
            let t1432 = t662 * t513;
            let t1436 = f64x8::splat(1.0) / t1046;
            let t1440 = f64x8::splat(0.00012507575864721394) * t323 * t654 * t88 - f64x8::splat(1.7371633145446381e-06) * t653 * t662 * t1035 - f64x8::splat(1.3672617067411937e-08) * t337 * t1432 * v_sigma2 + f64x8::splat(1.5885549166933983e-16) * t496 * t1436 * t1051;
            let t1441 = t199 * t1440;
            let t1445 = ((t69).select(f64x8::splat(0.0), -f64x8::splat(0.9930657218247793) * t196 * t1420 + f64x8::splat(0.9930657218247793) * t582 * t1423 - f64x8::splat(0.1655109536374632) * t196 * t1441));
            let tv3sigma39 = t6 * t1445;
            acc_v3sigma3_9 = tv3sigma39;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v2rhosigma, ip, m, 6, 0, acc_v2rhosigma_0);
        store_strided(v2rhosigma, ip, m, 6, 1, acc_v2rhosigma_1);
        store_strided(v2rhosigma, ip, m, 6, 2, acc_v2rhosigma_2);
        store_strided(v2rhosigma, ip, m, 6, 3, acc_v2rhosigma_3);
        store_strided(v2rhosigma, ip, m, 6, 4, acc_v2rhosigma_4);
        store_strided(v2rhosigma, ip, m, 6, 5, acc_v2rhosigma_5);
        store_strided(v2sigma2, ip, m, 6, 0, acc_v2sigma2_0);
        store_strided(v2sigma2, ip, m, 6, 1, acc_v2sigma2_1);
        store_strided(v2sigma2, ip, m, 6, 2, acc_v2sigma2_2);
        store_strided(v2sigma2, ip, m, 6, 3, acc_v2sigma2_3);
        store_strided(v2sigma2, ip, m, 6, 4, acc_v2sigma2_4);
        store_strided(v2sigma2, ip, m, 6, 5, acc_v2sigma2_5);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v3rho2sigma, ip, m, 9, 0, acc_v3rho2sigma_0);
        store_strided(v3rho2sigma, ip, m, 9, 1, acc_v3rho2sigma_1);
        store_strided(v3rho2sigma, ip, m, 9, 2, acc_v3rho2sigma_2);
        store_strided(v3rho2sigma, ip, m, 9, 3, acc_v3rho2sigma_3);
        store_strided(v3rho2sigma, ip, m, 9, 4, acc_v3rho2sigma_4);
        store_strided(v3rho2sigma, ip, m, 9, 5, acc_v3rho2sigma_5);
        store_strided(v3rho2sigma, ip, m, 9, 6, acc_v3rho2sigma_6);
        store_strided(v3rho2sigma, ip, m, 9, 7, acc_v3rho2sigma_7);
        store_strided(v3rho2sigma, ip, m, 9, 8, acc_v3rho2sigma_8);
        store_strided(v3rhosigma2, ip, m, 12, 0, acc_v3rhosigma2_0);
        store_strided(v3rhosigma2, ip, m, 12, 1, acc_v3rhosigma2_1);
        store_strided(v3rhosigma2, ip, m, 12, 2, acc_v3rhosigma2_2);
        store_strided(v3rhosigma2, ip, m, 12, 3, acc_v3rhosigma2_3);
        store_strided(v3rhosigma2, ip, m, 12, 4, acc_v3rhosigma2_4);
        store_strided(v3rhosigma2, ip, m, 12, 5, acc_v3rhosigma2_5);
        store_strided(v3rhosigma2, ip, m, 12, 6, acc_v3rhosigma2_6);
        store_strided(v3rhosigma2, ip, m, 12, 7, acc_v3rhosigma2_7);
        store_strided(v3rhosigma2, ip, m, 12, 8, acc_v3rhosigma2_8);
        store_strided(v3rhosigma2, ip, m, 12, 9, acc_v3rhosigma2_9);
        store_strided(v3rhosigma2, ip, m, 12, 10, acc_v3rhosigma2_10);
        store_strided(v3rhosigma2, ip, m, 12, 11, acc_v3rhosigma2_11);
        store_strided(v3sigma3, ip, m, 10, 0, acc_v3sigma3_0);
        store_strided(v3sigma3, ip, m, 10, 1, acc_v3sigma3_1);
        store_strided(v3sigma3, ip, m, 10, 2, acc_v3sigma3_2);
        store_strided(v3sigma3, ip, m, 10, 3, acc_v3sigma3_3);
        store_strided(v3sigma3, ip, m, 10, 4, acc_v3sigma3_4);
        store_strided(v3sigma3, ip, m, 10, 5, acc_v3sigma3_5);
        store_strided(v3sigma3, ip, m, 10, 6, acc_v3sigma3_6);
        store_strided(v3sigma3, ip, m, 10, 7, acc_v3sigma3_7);
        store_strided(v3sigma3, ip, m, 10, 8, acc_v3sigma3_8);
        store_strided(v3sigma3, ip, m, 10, 9, acc_v3sigma3_9);
        ip += 8;
    }
}
