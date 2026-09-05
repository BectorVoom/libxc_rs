//! GGA_X_ITYH vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh.c`
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
pub fn gga_x_ityh_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
            let t26 = t5 * t25;
            let t27 = (simd::cbrt(t6));
            let t28 = t2 * t2;
            let t29 = f64x8::splat(M_PI) * t28;
            let t30 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = f64x8::splat(M_CBRT4);
            let t34 = t32 * t33;
            let t35 = t28 * t32;
            let t36 = t35 * t33;
            let t37 = v_rho0 * v_rho0;
            let t38 = (simd::cbrt(v_rho0));
            let t39 = t38 * t38;
            let t41 = f64x8::splat(1.0) / t39 / t37;
            let t42 = v_sigma0 * t41;
            let t43 = ((v_sigma0).sqrt());
            let t45 = f64x8::splat(1.0) / t38 / v_rho0;
            let t46 = t43 * t45;
            let t47 = (simd::ln(t46 + ((t46 * t46 + f64x8::splat(1.0)).sqrt())));
            let t50 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t46 * t47;
            let t51 = f64x8::splat(1.0) / t50;
            let t55 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t36 * t42 * t51;
            let t58 = t29 * t34 / t55;
            let t59 = ((t58).sqrt());
            let t61 = param_hyb_omega_0 / t59;
            let t62 = f64x8::splat(M_CBRT2);
            let t63 = t19 * t6;
            let t64 = (simd::cbrt(t63));
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t62 * t65;
            let t68 = t61 * t66 / f64x8::splat(2.0);
            let t69 = (f64x8::splat(1.35)).simd_le(t68);
            let t70 = (f64x8::splat(1.35)).simd_lt(t68);
            let t71 = ((t70).select(t68, f64x8::splat(1.35)));
            let t72 = t71 * t71;
            let t75 = t72 * t72;
            let t76 = f64x8::splat(1.0) / t75;
            let t78 = t75 * t72;
            let t79 = f64x8::splat(1.0) / t78;
            let t81 = t75 * t75;
            let t82 = f64x8::splat(1.0) / t81;
            let t85 = f64x8::splat(1.0) / t81 / t72;
            let t88 = f64x8::splat(1.0) / t81 / t75;
            let t91 = f64x8::splat(1.0) / t81 / t78;
            let t93 = t81 * t81;
            let t94 = f64x8::splat(1.0) / t93;
            let t97 = ((t70).select(f64x8::splat(1.35), t68));
            let t98 = ((f64x8::splat(M_PI)).sqrt());
            let t99 = f64x8::splat(1.0) / t97;
            let t101 = (simd::erf(t99 / f64x8::splat(2.0)));
            let t103 = t97 * t97;
            let t104 = f64x8::splat(1.0) / t103;
            let t106 = (simd::exp(-t104 / f64x8::splat(4.0)));
            let t107 = t106 - f64x8::splat(1.0);
            let t110 = t106 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t103 * t107;
            let t113 = t98 * t101 + f64x8::splat(2.0) * t97 * t110;
            let t117 = ((t69).select(f64x8::splat(1.0) / t72 / f64x8::splat(36.0) - t76 / f64x8::splat(960.0) + t79 / f64x8::splat(26880.0) - t82 / f64x8::splat(829440.0) + t85 / f64x8::splat(28385280.0) - t88 / f64x8::splat(1073479680.0) + t91 / f64x8::splat(44590694400.0) - t94 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t113));
            let t118 = t27 * t117;
            let t119 = t118 * t55;
            let t122 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t119));
            let t123 = (v_rho1).simd_le(dens_threshold);
            let t124 = -t16;
            let t126 = ((t14).select(t11, (t10).select(t15, t124 * t7)));
            let t127 = f64x8::splat(1.0) + t126;
            let t128 = (t127).simd_le(zeta_threshold);
            let t129 = (simd::cbrt(t127));
            let t131 = ((t128).select(t22, t129 * t127));
            let t132 = t5 * t131;
            let t133 = v_rho1 * v_rho1;
            let t134 = (simd::cbrt(v_rho1));
            let t135 = t134 * t134;
            let t137 = f64x8::splat(1.0) / t135 / t133;
            let t138 = v_sigma2 * t137;
            let t139 = ((v_sigma2).sqrt());
            let t141 = f64x8::splat(1.0) / t134 / v_rho1;
            let t142 = t139 * t141;
            let t143 = (simd::ln(t142 + ((t142 * t142 + f64x8::splat(1.0)).sqrt())));
            let t146 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t142 * t143;
            let t147 = f64x8::splat(1.0) / t146;
            let t151 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t36 * t138 * t147;
            let t154 = t29 * t34 / t151;
            let t155 = ((t154).sqrt());
            let t157 = param_hyb_omega_0 / t155;
            let t158 = t127 * t6;
            let t159 = (simd::cbrt(t158));
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = t62 * t160;
            let t163 = t157 * t161 / f64x8::splat(2.0);
            let t164 = (f64x8::splat(1.35)).simd_le(t163);
            let t165 = (f64x8::splat(1.35)).simd_lt(t163);
            let t166 = ((t165).select(t163, f64x8::splat(1.35)));
            let t167 = t166 * t166;
            let t170 = t167 * t167;
            let t171 = f64x8::splat(1.0) / t170;
            let t173 = t170 * t167;
            let t174 = f64x8::splat(1.0) / t173;
            let t176 = t170 * t170;
            let t177 = f64x8::splat(1.0) / t176;
            let t180 = f64x8::splat(1.0) / t176 / t167;
            let t183 = f64x8::splat(1.0) / t176 / t170;
            let t186 = f64x8::splat(1.0) / t176 / t173;
            let t188 = t176 * t176;
            let t189 = f64x8::splat(1.0) / t188;
            let t192 = ((t165).select(f64x8::splat(1.35), t163));
            let t193 = f64x8::splat(1.0) / t192;
            let t195 = (simd::erf(t193 / f64x8::splat(2.0)));
            let t197 = t192 * t192;
            let t198 = f64x8::splat(1.0) / t197;
            let t200 = (simd::exp(-t198 / f64x8::splat(4.0)));
            let t201 = t200 - f64x8::splat(1.0);
            let t204 = t200 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t197 * t201;
            let t207 = f64x8::splat(2.0) * t192 * t204 + t98 * t195;
            let t211 = ((t164).select(f64x8::splat(1.0) / t167 / f64x8::splat(36.0) - t171 / f64x8::splat(960.0) + t174 / f64x8::splat(26880.0) - t177 / f64x8::splat(829440.0) + t180 / f64x8::splat(28385280.0) - t183 / f64x8::splat(1073479680.0) + t186 / f64x8::splat(44590694400.0) - t189 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t192 * t207));
            let t212 = t27 * t211;
            let t213 = t212 * t151;
            let t216 = ((t123).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t132 * t213));
            let tzk0 = t122 + t216;
            acc_zk = tzk0;
            let t217 = t6 * t6;
            let t218 = f64x8::splat(1.0) / t217;
            let t219 = t16 * t218;
            let t221 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t219)));
            let t224 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t221));
            let t225 = t5 * t224;
            let t228 = t27 * t27;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t229 * t117;
            let t231 = t230 * t55;
            let t233 = t26 * t231 / f64x8::splat(8.0);
            let t234 = t72 * t71;
            let t235 = f64x8::splat(1.0) / t234;
            let t238 = param_hyb_omega_0 / t59 / t58;
            let t240 = t238 * t66 * f64x8::splat(M_PI);
            let t241 = t55 * t55;
            let t242 = f64x8::splat(1.0) / t241;
            let t243 = t33 * t242;
            let t244 = t37 * v_rho0;
            let t246 = f64x8::splat(1.0) / t39 / t244;
            let t247 = v_sigma0 * t246;
            let t251 = t50 * t50;
            let t252 = f64x8::splat(1.0) / t251;
            let t254 = f64x8::splat(1.0) / t38 / t37;
            let t258 = t42 + f64x8::splat(1.0);
            let t259 = ((t258).sqrt());
            let t260 = f64x8::splat(1.0) / t259;
            let t263 = -f64x8::splat(0.0336) * t43 * t254 * t47 - f64x8::splat(0.0336) * t247 * t260;
            let t264 = t252 * t263;
            let t268 = -f64x8::splat(0.002488888888888889) * t36 * t247 * t51 - f64x8::splat(0.0009333333333333333) * t36 * t42 * t264;
            let t269 = t243 * t268;
            let t274 = f64x8::splat(1.0) / t64 / t63;
            let t275 = t62 * t274;
            let t277 = t221 * t6 + t18 + f64x8::splat(1.0);
            let t281 = t240 * t35 * t269 / f64x8::splat(4.0) - t61 * t275 * t277 / f64x8::splat(6.0);
            let t282 = ((t70).select(t281, f64x8::splat(0.0)));
            let t285 = t75 * t71;
            let t286 = f64x8::splat(1.0) / t285;
            let t289 = t75 * t234;
            let t290 = f64x8::splat(1.0) / t289;
            let t294 = f64x8::splat(1.0) / t81 / t71;
            let t298 = f64x8::splat(1.0) / t81 / t234;
            let t302 = f64x8::splat(1.0) / t81 / t285;
            let t306 = f64x8::splat(1.0) / t81 / t289;
            let t310 = f64x8::splat(1.0) / t93 / t71;
            let t314 = ((t70).select(f64x8::splat(0.0), t281));
            let t316 = t106 * t104;
            let t320 = t103 * t97;
            let t321 = f64x8::splat(1.0) / t320;
            let t325 = t97 * t107;
            let t330 = t321 * t314 * t106 / f64x8::splat(2.0) - f64x8::splat(4.0) * t325 * t314 - t99 * t314 * t106;
            let t333 = f64x8::splat(2.0) * t314 * t110 - t316 * t314 + f64x8::splat(2.0) * t97 * t330;
            let t337 = ((t69).select(-t235 * t282 / f64x8::splat(18.0) + t286 * t282 / f64x8::splat(240.0) - t290 * t282 / f64x8::splat(4480.0) + t294 * t282 / f64x8::splat(103680.0) - t298 * t282 / f64x8::splat(2838528.0) + t302 * t282 / f64x8::splat(89456640.0) - t306 * t282 / f64x8::splat(3185049600.0) + t310 * t282 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t314 * t113 - f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t333));
            let t338 = t27 * t337;
            let t339 = t338 * t55;
            let t342 = t118 * t268;
            let t346 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t225 * t119 - t233 - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t339 - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t342));
            let t347 = t124 * t218;
            let t349 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t347)));
            let t352 = ((t128).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t129 * t349));
            let t353 = t5 * t352;
            let t356 = t229 * t211;
            let t357 = t356 * t151;
            let t359 = t132 * t357 / f64x8::splat(8.0);
            let t360 = t167 * t166;
            let t361 = f64x8::splat(1.0) / t360;
            let t363 = f64x8::splat(1.0) / t159 / t158;
            let t364 = t62 * t363;
            let t366 = t349 * t6 + t126 + f64x8::splat(1.0);
            let t369 = t157 * t364 * t366 / f64x8::splat(6.0);
            let t370 = ((t165).select(-t369, f64x8::splat(0.0)));
            let t373 = t170 * t166;
            let t374 = f64x8::splat(1.0) / t373;
            let t377 = t170 * t360;
            let t378 = f64x8::splat(1.0) / t377;
            let t382 = f64x8::splat(1.0) / t176 / t166;
            let t386 = f64x8::splat(1.0) / t176 / t360;
            let t390 = f64x8::splat(1.0) / t176 / t373;
            let t394 = f64x8::splat(1.0) / t176 / t377;
            let t398 = f64x8::splat(1.0) / t188 / t166;
            let t402 = ((t165).select(f64x8::splat(0.0), -t369));
            let t404 = t200 * t198;
            let t408 = t197 * t192;
            let t409 = f64x8::splat(1.0) / t408;
            let t413 = t192 * t201;
            let t418 = t409 * t402 * t200 / f64x8::splat(2.0) - f64x8::splat(4.0) * t413 * t402 - t193 * t402 * t200;
            let t421 = f64x8::splat(2.0) * t192 * t418 + f64x8::splat(2.0) * t402 * t204 - t404 * t402;
            let t425 = ((t164).select(-t361 * t370 / f64x8::splat(18.0) + t374 * t370 / f64x8::splat(240.0) - t378 * t370 / f64x8::splat(4480.0) + t382 * t370 / f64x8::splat(103680.0) - t386 * t370 / f64x8::splat(2838528.0) + t390 * t370 / f64x8::splat(89456640.0) - t394 * t370 / f64x8::splat(3185049600.0) + t398 * t370 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t192 * t421 - f64x8::splat(8.0) / f64x8::splat(3.0) * t402 * t207));
            let t426 = t27 * t425;
            let t427 = t426 * t151;
            let t431 = ((t123).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t353 * t213 - t359 - f64x8::splat(3.0) / f64x8::splat(8.0) * t132 * t427));
            let tvrho0 = t122 + t216 + t6 * (t346 + t431);
            acc_vrho_0 = tvrho0;
            let t435 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t219)));
            let t438 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t435));
            let t439 = t5 * t438;
            let t443 = t435 * t6 + t18 + f64x8::splat(1.0);
            let t444 = t275 * t443;
            let t446 = t61 * t444 / f64x8::splat(6.0);
            let t447 = ((t70).select(-t446, f64x8::splat(0.0)));
            let t450 = t286 * t447;
            let t452 = t290 * t447;
            let t454 = t294 * t447;
            let t456 = t298 * t447;
            let t458 = t302 * t447;
            let t460 = t306 * t447;
            let t462 = t310 * t447;
            let t465 = ((t70).select(f64x8::splat(0.0), -t446));
            let t477 = t321 * t465 * t106 / f64x8::splat(2.0) - f64x8::splat(4.0) * t325 * t465 - t99 * t465 * t106;
            let t480 = f64x8::splat(2.0) * t465 * t110 - t316 * t465 + f64x8::splat(2.0) * t97 * t477;
            let t484 = ((t69).select(-t235 * t447 / f64x8::splat(18.0) + t450 / f64x8::splat(240.0) - t452 / f64x8::splat(4480.0) + t454 / f64x8::splat(103680.0) - t456 / f64x8::splat(2838528.0) + t458 / f64x8::splat(89456640.0) - t460 / f64x8::splat(3185049600.0) + t462 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t465 * t113 - f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t480));
            let t485 = t27 * t484;
            let t486 = t485 * t55;
            let t490 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t439 * t119 - t233 - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t486));
            let t492 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t347)));
            let t495 = ((t128).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t129 * t492));
            let t496 = t5 * t495;
            let t501 = param_hyb_omega_0 / t155 / t154;
            let t503 = t501 * t161 * f64x8::splat(M_PI);
            let t504 = t151 * t151;
            let t505 = f64x8::splat(1.0) / t504;
            let t506 = t33 * t505;
            let t507 = t133 * v_rho1;
            let t509 = f64x8::splat(1.0) / t135 / t507;
            let t510 = v_sigma2 * t509;
            let t514 = t146 * t146;
            let t515 = f64x8::splat(1.0) / t514;
            let t517 = f64x8::splat(1.0) / t134 / t133;
            let t521 = t138 + f64x8::splat(1.0);
            let t522 = ((t521).sqrt());
            let t523 = f64x8::splat(1.0) / t522;
            let t526 = -f64x8::splat(0.0336) * t139 * t517 * t143 - f64x8::splat(0.0336) * t510 * t523;
            let t527 = t515 * t526;
            let t531 = -f64x8::splat(0.002488888888888889) * t36 * t510 * t147 - f64x8::splat(0.0009333333333333333) * t36 * t138 * t527;
            let t537 = t492 * t6 + t126 + f64x8::splat(1.0);
            let t541 = t503 * t35 * t506 * t531 / f64x8::splat(4.0) - t157 * t364 * t537 / f64x8::splat(6.0);
            let t542 = ((t165).select(t541, f64x8::splat(0.0)));
            let t545 = t374 * t542;
            let t547 = t378 * t542;
            let t549 = t382 * t542;
            let t551 = t386 * t542;
            let t553 = t390 * t542;
            let t555 = t394 * t542;
            let t557 = t398 * t542;
            let t560 = ((t165).select(f64x8::splat(0.0), t541));
            let t572 = t409 * t560 * t200 / f64x8::splat(2.0) - f64x8::splat(4.0) * t413 * t560 - t193 * t560 * t200;
            let t575 = f64x8::splat(2.0) * t192 * t572 + f64x8::splat(2.0) * t560 * t204 - t404 * t560;
            let t579 = ((t164).select(-t361 * t542 / f64x8::splat(18.0) + t545 / f64x8::splat(240.0) - t547 / f64x8::splat(4480.0) + t549 / f64x8::splat(103680.0) - t551 / f64x8::splat(2838528.0) + t553 / f64x8::splat(89456640.0) - t555 / f64x8::splat(3185049600.0) + t557 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t192 * t575 - f64x8::splat(8.0) / f64x8::splat(3.0) * t560 * t207));
            let t580 = t27 * t579;
            let t581 = t580 * t151;
            let t584 = t212 * t531;
            let t588 = ((t123).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t496 * t213 - t359 - f64x8::splat(3.0) / f64x8::splat(8.0) * t132 * t581 - f64x8::splat(3.0) / f64x8::splat(8.0) * t132 * t584));
            let tvrho1 = t122 + t216 + t6 * (t490 + t588);
            acc_vrho_1 = tvrho1;
            let t595 = f64x8::splat(1.0) / t43;
            let t601 = f64x8::splat(0.0126) * t595 * t45 * t47 + f64x8::splat(0.0126) * t41 * t260;
            let t602 = t252 * t601;
            let t606 = f64x8::splat(0.0009333333333333333) * t35 * t33 * t41 * t51 - f64x8::splat(0.0009333333333333333) * t36 * t42 * t602;
            let t610 = t240 * t35 * t243 * t606 / f64x8::splat(4.0);
            let t611 = ((t70).select(t610, f64x8::splat(0.0)));
            let t614 = t286 * t611;
            let t616 = t290 * t611;
            let t618 = t294 * t611;
            let t620 = t298 * t611;
            let t622 = t302 * t611;
            let t624 = t306 * t611;
            let t626 = t310 * t611;
            let t629 = ((t70).select(f64x8::splat(0.0), t610));
            let t641 = t321 * t629 * t106 / f64x8::splat(2.0) - f64x8::splat(4.0) * t325 * t629 - t99 * t629 * t106;
            let t644 = f64x8::splat(2.0) * t629 * t110 - t316 * t629 + f64x8::splat(2.0) * t97 * t641;
            let t648 = ((t69).select(-t235 * t611 / f64x8::splat(18.0) + t614 / f64x8::splat(240.0) - t616 / f64x8::splat(4480.0) + t618 / f64x8::splat(103680.0) - t620 / f64x8::splat(2838528.0) + t622 / f64x8::splat(89456640.0) - t624 / f64x8::splat(3185049600.0) + t626 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t629 * t113 - f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t644));
            let t649 = t27 * t648;
            let t650 = t649 * t55;
            let t652 = t118 * t606;
            let t656 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t650 - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t652));
            let tvsigma0 = t6 * t656;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t661 = f64x8::splat(1.0) / t139;
            let t667 = f64x8::splat(0.0126) * t661 * t141 * t143 + f64x8::splat(0.0126) * t137 * t523;
            let t668 = t515 * t667;
            let t672 = f64x8::splat(0.0009333333333333333) * t35 * t33 * t137 * t147 - f64x8::splat(0.0009333333333333333) * t36 * t138 * t668;
            let t676 = t503 * t35 * t506 * t672 / f64x8::splat(4.0);
            let t677 = ((t165).select(t676, f64x8::splat(0.0)));
            let t680 = t374 * t677;
            let t682 = t378 * t677;
            let t684 = t382 * t677;
            let t686 = t386 * t677;
            let t688 = t390 * t677;
            let t690 = t394 * t677;
            let t692 = t398 * t677;
            let t695 = ((t165).select(f64x8::splat(0.0), t676));
            let t707 = t409 * t695 * t200 / f64x8::splat(2.0) - f64x8::splat(4.0) * t413 * t695 - t193 * t695 * t200;
            let t710 = f64x8::splat(2.0) * t192 * t707 + f64x8::splat(2.0) * t695 * t204 - t404 * t695;
            let t714 = ((t164).select(-t361 * t677 / f64x8::splat(18.0) + t680 / f64x8::splat(240.0) - t682 / f64x8::splat(4480.0) + t684 / f64x8::splat(103680.0) - t686 / f64x8::splat(2838528.0) + t688 / f64x8::splat(89456640.0) - t690 / f64x8::splat(3185049600.0) + t692 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t192 * t710 - f64x8::splat(8.0) / f64x8::splat(3.0) * t695 * t207));
            let t715 = t27 * t714;
            let t716 = t715 * t151;
            let t718 = t212 * t672;
            let t722 = ((t123).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t132 * t716 - f64x8::splat(3.0) / f64x8::splat(8.0) * t132 * t718));
            let tvsigma2 = t6 * t722;
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
