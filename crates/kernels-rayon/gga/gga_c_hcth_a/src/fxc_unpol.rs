//! GGA_C_HCTH_A fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_hcth_a.c`
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
pub fn gga_c_hcth_a_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
            let t3 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t4 = ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold)) | (t3);
            let t5 = ((t3).select(zeta_threshold, f64x8::splat(1.0)));
            let t6 = f64x8::splat(M_CBRT3);
            let t7 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t8 = (simd::cbrt(t7));
            let t9 = t6 * t8;
            let t10 = f64x8::splat(M_CBRT4);
            let t11 = t10 * t10;
            let t12 = t9 * t11;
            let t13 = (simd::cbrt(v_rho));
            let t14 = f64x8::splat(1.0) / t13;
            let t15 = f64x8::splat(M_CBRT2);
            let t16 = t14 * t15;
            let t17 = (simd::cbrt(zeta_threshold));
            let t19 = ((t3).select(f64x8::splat(1.0) / t17, f64x8::splat(1.0)));
            let t21 = t12 * t16 * t19;
            let t22 = t21 / f64x8::splat(4.0);
            let t23 = ((t21).sqrt());
            let t25 = t22 + f64x8::splat(1.86372) * t23 + f64x8::splat(12.9352);
            let t26 = f64x8::splat(1.0) / t25;
            let t27 = t19 * t26;
            let t31 = (simd::ln(t12 * t16 * t27 / f64x8::splat(4.0)));
            let t32 = f64x8::splat(0.0310907) * t31;
            let t33 = t23 + f64x8::splat(3.72744);
            let t36 = (simd::atan(f64x8::splat(6.15199081975908) / t33));
            let t37 = f64x8::splat(0.038783294878113016) * t36;
            let t38 = t23 / f64x8::splat(2.0);
            let t39 = t38 + f64x8::splat(0.10498);
            let t40 = t39 * t39;
            let t42 = (simd::ln(t40 * t26));
            let t43 = f64x8::splat(0.0009690227711544374) * t42;
            let t45 = t22 + f64x8::splat(3.53021) * t23 + f64x8::splat(18.0578);
            let t46 = f64x8::splat(1.0) / t45;
            let t47 = t19 * t46;
            let t51 = (simd::ln(t12 * t16 * t47 / f64x8::splat(4.0)));
            let t53 = t23 + f64x8::splat(7.06042);
            let t56 = (simd::atan(f64x8::splat(4.730926909560113) / t53));
            let t58 = t38 + f64x8::splat(0.325);
            let t59 = t58 * t58;
            let t61 = (simd::ln(t59 * t46));
            let t65 = t17 * zeta_threshold;
            let t67 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t65, f64x8::splat(2.0) * t15));
            let t69 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t65, f64x8::splat(0.0)));
            let t70 = t67 + t69 - f64x8::splat(2.0);
            let t72 = t15 - f64x8::splat(1.0);
            let t74 = f64x8::splat(1.0) / t72 / f64x8::splat(2.0);
            let t79 = ((t4).select(f64x8::splat(0.0), t5 * (t32 + t37 + t43 + (f64x8::splat(0.01554535) * t51 + f64x8::splat(0.05249139316978094) * t56 + f64x8::splat(0.0022478670955426118) * t61 - t32 - t37 - t43) * t70 * t74) / f64x8::splat(2.0)));
            let t80 = t15 * t15;
            let t81 = v_sigma * t80;
            let t82 = v_rho * v_rho;
            let t83 = t13 * t13;
            let t85 = f64x8::splat(1.0) / t83 / t82;
            let t86 = t81 * t85;
            let t88 = f64x8::splat(1.0) + f64x8::splat(0.2) * t86;
            let t89 = f64x8::splat(1.0) / t88;
            let t93 = v_sigma * v_sigma;
            let t94 = t93 * t15;
            let t95 = t82 * t82;
            let t96 = t95 * v_rho;
            let t98 = f64x8::splat(1.0) / t13 / t96;
            let t99 = t88 * t88;
            let t100 = f64x8::splat(1.0) / t99;
            let t101 = t98 * t100;
            let t104 = t93 * v_sigma;
            let t105 = t95 * t95;
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t104 * t106;
            let t108 = t99 * t88;
            let t109 = f64x8::splat(1.0) / t108;
            let t112 = f64x8::splat(0.0136823) + f64x8::splat(0.053784) * t81 * t85 * t89 - f64x8::splat(0.04406152) * t94 * t101 + f64x8::splat(0.03326304) * t107 * t109;
            let t114 = f64x8::splat(2.0) * t79 * t112;
            let t115 = t11 * t14;
            let t116 = t9 * t115;
            let t117 = t116 / f64x8::splat(4.0);
            let t118 = ((t116).sqrt());
            let t120 = t117 + f64x8::splat(1.86372) * t118 + f64x8::splat(12.9352);
            let t121 = f64x8::splat(1.0) / t120;
            let t125 = (simd::ln(t9 * t115 * t121 / f64x8::splat(4.0)));
            let t127 = t118 + f64x8::splat(3.72744);
            let t130 = (simd::atan(f64x8::splat(6.15199081975908) / t127));
            let t132 = t118 / f64x8::splat(2.0);
            let t133 = t132 + f64x8::splat(0.10498);
            let t134 = t133 * t133;
            let t136 = (simd::ln(t134 * t121));
            let t138 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t139 = f64x8::splat(1.0) / t138;
            let t141 = t117 + f64x8::splat(0.565535) * t118 + f64x8::splat(13.0045);
            let t142 = f64x8::splat(1.0) / t141;
            let t146 = (simd::ln(t9 * t115 * t142 / f64x8::splat(4.0)));
            let t147 = t118 + f64x8::splat(1.13107);
            let t150 = (simd::atan(f64x8::splat(7.123108917818118) / t147));
            let t152 = t132 + f64x8::splat(0.0047584);
            let t153 = t152 * t152;
            let t155 = (simd::ln(t153 * t142));
            let t159 = ((t3).select(t65, f64x8::splat(1.0)));
            let t164 = f64x8::splat(9.0) * (f64x8::splat(2.0) * t159 - f64x8::splat(2.0)) * t74 * t72;
            let t168 = f64x8::splat(0.0310907) * t125 + f64x8::splat(0.038783294878113016) * t130 + f64x8::splat(0.0009690227711544374) * t136 - t139 * (t146 + f64x8::splat(0.31770800474394145) * t150 + f64x8::splat(0.00041403379428206277) * t155) * t164 / f64x8::splat(24.0) - f64x8::splat(2.0) * t79;
            let t170 = f64x8::splat(1.0) + f64x8::splat(0.006) * t86;
            let t171 = f64x8::splat(1.0) / t170;
            let t175 = t170 * t170;
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t98 * t176;
            let t180 = t175 * t170;
            let t181 = f64x8::splat(1.0) / t180;
            let t184 = f64x8::splat(0.836897) + f64x8::splat(0.01032306) * t81 * t85 * t171 - f64x8::splat(0.00020051856) * t94 * t177 - f64x8::splat(3.95283456e-06) * t107 * t181;
            let t185 = t168 * t184;
            let tzk0 = t114 + t185;
            acc_zk = tzk0;
            let t187 = f64x8::splat(1.0) / t13 / v_rho;
            let t188 = t187 * t15;
            let t192 = t15 * t19;
            let t193 = t25 * t25;
            let t194 = f64x8::splat(1.0) / t193;
            let t195 = t188 * t19;
            let t197 = t12 * t195 / f64x8::splat(12.0);
            let t198 = f64x8::splat(1.0) / t23;
            let t199 = t198 * t6;
            let t200 = t199 * t8;
            let t201 = t11 * t187;
            let t203 = t200 * t201 * t192;
            let t205 = -t197 - f64x8::splat(0.31062) * t203;
            let t207 = t192 * t194 * t205;
            let t211 = t6 * t6;
            let t213 = f64x8::splat(1.0) / t8;
            let t214 = t213 * t10;
            let t215 = (-t12 * t188 * t27 / f64x8::splat(12.0) - t116 * t207 / f64x8::splat(4.0)) * t211 * t214;
            let t216 = t13 * t80;
            let t217 = f64x8::splat(1.0) / t19;
            let t218 = t217 * t25;
            let t219 = t216 * t218;
            let t221 = f64x8::splat(0.005181783333333334) * t215 * t219;
            let t222 = t33 * t33;
            let t223 = f64x8::splat(1.0) / t222;
            let t225 = t223 * t198 * t9;
            let t227 = f64x8::splat(37.8469910464) * t223 + f64x8::splat(1.0);
            let t228 = f64x8::splat(1.0) / t227;
            let t229 = t192 * t228;
            let t232 = f64x8::splat(0.03976574567502677) * t225 * t201 * t229;
            let t233 = t39 * t26;
            let t234 = t233 * t199;
            let t235 = t8 * t11;
            let t236 = t235 * t195;
            let t239 = t40 * t194;
            let t241 = -t234 * t236 / f64x8::splat(6.0) - t239 * t205;
            let t242 = f64x8::splat(1.0) / t40;
            let t243 = t241 * t242;
            let t245 = f64x8::splat(0.0009690227711544374) * t243 * t25;
            let t249 = t45 * t45;
            let t250 = f64x8::splat(1.0) / t249;
            let t252 = -t197 - f64x8::splat(0.5883683333333334) * t203;
            let t254 = t192 * t250 * t252;
            let t259 = (-t12 * t188 * t47 / f64x8::splat(12.0) - t116 * t254 / f64x8::splat(4.0)) * t211 * t214;
            let t260 = t217 * t45;
            let t261 = t216 * t260;
            let t264 = t53 * t53;
            let t265 = f64x8::splat(1.0) / t264;
            let t267 = t265 * t198 * t9;
            let t269 = f64x8::splat(22.3816694236) * t265 + f64x8::splat(1.0);
            let t270 = f64x8::splat(1.0) / t269;
            let t271 = t192 * t270;
            let t275 = t58 * t46;
            let t276 = t275 * t199;
            let t279 = t59 * t250;
            let t281 = -t276 * t236 / f64x8::splat(6.0) - t279 * t252;
            let t282 = f64x8::splat(1.0) / t59;
            let t283 = t281 * t282;
            let t292 = ((t4).select(f64x8::splat(0.0), t5 * (t221 + t232 + t245 + (f64x8::splat(0.002590891666666667) * t259 * t261 + f64x8::splat(0.041388824077869424) * t267 * t201 * t271 + f64x8::splat(0.0022478670955426118) * t283 * t45 - t221 - t232 - t245) * t70 * t74) / f64x8::splat(2.0)));
            let t293 = t292 * t112;
            let t295 = t82 * v_rho;
            let t297 = f64x8::splat(1.0) / t83 / t295;
            let t301 = t95 * t82;
            let t303 = f64x8::splat(1.0) / t13 / t301;
            let t307 = t105 * v_rho;
            let t308 = f64x8::splat(1.0) / t307;
            let t309 = t104 * t308;
            let t312 = t93 * t93;
            let t313 = t105 * t295;
            let t315 = f64x8::splat(1.0) / t83 / t313;
            let t316 = t312 * t315;
            let t317 = t99 * t99;
            let t318 = f64x8::splat(1.0) / t317;
            let t319 = t318 * t80;
            let t322 = -f64x8::splat(0.143424) * t81 * t297 * t89 + f64x8::splat(0.2923643733333333) * t94 * t303 * t100 - f64x8::splat(0.36010222933333336) * t309 * t109 + f64x8::splat(0.053220864) * t316 * t319;
            let t323 = t79 * t322;
            let t328 = t120 * t120;
            let t329 = f64x8::splat(1.0) / t328;
            let t330 = t14 * t329;
            let t331 = t9 * t201;
            let t332 = t331 / f64x8::splat(12.0);
            let t333 = f64x8::splat(1.0) / t118;
            let t334 = t333 * t6;
            let t336 = t334 * t235 * t187;
            let t338 = -t332 - f64x8::splat(0.31062) * t336;
            let t344 = (-t9 * t201 * t121 / f64x8::splat(12.0) - t12 * t330 * t338 / f64x8::splat(4.0)) * t211 * t213;
            let t345 = t10 * t13;
            let t346 = t345 * t120;
            let t349 = t127 * t127;
            let t350 = f64x8::splat(1.0) / t349;
            let t352 = t350 * t333 * t6;
            let t354 = f64x8::splat(37.8469910464) * t350 + f64x8::splat(1.0);
            let t355 = f64x8::splat(1.0) / t354;
            let t360 = t133 * t121;
            let t361 = t360 * t333;
            let t364 = t134 * t329;
            let t366 = -t361 * t331 / f64x8::splat(6.0) - t364 * t338;
            let t367 = f64x8::splat(1.0) / t134;
            let t368 = t366 * t367;
            let t374 = t141 * t141;
            let t375 = f64x8::splat(1.0) / t374;
            let t376 = t14 * t375;
            let t378 = -t332 - f64x8::splat(0.09425583333333333) * t336;
            let t384 = (-t9 * t201 * t142 / f64x8::splat(12.0) - t12 * t376 * t378 / f64x8::splat(4.0)) * t211 * t213;
            let t385 = t345 * t141;
            let t388 = t147 * t147;
            let t389 = f64x8::splat(1.0) / t388;
            let t391 = t389 * t333 * t6;
            let t393 = f64x8::splat(50.7386806551) * t389 + f64x8::splat(1.0);
            let t394 = f64x8::splat(1.0) / t393;
            let t399 = t152 * t142;
            let t400 = t399 * t333;
            let t403 = t153 * t375;
            let t405 = -t400 * t331 / f64x8::splat(6.0) - t403 * t378;
            let t406 = f64x8::splat(1.0) / t153;
            let t407 = t405 * t406;
            let t415 = f64x8::splat(0.010363566666666667) * t344 * t346 + f64x8::splat(0.03976574567502677) * t352 * t235 * t187 * t355 + f64x8::splat(0.0009690227711544374) * t368 * t120 - t139 * (t384 * t385 / f64x8::splat(3.0) + f64x8::splat(0.37717812030896175) * t391 * t235 * t187 * t394 + f64x8::splat(0.00041403379428206277) * t407 * t141) * t164 / f64x8::splat(24.0) - f64x8::splat(2.0) * t292;
            let t416 = t415 * t184;
            let t425 = t175 * t175;
            let t426 = f64x8::splat(1.0) / t425;
            let t427 = t426 * t80;
            let t430 = -f64x8::splat(0.02752816) * t81 * t297 * t171 + f64x8::splat(0.00139977024) * t94 * t303 * t176 + f64x8::splat(1.878948864e-05) * t309 * t181 - f64x8::splat(1.8973605888e-07) * t316 * t427;
            let t431 = t168 * t430;
            let tvrho0 = t114 + t185 + v_rho * (f64x8::splat(2.0) * t293 + f64x8::splat(2.0) * t323 + t416 + t431);
            acc_vrho = tvrho0;
            let t434 = t80 * t85;
            let t437 = v_sigma * t15;
            let t440 = t93 * t106;
            let t443 = t105 * t82;
            let t445 = f64x8::splat(1.0) / t83 / t443;
            let t446 = t104 * t445;
            let t449 = f64x8::splat(0.053784) * t434 * t89 - f64x8::splat(0.10963664) * t437 * t101 + f64x8::splat(0.135038336) * t440 * t109 - f64x8::splat(0.019957824) * t446 * t319;
            let t451 = f64x8::splat(2.0) * t79 * t449;
            let t460 = f64x8::splat(0.01032306) * t434 * t171 - f64x8::splat(0.00052491384) * t437 * t177 - f64x8::splat(7.04605824e-06) * t440 * t181 + f64x8::splat(7.115102208e-08) * t446 * t427;
            let t461 = t168 * t460;
            let tvsigma0 = v_rho * (t451 + t461);
            acc_vsigma = tvsigma0;
            let t468 = f64x8::splat(1.0) / t13 / t82;
            let t469 = t468 * t15;
            let t471 = t12 * t469 * t27;
            let t476 = f64x8::splat(1.0) / t193 / t25;
            let t477 = t205 * t205;
            let t479 = t192 * t476 * t477;
            let t482 = t469 * t19;
            let t484 = t12 * t482 / f64x8::splat(9.0);
            let t486 = f64x8::splat(1.0) / t23 / t21;
            let t487 = t486 * t211;
            let t488 = t8 * t8;
            let t489 = t487 * t488;
            let t490 = t10 * t85;
            let t491 = t19 * t19;
            let t492 = t80 * t491;
            let t494 = t489 * t490 * t492;
            let t496 = t11 * t468;
            let t498 = t200 * t496 * t192;
            let t500 = t484 - f64x8::splat(0.20708) * t494 + f64x8::splat(0.41416) * t498;
            let t502 = t192 * t194 * t500;
            let t507 = (t471 / f64x8::splat(9.0) + t331 * t207 / f64x8::splat(6.0) + t116 * t479 / f64x8::splat(2.0) - t116 * t502 / f64x8::splat(4.0)) * t211 * t214;
            let t509 = f64x8::splat(0.005181783333333334) * t507 * t219;
            let t510 = f64x8::splat(1.0) / t83;
            let t511 = t510 * t80;
            let t512 = t511 * t218;
            let t514 = f64x8::splat(0.001727261111111111) * t215 * t512;
            let t515 = t217 * t205;
            let t516 = t216 * t515;
            let t518 = f64x8::splat(0.005181783333333334) * t215 * t516;
            let t519 = t222 * t33;
            let t521 = f64x8::splat(1.0) / t519 * t6;
            let t522 = t521 * t235;
            let t523 = t19 * t228;
            let t526 = f64x8::splat(0.013255248558342257) * t522 * t469 * t523;
            let t528 = t211 * t488;
            let t529 = t223 * t486 * t528;
            let t530 = t492 * t228;
            let t533 = f64x8::splat(0.026510497116684514) * t529 * t490 * t530;
            let t536 = f64x8::splat(0.05302099423336903) * t225 * t496 * t229;
            let t537 = t222 * t222;
            let t539 = f64x8::splat(1.0) / t537 / t33;
            let t540 = t539 * t6;
            let t541 = t540 * t235;
            let t542 = t227 * t227;
            let t543 = f64x8::splat(1.0) / t542;
            let t544 = t19 * t543;
            let t547 = f64x8::splat(0.5016712735053859) * t541 * t469 * t544;
            let t549 = t39 * t194;
            let t550 = t549 * t200;
            let t551 = t192 * t205;
            let t555 = t233 * t487;
            let t556 = t488 * t10;
            let t558 = t556 * t434 * t491;
            let t561 = t235 * t482;
            let t564 = t40 * t476;
            let t568 = t471 / f64x8::splat(72.0) + t550 * t201 * t551 / f64x8::splat(3.0) - t555 * t558 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t234 * t561 + f64x8::splat(2.0) * t564 * t477 - t239 * t500;
            let t569 = t568 * t242;
            let t571 = f64x8::splat(0.0009690227711544374) * t569 * t25;
            let t573 = f64x8::splat(1.0) / t40 / t39;
            let t574 = t241 * t573;
            let t576 = t25 * t198 * t6;
            let t577 = t574 * t576;
            let t579 = f64x8::splat(0.00016150379519240624) * t577 * t236;
            let t581 = f64x8::splat(0.0009690227711544374) * t243 * t205;
            let t583 = t12 * t469 * t47;
            let t588 = f64x8::splat(1.0) / t249 / t45;
            let t589 = t252 * t252;
            let t591 = t192 * t588 * t589;
            let t596 = t484 - f64x8::splat(0.39224555555555557) * t494 + f64x8::splat(0.7844911111111111) * t498;
            let t598 = t192 * t250 * t596;
            let t603 = (t583 / f64x8::splat(9.0) + t331 * t254 / f64x8::splat(6.0) + t116 * t591 / f64x8::splat(2.0) - t116 * t598 / f64x8::splat(4.0)) * t211 * t214;
            let t606 = t511 * t260;
            let t609 = t217 * t252;
            let t610 = t216 * t609;
            let t613 = t264 * t53;
            let t615 = f64x8::splat(1.0) / t613 * t6;
            let t616 = t615 * t235;
            let t617 = t19 * t270;
            let t622 = t265 * t486 * t528;
            let t623 = t492 * t270;
            let t630 = t264 * t264;
            let t632 = f64x8::splat(1.0) / t630 / t53;
            let t633 = t632 * t6;
            let t634 = t633 * t235;
            let t635 = t269 * t269;
            let t636 = f64x8::splat(1.0) / t635;
            let t637 = t19 * t636;
            let t642 = t58 * t250;
            let t643 = t642 * t200;
            let t644 = t192 * t252;
            let t648 = t275 * t487;
            let t653 = t59 * t588;
            let t657 = t583 / f64x8::splat(72.0) + t643 * t201 * t644 / f64x8::splat(3.0) - t648 * t558 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t276 * t561 + f64x8::splat(2.0) * t653 * t589 - t279 * t596;
            let t658 = t657 * t282;
            let t662 = f64x8::splat(1.0) / t59 / t58;
            let t663 = t281 * t662;
            let t665 = t45 * t198 * t6;
            let t666 = t663 * t665;
            let t671 = f64x8::splat(0.002590891666666667) * t603 * t261 + f64x8::splat(0.0008636305555555555) * t259 * t606 + f64x8::splat(0.002590891666666667) * t259 * t610 + f64x8::splat(0.013796274692623142) * t616 * t469 * t617 + f64x8::splat(0.027592549385246284) * t622 * t490 * t623 - f64x8::splat(0.05518509877049257) * t267 * t496 * t271 - f64x8::splat(0.3087836594474698) * t634 * t469 * t637 + f64x8::splat(0.0022478670955426118) * t658 * t45 + f64x8::splat(0.00037464451592376865) * t666 * t236 + f64x8::splat(0.0022478670955426118) * t283 * t252 - t509 - t514 - t518 - t526 - t533 + t536 + t547 - t571 - t579 - t581;
            let t674 = t671 * t70 * t74 + t509 + t514 + t518 + t526 + t533 - t536 - t547 + t571 + t579 + t581;
            let t677 = ((t4).select(f64x8::splat(0.0), t5 * t674 / f64x8::splat(2.0)));
            let t678 = t677 * t112;
            let t680 = t292 * t322;
            let t683 = f64x8::splat(1.0) / t83 / t95;
            let t687 = t95 * t295;
            let t689 = f64x8::splat(1.0) / t13 / t687;
            let t693 = f64x8::splat(1.0) / t443;
            let t694 = t104 * t693;
            let t697 = t105 * t95;
            let t699 = f64x8::splat(1.0) / t83 / t697;
            let t700 = t312 * t699;
            let t703 = t312 * v_sigma;
            let t706 = f64x8::splat(1.0) / t13 / t105 / t687;
            let t707 = t703 * t706;
            let t710 = f64x8::splat(1.0) / t317 / t88 * t15;
            let t713 = f64x8::splat(0.525888) * t81 * t683 * t89 - f64x8::splat(2.004626631111111) * t94 * t689 * t100 + f64x8::splat(3.8646307271111113) * t694 * t109 - f64x8::splat(1.1970736469333334) * t700 * t319 + f64x8::splat(0.2270756864) * t707 * t710;
            let t714 = t79 * t713;
            let t717 = t9 * t496 * t121;
            let t719 = t187 * t329;
            let t724 = f64x8::splat(1.0) / t328 / t120;
            let t725 = t14 * t724;
            let t726 = t338 * t338;
            let t730 = t9 * t496;
            let t731 = t730 / f64x8::splat(9.0);
            let t733 = f64x8::splat(1.0) / t118 / t116;
            let t734 = t733 * t211;
            let t736 = t734 * t556 * t85;
            let t739 = t334 * t235 * t468;
            let t741 = t731 - f64x8::splat(0.20708) * t736 + f64x8::splat(0.41416) * t739;
            let t747 = (t717 / f64x8::splat(9.0) + t12 * t719 * t338 / f64x8::splat(6.0) + t12 * t725 * t726 / f64x8::splat(2.0) - t12 * t330 * t741 / f64x8::splat(4.0)) * t211 * t213;
            let t750 = t10 * t510;
            let t751 = t750 * t120;
            let t754 = t345 * t338;
            let t757 = t349 * t127;
            let t759 = f64x8::splat(1.0) / t757 * t6;
            let t760 = t759 * t8;
            let t765 = t350 * t733 * t211;
            let t774 = t349 * t349;
            let t776 = f64x8::splat(1.0) / t774 / t127;
            let t777 = t776 * t6;
            let t778 = t777 * t8;
            let t779 = t354 * t354;
            let t780 = f64x8::splat(1.0) / t779;
            let t785 = t133 * t329;
            let t786 = t785 * t334;
            let t787 = t187 * t338;
            let t791 = t360 * t733;
            let t792 = t528 * t490;
            let t797 = t134 * t724;
            let t801 = t717 / f64x8::splat(72.0) + t786 * t235 * t787 / f64x8::splat(3.0) - t791 * t792 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t361 * t730 + f64x8::splat(2.0) * t797 * t726 - t364 * t741;
            let t802 = t801 * t367;
            let t806 = f64x8::splat(1.0) / t134 / t133;
            let t807 = t366 * t806;
            let t808 = t120 * t333;
            let t809 = t807 * t808;
            let t815 = t9 * t496 * t142;
            let t817 = t187 * t375;
            let t822 = f64x8::splat(1.0) / t374 / t141;
            let t823 = t14 * t822;
            let t824 = t378 * t378;
            let t830 = t731 - f64x8::splat(0.06283722222222222) * t736 + f64x8::splat(0.12567444444444445) * t739;
            let t836 = (t815 / f64x8::splat(9.0) + t12 * t817 * t378 / f64x8::splat(6.0) + t12 * t823 * t824 / f64x8::splat(2.0) - t12 * t376 * t830 / f64x8::splat(4.0)) * t211 * t213;
            let t839 = t750 * t141;
            let t842 = t345 * t378;
            let t845 = t388 * t147;
            let t847 = f64x8::splat(1.0) / t845 * t6;
            let t848 = t847 * t8;
            let t853 = t389 * t733 * t211;
            let t862 = t388 * t388;
            let t864 = f64x8::splat(1.0) / t862 / t147;
            let t865 = t864 * t6;
            let t866 = t865 * t8;
            let t867 = t393 * t393;
            let t868 = f64x8::splat(1.0) / t867;
            let t873 = t152 * t375;
            let t874 = t873 * t334;
            let t875 = t187 * t378;
            let t879 = t399 * t733;
            let t884 = t153 * t822;
            let t888 = t815 / f64x8::splat(72.0) + t874 * t235 * t875 / f64x8::splat(3.0) - t879 * t792 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t400 * t730 + f64x8::splat(2.0) * t884 * t824 - t403 * t830;
            let t889 = t888 * t406;
            let t893 = f64x8::splat(1.0) / t153 / t152;
            let t894 = t405 * t893;
            let t895 = t141 * t333;
            let t896 = t894 * t895;
            let t906 = f64x8::splat(0.010363566666666667) * t747 * t346 + f64x8::splat(0.003454522222222222) * t344 * t751 + f64x8::splat(0.010363566666666667) * t344 * t754 + f64x8::splat(0.013255248558342257) * t760 * t496 * t355 + f64x8::splat(0.026510497116684514) * t765 * t556 * t85 * t355 - f64x8::splat(0.05302099423336903) * t352 * t235 * t468 * t355 - f64x8::splat(0.5016712735053859) * t778 * t496 * t780 + f64x8::splat(0.0009690227711544374) * t802 * t120 + f64x8::splat(0.00016150379519240624) * t809 * t331 + f64x8::splat(0.0009690227711544374) * t368 * t338 - t139 * (t836 * t385 / f64x8::splat(3.0) + t384 * t839 / f64x8::splat(9.0) + t384 * t842 / f64x8::splat(3.0) + f64x8::splat(0.12572604010298724) * t848 * t496 * t394 + f64x8::splat(0.2514520802059745) * t853 * t556 * t85 * t394 - f64x8::splat(0.502904160411949) * t391 * t235 * t468 * t394 - f64x8::splat(6.379173398815766) * t866 * t496 * t868 + f64x8::splat(0.00041403379428206277) * t889 * t141 + f64x8::splat(6.900563238034379e-05) * t896 * t331 + f64x8::splat(0.00041403379428206277) * t407 * t378) * t164 / f64x8::splat(24.0) - f64x8::splat(2.0) * t677;
            let t907 = t906 * t184;
            let t908 = t415 * t430;
            let t922 = f64x8::splat(1.0) / t425 / t170 * t15;
            let t925 = f64x8::splat(0.10093658666666666) * t81 * t683 * t171 - f64x8::splat(0.00974611264) * t94 * t689 * t176 - f64x8::splat(7.95201024e-05) * t694 * t181 + f64x8::splat(3.11548280832e-06) * t700 * t427 - f64x8::splat(2.428621553664e-08) * t707 * t922;
            let t926 = t168 * t925;
            let tv2rho20 = f64x8::splat(4.0) * t293 + f64x8::splat(4.0) * t323 + f64x8::splat(2.0) * t416 + f64x8::splat(2.0) * t431 + v_rho * (f64x8::splat(2.0) * t678 + f64x8::splat(4.0) * t680 + f64x8::splat(2.0) * t714 + t907 + f64x8::splat(2.0) * t908 + t926);
            acc_v2rho2 = tv2rho20;
            let t929 = t292 * t449;
            let t931 = t80 * t297;
            let t934 = t15 * t303;
            let t935 = t100 * v_sigma;
            let t938 = t93 * t308;
            let t941 = t104 * t315;
            let t944 = t105 * t301;
            let t946 = f64x8::splat(1.0) / t13 / t944;
            let t947 = t312 * t946;
            let t950 = -f64x8::splat(0.143424) * t931 * t89 + f64x8::splat(0.6420983466666667) * t934 * t935 - f64x8::splat(1.3141981866666668) * t938 * t109 + f64x8::splat(0.4289447936) * t941 * t319 - f64x8::splat(0.0851533824) * t947 * t710;
            let t951 = t79 * t950;
            let t953 = t415 * t460;
            let t956 = t176 * v_sigma;
            let t965 = -f64x8::splat(0.02752816) * t931 * t171 + f64x8::splat(0.0031298784) * t934 * t956 + f64x8::splat(2.277398016e-05) * t938 * t181 - f64x8::splat(1.09715503104e-06) * t941 * t427 + f64x8::splat(9.10733082624e-09) * t947 * t922;
            let t966 = t168 * t965;
            let tv2rhosigma0 = t451 + t461 + v_rho * (f64x8::splat(2.0) * t929 + f64x8::splat(2.0) * t951 + t953 + t966);
            acc_v2rhosigma = tv2rhosigma0;
            let t969 = t15 * t98;
            let t972 = v_sigma * t106;
            let t975 = t93 * t445;
            let t978 = t105 * t96;
            let t980 = f64x8::splat(1.0) / t13 / t978;
            let t981 = t104 * t980;
            let t984 = -f64x8::splat(0.13115024) * t969 * t100 + f64x8::splat(0.357785984) * t972 * t109 - f64x8::splat(0.1408964736) * t975 * t319 + f64x8::splat(0.0319325184) * t981 * t710;
            let t986 = f64x8::splat(2.0) * t79 * t984;
            let t995 = -f64x8::splat(0.00064879056) * t969 * t176 - f64x8::splat(1.49418432e-06) * t972 * t181 + f64x8::splat(3.4028211456e-07) * t975 * t427 - f64x8::splat(3.41524905984e-09) * t981 * t922;
            let t996 = t168 * t995;
            let tv2sigma20 = v_rho * (t986 + t996);
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
