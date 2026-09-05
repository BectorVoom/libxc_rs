//! GGA_X_CAP kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_cap.c`
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
pub fn gga_x_cap_kxc_pol(
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
    param_alphaoAx: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alphaoAx = f64x8::splat(param_alphaoAx);
    let param_c = f64x8::splat(param_c);
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
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = t28 * t28;
            let t31 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t32 = (simd::cbrt(t31));
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = param_alphaoAx * t29 * t33;
            let t35 = ((v_sigma0).sqrt());
            let t36 = (simd::cbrt(v_rho0));
            let t38 = f64x8::splat(1.0) / t36 / v_rho0;
            let t39 = t35 * t38;
            let t40 = t29 * t33;
            let t43 = f64x8::splat(1.0) + t40 * t39 / f64x8::splat(12.0);
            let t44 = (simd::ln(t43));
            let t46 = param_c * t44 + f64x8::splat(1.0);
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t44 * t47;
            let t52 = f64x8::splat(1.0) - t34 * t39 * t48 / f64x8::splat(12.0);
            let t56 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t52));
            let t57 = (v_rho1).simd_le(dens_threshold);
            let t58 = -t16;
            let t60 = ((t14).select(t11, (t10).select(t15, t58 * t7)));
            let t61 = f64x8::splat(1.0) + t60;
            let t62 = (t61).simd_le(zeta_threshold);
            let t63 = (simd::cbrt(t61));
            let t65 = ((t62).select(t22, t63 * t61));
            let t66 = t65 * t26;
            let t67 = ((v_sigma2).sqrt());
            let t68 = (simd::cbrt(v_rho1));
            let t70 = f64x8::splat(1.0) / t68 / v_rho1;
            let t71 = t67 * t70;
            let t74 = f64x8::splat(1.0) + t40 * t71 / f64x8::splat(12.0);
            let t75 = (simd::ln(t74));
            let t77 = param_c * t75 + f64x8::splat(1.0);
            let t78 = f64x8::splat(1.0) / t77;
            let t79 = t75 * t78;
            let t83 = f64x8::splat(1.0) - t34 * t71 * t79 / f64x8::splat(12.0);
            let t87 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t83));
            let tzk0 = t56 + t87;
            acc_zk = tzk0;
            let t88 = t6 * t6;
            let t89 = f64x8::splat(1.0) / t88;
            let t90 = t16 * t89;
            let t92 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t90)));
            let t95 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t92));
            let t96 = t95 * t26;
            let t100 = t26 * t26;
            let t101 = f64x8::splat(1.0) / t100;
            let t102 = t25 * t101;
            let t105 = t5 * t102 * t52 / f64x8::splat(8.0);
            let t106 = v_rho0 * v_rho0;
            let t108 = f64x8::splat(1.0) / t36 / t106;
            let t113 = param_alphaoAx * t28;
            let t114 = t32 * t32;
            let t115 = f64x8::splat(1.0) / t114;
            let t116 = t113 * t115;
            let t117 = t106 * v_rho0;
            let t118 = t36 * t36;
            let t120 = f64x8::splat(1.0) / t118 / t117;
            let t122 = f64x8::splat(1.0) / t43;
            let t123 = t122 * t47;
            let t128 = t113 * t115 * v_sigma0;
            let t130 = t46 * t46;
            let t131 = f64x8::splat(1.0) / t130;
            let t132 = t131 * param_c;
            let t133 = t132 * t122;
            let t134 = t120 * t44 * t133;
            let t137 = t34 * t35 * t108 * t48 / f64x8::splat(9.0) + t116 * v_sigma0 * t120 * t123 / f64x8::splat(18.0) - t128 * t134 / f64x8::splat(18.0);
            let t142 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t96 * t52 - t105 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t137));
            let t143 = t58 * t89;
            let t145 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t143)));
            let t148 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t145));
            let t149 = t148 * t26;
            let t153 = t65 * t101;
            let t156 = t5 * t153 * t83 / f64x8::splat(8.0);
            let t158 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t149 * t83 - t156));
            let tvrho0 = t56 + t87 + t6 * (t142 + t158);
            acc_vrho_0 = tvrho0;
            let t162 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t90)));
            let t165 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t162));
            let t166 = t165 * t26;
            let t171 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t166 * t52 - t105));
            let t173 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t143)));
            let t176 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t173));
            let t177 = t176 * t26;
            let t181 = v_rho1 * v_rho1;
            let t183 = f64x8::splat(1.0) / t68 / t181;
            let t188 = t181 * v_rho1;
            let t189 = t68 * t68;
            let t191 = f64x8::splat(1.0) / t189 / t188;
            let t193 = f64x8::splat(1.0) / t74;
            let t194 = t193 * t78;
            let t199 = t113 * t115 * v_sigma2;
            let t201 = t77 * t77;
            let t202 = f64x8::splat(1.0) / t201;
            let t203 = t202 * param_c;
            let t204 = t203 * t193;
            let t205 = t191 * t75 * t204;
            let t208 = t34 * t67 * t183 * t79 / f64x8::splat(9.0) + t116 * v_sigma2 * t191 * t194 / f64x8::splat(18.0) - t199 * t205 / f64x8::splat(18.0);
            let t213 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t177 * t83 - t156 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t208));
            let tvrho1 = t56 + t87 + t6 * (t171 + t213);
            acc_vrho_1 = tvrho1;
            let t216 = f64x8::splat(1.0) / t35;
            let t222 = f64x8::splat(1.0) / t118 / t106;
            let t229 = t44 * t131;
            let t231 = t229 * param_c * t122;
            let t234 = -t34 * t216 * t38 * t48 / f64x8::splat(24.0) - t116 * t222 * t122 * t47 / f64x8::splat(48.0) + t113 * t115 * t222 * t231 / f64x8::splat(48.0);
            let t238 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t234));
            let tvsigma0 = t6 * t238;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t239 = f64x8::splat(1.0) / t67;
            let t245 = f64x8::splat(1.0) / t189 / t181;
            let t252 = t75 * t202;
            let t254 = t252 * param_c * t193;
            let t257 = -t34 * t239 * t70 * t79 / f64x8::splat(24.0) - t116 * t245 * t193 * t78 / f64x8::splat(48.0) + t113 * t115 * t245 * t254 / f64x8::splat(48.0);
            let t261 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t257));
            let tvsigma2 = t6 * t261;
            acc_vsigma_2 = tvsigma2;
            let t264 = t23 * t23;
            let t265 = f64x8::splat(1.0) / t264;
            let t266 = t92 * t92;
            let t269 = t88 * t6;
            let t270 = f64x8::splat(1.0) / t269;
            let t271 = t16 * t270;
            let t274 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t89 + f64x8::splat(2.0) * t271)));
            let t278 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t265 * t266 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t274));
            let t279 = t278 * t26;
            let t283 = t95 * t101;
            let t285 = t5 * t283 * t52;
            let t291 = f64x8::splat(1.0) / t100 / t6;
            let t292 = t25 * t291;
            let t295 = t5 * t292 * t52 / f64x8::splat(12.0);
            let t297 = t5 * t102 * t137;
            let t300 = f64x8::splat(1.0) / t36 / t117;
            let t305 = t106 * t106;
            let t307 = f64x8::splat(1.0) / t118 / t305;
            let t313 = t307 * t44 * t133;
            let t317 = param_alphaoAx / t31;
            let t318 = t35 * v_sigma0;
            let t319 = t317 * t318;
            let t320 = t305 * t106;
            let t321 = f64x8::splat(1.0) / t320;
            let t322 = t43 * t43;
            let t323 = f64x8::splat(1.0) / t322;
            let t324 = t321 * t323;
            let t325 = t324 * t47;
            let t328 = t324 * t132;
            let t332 = t317 * t318 * t321;
            let t334 = f64x8::splat(1.0) / t130 / t46;
            let t336 = param_c * param_c;
            let t338 = t44 * t334 * t336 * t323;
            let t342 = t229 * param_c * t323;
            let t345 = -f64x8::splat(7.0) / f64x8::splat(27.0) * t34 * t35 * t300 * t48 - f64x8::splat(5.0) / f64x8::splat(18.0) * t116 * v_sigma0 * t307 * t123 + f64x8::splat(5.0) / f64x8::splat(18.0) * t128 * t313 + t319 * t325 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(27.0) * t319 * t328 - f64x8::splat(2.0) / f64x8::splat(27.0) * t332 * t338 - t332 * t342 / f64x8::splat(27.0);
            let t350 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t279 * t52 - t285 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t96 * t137 + t295 - t297 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t345));
            let t351 = t63 * t63;
            let t352 = f64x8::splat(1.0) / t351;
            let t353 = t145 * t145;
            let t356 = t58 * t270;
            let t359 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t89 + f64x8::splat(2.0) * t356)));
            let t363 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t352 * t353 + f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t359));
            let t364 = t363 * t26;
            let t368 = t148 * t101;
            let t370 = t5 * t368 * t83;
            let t372 = t65 * t291;
            let t375 = t5 * t372 * t83 / f64x8::splat(12.0);
            let t377 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t364 * t83 - t370 / f64x8::splat(4.0) + t375));
            let tv2rho20 = f64x8::splat(2.0) * t142 + f64x8::splat(2.0) * t158 + t6 * (t350 + t377);
            acc_v2rho2_0 = tv2rho20;
            let t380 = t265 * t162;
            let t384 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t271)));
            let t388 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t380 * t92 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t384));
            let t389 = t388 * t26;
            let t393 = t165 * t101;
            let t395 = t5 * t393 * t52;
            let t403 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t389 * t52 - t395 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t166 * t137 - t285 / f64x8::splat(8.0) + t295 - t297 / f64x8::splat(8.0)));
            let t404 = t352 * t173;
            let t408 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t356)));
            let t412 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t404 * t145 + f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t408));
            let t413 = t412 * t26;
            let t417 = t176 * t101;
            let t419 = t5 * t417 * t83;
            let t426 = t5 * t153 * t208;
            let t429 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t413 * t83 - t419 / f64x8::splat(8.0) - t370 / f64x8::splat(8.0) + t375 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t149 * t208 - t426 / f64x8::splat(8.0)));
            let tv2rho21 = t142 + t158 + t171 + t213 + t6 * (t403 + t429);
            acc_v2rho2_1 = tv2rho21;
            let t434 = t162 * t162;
            let t439 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t89 + f64x8::splat(2.0) * t271)));
            let t443 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t265 * t434 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t439));
            let t444 = t443 * t26;
            let t450 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t444 * t52 - t395 / f64x8::splat(4.0) + t295));
            let t451 = t173 * t173;
            let t456 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t89 + f64x8::splat(2.0) * t356)));
            let t460 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t352 * t451 + f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t456));
            let t461 = t460 * t26;
            let t471 = f64x8::splat(1.0) / t68 / t188;
            let t476 = t181 * t181;
            let t478 = f64x8::splat(1.0) / t189 / t476;
            let t484 = t478 * t75 * t204;
            let t487 = t67 * v_sigma2;
            let t488 = t317 * t487;
            let t489 = t476 * t181;
            let t490 = f64x8::splat(1.0) / t489;
            let t491 = t74 * t74;
            let t492 = f64x8::splat(1.0) / t491;
            let t493 = t490 * t492;
            let t494 = t493 * t78;
            let t497 = t493 * t203;
            let t501 = t317 * t487 * t490;
            let t503 = f64x8::splat(1.0) / t201 / t77;
            let t506 = t75 * t503 * t336 * t492;
            let t510 = t252 * param_c * t492;
            let t513 = -f64x8::splat(7.0) / f64x8::splat(27.0) * t34 * t67 * t471 * t79 - f64x8::splat(5.0) / f64x8::splat(18.0) * t116 * v_sigma2 * t478 * t194 + f64x8::splat(5.0) / f64x8::splat(18.0) * t199 * t484 + t488 * t494 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(27.0) * t488 * t497 - f64x8::splat(2.0) / f64x8::splat(27.0) * t501 * t506 - t501 * t510 / f64x8::splat(27.0);
            let t518 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t461 * t83 - t419 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t177 * t208 + t375 - t426 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t513));
            let tv2rho22 = f64x8::splat(2.0) * t171 + f64x8::splat(2.0) * t213 + t6 * (t450 + t518);
            acc_v2rho2_2 = tv2rho22;
            let t526 = t5 * t102 * t234 / f64x8::splat(8.0);
            let t539 = t305 * v_rho0;
            let t540 = f64x8::splat(1.0) / t539;
            let t541 = t317 * t540;
            let t542 = t323 * t47;
            let t543 = t542 * t35;
            let t546 = t323 * t131;
            let t548 = t546 * param_c * t35;
            let t552 = t317 * t540 * t44;
            let t553 = t334 * t336;
            let t555 = t553 * t323 * t35;
            let t560 = t34 * t216 * t108 * t48 / f64x8::splat(18.0) + t116 * t120 * t122 * t47 / f64x8::splat(12.0) - t113 * t115 * t120 * t231 / f64x8::splat(12.0) - t541 * t543 / f64x8::splat(72.0) - t541 * t548 / f64x8::splat(36.0) + t552 * t555 / f64x8::splat(36.0) + t552 * t548 / f64x8::splat(72.0);
            let t565 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t96 * t234 - t526 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t560));
            let tv2rhosigma0 = t6 * t565 + t238;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t572 = t5 * t153 * t257 / f64x8::splat(8.0);
            let t574 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t149 * t257 - t572));
            let tv2rhosigma2 = t6 * t574 + t261;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t580 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t166 * t234 - t526));
            let tv2rhosigma3 = t6 * t580 + t238;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t597 = t476 * v_rho1;
            let t598 = f64x8::splat(1.0) / t597;
            let t599 = t317 * t598;
            let t600 = t492 * t78;
            let t601 = t600 * t67;
            let t604 = t492 * t202;
            let t606 = t604 * param_c * t67;
            let t610 = t317 * t598 * t75;
            let t611 = t503 * t336;
            let t613 = t611 * t492 * t67;
            let t618 = t34 * t239 * t183 * t79 / f64x8::splat(18.0) + t116 * t191 * t193 * t78 / f64x8::splat(12.0) - t113 * t115 * t191 * t254 / f64x8::splat(12.0) - t599 * t601 / f64x8::splat(72.0) - t599 * t606 / f64x8::splat(36.0) + t610 * t613 / f64x8::splat(36.0) + t610 * t606 / f64x8::splat(72.0);
            let t623 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t177 * t257 - t572 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t618));
            let tv2rhosigma5 = t6 * t623 + t261;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t625 = f64x8::splat(1.0) / t318;
            let t630 = f64x8::splat(1.0) / v_sigma0;
            let t636 = t113 * t115 * t630;
            let t638 = t222 * t44 * t133;
            let t641 = f64x8::splat(1.0) / t305;
            let t642 = t317 * t641;
            let t647 = t546 * param_c * t216;
            let t651 = t317 * t641 * t44;
            let t658 = t34 * t625 * t38 * t48 / f64x8::splat(48.0) - t116 * t630 * t222 * t123 / f64x8::splat(96.0) + t636 * t638 / f64x8::splat(96.0) + t642 * t542 * t216 / f64x8::splat(192.0) + t642 * t647 / f64x8::splat(96.0) - t651 * t553 * t323 * t216 / f64x8::splat(96.0) - t651 * t647 / f64x8::splat(192.0);
            let t662 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t658));
            let tv2sigma20 = t6 * t662;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t663 = f64x8::splat(1.0) / t487;
            let t668 = f64x8::splat(1.0) / v_sigma2;
            let t674 = t113 * t115 * t668;
            let t676 = t245 * t75 * t204;
            let t679 = f64x8::splat(1.0) / t476;
            let t680 = t317 * t679;
            let t685 = t604 * param_c * t239;
            let t689 = t317 * t679 * t75;
            let t696 = t34 * t663 * t70 * t79 / f64x8::splat(48.0) - t116 * t668 * t245 * t194 / f64x8::splat(96.0) + t674 * t676 / f64x8::splat(96.0) + t680 * t600 * t239 / f64x8::splat(192.0) + t680 * t685 / f64x8::splat(96.0) - t689 * t611 * t492 * t239 / f64x8::splat(96.0) - t689 * t685 / f64x8::splat(192.0);
            let t700 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t696));
            let tv2sigma25 = t6 * t700;
            acc_v2sigma2_5 = tv2sigma25;
            let t704 = f64x8::splat(1.0) / t264 / t19;
            let t705 = t266 * t92;
            let t708 = t265 * t92;
            let t711 = t88 * t88;
            let t712 = f64x8::splat(1.0) / t711;
            let t713 = t16 * t712;
            let t716 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t270 - f64x8::splat(6.0) * t713)));
            let t720 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t704 * t705 + f64x8::splat(4.0) / f64x8::splat(3.0) * t708 * t274 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t716));
            let t721 = t720 * t26;
            let t725 = t278 * t101;
            let t727 = t5 * t725 * t52;
            let t732 = t95 * t291;
            let t734 = t5 * t732 * t52;
            let t737 = t5 * t283 * t137;
            let t743 = f64x8::splat(1.0) / t100 / t88;
            let t744 = t25 * t743;
            let t747 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t744 * t52;
            let t749 = t5 * t292 * t137;
            let t752 = t5 * t102 * t345;
            let t755 = f64x8::splat(1.0) / t36 / t305;
            let t761 = f64x8::splat(1.0) / t118 / t539;
            let t770 = t305 * t117;
            let t771 = f64x8::splat(1.0) / t770;
            let t772 = t771 * t323;
            let t780 = t317 * t318 * t771;
            let t785 = v_sigma0 * v_sigma0;
            let t786 = t305 * t305;
            let t788 = f64x8::splat(1.0) / t36 / t786;
            let t789 = t785 * t788;
            let t790 = t317 * t789;
            let t792 = f64x8::splat(1.0) / t322 / t43;
            let t794 = t792 * t47 * t40;
            let t799 = param_c * t29 * t33;
            let t800 = t792 * t131 * t799;
            let t805 = t336 * t29 * t33;
            let t806 = t792 * t334 * t805;
            let t810 = t317 * t789 * t44;
            let t811 = t130 * t130;
            let t812 = f64x8::splat(1.0) / t811;
            let t813 = t336 * param_c;
            let t814 = t812 * t813;
            let t817 = t814 * t792 * t29 * t33;
            let t824 = f64x8::splat(70.0) / f64x8::splat(81.0) * t34 * t35 * t755 * t48 + f64x8::splat(119.0) / f64x8::splat(81.0) * t116 * v_sigma0 * t761 * t123 - f64x8::splat(119.0) / f64x8::splat(81.0) * t128 * t761 * t44 * t133 - f64x8::splat(11.0) / f64x8::splat(27.0) * t319 * t772 * t47 - f64x8::splat(22.0) / f64x8::splat(27.0) * t319 * t772 * t132 + f64x8::splat(22.0) / f64x8::splat(27.0) * t780 * t338 + f64x8::splat(11.0) / f64x8::splat(27.0) * t780 * t342 + f64x8::splat(2.0) / f64x8::splat(243.0) * t790 * t794 + f64x8::splat(2.0) / f64x8::splat(81.0) * t790 * t800 + f64x8::splat(2.0) / f64x8::splat(81.0) * t790 * t806 - f64x8::splat(2.0) / f64x8::splat(81.0) * t810 * t817 - f64x8::splat(2.0) / f64x8::splat(81.0) * t810 * t806 - f64x8::splat(2.0) / f64x8::splat(243.0) * t810 * t800;
            let t829 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t721 * t52 - f64x8::splat(3.0) / f64x8::splat(8.0) * t727 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t279 * t137 + t734 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t737 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t96 * t345 - t747 + t749 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t752 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t824));
            let t831 = f64x8::splat(1.0) / t351 / t61;
            let t832 = t353 * t145;
            let t835 = t352 * t145;
            let t838 = t58 * t712;
            let t841 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t270 - f64x8::splat(6.0) * t838)));
            let t845 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t831 * t832 + f64x8::splat(4.0) / f64x8::splat(3.0) * t835 * t359 + f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t841));
            let t846 = t845 * t26;
            let t850 = t363 * t101;
            let t852 = t5 * t850 * t83;
            let t854 = t148 * t291;
            let t856 = t5 * t854 * t83;
            let t858 = t65 * t743;
            let t861 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t858 * t83;
            let t863 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t846 * t83 - f64x8::splat(3.0) / f64x8::splat(8.0) * t852 + t856 / f64x8::splat(4.0) - t861));
            let tv3rho30 = f64x8::splat(3.0) * t350 + f64x8::splat(3.0) * t377 + t6 * (t829 + t863);
            acc_v3rho3_0 = tv3rho30;
            let t866 = f64x8::splat(2.0) * t403;
            let t867 = f64x8::splat(2.0) * t429;
            let t868 = t704 * t162;
            let t871 = t265 * t384;
            let t876 = f64x8::splat(2.0) * t270;
            let t877 = f64x8::splat(6.0) * t713;
            let t879 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t876 - t877)));
            let t883 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t868 * t266 + f64x8::splat(8.0) / f64x8::splat(9.0) * t871 * t92 + f64x8::splat(4.0) / f64x8::splat(9.0) * t380 * t274 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t879));
            let t884 = t883 * t26;
            let t888 = t388 * t101;
            let t891 = t5 * t888 * t52 / f64x8::splat(4.0);
            let t895 = t165 * t291;
            let t897 = t5 * t895 * t52;
            let t901 = t5 * t393 * t137 / f64x8::splat(4.0);
            let t910 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t884 * t52 - t891 - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t389 * t137 + t897 / f64x8::splat(12.0) - t901 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t166 * t345 - t727 / f64x8::splat(8.0) + t734 / f64x8::splat(6.0) - t737 / f64x8::splat(4.0) - t747 + t749 / f64x8::splat(6.0) - t752 / f64x8::splat(8.0);
            let t911 = ((t1).select(f64x8::splat(0.0), t910));
            let t912 = t831 * t173;
            let t915 = t352 * t408;
            let t920 = f64x8::splat(6.0) * t838;
            let t922 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t876 - t920)));
            let t926 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t912 * t353 + f64x8::splat(8.0) / f64x8::splat(9.0) * t915 * t145 + f64x8::splat(4.0) / f64x8::splat(9.0) * t404 * t359 + f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t922));
            let t927 = t926 * t26;
            let t931 = t412 * t101;
            let t934 = t5 * t931 * t83 / f64x8::splat(4.0);
            let t935 = t176 * t291;
            let t937 = t5 * t935 * t83;
            let t946 = t5 * t368 * t208 / f64x8::splat(4.0);
            let t948 = t5 * t372 * t208;
            let t951 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t927 * t83 - t934 + t937 / f64x8::splat(12.0) - t852 / f64x8::splat(8.0) + t856 / f64x8::splat(6.0) - t861 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t364 * t208 - t946 + t948 / f64x8::splat(12.0)));
            let tv3rho31 = t350 + t377 + t866 + t867 + t6 * (t911 + t951);
            acc_v3rho3_1 = tv3rho31;
            let t954 = t704 * t434;
            let t959 = t265 * t439;
            let t963 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t876 - t877)));
            let t967 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t954 * t92 + f64x8::splat(8.0) / f64x8::splat(9.0) * t380 * t384 + f64x8::splat(4.0) / f64x8::splat(9.0) * t959 * t92 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t963));
            let t968 = t967 * t26;
            let t972 = t443 * t101;
            let t974 = t5 * t972 * t52;
            let t983 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t968 * t52 - t974 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t444 * t137 - t891 + t897 / f64x8::splat(6.0) - t901 + t734 / f64x8::splat(12.0) - t747 + t749 / f64x8::splat(12.0)));
            let t984 = t831 * t451;
            let t989 = t352 * t456;
            let t993 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t876 - t920)));
            let t997 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t984 * t145 + f64x8::splat(8.0) / f64x8::splat(9.0) * t404 * t408 + f64x8::splat(4.0) / f64x8::splat(9.0) * t989 * t145 + f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t993));
            let t998 = t997 * t26;
            let t1002 = t460 * t101;
            let t1004 = t5 * t1002 * t83;
            let t1011 = t5 * t417 * t208;
            let t1019 = t5 * t153 * t513;
            let t1021 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t998 * t83 - t1004 / f64x8::splat(8.0) - t934 + t937 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t413 * t208 - t1011 / f64x8::splat(4.0) + t856 / f64x8::splat(12.0) - t861 - t946 + t948 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t149 * t513 - t1019 / f64x8::splat(8.0);
            let t1022 = ((t57).select(f64x8::splat(0.0), t1021));
            let tv3rho32 = t866 + t867 + t450 + t518 + t6 * (t983 + t1022);
            acc_v3rho3_2 = tv3rho32;
            let t1027 = t434 * t162;
            let t1034 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t270 - f64x8::splat(6.0) * t713)));
            let t1038 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t704 * t1027 + f64x8::splat(4.0) / f64x8::splat(3.0) * t380 * t439 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t1034));
            let t1039 = t1038 * t26;
            let t1046 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1039 * t52 - f64x8::splat(3.0) / f64x8::splat(8.0) * t974 + t897 / f64x8::splat(4.0) - t747));
            let t1047 = t451 * t173;
            let t1054 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t270 - f64x8::splat(6.0) * t838)));
            let t1058 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t831 * t1047 + f64x8::splat(4.0) / f64x8::splat(3.0) * t404 * t456 + f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t1054));
            let t1059 = t1058 * t26;
            let t1075 = f64x8::splat(1.0) / t68 / t476;
            let t1081 = f64x8::splat(1.0) / t189 / t597;
            let t1090 = t476 * t188;
            let t1091 = f64x8::splat(1.0) / t1090;
            let t1092 = t1091 * t492;
            let t1100 = t317 * t487 * t1091;
            let t1105 = v_sigma2 * v_sigma2;
            let t1106 = t476 * t476;
            let t1108 = f64x8::splat(1.0) / t68 / t1106;
            let t1109 = t1105 * t1108;
            let t1110 = t317 * t1109;
            let t1112 = f64x8::splat(1.0) / t491 / t74;
            let t1114 = t1112 * t78 * t40;
            let t1118 = t1112 * t202 * t799;
            let t1122 = t1112 * t503 * t805;
            let t1126 = t317 * t1109 * t75;
            let t1127 = t201 * t201;
            let t1128 = f64x8::splat(1.0) / t1127;
            let t1129 = t1128 * t813;
            let t1132 = t1129 * t1112 * t29 * t33;
            let t1139 = f64x8::splat(70.0) / f64x8::splat(81.0) * t34 * t67 * t1075 * t79 + f64x8::splat(119.0) / f64x8::splat(81.0) * t116 * v_sigma2 * t1081 * t194 - f64x8::splat(119.0) / f64x8::splat(81.0) * t199 * t1081 * t75 * t204 - f64x8::splat(11.0) / f64x8::splat(27.0) * t488 * t1092 * t78 - f64x8::splat(22.0) / f64x8::splat(27.0) * t488 * t1092 * t203 + f64x8::splat(22.0) / f64x8::splat(27.0) * t1100 * t506 + f64x8::splat(11.0) / f64x8::splat(27.0) * t1100 * t510 + f64x8::splat(2.0) / f64x8::splat(243.0) * t1110 * t1114 + f64x8::splat(2.0) / f64x8::splat(81.0) * t1110 * t1118 + f64x8::splat(2.0) / f64x8::splat(81.0) * t1110 * t1122 - f64x8::splat(2.0) / f64x8::splat(81.0) * t1126 * t1132 - f64x8::splat(2.0) / f64x8::splat(81.0) * t1126 * t1122 - f64x8::splat(2.0) / f64x8::splat(243.0) * t1126 * t1118;
            let t1144 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1059 * t83 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1004 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t461 * t208 + t937 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t1011 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t177 * t513 - t861 + t948 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t1019 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t1139));
            let tv3rho33 = f64x8::splat(3.0) * t450 + f64x8::splat(3.0) * t518 + t6 * (t1046 + t1144);
            acc_v3rho3_3 = tv3rho33;
            let t1152 = t5 * t283 * t234;
            let t1159 = t5 * t292 * t234 / f64x8::splat(12.0);
            let t1161 = t5 * t102 * t560;
            let t1175 = t317 * t321;
            let t1181 = t317 * t321 * t44;
            let t1187 = f64x8::splat(1.0) / t36 / t770;
            let t1189 = t317 * t1187 * t792;
            let t1191 = t47 * v_sigma0 * t40;
            let t1195 = t131 * v_sigma0 * t799;
            let t1199 = v_sigma0 * t29 * t33;
            let t1200 = t553 * t1199;
            let t1203 = t1187 * t44;
            let t1206 = t813 * t792;
            let t1207 = t1206 * t1199;
            let t1212 = t336 * t792;
            let t1213 = t1212 * t1199;
            let t1218 = param_c * t792;
            let t1219 = t1218 * t1199;
            let t1222 = -f64x8::splat(7.0) / f64x8::splat(54.0) * t34 * t216 * t300 * t48 - f64x8::splat(37.0) / f64x8::splat(108.0) * t116 * t307 * t122 * t47 + f64x8::splat(37.0) / f64x8::splat(108.0) * t113 * t115 * t307 * t231 + t1175 * t543 / f64x8::splat(8.0) + t1175 * t548 / f64x8::splat(4.0) - t1181 * t555 / f64x8::splat(4.0) - t1181 * t548 / f64x8::splat(8.0) - t1189 * t1191 / f64x8::splat(324.0) - t1189 * t1195 / f64x8::splat(108.0) - t1189 * t1200 / f64x8::splat(108.0) + t317 * t1203 * t812 * t1207 / f64x8::splat(108.0) + t317 * t1203 * t334 * t1213 / f64x8::splat(108.0) + t317 * t1203 * t131 * t1219 / f64x8::splat(324.0);
            let t1227 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t279 * t234 - t1152 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t96 * t560 + t1159 - t1161 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1222));
            let tv3rho2sigma0 = t6 * t1227 + f64x8::splat(2.0) * t565;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t1234 = t5 * t368 * t257;
            let t1238 = t5 * t372 * t257 / f64x8::splat(12.0);
            let t1240 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t364 * t257 - t1234 / f64x8::splat(4.0) + t1238));
            let tv3rho2sigma2 = t6 * t1240 + f64x8::splat(2.0) * t574;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t1246 = t5 * t393 * t234;
            let t1254 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t389 * t234 - t1246 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t166 * t560 - t1152 / f64x8::splat(8.0) + t1159 - t1161 / f64x8::splat(8.0)));
            let tv3rho2sigma3 = t6 * t1254 + t565 + t580;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1260 = t5 * t417 * t257;
            let t1267 = t5 * t153 * t618;
            let t1270 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t413 * t257 - t1260 / f64x8::splat(8.0) - t1234 / f64x8::splat(8.0) + t1238 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t149 * t618 - t1267 / f64x8::splat(8.0)));
            let tv3rho2sigma5 = t6 * t1270 + t574 + t623;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1278 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t444 * t234 - t1246 / f64x8::splat(4.0) + t1159));
            let tv3rho2sigma6 = t6 * t1278 + f64x8::splat(2.0) * t580;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1301 = t317 * t490;
            let t1307 = t317 * t490 * t75;
            let t1313 = f64x8::splat(1.0) / t68 / t1090;
            let t1315 = t317 * t1313 * t1112;
            let t1317 = t78 * v_sigma2 * t40;
            let t1321 = t202 * v_sigma2 * t799;
            let t1325 = v_sigma2 * t29 * t33;
            let t1326 = t611 * t1325;
            let t1329 = t1313 * t75;
            let t1332 = t813 * t1112;
            let t1333 = t1332 * t1325;
            let t1338 = t336 * t1112;
            let t1339 = t1338 * t1325;
            let t1344 = param_c * t1112;
            let t1345 = t1344 * t1325;
            let t1348 = -f64x8::splat(7.0) / f64x8::splat(54.0) * t34 * t239 * t471 * t79 - f64x8::splat(37.0) / f64x8::splat(108.0) * t116 * t478 * t193 * t78 + f64x8::splat(37.0) / f64x8::splat(108.0) * t113 * t115 * t478 * t254 + t1301 * t601 / f64x8::splat(8.0) + t1301 * t606 / f64x8::splat(4.0) - t1307 * t613 / f64x8::splat(4.0) - t1307 * t606 / f64x8::splat(8.0) - t1315 * t1317 / f64x8::splat(324.0) - t1315 * t1321 / f64x8::splat(108.0) - t1315 * t1326 / f64x8::splat(108.0) + t317 * t1329 * t1128 * t1333 / f64x8::splat(108.0) + t317 * t1329 * t503 * t1339 / f64x8::splat(108.0) + t317 * t1329 * t202 * t1345 / f64x8::splat(324.0);
            let t1353 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t461 * t257 - t1260 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t177 * t618 + t1238 - t1267 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t1348));
            let tv3rho2sigma8 = t6 * t1353 + f64x8::splat(2.0) * t623;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1360 = t5 * t102 * t658 / f64x8::splat(8.0);
            let t1371 = t317 * t216;
            let t1372 = t540 * t323;
            let t1373 = t1372 * t47;
            let t1376 = t1372 * t132;
            let t1380 = t317 * t216 * t540;
            let t1386 = f64x8::splat(1.0) / t36 / t320;
            let t1391 = t317 * t1386 * t792;
            let t1392 = t132 * t40;
            let t1395 = t553 * t40;
            let t1398 = t1386 * t44;
            let t1399 = t317 * t1398;
            let t1406 = -t34 * t625 * t108 * t48 / f64x8::splat(36.0) + t116 * t630 * t120 * t123 / f64x8::splat(72.0) - t636 * t134 / f64x8::splat(72.0) - t1371 * t1373 / f64x8::splat(36.0) - t1371 * t1376 / f64x8::splat(18.0) + t1380 * t338 / f64x8::splat(18.0) + t1380 * t342 / f64x8::splat(36.0) + t317 * t1386 * t794 / f64x8::splat(864.0) + t1391 * t1392 / f64x8::splat(288.0) + t1391 * t1395 / f64x8::splat(288.0) - t1399 * t817 / f64x8::splat(288.0) - t1399 * t806 / f64x8::splat(288.0) - t1399 * t800 / f64x8::splat(864.0);
            let t1411 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t96 * t658 - t1360 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1406));
            let tv3rhosigma20 = t6 * t1411 + t662;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1418 = t5 * t153 * t696 / f64x8::splat(8.0);
            let t1420 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t149 * t696 - t1418));
            let tv3rhosigma25 = t6 * t1420 + t700;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1426 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t166 * t658 - t1360));
            let tv3rhosigma26 = t6 * t1426 + t662;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1441 = t317 * t239;
            let t1442 = t598 * t492;
            let t1443 = t1442 * t78;
            let t1446 = t1442 * t203;
            let t1450 = t317 * t239 * t598;
            let t1456 = f64x8::splat(1.0) / t68 / t489;
            let t1461 = t317 * t1456 * t1112;
            let t1462 = t203 * t40;
            let t1465 = t611 * t40;
            let t1468 = t1456 * t75;
            let t1469 = t317 * t1468;
            let t1476 = -t34 * t663 * t183 * t79 / f64x8::splat(36.0) + t116 * t668 * t191 * t194 / f64x8::splat(72.0) - t674 * t205 / f64x8::splat(72.0) - t1441 * t1443 / f64x8::splat(36.0) - t1441 * t1446 / f64x8::splat(18.0) + t1450 * t506 / f64x8::splat(18.0) + t1450 * t510 / f64x8::splat(36.0) + t317 * t1456 * t1114 / f64x8::splat(864.0) + t1461 * t1462 / f64x8::splat(288.0) + t1461 * t1465 / f64x8::splat(288.0) - t1469 * t1132 / f64x8::splat(288.0) - t1469 * t1122 / f64x8::splat(288.0) - t1469 * t1118 / f64x8::splat(864.0);
            let t1481 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t177 * t696 - t1418 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t1476));
            let tv3rhosigma211 = t6 * t1481 + t700;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1483 = t35 * t785;
            let t1484 = f64x8::splat(1.0) / t1483;
            let t1489 = f64x8::splat(1.0) / t785;
            let t1495 = t113 * t115 * t1489;
            let t1499 = f64x8::splat(1.0) / t36 / t539;
            let t1501 = t317 * t1499 * t792;
            let t1503 = t47 * t630 * t40;
            let t1507 = t131 * t630 * t799;
            let t1511 = t630 * t29 * t33;
            let t1512 = t553 * t1511;
            let t1515 = t1499 * t44;
            let t1517 = t317 * t1515 * t812;
            let t1518 = t1206 * t1511;
            let t1522 = t317 * t1515 * t334;
            let t1523 = t1212 * t1511;
            let t1527 = t317 * t1515 * t131;
            let t1528 = t1218 * t1511;
            let t1531 = -t34 * t1484 * t38 * t48 / f64x8::splat(32.0) + t116 * t1489 * t222 * t123 / f64x8::splat(64.0) - t1495 * t638 / f64x8::splat(64.0) - t1501 * t1503 / f64x8::splat(2304.0) - t1501 * t1507 / f64x8::splat(768.0) - t1501 * t1512 / f64x8::splat(768.0) + t1517 * t1518 / f64x8::splat(768.0) + t1522 * t1523 / f64x8::splat(768.0) + t1527 * t1528 / f64x8::splat(2304.0);
            let t1535 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1531));
            let tv3sigma30 = t6 * t1535;
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
            let t1536 = t67 * t1105;
            let t1537 = f64x8::splat(1.0) / t1536;
            let t1542 = f64x8::splat(1.0) / t1105;
            let t1548 = t113 * t115 * t1542;
            let t1552 = f64x8::splat(1.0) / t68 / t597;
            let t1554 = t317 * t1552 * t1112;
            let t1556 = t78 * t668 * t40;
            let t1560 = t202 * t668 * t799;
            let t1564 = t668 * t29 * t33;
            let t1565 = t611 * t1564;
            let t1568 = t1552 * t75;
            let t1570 = t317 * t1568 * t1128;
            let t1571 = t1332 * t1564;
            let t1575 = t317 * t1568 * t503;
            let t1576 = t1338 * t1564;
            let t1580 = t317 * t1568 * t202;
            let t1581 = t1344 * t1564;
            let t1584 = -t34 * t1537 * t70 * t79 / f64x8::splat(32.0) + t116 * t1542 * t245 * t194 / f64x8::splat(64.0) - t1548 * t676 / f64x8::splat(64.0) - t1554 * t1556 / f64x8::splat(2304.0) - t1554 * t1560 / f64x8::splat(768.0) - t1554 * t1565 / f64x8::splat(768.0) + t1570 * t1571 / f64x8::splat(768.0) + t1575 * t1576 / f64x8::splat(768.0) + t1580 * t1581 / f64x8::splat(2304.0);
            let t1588 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t1584));
            let tv3sigma39 = t6 * t1588;
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
