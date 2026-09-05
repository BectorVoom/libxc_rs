//! GGA_X_PBE_ERF_GWS fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbe_erf_gws.c`
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
pub fn gga_x_pbe_erf_gws_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_ax: f64,
    param_b_PBE: f64,
    param_kappa: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_ax = f64x8::splat(param_ax);
    let param_b_PBE = f64x8::splat(param_b_PBE);
    let param_kappa = f64x8::splat(param_kappa);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = param_hyb_omega_0 * param_hyb_omega_0;
            let t4 = param_ax * t3;
            let t5 = f64x8::splat(M_CBRT3);
            let t7 = f64x8::splat(M_CBRTPI);
            let t8 = t7 * f64x8::splat(M_PI);
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t11 = (simd::cbrt(zeta_threshold));
            let t12 = f64x8::splat(M_CBRT2);
            let t13 = ((t10).select(t11, t12));
            let t14 = t13 * t13;
            let t15 = f64x8::splat(1.0) / t14;
            let t16 = t9 * t15;
            let t17 = t12 * t12;
            let t18 = (simd::cbrt(v_rho));
            let t19 = t18 * t18;
            let t20 = f64x8::splat(1.0) / t19;
            let t25 = (simd::exp(-t4 * t5 * t16 * t17 * t20 / f64x8::splat(12.0)));
            let t26 = param_b_PBE * t25;
            let t27 = t26 * v_sigma;
            let t28 = param_kappa + f64x8::splat(1.0);
            let t29 = t5 * t28;
            let t30 = t5 * t5;
            let t31 = t12 * t30;
            let t32 = t7 * t7;
            let t34 = t31 / t32;
            let t35 = f64x8::splat(1.0) / t18;
            let t37 = f64x8::splat(1.0) / t13;
            let t40 = t34 * param_hyb_omega_0 * t35 * t37 / f64x8::splat(6.0);
            let t41 = (t40).simd_lt(f64x8::splat(0.05));
            let t42 = t14 * t14;
            let t43 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t44 = t32 * t43;
            let t45 = t42 * t44;
            let t46 = t18 * v_rho;
            let t47 = t45 * t46;
            let t49 = t14 * t8;
            let t50 = t49 * t17;
            let t52 = t5 * t19 * t3;
            let t53 = t50 * t52;
            let t55 = f64x8::splat(7.0) * t47 - f64x8::splat(6.0) * t53;
            let t56 = t14 * t13;
            let t57 = f64x8::splat(1.0) / param_hyb_omega_0;
            let t64 = (simd::erf(t57 * t5 * t32 * t13 * t17 * t18 / f64x8::splat(2.0)));
            let t66 = ((f64x8::splat(M_PI)).sqrt());
            let t67 = t66 * t43;
            let t68 = t56 * t64 * t67;
            let t69 = v_rho * param_hyb_omega_0;
            let t75 = t3 * t3;
            let t76 = t75 * t30;
            let t78 = f64x8::splat(12.0) * t76 * t12;
            let t79 = -f64x8::splat(36.0) * t68 * t31 * t69 + f64x8::splat(81.0) * t47 + f64x8::splat(54.0) * t53 - t78;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = (f64x8::splat(10000000000.0)).simd_lt(t40);
            let t83 = t43 * t43;
            let t84 = v_rho * v_rho;
            let t86 = t42 * t14;
            let t90 = t44 * t17 * t5;
            let t96 = t8 * t12 * t30;
            let t102 = t75 * t3;
            let t103 = f64x8::splat(1.0) / t102;
            let t107 = f64x8::splat(1.0) / t3;
            let t108 = t107 * t30;
            let t113 = t108 * t8 * t14 * t12 * t19 / f64x8::splat(2.0);
            let t114 = (simd::exp(t113));
            let t115 = t114 * t8;
            let t118 = t5 * t3;
            let t119 = t14 * t17 * t118;
            let t123 = t114 * t12;
            let t127 = (f64x8::splat(7.0) * t115 * t19 * t119 - f64x8::splat(12.0) * t123 * t76 + f64x8::splat(6.0) * t47 + f64x8::splat(11.0) * t53 + t78) * t8;
            let t128 = t19 * t14;
            let t129 = t127 * t128;
            let t130 = t17 * t30;
            let t131 = t42 * t114;
            let t132 = t44 * t12;
            let t136 = t56 * t114;
            let t143 = t14 * t114 * t8;
            let t148 = t114 * t17;
            let t153 = f64x8::splat(12.0) * t136 * t64 * t67 * t130 * t69 - f64x8::splat(27.0) * t131 * t132 * t46 - f64x8::splat(4.0) * t130 * t75 - f64x8::splat(36.0) * t143 * t52 + f64x8::splat(4.0) * t148 * t76 + f64x8::splat(24.0) * t49 * t52;
            let t156 = t130 * t107 / t153;
            let t159 = ((t41).select(t55 * t80, (t82).select((f64x8::splat(5600.0) * t96 * t19 * t75 * t14 - f64x8::splat(140.0) * t90 * t46 * t3 * t42 - f64x8::splat(1863.0) * t83 * t84 * t86) * t103 / f64x8::splat(201600.0), -t129 * t156 / f64x8::splat(18.0))));
            let t163 = t19 * t84;
            let t165 = param_kappa * t163 * t8;
            let t166 = f64x8::splat(27.0) / f64x8::splat(28.0) * t27 * t29 * t159 + t165;
            let t167 = t166 * t46;
            let t170 = ((t10).select(t11 * zeta_threshold, f64x8::splat(2.0) * t12));
            let t171 = t170 * t17;
            let t172 = t167 * t171;
            let t173 = (f64x8::splat(1.35)).simd_le(t40);
            let t174 = (f64x8::splat(1.35)).simd_lt(t40);
            let t175 = ((t174).select(t40, f64x8::splat(1.35)));
            let t176 = t175 * t175;
            let t177 = t176 * t176;
            let t178 = t177 * t176;
            let t179 = t177 * t177;
            let t182 = t179 * t177;
            let t184 = t179 * t176;
            let t190 = f64x8::splat(24088884019200.0) * t179 * t178 + f64x8::splat(19448.0) * t176 - f64x8::splat(807840.0) * t177 + f64x8::splat(30551040.0) * t178 - f64x8::splat(1045524480.0) * t179 - f64x8::splat(903333150720.0) * t182 + f64x8::splat(32261898240.0) * t184 - f64x8::splat(429.0);
            let t191 = t179 * t179;
            let t192 = f64x8::splat(1.0) / t191;
            let t195 = ((t174).select(f64x8::splat(1.35), t40));
            let t196 = t195 * t195;
            let t197 = t196 * t196;
            let t200 = f64x8::splat(32.0) * t197 - f64x8::splat(16.0) * t196;
            let t203 = (simd::exp(-f64x8::splat(1.0) / t196 / f64x8::splat(4.0)));
            let t207 = f64x8::splat(1.0) / t195;
            let t209 = (simd::erf(t207 / f64x8::splat(2.0)));
            let t210 = t66 * t209;
            let t215 = ((t173).select(t190 * t192 / f64x8::splat(867199824691200.0), t200 * t203 / f64x8::splat(3.0) - f64x8::splat(32.0) / f64x8::splat(3.0) * t197 - f64x8::splat(8.0) / f64x8::splat(3.0) * t210 * t195 + f64x8::splat(8.0) * t196 + f64x8::splat(1.0)));
            let t216 = f64x8::splat(1.0) / t7;
            let t217 = t215 * t216;
            let t218 = param_b_PBE * t159;
            let t220 = t25 * v_sigma * t5;
            let t224 = f64x8::splat(864.0) * t218 * t220 + f64x8::splat(896.0) * t165;
            let t225 = f64x8::splat(1.0) / t224;
            let t226 = t5 * t225;
            let t227 = t217 * t226;
            let t230 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(84.0) * t172 * t227));
            let t231 = f64x8::splat(1.0) / v_rho;
            let tzk0 = f64x8::splat(2.0) * t230 * t231;
            acc_zk = tzk0;
            let t234 = param_b_PBE * param_ax * t3;
            let t237 = t234 * t30 * t9 * t15;
            let t238 = t19 * v_rho;
            let t239 = f64x8::splat(1.0) / t238;
            let t241 = t17 * t239 * t25;
            let t242 = v_sigma * t28;
            let t243 = t242 * t159;
            let t247 = t45 * t18;
            let t249 = t5 * t35;
            let t250 = t249 * t3;
            let t251 = t50 * t250;
            let t253 = f64x8::splat(28.0) / f64x8::splat(3.0) * t247 - f64x8::splat(4.0) * t251;
            let t255 = t79 * t79;
            let t256 = f64x8::splat(1.0) / t255;
            let t257 = t55 * t256;
            let t258 = (simd::exp(-t113));
            let t267 = -f64x8::splat(72.0) * t45 * t258 * t18 - f64x8::splat(36.0) * t68 * t31 * param_hyb_omega_0 + f64x8::splat(108.0) * t247 + f64x8::splat(36.0) * t251;
            let t293 = (f64x8::splat(8.0) * t247 + f64x8::splat(14.0) * t45 * t18 * t114 - f64x8::splat(22.0) / f64x8::splat(3.0) * t115 * t35 * t119 + f64x8::splat(22.0) / f64x8::splat(3.0) * t251) * t8;
            let t294 = t293 * t128;
            let t297 = t35 * t14;
            let t298 = t127 * t297;
            let t301 = t153 * t153;
            let t303 = t107 / t301;
            let t304 = t86 * t107;
            let t305 = t304 * t30;
            let t306 = t83 * t17;
            let t316 = t42 * t13 * t57 * t5;
            let t317 = t43 * f64x8::splat(M_PI);
            let t318 = (simd::pow(f64x8::splat(M_PI), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t319 = t318 * t318;
            let t320 = t319 * t319;
            let t321 = t320 * t318;
            let t322 = t321 * t317;
            let t324 = t114 * t64;
            let t328 = t131 * t44;
            let t329 = t258 * t12;
            let t343 = f64x8::splat(12.0) * t136 * t64 * t67 * t17 * t30 * param_hyb_omega_0 - f64x8::splat(9.0) * t305 * t306 * v_rho * t114 + f64x8::splat(24.0) * t316 * t322 * t19 * t324 - f64x8::splat(72.0) * t131 * t132 * t18 + f64x8::splat(24.0) * t328 * t329 * t18 - f64x8::splat(16.0) * t143 * t250 + f64x8::splat(16.0) * t49 * t250;
            let t345 = t130 * t303 * t343;
            let t349 = ((t41).select(t253 * t80 - t257 * t267, (t82).select((-f64x8::splat(3726.0) * t83 * v_rho * t86 - f64x8::splat(560.0) / f64x8::splat(3.0) * t90 * t18 * t3 * t42 + f64x8::splat(11200.0) / f64x8::splat(3.0) * t96 * t35 * t75 * t14) * t103 / f64x8::splat(201600.0), -t294 * t156 / f64x8::splat(18.0) - t298 * t156 / f64x8::splat(27.0) + t129 * t345 / f64x8::splat(18.0))));
            let t354 = param_kappa * t238 * t8;
            let t356 = f64x8::splat(3.0) / f64x8::splat(56.0) * t237 * t241 * t243 + f64x8::splat(27.0) / f64x8::splat(28.0) * t27 * t29 * t349 + f64x8::splat(8.0) / f64x8::splat(3.0) * t354;
            let t357 = t356 * t46;
            let t358 = t357 * t171;
            let t361 = t166 * t18;
            let t362 = t361 * t171;
            let t365 = t177 * t175;
            let t366 = t179 * t365;
            let t367 = f64x8::splat(1.0) / t46;
            let t371 = t34 * param_hyb_omega_0 * t367 * t37 / f64x8::splat(18.0);
            let t372 = ((t174).select(-t371, f64x8::splat(0.0)));
            let t375 = t176 * t175;
            let t376 = t179 * t375;
            let t379 = t179 * t175;
            let t382 = t177 * t375;
            let t391 = f64x8::splat(38896.0) * t175 * t372 + f64x8::splat(183306240.0) * t365 * t372 + f64x8::splat(337244376268800.0) * t366 * t372 - f64x8::splat(3231360.0) * t375 * t372 - f64x8::splat(10839997808640.0) * t376 * t372 + f64x8::splat(322618982400.0) * t379 * t372 - f64x8::splat(8364195840.0) * t382 * t372;
            let t395 = f64x8::splat(1.0) / t191 / t175;
            let t396 = t190 * t395;
            let t400 = t196 * t195;
            let t401 = ((t174).select(f64x8::splat(0.0), -t371));
            let t402 = t400 * t401;
            let t404 = t195 * t401;
            let t406 = f64x8::splat(128.0) * t402 - f64x8::splat(32.0) * t404;
            let t409 = f64x8::splat(1.0) / t400;
            let t410 = t200 * t409;
            let t411 = t401 * t203;
            let t415 = t203 * t207;
            let t422 = ((t173).select(t391 * t192 / f64x8::splat(867199824691200.0) - t396 * t372 / f64x8::splat(54199989043200.0), t406 * t203 / f64x8::splat(3.0) + t410 * t411 / f64x8::splat(6.0) - f64x8::splat(128.0) / f64x8::splat(3.0) * t402 + f64x8::splat(8.0) / f64x8::splat(3.0) * t415 * t401 - f64x8::splat(8.0) / f64x8::splat(3.0) * t210 * t401 + f64x8::splat(16.0) * t404));
            let t423 = t422 * t216;
            let t424 = t423 * t226;
            let t427 = t224 * t224;
            let t428 = f64x8::splat(1.0) / t427;
            let t429 = t5 * t428;
            let t430 = param_b_PBE * t349;
            let t433 = t4 * t30;
            let t434 = t218 * t433;
            let t435 = t16 * t17;
            let t438 = t435 * t239 * t25 * v_sigma;
            let t442 = f64x8::splat(864.0) * t430 * t220 + f64x8::splat(48.0) * t434 * t438 + f64x8::splat(7168.0) / f64x8::splat(3.0) * t354;
            let t443 = t429 * t442;
            let t444 = t217 * t443;
            let t448 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(84.0) * t172 * t424 + f64x8::splat(84.0) * t172 * t444 - f64x8::splat(84.0) * t358 * t227 - f64x8::splat(112.0) * t362 * t227));
            let tvrho0 = f64x8::splat(2.0) * t448;
            acc_vrho = tvrho0;
            let t449 = t30 * t28;
            let t451 = t26 * t449 * t159;
            let t453 = t46 * t170 * t17;
            let t454 = t217 * t225;
            let t455 = t453 * t454;
            let t458 = t171 * t215;
            let t459 = t167 * t458;
            let t461 = t216 * t30 * t428;
            let t463 = t461 * t218 * t25;
            let t467 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(81.0) * t451 * t455 + f64x8::splat(72576.0) * t459 * t463));
            let tvsigma0 = f64x8::splat(2.0) * t467;
            acc_vsigma = tvsigma0;
            let t468 = f64x8::splat(1.0) / t163;
            let t470 = t17 * t468 * t25;
            let t474 = param_ax * param_ax;
            let t475 = param_b_PBE * t474;
            let t476 = f64x8::splat(1.0) / t44;
            let t478 = f64x8::splat(1.0) / t42;
            let t480 = t475 * t75 * t476 * t478;
            let t481 = t84 * v_rho;
            let t483 = f64x8::splat(1.0) / t18 / t481;
            let t485 = t12 * t483 * t25;
            let t489 = t242 * t349;
            let t493 = t45 * t20;
            let t496 = t5 * t367 * t3;
            let t497 = t50 * t496;
            let t499 = f64x8::splat(28.0) / f64x8::splat(9.0) * t493 + f64x8::splat(4.0) / f64x8::splat(3.0) * t497;
            let t501 = t253 * t256;
            let t505 = f64x8::splat(1.0) / t255 / t79;
            let t506 = t55 * t505;
            let t507 = t267 * t267;
            let t510 = t86 * t83;
            let t511 = t510 * t107;
            let t515 = t258 * t20;
            let t520 = f64x8::splat(24.0) * t511 * t31 * t258 - f64x8::splat(96.0) * t45 * t515 + f64x8::splat(36.0) * t493 - f64x8::splat(12.0) * t497;
            let t547 = (f64x8::splat(8.0) / f64x8::splat(3.0) * t493 - f64x8::splat(10.0) * t45 * t20 * t114 + f64x8::splat(14.0) / f64x8::splat(3.0) * t511 * t31 * t114 + f64x8::splat(22.0) / f64x8::splat(9.0) * t115 * t367 * t119 - f64x8::splat(22.0) / f64x8::splat(9.0) * t497) * t8;
            let t548 = t547 * t128;
            let t551 = t293 * t297;
            let t556 = t367 * t14;
            let t557 = t127 * t556;
            let t563 = f64x8::splat(1.0) / t301 / t153;
            let t564 = t107 * t563;
            let t565 = t343 * t343;
            let t567 = t130 * t564 * t565;
            let t573 = t42 * t42;
            let t574 = f64x8::splat(1.0) / t75;
            let t576 = t573 * t574 * t5;
            let t577 = t83 * f64x8::splat(M_PI);
            let t578 = t7 * t577;
            let t591 = t3 * param_hyb_omega_0;
            let t595 = t42 * t56 / t591 * t318 * t577;
            let t596 = t18 * t12;
            let t601 = t258 * t17;
            let t612 = -f64x8::splat(33.0) * t305 * t306 * t114 - f64x8::splat(18.0) * t576 * t578 * t19 * t114 - f64x8::splat(40.0) * t131 * t132 * t20 + f64x8::splat(40.0) * t316 * t322 * t35 * t324 + f64x8::splat(24.0) * t595 * t596 * t324 + f64x8::splat(8.0) * t305 * t83 * t114 * t601 + f64x8::splat(32.0) * t328 * t329 * t20 + f64x8::splat(16.0) / f64x8::splat(3.0) * t143 * t496 - f64x8::splat(16.0) / f64x8::splat(3.0) * t49 * t496;
            let t614 = t130 * t303 * t612;
            let t618 = ((t41).select(-t257 * t520 - f64x8::splat(2.0) * t501 * t267 + t499 * t80 + f64x8::splat(2.0) * t506 * t507, (t82).select((-f64x8::splat(3726.0) * t510 - f64x8::splat(560.0) / f64x8::splat(9.0) * t90 * t20 * t3 * t42 - f64x8::splat(11200.0) / f64x8::splat(9.0) * t96 * t367 * t75 * t14) * t103 / f64x8::splat(201600.0), -t548 * t156 / f64x8::splat(18.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t551 * t156 + t294 * t345 / f64x8::splat(9.0) + t557 * t156 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(27.0) * t298 * t345 - t129 * t567 / f64x8::splat(9.0) + t129 * t614 / f64x8::splat(18.0))));
            let t623 = param_kappa * t19 * t8;
            let t625 = -f64x8::splat(5.0) / f64x8::splat(56.0) * t237 * t470 * t243 + t480 * t485 * t243 / f64x8::splat(56.0) + f64x8::splat(3.0) / f64x8::splat(28.0) * t237 * t241 * t489 + f64x8::splat(27.0) / f64x8::splat(28.0) * t27 * t29 * t618 + f64x8::splat(40.0) / f64x8::splat(9.0) * t623;
            let t626 = t625 * t46;
            let t627 = t626 * t171;
            let t630 = t356 * t18;
            let t631 = t630 * t171;
            let t638 = t166 * t20;
            let t639 = t638 * t171;
            let t646 = t372 * t372;
            let t650 = f64x8::splat(1.0) / t18 / t84;
            let t654 = f64x8::splat(2.0) / f64x8::splat(27.0) * t34 * param_hyb_omega_0 * t650 * t37;
            let t655 = ((t174).select(t654, f64x8::splat(0.0)));
            let t681 = f64x8::splat(38896.0) * t175 * t655 - f64x8::splat(9694080.0) * t176 * t646 + f64x8::splat(916531200.0) * t177 * t646 - f64x8::splat(58549370880.0) * t178 * t646 + f64x8::splat(2903570841600.0) * t179 * t646 + f64x8::splat(4384176891494400.0) * t182 * t646 - f64x8::splat(119239975895040.0) * t184 * t646 + f64x8::splat(183306240.0) * t365 * t655 + f64x8::splat(337244376268800.0) * t366 * t655 - f64x8::splat(3231360.0) * t375 * t655 - f64x8::splat(10839997808640.0) * t376 * t655 + f64x8::splat(322618982400.0) * t379 * t655 - f64x8::splat(8364195840.0) * t382 * t655 + f64x8::splat(38896.0) * t646;
            let t684 = t391 * t395;
            let t688 = f64x8::splat(1.0) / t191 / t176;
            let t689 = t190 * t688;
            let t695 = t401 * t401;
            let t696 = t196 * t695;
            let t698 = ((t174).select(f64x8::splat(0.0), t654));
            let t699 = t400 * t698;
            let t702 = t195 * t698;
            let t704 = f64x8::splat(384.0) * t696 + f64x8::splat(128.0) * t699 - f64x8::splat(32.0) * t695 - f64x8::splat(32.0) * t702;
            let t707 = t406 * t409;
            let t710 = f64x8::splat(1.0) / t197;
            let t711 = t200 * t710;
            let t712 = t695 * t203;
            let t715 = t698 * t203;
            let t719 = f64x8::splat(1.0) / t197 / t196;
            let t720 = t200 * t719;
            let t734 = t704 * t203 / f64x8::splat(3.0) + t707 * t411 / f64x8::splat(3.0) - t711 * t712 / f64x8::splat(2.0) + t410 * t715 / f64x8::splat(6.0) + t720 * t712 / f64x8::splat(12.0) - f64x8::splat(128.0) * t696 - f64x8::splat(128.0) / f64x8::splat(3.0) * t699 + f64x8::splat(4.0) / f64x8::splat(3.0) * t710 * t695 * t203 + f64x8::splat(8.0) / f64x8::splat(3.0) * t415 * t698 - f64x8::splat(8.0) / f64x8::splat(3.0) * t210 * t698 + f64x8::splat(16.0) * t695 + f64x8::splat(16.0) * t702;
            let t735 = ((t173).select(t681 * t192 / f64x8::splat(867199824691200.0) - t684 * t372 / f64x8::splat(27099994521600.0) + t689 * t646 / f64x8::splat(3188234649600.0) - t396 * t655 / f64x8::splat(54199989043200.0), t734));
            let t736 = t735 * t216;
            let t737 = t736 * t226;
            let t740 = t423 * t443;
            let t744 = f64x8::splat(1.0) / t427 / t224;
            let t745 = t5 * t744;
            let t746 = t442 * t442;
            let t747 = t745 * t746;
            let t748 = t217 * t747;
            let t751 = param_b_PBE * t618;
            let t754 = t430 * t433;
            let t759 = t435 * t468 * t25 * v_sigma;
            let t762 = t474 * t75;
            let t763 = t762 * t476;
            let t764 = t218 * t763;
            let t765 = t478 * t12;
            let t768 = t765 * t483 * t25 * v_sigma;
            let t772 = f64x8::splat(864.0) * t751 * t220 + f64x8::splat(96.0) * t754 * t438 - f64x8::splat(80.0) * t434 * t759 + f64x8::splat(16.0) * t764 * t768 + f64x8::splat(35840.0) / f64x8::splat(9.0) * t623;
            let t773 = t429 * t772;
            let t774 = t217 * t773;
            let t777 = -f64x8::splat(84.0) * t627 * t227 - f64x8::splat(224.0) * t631 * t227 - f64x8::splat(168.0) * t358 * t424 + f64x8::splat(168.0) * t358 * t444 - f64x8::splat(112.0) / f64x8::splat(3.0) * t639 * t227 - f64x8::splat(224.0) * t362 * t424 + f64x8::splat(224.0) * t362 * t444 - f64x8::splat(84.0) * t172 * t737 + f64x8::splat(168.0) * t172 * t740 - f64x8::splat(168.0) * t172 * t748 + f64x8::splat(84.0) * t172 * t774;
            let t778 = ((t2).select(f64x8::splat(0.0), t777));
            let tv2rho20 = f64x8::splat(2.0) * t778;
            acc_v2rho2 = tv2rho20;
            let t780 = f64x8::splat(1.0) / t32 / f64x8::splat(M_PI);
            let t781 = t780 * t15;
            let t783 = t234 * t781 * t12;
            let t785 = t35 * t25 * t28;
            let t786 = t159 * t170;
            let t787 = t215 * t225;
            let t788 = t786 * t787;
            let t793 = t26 * t449 * t349;
            let t797 = t18 * t170 * t17;
            let t798 = t797 * t454;
            let t801 = t423 * t225;
            let t802 = t453 * t801;
            let t805 = t26 * t30;
            let t806 = t28 * t159;
            let t807 = t806 * t46;
            let t808 = t805 * t807;
            let t809 = t216 * t428;
            let t810 = t809 * t442;
            let t811 = t458 * t810;
            let t814 = t357 * t458;
            let t817 = t361 * t458;
            let t820 = t171 * t422;
            let t821 = t167 * t820;
            let t824 = t167 * t170;
            let t826 = t17 * t215 * t216;
            let t827 = t824 * t826;
            let t829 = t30 * t744 * param_b_PBE;
            let t830 = t159 * t25;
            let t832 = t829 * t830 * t442;
            let t836 = t461 * t430 * t25;
            let t840 = t166 * t35 * t170;
            let t841 = t12 * t215;
            let t842 = t841 * t780;
            let t843 = t840 * t842;
            let t844 = t428 * param_b_PBE;
            let t847 = t4 * t15 * t25;
            let t848 = t844 * t159 * t847;
            let t851 = -f64x8::splat(27.0) * t783 * t785 * t788 - f64x8::splat(108.0) * t451 * t798 - f64x8::splat(81.0) * t451 * t802 - f64x8::splat(81.0) * t793 * t455 + f64x8::splat(72576.0) * t459 * t836 + f64x8::splat(72576.0) * t814 * t463 + f64x8::splat(96768.0) * t817 * t463 + f64x8::splat(72576.0) * t821 * t463 + f64x8::splat(81.0) * t808 * t811 - f64x8::splat(145152.0) * t827 * t832 + f64x8::splat(24192.0) * t843 * t848;
            let t852 = ((t2).select(f64x8::splat(0.0), t851));
            let tv2rhosigma0 = f64x8::splat(2.0) * t852;
            acc_v2rhosigma = tv2rhosigma0;
            let t853 = param_b_PBE * param_b_PBE;
            let t854 = t25 * t25;
            let t855 = t853 * t854;
            let t856 = t159 * t159;
            let t857 = t28 * t856;
            let t859 = t855 * t857 * t46;
            let t861 = t171 * t217 * t428;
            let t864 = t216 * t744;
            let t865 = t853 * t856;
            let t866 = t865 * t854;
            let t867 = t864 * t866;
            let t871 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(376233984.0) * t459 * t867 + f64x8::splat(419904.0) * t859 * t861));
            let tv2sigma20 = f64x8::splat(2.0) * t871;
            acc_v2sigma2 = tv2sigma20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        ip += 8;
    }
}
