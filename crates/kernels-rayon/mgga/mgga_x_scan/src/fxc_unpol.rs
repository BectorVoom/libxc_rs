//! MGGA_X_SCAN fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_scan.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_scan_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_k1 = f64x8::splat(param_k1);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = t4 / t5 * t18;
            let t20 = (simd::cbrt(v_rho));
            let t21 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t21 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t20 * t20;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = t29 * t33;
            let t35 = t26 * t34;
            let t39 = f64x8::splat(100.0) / f64x8::splat(6561.0) / param_k1 - f64x8::splat(73.0) / f64x8::splat(648.0);
            let t40 = t21 * t21;
            let t42 = t23 * t22;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t39 * t40 * t43;
            let t45 = v_sigma * v_sigma;
            let t46 = t45 * t27;
            let t47 = t30 * t30;
            let t48 = t47 * v_rho;
            let t50 = f64x8::splat(1.0) / t20 / t48;
            let t55 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t39 * t21 * t25 * t34));
            let t56 = t50 * t55;
            let t60 = ((f64x8::splat(146.0)).sqrt());
            let t61 = t60 * t21;
            let t62 = t61 * t25;
            let t65 = v_tau * t28;
            let t66 = t31 * v_rho;
            let t67 = f64x8::splat(1.0) / t66;
            let t73 = f64x8::splat(5.0) / f64x8::splat(9.0) * (t65 * t67 - t34 / f64x8::splat(8.0)) * t21 * t25;
            let t74 = f64x8::splat(1.0) - t73;
            let t76 = t74 * t74;
            let t78 = (simd::exp(-t76 / f64x8::splat(2.0)));
            let t81 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t62 * t34 + t60 * t74 * t78 / f64x8::splat(100.0);
            let t82 = t81 * t81;
            let t83 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t35 + t44 * t46 * t56 / f64x8::splat(288.0) + t82;
            let t88 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t83);
            let t89 = (t73).simd_le(f64x8::splat(1.0));
            let t90 = (simd::ln(f64x8::splat(f64::EPSILON)));
            let t93 = t90 / (-t90 + param_c1);
            let t94 = (-t93).simd_lt(t73);
            let t95 = (t73).simd_lt(-t93);
            let t96 = ((t95).select(t73, -t93));
            let t97 = param_c1 * t96;
            let t98 = f64x8::splat(1.0) - t96;
            let t99 = f64x8::splat(1.0) / t98;
            let t101 = (simd::exp(-t97 * t99));
            let t102 = ((t94).select(f64x8::splat(0.0), t101));
            let t103 = ((param_d).abs());
            let t106 = (simd::ln(f64x8::splat(f64::EPSILON) / t103));
            let t109 = (-t106 + param_c2) / t106;
            let t110 = (t73).simd_lt(-t109);
            let t111 = ((t110).select(-t109, t73));
            let t112 = f64x8::splat(1.0) - t111;
            let t115 = (simd::exp(param_c2 / t112));
            let t117 = ((t110).select(f64x8::splat(0.0), -param_d * t115));
            let t118 = ((t89).select(t102, t117));
            let t119 = f64x8::splat(1.0) - t118;
            let t122 = t88 * t119 + f64x8::splat(1.174) * t118;
            let t124 = ((f64x8::splat(3.0)).sqrt());
            let t125 = f64x8::splat(1.0) / t23;
            let t126 = t40 * t125;
            let t127 = ((v_sigma).sqrt());
            let t128 = t127 * t27;
            let t130 = f64x8::splat(1.0) / t20 / v_rho;
            let t132 = t126 * t128 * t130;
            let t133 = ((t132).sqrt());
            let t137 = (simd::exp(-f64x8::splat(9.8958) * t124 / t133));
            let t138 = f64x8::splat(1.0) - t137;
            let t142 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t122 * t138));
            let tzk0 = f64x8::splat(2.0) * t142;
            acc_zk = tzk0;
            let t143 = f64x8::splat(1.0) / t31;
            let t148 = param_k1 * param_k1;
            let t149 = t83 * t83;
            let t151 = t148 / t149;
            let t152 = t30 * v_rho;
            let t154 = f64x8::splat(1.0) / t31 / t152;
            let t155 = t29 * t154;
            let t158 = t47 * t30;
            let t160 = f64x8::splat(1.0) / t20 / t158;
            let t161 = t160 * t55;
            let t165 = t39 * t39;
            let t166 = t22 * t22;
            let t167 = f64x8::splat(1.0) / t166;
            let t168 = t165 * t167;
            let t169 = t45 * v_sigma;
            let t170 = t47 * t47;
            let t171 = t170 * v_rho;
            let t172 = f64x8::splat(1.0) / t171;
            let t182 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t65 * t33 + t155 / f64x8::splat(3.0);
            let t184 = t26 * t78;
            let t187 = t60 * t76;
            let t191 = -f64x8::splat(7.0) / f64x8::splat(4860.0) * t62 * t155 - t60 * t182 * t184 / f64x8::splat(180.0) + t187 * t182 * t184 / f64x8::splat(180.0);
            let t194 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t26 * t155 - t44 * t46 * t161 / f64x8::splat(54.0) + f64x8::splat(3.0) / f64x8::splat(80.0) * t168 * t169 * t172 * t55 + f64x8::splat(2.0) * t81 * t191;
            let t195 = t194 * t119;
            let t197 = t182 * t21;
            let t199 = f64x8::splat(5.0) / f64x8::splat(9.0) * t197 * t25;
            let t200 = ((t95).select(t199, f64x8::splat(0.0)));
            let t203 = t98 * t98;
            let t204 = f64x8::splat(1.0) / t203;
            let t205 = t204 * t200;
            let t207 = -param_c1 * t200 * t99 - t97 * t205;
            let t208 = t207 * t101;
            let t209 = ((t94).select(f64x8::splat(0.0), t208));
            let t210 = param_d * param_c2;
            let t211 = t112 * t112;
            let t212 = f64x8::splat(1.0) / t211;
            let t213 = ((t110).select(f64x8::splat(0.0), t199));
            let t217 = ((t110).select(f64x8::splat(0.0), -t210 * t212 * t213 * t115));
            let t218 = ((t89).select(t209, t217));
            let t221 = t151 * t195 - t88 * t218 + f64x8::splat(1.174) * t218;
            let t226 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t227 = t226 * t226;
            let t228 = t227 * t227;
            let t230 = t228 * t226 * t18;
            let t231 = f64x8::splat(1.0) / t30;
            let t232 = t231 * t122;
            let t234 = f64x8::splat(1.0) / t133 / t132;
            let t236 = t230 * t232 * t234;
            let t238 = t126 * t128 * t137;
            let t242 = ((t3).select(f64x8::splat(0.0), -t19 * t143 * t122 * t138 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t221 * t138 - f64x8::splat(1.6891736332904388) * t236 * t238));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t242 + f64x8::splat(2.0) * t142;
            acc_vrho = tvrho0;
            let t245 = t28 * t33;
            let t246 = t245 * t26;
            let t248 = v_sigma * t27;
            let t252 = f64x8::splat(1.0) / t170;
            let t257 = t25 * t28;
            let t261 = t60 * t28;
            let t262 = t261 * t33;
            let t263 = t262 * t184;
            let t265 = t187 * t28;
            let t267 = t25 * t78;
            let t269 = t265 * t33 * t21 * t267;
            let t271 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t61 * t257 * t33 + t263 / f64x8::splat(1440.0) - t269 / f64x8::splat(1440.0);
            let t274 = f64x8::splat(5.0) / f64x8::splat(972.0) * t246 + t44 * t248 * t56 / f64x8::splat(144.0) - f64x8::splat(9.0) / f64x8::splat(640.0) * t168 * t45 * t252 * t55 + f64x8::splat(2.0) * t81 * t271;
            let t275 = t274 * t119;
            let t277 = f64x8::splat(5.0) / f64x8::splat(72.0) * t246;
            let t278 = ((t95).select(-t277, f64x8::splat(0.0)));
            let t279 = param_c1 * t278;
            let t281 = t204 * t278;
            let t283 = -t279 * t99 - t97 * t281;
            let t284 = t283 * t101;
            let t285 = ((t94).select(f64x8::splat(0.0), t284));
            let t286 = ((t110).select(f64x8::splat(0.0), -t277));
            let t290 = ((t110).select(f64x8::splat(0.0), -t210 * t212 * t286 * t115));
            let t291 = ((t89).select(t285, t290));
            let t294 = t151 * t275 - t88 * t291 + f64x8::splat(1.174) * t291;
            let t299 = f64x8::splat(1.0) / v_rho;
            let t300 = t299 * t122;
            let t302 = t230 * t300 * t234;
            let t303 = f64x8::splat(1.0) / t127;
            let t306 = t126 * t303 * t27 * t137;
            let t310 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t294 * t138 + f64x8::splat(0.6334401124839145) * t302 * t306));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t310;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t312 = t261 * t67;
            let t318 = t265 * t67 * t21 * t267 / f64x8::splat(180.0) - t312 * t184 / f64x8::splat(180.0);
            let t319 = t81 * t318;
            let t323 = t28 * t67;
            let t325 = f64x8::splat(5.0) / f64x8::splat(9.0) * t323 * t26;
            let t326 = ((t95).select(t325, f64x8::splat(0.0)));
            let t327 = param_c1 * t326;
            let t331 = -t97 * t204 * t326 - t327 * t99;
            let t332 = t331 * t101;
            let t333 = ((t94).select(f64x8::splat(0.0), t332));
            let t334 = ((t110).select(f64x8::splat(0.0), t325));
            let t338 = ((t110).select(f64x8::splat(0.0), -t210 * t212 * t334 * t115));
            let t339 = ((t89).select(t333, t338));
            let t342 = f64x8::splat(2.0) * t151 * t319 * t119 - t88 * t339 + f64x8::splat(1.174) * t339;
            let t347 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t342 * t138));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t347;
            acc_vtau = tvtau0;
            let t358 = f64x8::splat(1.0) / t152;
            let t359 = t358 * t122;
            let t361 = t230 * t359 * t234;
            let t366 = t148 / t149 / t83;
            let t367 = t194 * t194;
            let t368 = t367 * t119;
            let t372 = f64x8::splat(1.0) / t31 / t47;
            let t373 = t29 * t372;
            let t376 = t47 * t152;
            let t378 = f64x8::splat(1.0) / t20 / t376;
            let t379 = t378 * t55;
            let t383 = t170 * t30;
            let t384 = f64x8::splat(1.0) / t383;
            let t390 = t165 * t39 * t167;
            let t391 = t45 * t45;
            let t392 = t170 * t47;
            let t394 = f64x8::splat(1.0) / t31 / t392;
            let t397 = t28 * t55;
            let t398 = t26 * t397;
            let t401 = t191 * t191;
            let t408 = f64x8::splat(40.0) / f64x8::splat(9.0) * t65 * t154 - f64x8::splat(11.0) / f64x8::splat(9.0) * t373;
            let t409 = t60 * t408;
            let t412 = t182 * t182;
            let t415 = t43 * t74;
            let t416 = t415 * t78;
            let t422 = t76 * t74;
            let t423 = t60 * t422;
            let t425 = t40 * t43;
            let t426 = t425 * t78;
            let t429 = f64x8::splat(77.0) / f64x8::splat(14580.0) * t62 * t373 - t409 * t184 / f64x8::splat(180.0) - t60 * t412 * t40 * t416 / f64x8::splat(108.0) + t187 * t408 * t184 / f64x8::splat(180.0) + t423 * t412 * t426 / f64x8::splat(324.0);
            let t432 = f64x8::splat(110.0) / f64x8::splat(2187.0) * t26 * t373 + f64x8::splat(19.0) / f64x8::splat(162.0) * t44 * t46 * t379 - f64x8::splat(43.0) / f64x8::splat(80.0) * t168 * t169 * t384 * t55 + f64x8::splat(27.0) / f64x8::splat(800.0) * t390 * t391 * t394 * t398 + f64x8::splat(2.0) * t401 + f64x8::splat(2.0) * t81 * t429;
            let t435 = t194 * t218;
            let t440 = f64x8::splat(5.0) / f64x8::splat(9.0) * t408 * t21 * t25;
            let t441 = ((t95).select(t440, f64x8::splat(0.0)));
            let t442 = param_c1 * t441;
            let t444 = t200 * t200;
            let t449 = f64x8::splat(1.0) / t203 / t98;
            let t450 = t449 * t444;
            let t453 = t204 * t441;
            let t455 = -f64x8::splat(2.0) * param_c1 * t444 * t204 - t442 * t99 - f64x8::splat(2.0) * t97 * t450 - t97 * t453;
            let t456 = t455 * t101;
            let t457 = t207 * t207;
            let t458 = t457 * t101;
            let t460 = ((t94).select(f64x8::splat(0.0), t456 + t458));
            let t461 = t211 * t112;
            let t462 = f64x8::splat(1.0) / t461;
            let t463 = t213 * t213;
            let t468 = ((t110).select(f64x8::splat(0.0), t440));
            let t472 = param_c2 * param_c2;
            let t473 = param_d * t472;
            let t474 = t211 * t211;
            let t475 = f64x8::splat(1.0) / t474;
            let t480 = ((t110).select(f64x8::splat(0.0), -t210 * t212 * t468 * t115 - f64x8::splat(2.0) * t210 * t462 * t463 * t115 - t473 * t475 * t463 * t115));
            let t481 = ((t89).select(t460, t480));
            let t484 = -f64x8::splat(2.0) * t366 * t368 + t151 * t432 * t119 - f64x8::splat(2.0) * t151 * t435 - t88 * t481 + f64x8::splat(1.174) * t481;
            let t491 = t230 * t231 * t221 * t234;
            let t495 = f64x8::splat(1.0) / t20 / t47;
            let t496 = t495 * t122;
            let t499 = f64x8::splat(1.0) / t133 / t35 / f64x8::splat(6.0);
            let t501 = t230 * t496 * t499;
            let t503 = t26 * t29 * t137;
            let t506 = t4 * t18;
            let t507 = f64x8::splat(1.0) / t20;
            let t509 = t506 * t507 * t122;
            let t511 = t257 * t137;
            let t512 = t303 * t21 * t511;
            let t516 = ((t3).select(f64x8::splat(0.0), t19 * t67 * t122 * t138 / f64x8::splat(12.0) - t19 * t143 * t221 * t138 / f64x8::splat(4.0) + f64x8::splat(2.8152893888173978) * t361 * t238 - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t484 * t138 - f64x8::splat(3.3783472665808776) * t491 * t238 - f64x8::splat(20.270083599485265) * t501 * t503 + f64x8::splat(27.496264583922507) * t509 * t512));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t516 + f64x8::splat(4.0) * t242;
            acc_v2rho2 = tv2rho20;
            let t523 = t275 * t194;
            let t526 = t28 * t154;
            let t527 = t526 * t26;
            let t536 = t170 * t152;
            let t538 = f64x8::splat(1.0) / t31 / t536;
            let t548 = t261 * t154;
            let t549 = t548 * t184;
            let t552 = t261 * t33 * t40;
            let t553 = t182 * t78;
            let t554 = t415 * t553;
            let t555 = t552 * t554;
            let t559 = t265 * t154 * t21 * t267;
            let t561 = t423 * t245;
            let t562 = t425 * t553;
            let t563 = t561 * t562;
            let t565 = -f64x8::splat(7.0) / f64x8::splat(4860.0) * t61 * t257 * t154 - t549 / f64x8::splat(540.0) + t555 / f64x8::splat(864.0) + t559 / f64x8::splat(540.0) - t563 / f64x8::splat(2592.0);
            let t568 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t527 - t44 * t248 * t161 / f64x8::splat(27.0) + f64x8::splat(3.0) / f64x8::splat(16.0) * t168 * t45 * t172 * t55 - f64x8::splat(81.0) / f64x8::splat(6400.0) * t390 * t169 * t538 * t398 + f64x8::splat(2.0) * t191 * t271 + f64x8::splat(2.0) * t81 * t565;
            let t569 = t568 * t119;
            let t571 = t274 * t218;
            let t573 = t194 * t291;
            let t575 = f64x8::splat(5.0) / f64x8::splat(27.0) * t527;
            let t576 = ((t95).select(t575, f64x8::splat(0.0)));
            let t577 = param_c1 * t576;
            let t581 = t449 * t278;
            let t582 = t581 * t200;
            let t585 = t204 * t576;
            let t587 = -f64x8::splat(2.0) * t279 * t205 - t577 * t99 - f64x8::splat(2.0) * t97 * t582 - t97 * t585;
            let t588 = t587 * t101;
            let t589 = t283 * t207;
            let t592 = ((t94).select(f64x8::splat(0.0), t589 * t101 + t588));
            let t593 = t210 * t462;
            let t594 = t286 * t115;
            let t595 = t594 * t213;
            let t598 = ((t110).select(f64x8::splat(0.0), t575));
            let t602 = t473 * t475;
            let t605 = ((t110).select(f64x8::splat(0.0), -t210 * t212 * t598 * t115 - f64x8::splat(2.0) * t593 * t595 - t602 * t595));
            let t606 = ((t89).select(t592, t605));
            let t609 = -f64x8::splat(2.0) * t366 * t523 + t151 * t569 - t151 * t571 - t151 * t573 - t88 * t606 + f64x8::splat(1.174) * t606;
            let t616 = t230 * t231 * t294 * t234;
            let t623 = t230 * t299 * t221 * t234;
            let t627 = f64x8::splat(1.0) / t20 / t152;
            let t628 = t627 * t122;
            let t631 = t499 * t21 * t511;
            let t635 = t506 * t31 * t122;
            let t636 = t127 * v_sigma;
            let t637 = f64x8::splat(1.0) / t636;
            let t639 = t637 * t21 * t511;
            let t643 = ((t3).select(f64x8::splat(0.0), -t19 * t143 * t294 * t138 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t609 * t138 - f64x8::splat(1.6891736332904388) * t616 * t238 - f64x8::splat(0.6334401124839145) * t236 * t306 + f64x8::splat(0.6334401124839145) * t623 * t306 + f64x8::splat(7.601281349806975) * t230 * t628 * t631 - f64x8::splat(10.311099218970941) * t635 * t639));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t643 + f64x8::splat(2.0) * t310;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t650 = t366 * t81;
            let t651 = t318 * t119;
            let t652 = t651 * t194;
            let t655 = t191 * t318;
            let t661 = t261 * t67 * t40;
            let t665 = t423 * t323;
            let t668 = t263 / f64x8::splat(108.0) - t661 * t554 / f64x8::splat(108.0) - t269 / f64x8::splat(108.0) + t665 * t562 / f64x8::splat(324.0);
            let t669 = t81 * t668;
            let t676 = t194 * t339;
            let t678 = f64x8::splat(25.0) / f64x8::splat(27.0) * t246;
            let t679 = ((t95).select(-t678, f64x8::splat(0.0)));
            let t680 = param_c1 * t679;
            let t684 = t449 * t326;
            let t688 = t204 * t679;
            let t690 = -f64x8::splat(2.0) * t97 * t684 * t200 - f64x8::splat(2.0) * t327 * t205 - t680 * t99 - t97 * t688;
            let t691 = t690 * t101;
            let t692 = t331 * t207;
            let t695 = ((t94).select(f64x8::splat(0.0), t692 * t101 + t691));
            let t696 = t334 * t115;
            let t697 = t696 * t213;
            let t700 = ((t110).select(f64x8::splat(0.0), -t678));
            let t706 = ((t110).select(f64x8::splat(0.0), -t210 * t212 * t700 * t115 - f64x8::splat(2.0) * t593 * t697 - t602 * t697));
            let t707 = ((t89).select(t695, t706));
            let t710 = -f64x8::splat(4.0) * t650 * t652 + f64x8::splat(2.0) * t151 * t655 * t119 + f64x8::splat(2.0) * t151 * t669 * t119 - f64x8::splat(2.0) * t151 * t319 * t218 - t151 * t676 - t88 * t707 + f64x8::splat(1.174) * t707;
            let t717 = t230 * t231 * t342 * t234;
            let t721 = ((t3).select(f64x8::splat(0.0), -t19 * t143 * t342 * t138 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t710 * t138 - f64x8::splat(1.6891736332904388) * t717 * t238));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t721 + f64x8::splat(2.0) * t347;
            acc_v2rhotau = tv2rhotau0;
            let t724 = t274 * t274;
            let t725 = t724 * t119;
            let t728 = t27 * t50;
            let t737 = f64x8::splat(1.0) / t31 / t383;
            let t742 = t271 * t271;
            let t744 = t60 * t27;
            let t745 = t744 * t50;
            let t747 = t425 * t74 * t78;
            let t748 = t745 * t747;
            let t750 = t423 * t27;
            let t752 = t43 * t78;
            let t754 = t750 * t50 * t40 * t752;
            let t756 = -t748 / f64x8::splat(3456.0) + t754 / f64x8::splat(10368.0);
            let t759 = t44 * t728 * t55 / f64x8::splat(144.0) - f64x8::splat(9.0) / f64x8::splat(160.0) * t168 * v_sigma * t252 * t55 + f64x8::splat(243.0) / f64x8::splat(51200.0) * t390 * t45 * t737 * t398 + f64x8::splat(2.0) * t742 + f64x8::splat(2.0) * t81 * t756;
            let t760 = t759 * t119;
            let t762 = t274 * t291;
            let t765 = ((t95).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t766 = param_c1 * t765;
            let t767 = t766 * t99;
            let t768 = t278 * t278;
            let t769 = param_c1 * t768;
            let t772 = t449 * t768;
            let t775 = t204 * t765;
            let t776 = t97 * t775;
            let t777 = -f64x8::splat(2.0) * t769 * t204 - f64x8::splat(2.0) * t97 * t772 - t767 - t776;
            let t779 = t283 * t283;
            let t780 = t779 * t101;
            let t782 = ((t94).select(f64x8::splat(0.0), t777 * t101 + t780));
            let t783 = t286 * t286;
            let t788 = ((t110).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t791 = t210 * t212 * t788 * t115;
            let t796 = ((t110).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t210 * t462 * t783 * t115 - t473 * t475 * t783 * t115 - t791));
            let t797 = ((t89).select(t782, t796));
            let t800 = -f64x8::splat(2.0) * t366 * t725 + t151 * t760 - f64x8::splat(2.0) * t151 * t762 - t88 * t797 + f64x8::splat(1.174) * t797;
            let t807 = t230 * t299 * t294 * t234;
            let t811 = f64x8::splat(1.0) / t20 / t30;
            let t812 = t811 * t122;
            let t814 = t230 * t812 * t499;
            let t815 = f64x8::splat(1.0) / v_sigma;
            let t818 = t26 * t815 * t28 * t137;
            let t823 = t126 * t637 * t27 * t137;
            let t827 = t506 * t66 * t122;
            let t829 = f64x8::splat(1.0) / t127 / t45;
            let t831 = t829 * t21 * t511;
            let t835 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t800 * t138 + f64x8::splat(1.266880224967829) * t807 * t306 - f64x8::splat(2.8504805061776155) * t814 * t818 - f64x8::splat(0.31672005624195726) * t302 * t823 + f64x8::splat(3.8666622071141026) * t827 * t831));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t835;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t837 = t651 * t274;
            let t840 = t271 * t318;
            let t844 = t744 * t495;
            let t845 = t844 * t747;
            let t849 = t750 * t495 * t40 * t752;
            let t851 = t845 / f64x8::splat(432.0) - t849 / f64x8::splat(1296.0);
            let t852 = t81 * t851;
            let t859 = t274 * t339;
            let t866 = -f64x8::splat(2.0) * t97 * t684 * t278 - f64x8::splat(2.0) * t327 * t281 - t767 - t776;
            let t867 = t866 * t101;
            let t868 = t331 * t283;
            let t871 = ((t94).select(f64x8::splat(0.0), t868 * t101 + t867));
            let t872 = t696 * t286;
            let t877 = ((t110).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t593 * t872 - t602 * t872 - t791));
            let t878 = ((t89).select(t871, t877));
            let t881 = -f64x8::splat(4.0) * t650 * t837 + f64x8::splat(2.0) * t151 * t840 * t119 + f64x8::splat(2.0) * t151 * t852 * t119 - f64x8::splat(2.0) * t151 * t319 * t291 - t151 * t859 - t88 * t878 + f64x8::splat(1.174) * t878;
            let t888 = t230 * t299 * t342 * t234;
            let t892 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t881 * t138 + f64x8::splat(0.6334401124839145) * t888 * t306));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t892;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t894 = t318 * t318;
            let t895 = t82 * t894;
            let t899 = t894 * t119;
            let t902 = t744 * t627;
            let t909 = -t902 * t747 / f64x8::splat(54.0) + t750 * t627 * t40 * t752 / f64x8::splat(162.0);
            let t910 = t81 * t909;
            let t917 = t326 * t326;
            let t918 = param_c1 * t917;
            let t924 = -f64x8::splat(2.0) * t97 * t449 * t917 - f64x8::splat(2.0) * t918 * t204 - t767 - t776;
            let t926 = t331 * t331;
            let t927 = t926 * t101;
            let t929 = ((t94).select(f64x8::splat(0.0), t924 * t101 + t927));
            let t930 = t334 * t334;
            let t939 = ((t110).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t210 * t462 * t930 * t115 - t473 * t475 * t930 * t115 - t791));
            let t940 = ((t89).select(t929, t939));
            let t943 = -f64x8::splat(8.0) * t366 * t895 * t119 + f64x8::splat(2.0) * t151 * t899 + f64x8::splat(2.0) * t151 * t910 * t119 - f64x8::splat(4.0) * t151 * t319 * t339 - t88 * t940 + f64x8::splat(1.174) * t940;
            let t948 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t943 * t138));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t948;
            acc_v2tau2 = tv2tau20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2rholapl, ip, m, acc_v2rholapl);
        store_add(v2rhotau, ip, m, acc_v2rhotau);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v2sigmalapl, ip, m, acc_v2sigmalapl);
        store_add(v2sigmatau, ip, m, acc_v2sigmatau);
        store_add(v2lapl2, ip, m, acc_v2lapl2);
        store_add(v2lapltau, ip, m, acc_v2lapltau);
        store_add(v2tau2, ip, m, acc_v2tau2);
        ip += 8;
    }
}
