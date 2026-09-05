//! GGA_C_P86VWN kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_p86vwn.c`
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
pub fn gga_c_p86vwn_kxc_unpol(
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
    param_aa: f64,
    param_bb: f64,
    param_ftilde: f64,
    param_malpha: f64,
    param_mbeta: f64,
    param_mgamma: f64,
    param_mdelta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_aa = f64x8::splat(param_aa);
    let param_bb = f64x8::splat(param_bb);
    let param_ftilde = f64x8::splat(param_ftilde);
    let param_malpha = f64x8::splat(param_malpha);
    let param_mbeta = f64x8::splat(param_mbeta);
    let param_mgamma = f64x8::splat(param_mgamma);
    let param_mdelta = f64x8::splat(param_mdelta);
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
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t6 * t8;
            let t10 = t4 * t9;
            let t11 = t10 / f64x8::splat(4.0);
            let t12 = ((t10).sqrt());
            let t14 = t11 + f64x8::splat(1.86372) * t12 + f64x8::splat(12.9352);
            let t15 = f64x8::splat(1.0) / t14;
            let t19 = (simd::ln(t4 * t9 * t15 / f64x8::splat(4.0)));
            let t20 = f64x8::splat(0.0310907) * t19;
            let t21 = t12 + f64x8::splat(3.72744);
            let t24 = (simd::atan(f64x8::splat(6.15199081975908) / t21));
            let t25 = f64x8::splat(0.038783294878113016) * t24;
            let t26 = t12 / f64x8::splat(2.0);
            let t27 = t26 + f64x8::splat(0.10498);
            let t28 = t27 * t27;
            let t30 = (simd::ln(t28 * t15));
            let t31 = f64x8::splat(0.0009690227711544374) * t30;
            let t32 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t33 = f64x8::splat(1.0) / t32;
            let t35 = t11 + f64x8::splat(0.565535) * t12 + f64x8::splat(13.0045);
            let t36 = f64x8::splat(1.0) / t35;
            let t40 = (simd::ln(t4 * t9 * t36 / f64x8::splat(4.0)));
            let t41 = t12 + f64x8::splat(1.13107);
            let t44 = (simd::atan(f64x8::splat(7.123108917818118) / t41));
            let t46 = t26 + f64x8::splat(0.0047584);
            let t47 = t46 * t46;
            let t49 = (simd::ln(t47 * t36));
            let t53 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t54 = (simd::cbrt(zeta_threshold));
            let t56 = ((t53).select(t54 * zeta_threshold, f64x8::splat(1.0)));
            let t59 = f64x8::splat(M_CBRT2);
            let t60 = t59 - f64x8::splat(1.0);
            let t65 = f64x8::splat(9.0) * t56 - f64x8::splat(9.0);
            let t67 = t33 * (t40 + f64x8::splat(0.31770800474394145) * t44 + f64x8::splat(0.00041403379428206277) * t49) * t65 / f64x8::splat(24.0);
            let t68 = v_rho * v_rho;
            let t70 = f64x8::splat(1.0) / t7 / t68;
            let t71 = v_sigma * t70;
            let t72 = param_aa + param_bb;
            let t73 = param_ftilde * t72;
            let t74 = param_malpha * t1;
            let t75 = t3 * t6;
            let t76 = t75 * t8;
            let t79 = t1 * t1;
            let t80 = param_mbeta * t79;
            let t81 = t3 * t3;
            let t82 = t81 * t5;
            let t83 = t7 * t7;
            let t84 = f64x8::splat(1.0) / t83;
            let t85 = t82 * t84;
            let t88 = param_bb + t74 * t76 / f64x8::splat(4.0) + t80 * t85 / f64x8::splat(4.0);
            let t89 = param_mgamma * t1;
            let t92 = param_mdelta * t79;
            let t95 = f64x8::splat(1.0) / v_rho;
            let t98 = f64x8::splat(1.0) + t89 * t76 / f64x8::splat(4.0) + t92 * t85 / f64x8::splat(4.0) + f64x8::splat(2387.32414637843) * param_mbeta * t95;
            let t99 = f64x8::splat(1.0) / t98;
            let t101 = t88 * t99 + param_aa;
            let t102 = f64x8::splat(1.0) / t101;
            let t103 = ((v_sigma).sqrt());
            let t104 = t102 * t103;
            let t105 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t107 = f64x8::splat(1.0) / t105 / v_rho;
            let t110 = (simd::exp(-t73 * t104 * t107));
            let t112 = t54 * t54;
            let t114 = ((t53).select(t112 * zeta_threshold, f64x8::splat(1.0)));
            let t115 = ((t114).sqrt());
            let t116 = f64x8::splat(1.0) / t115;
            let t117 = t110 * t101 * t116;
            let t118 = t71 * t117;
            let tzk0 = t20 + t25 + t31 - t67 + t118;
            acc_zk = tzk0;
            let t120 = f64x8::splat(1.0) / t7 / v_rho;
            let t121 = t6 * t120;
            let t125 = t4 * t6;
            let t126 = t14 * t14;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t8 * t127;
            let t129 = t4 * t121;
            let t130 = t129 / f64x8::splat(12.0);
            let t131 = f64x8::splat(1.0) / t12;
            let t132 = t131 * t1;
            let t133 = t75 * t120;
            let t134 = t132 * t133;
            let t136 = -t130 - f64x8::splat(0.31062) * t134;
            let t142 = f64x8::splat(1.0) / t3;
            let t143 = (-t4 * t121 * t15 / f64x8::splat(12.0) - t125 * t128 * t136 / f64x8::splat(4.0)) * t79 * t142;
            let t144 = t5 * t7;
            let t145 = t144 * t14;
            let t146 = t143 * t145;
            let t148 = t21 * t21;
            let t149 = f64x8::splat(1.0) / t148;
            let t151 = t149 * t131 * t1;
            let t153 = f64x8::splat(37.8469910464) * t149 + f64x8::splat(1.0);
            let t154 = f64x8::splat(1.0) / t153;
            let t157 = t151 * t75 * t120 * t154;
            let t159 = t27 * t15;
            let t160 = t159 * t131;
            let t163 = t28 * t127;
            let t165 = -t160 * t129 / f64x8::splat(6.0) - t163 * t136;
            let t166 = f64x8::splat(1.0) / t28;
            let t167 = t165 * t166;
            let t168 = t167 * t14;
            let t173 = t35 * t35;
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t8 * t174;
            let t177 = -t130 - f64x8::splat(0.09425583333333333) * t134;
            let t183 = (-t4 * t121 * t36 / f64x8::splat(12.0) - t125 * t175 * t177 / f64x8::splat(4.0)) * t79 * t142;
            let t184 = t144 * t35;
            let t187 = t41 * t41;
            let t188 = f64x8::splat(1.0) / t187;
            let t190 = t188 * t131 * t1;
            let t192 = f64x8::splat(50.7386806551) * t188 + f64x8::splat(1.0);
            let t193 = f64x8::splat(1.0) / t192;
            let t198 = t46 * t36;
            let t199 = t198 * t131;
            let t202 = t47 * t174;
            let t204 = -t199 * t129 / f64x8::splat(6.0) - t202 * t177;
            let t205 = f64x8::splat(1.0) / t47;
            let t206 = t204 * t205;
            let t211 = t33 * (t183 * t184 / f64x8::splat(3.0) + f64x8::splat(0.37717812030896175) * t190 * t75 * t120 * t193 + f64x8::splat(0.00041403379428206277) * t206 * t35) * t65;
            let t213 = t68 * v_rho;
            let t215 = f64x8::splat(1.0) / t7 / t213;
            let t216 = v_sigma * t215;
            let t217 = t216 * t117;
            let t219 = t101 * t101;
            let t220 = f64x8::splat(1.0) / t219;
            let t221 = t73 * t220;
            let t222 = t103 * t107;
            let t226 = f64x8::splat(1.0) / t83 / v_rho;
            let t227 = t82 * t226;
            let t230 = -t74 * t133 / f64x8::splat(12.0) - t80 * t227 / f64x8::splat(6.0);
            let t232 = t98 * t98;
            let t233 = f64x8::splat(1.0) / t232;
            let t234 = t88 * t233;
            let t242 = -t89 * t133 / f64x8::splat(12.0) - t92 * t227 / f64x8::splat(6.0) - f64x8::splat(2387.32414637843) * param_mbeta / t68;
            let t244 = t230 * t99 - t234 * t242;
            let t248 = f64x8::splat(1.0) / t105 / t68;
            let t252 = t221 * t222 * t244 + f64x8::splat(7.0) / f64x8::splat(6.0) * t73 * t104 * t248;
            let t253 = t71 * t252;
            let t254 = t253 * t117;
            let t256 = t110 * t244 * t116;
            let t257 = t71 * t256;
            let tvrho0 = t20 + t25 + t31 - t67 + t118 + v_rho * (f64x8::splat(0.010363566666666667) * t146 + f64x8::splat(0.03976574567502677) * t157 + f64x8::splat(0.0009690227711544374) * t168 - t211 / f64x8::splat(24.0) - f64x8::splat(7.0) / f64x8::splat(3.0) * t217 + t254 + t257);
            acc_vrho = tvrho0;
            let t260 = t70 * t110;
            let t261 = t101 * t116;
            let t262 = t260 * t261;
            let t263 = ((v_rho).sqrt());
            let t265 = f64x8::splat(1.0) / t263 / t213;
            let t266 = t103 * t265;
            let t267 = t266 * param_ftilde;
            let t269 = t72 * t110 * t116;
            let t271 = t267 * t269 / f64x8::splat(2.0);
            let tvsigma0 = v_rho * (t262 - t271);
            acc_vsigma = tvsigma0;
            let t280 = t6 * t70;
            let t282 = t4 * t280 * t15;
            let t284 = t120 * t127;
            let t289 = f64x8::splat(1.0) / t126 / t14;
            let t290 = t8 * t289;
            let t291 = t136 * t136;
            let t295 = t4 * t280;
            let t296 = t295 / f64x8::splat(9.0);
            let t298 = f64x8::splat(1.0) / t12 / t10;
            let t299 = t298 * t79;
            let t301 = f64x8::splat(1.0) / t83 / t68;
            let t302 = t82 * t301;
            let t303 = t299 * t302;
            let t305 = t75 * t70;
            let t306 = t132 * t305;
            let t308 = t296 - f64x8::splat(0.20708) * t303 + f64x8::splat(0.41416) * t306;
            let t314 = (t282 / f64x8::splat(9.0) + t125 * t284 * t136 / f64x8::splat(6.0) + t125 * t290 * t291 / f64x8::splat(2.0) - t125 * t128 * t308 / f64x8::splat(4.0)) * t79 * t142;
            let t315 = t314 * t145;
            let t317 = t5 * t84;
            let t318 = t317 * t14;
            let t319 = t143 * t318;
            let t321 = t144 * t136;
            let t322 = t143 * t321;
            let t324 = t148 * t21;
            let t325 = f64x8::splat(1.0) / t324;
            let t326 = t325 * t1;
            let t327 = t326 * t3;
            let t329 = t327 * t280 * t154;
            let t332 = t149 * t298 * t79;
            let t335 = t332 * t82 * t301 * t154;
            let t339 = t151 * t75 * t70 * t154;
            let t341 = t148 * t148;
            let t343 = f64x8::splat(1.0) / t341 / t21;
            let t344 = t343 * t1;
            let t345 = t344 * t3;
            let t346 = t153 * t153;
            let t347 = f64x8::splat(1.0) / t346;
            let t349 = t345 * t280 * t347;
            let t352 = t27 * t127;
            let t353 = t352 * t132;
            let t354 = t120 * t136;
            let t358 = t159 * t298;
            let t359 = t79 * t81;
            let t360 = t5 * t301;
            let t361 = t359 * t360;
            let t366 = t28 * t289;
            let t370 = t282 / f64x8::splat(72.0) + t353 * t75 * t354 / f64x8::splat(3.0) - t358 * t361 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t160 * t295 + f64x8::splat(2.0) * t366 * t291 - t163 * t308;
            let t371 = t370 * t166;
            let t372 = t371 * t14;
            let t375 = f64x8::splat(1.0) / t28 / t27;
            let t376 = t165 * t375;
            let t377 = t14 * t131;
            let t378 = t376 * t377;
            let t379 = t378 * t129;
            let t381 = t167 * t136;
            let t384 = t4 * t280 * t36;
            let t386 = t120 * t174;
            let t391 = f64x8::splat(1.0) / t173 / t35;
            let t392 = t8 * t391;
            let t393 = t177 * t177;
            let t399 = t296 - f64x8::splat(0.06283722222222222) * t303 + f64x8::splat(0.12567444444444445) * t306;
            let t405 = (t384 / f64x8::splat(9.0) + t125 * t386 * t177 / f64x8::splat(6.0) + t125 * t392 * t393 / f64x8::splat(2.0) - t125 * t175 * t399 / f64x8::splat(4.0)) * t79 * t142;
            let t408 = t317 * t35;
            let t411 = t144 * t177;
            let t414 = t187 * t41;
            let t415 = f64x8::splat(1.0) / t414;
            let t416 = t415 * t1;
            let t417 = t416 * t3;
            let t422 = t188 * t298 * t79;
            let t431 = t187 * t187;
            let t433 = f64x8::splat(1.0) / t431 / t41;
            let t434 = t433 * t1;
            let t435 = t434 * t3;
            let t436 = t192 * t192;
            let t437 = f64x8::splat(1.0) / t436;
            let t442 = t46 * t174;
            let t443 = t442 * t132;
            let t444 = t120 * t177;
            let t448 = t198 * t298;
            let t453 = t47 * t391;
            let t457 = t384 / f64x8::splat(72.0) + t443 * t75 * t444 / f64x8::splat(3.0) - t448 * t361 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t199 * t295 + f64x8::splat(2.0) * t453 * t393 - t202 * t399;
            let t458 = t457 * t205;
            let t462 = f64x8::splat(1.0) / t47 / t46;
            let t463 = t204 * t462;
            let t464 = t35 * t131;
            let t465 = t463 * t464;
            let t472 = t33 * (t405 * t184 / f64x8::splat(3.0) + t183 * t408 / f64x8::splat(9.0) + t183 * t411 / f64x8::splat(3.0) + f64x8::splat(0.12572604010298724) * t417 * t280 * t193 + f64x8::splat(0.2514520802059745) * t422 * t82 * t301 * t193 - f64x8::splat(0.502904160411949) * t190 * t75 * t70 * t193 - f64x8::splat(6.379173398815766) * t435 * t280 * t437 + f64x8::splat(0.00041403379428206277) * t458 * t35 + f64x8::splat(6.900563238034379e-05) * t465 * t129 + f64x8::splat(0.00041403379428206277) * t206 * t177) * t65;
            let t474 = t68 * t68;
            let t476 = f64x8::splat(1.0) / t7 / t474;
            let t477 = v_sigma * t476;
            let t478 = t477 * t117;
            let t480 = t216 * t252;
            let t481 = t480 * t117;
            let t483 = t216 * t256;
            let t486 = f64x8::splat(1.0) / t219 / t101;
            let t487 = t73 * t486;
            let t488 = t244 * t244;
            let t492 = t103 * t248;
            let t500 = t74 * t305 / f64x8::splat(9.0) + f64x8::splat(5.0) / f64x8::splat(18.0) * t80 * t302;
            let t502 = t230 * t233;
            let t506 = f64x8::splat(1.0) / t232 / t98;
            let t507 = t88 * t506;
            let t508 = t242 * t242;
            let t518 = t89 * t305 / f64x8::splat(9.0) + f64x8::splat(5.0) / f64x8::splat(18.0) * t92 * t302 + f64x8::splat(4774.64829275686) * param_mbeta / t213;
            let t520 = -t234 * t518 - f64x8::splat(2.0) * t242 * t502 + t500 * t99 + f64x8::splat(2.0) * t507 * t508;
            let t524 = f64x8::splat(1.0) / t105 / t213;
            let t528 = -f64x8::splat(2.0) * t487 * t222 * t488 - f64x8::splat(7.0) / f64x8::splat(3.0) * t221 * t492 * t244 + t221 * t222 * t520 - f64x8::splat(91.0) / f64x8::splat(36.0) * t73 * t104 * t524;
            let t529 = t71 * t528;
            let t530 = t529 * t117;
            let t531 = t252 * t252;
            let t532 = t71 * t531;
            let t533 = t532 * t117;
            let t534 = t253 * t256;
            let t537 = t110 * t520 * t116;
            let t538 = t71 * t537;
            let t539 = f64x8::splat(0.010363566666666667) * t315 + f64x8::splat(0.003454522222222222) * t319 + f64x8::splat(0.010363566666666667) * t322 + f64x8::splat(0.013255248558342257) * t329 + f64x8::splat(0.026510497116684514) * t335 - f64x8::splat(0.05302099423336903) * t339 - f64x8::splat(0.5016712735053859) * t349 + f64x8::splat(0.0009690227711544374) * t372 + f64x8::splat(0.00016150379519240624) * t379 + f64x8::splat(0.0009690227711544374) * t381 - t472 / f64x8::splat(24.0) + f64x8::splat(70.0) / f64x8::splat(9.0) * t478 - f64x8::splat(14.0) / f64x8::splat(3.0) * t481 - f64x8::splat(14.0) / f64x8::splat(3.0) * t483 + t530 + t533 + f64x8::splat(2.0) * t534 + t538;
            let tv2rho20 = f64x8::splat(0.020727133333333335) * t146 + f64x8::splat(0.07953149135005354) * t157 + f64x8::splat(0.001938045542308875) * t168 - t211 / f64x8::splat(12.0) - f64x8::splat(14.0) / f64x8::splat(3.0) * t217 + f64x8::splat(2.0) * t254 + f64x8::splat(2.0) * t257 + v_rho * t539;
            acc_v2rho2 = tv2rho20;
            let t541 = t215 * t110;
            let t542 = t541 * t261;
            let t544 = t70 * t252;
            let t545 = t544 * t117;
            let t546 = t244 * t116;
            let t547 = t260 * t546;
            let t549 = f64x8::splat(1.0) / t263 / t474;
            let t551 = t103 * t549 * param_ftilde;
            let t552 = t551 * t269;
            let t555 = t110 * t116;
            let t556 = t72 * t252 * t555;
            let t557 = t267 * t556;
            let tv2rhosigma0 = t262 - t271 + v_rho * (-f64x8::splat(7.0) / f64x8::splat(3.0) * t542 + t545 + t547 + f64x8::splat(7.0) / f64x8::splat(4.0) * t552 - t557 / f64x8::splat(2.0));
            acc_v2rhosigma = tv2rhosigma0;
            let t562 = t265 * param_ftilde * t72;
            let t563 = f64x8::splat(1.0) / t103;
            let t564 = t563 * t110;
            let t565 = t564 * t116;
            let t567 = f64x8::splat(3.0) / f64x8::splat(4.0) * t562 * t565;
            let t569 = f64x8::splat(1.0) / t83 / t474;
            let t570 = param_ftilde * param_ftilde;
            let t571 = t569 * t570;
            let t572 = t72 * t72;
            let t573 = t571 * t572;
            let t575 = t102 * t110 * t116;
            let t577 = t573 * t575 / f64x8::splat(4.0);
            let tv2sigma20 = v_rho * (-t567 + t577);
            acc_v2sigma2 = tv2sigma20;
            let t579 = t474 * v_rho;
            let t581 = f64x8::splat(1.0) / t7 / t579;
            let t582 = v_sigma * t581;
            let t583 = t582 * t117;
            let t585 = t216 * t537;
            let t587 = t75 * t215;
            let t591 = f64x8::splat(1.0) / t83 / t213;
            let t592 = t82 * t591;
            let t595 = -f64x8::splat(7.0) / f64x8::splat(27.0) * t74 * t587 - f64x8::splat(20.0) / f64x8::splat(27.0) * t80 * t592;
            let t597 = t500 * t233;
            let t600 = t230 * t506;
            let t605 = t232 * t232;
            let t606 = f64x8::splat(1.0) / t605;
            let t607 = t88 * t606;
            let t608 = t508 * t242;
            let t611 = t242 * t518;
            let t618 = f64x8::splat(1.0) / t474;
            let t621 = -f64x8::splat(7.0) / f64x8::splat(27.0) * t89 * t587 - f64x8::splat(20.0) / f64x8::splat(27.0) * t92 * t592 - f64x8::splat(14323.94487827058) * param_mbeta * t618;
            let t623 = -t234 * t621 - f64x8::splat(3.0) * t242 * t597 - f64x8::splat(3.0) * t502 * t518 + f64x8::splat(6.0) * t507 * t611 + f64x8::splat(6.0) * t508 * t600 + t595 * t99 - f64x8::splat(6.0) * t607 * t608;
            let t625 = t110 * t623 * t116;
            let t626 = t71 * t625;
            let t627 = t6 * t215;
            let t629 = t4 * t627 * t15;
            let t631 = t70 * t127;
            let t633 = t125 * t631 * t136;
            let t635 = t27 * t289;
            let t636 = t635 * t132;
            let t640 = t352 * t299;
            let t653 = f64x8::splat(1.0) / t81;
            let t655 = t1 * t653 * t6;
            let t656 = t215 * t2;
            let t663 = f64x8::splat(1.0) / t12 / t359 / t317 / f64x8::splat(4.0);
            let t664 = t663 * t2;
            let t665 = t664 * t618;
            let t668 = t5 * t591;
            let t669 = t359 * t668;
            let t672 = t4 * t627;
            let t675 = t126 * t126;
            let t676 = f64x8::splat(1.0) / t675;
            let t677 = t28 * t676;
            let t678 = t291 * t136;
            let t681 = t136 * t308;
            let t684 = f64x8::splat(7.0) / f64x8::splat(27.0) * t672;
            let t686 = t299 * t592;
            let t688 = t132 * t587;
            let t690 = -t684 - f64x8::splat(1.24248) * t665 + f64x8::splat(0.82832) * t686 - f64x8::splat(0.9663733333333333) * t688;
            let t692 = -f64x8::splat(11.0) / f64x8::splat(216.0) * t629 - t633 / f64x8::splat(24.0) - t636 * t75 * t120 * t291 + t640 * t82 * t301 * t136 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t353 * t75 * t70 * t136 + t353 * t75 * t120 * t308 / f64x8::splat(2.0) + t655 * t656 * t15 / f64x8::splat(432.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t159 * t665 + f64x8::splat(4.0) / f64x8::splat(9.0) * t358 * t669 - f64x8::splat(14.0) / f64x8::splat(27.0) * t160 * t672 - f64x8::splat(6.0) * t677 * t678 + f64x8::splat(6.0) * t366 * t681 - t163 * t690;
            let t693 = t692 * t166;
            let t694 = t693 * t14;
            let t696 = t371 * t136;
            let t698 = t167 * t308;
            let t700 = t370 * t375;
            let t701 = t700 * t377;
            let t702 = t701 * t129;
            let t704 = t136 * t131;
            let t705 = t376 * t704;
            let t706 = t705 * t129;
            let t708 = t14 * t298;
            let t709 = t376 * t708;
            let t710 = t709 * t361;
            let t712 = t378 * t295;
            let t716 = t332 * t82 * t591 * t154;
            let t720 = t151 * t75 * t215 * t154;
            let t722 = t252 * t110;
            let t723 = t722 * t261;
            let t724 = t529 * t723;
            let t726 = f64x8::splat(1.0) / t341;
            let t727 = t726 * t79;
            let t728 = t727 * t81;
            let t729 = t154 * t131;
            let t731 = t728 * t668 * t729;
            let t734 = f64x8::splat(1.0) / t341 / t148;
            let t735 = t734 * t79;
            let t736 = t735 * t81;
            let t737 = t347 * t131;
            let t739 = t736 * t668 * t737;
            let t741 = t326 * t653;
            let t742 = t2 * t154;
            let t744 = t741 * t627 * t742;
            let t746 = t341 * t341;
            let t747 = f64x8::splat(1.0) / t746;
            let t748 = t747 * t79;
            let t749 = t748 * t81;
            let t751 = f64x8::splat(1.0) / t346 / t153;
            let t752 = t751 * t131;
            let t754 = t749 * t668 * t752;
            let t756 = t28 * t28;
            let t757 = f64x8::splat(1.0) / t756;
            let t758 = t165 * t757;
            let t759 = t758 * t14;
            let t760 = t759 * t295;
            let t762 = t531 * t252;
            let t763 = t71 * t762;
            let t764 = t763 * t117;
            let t765 = -f64x8::splat(910.0) / f64x8::splat(27.0) * t583 - f64x8::splat(7.0) * t585 + t626 + f64x8::splat(0.0009690227711544374) * t694 + f64x8::splat(0.001938045542308875) * t696 + f64x8::splat(0.0009690227711544374) * t698 + f64x8::splat(0.0003230075903848125) * t702 + f64x8::splat(0.0003230075903848125) * t706 + f64x8::splat(0.00010766919679493748) * t710 - f64x8::splat(0.00021533839358987497) * t712 - f64x8::splat(0.10604198846673805) * t716 + f64x8::splat(0.12371565321119439) * t720 + f64x8::splat(3.0) * t724 + f64x8::splat(0.026510497116684514) * t731 - f64x8::splat(2.341132609691801) * t739 + f64x8::splat(0.002209208093057043) * t744 + f64x8::splat(50.6313285242518) * t754 + f64x8::splat(4.037594879810156e-05) * t760 + t764;
            let t766 = t477 * t256;
            let t768 = t216 * t531;
            let t769 = t768 * t117;
            let t771 = t253 * t537;
            let t773 = t529 * t256;
            let t775 = t532 * t256;
            let t777 = t477 * t252;
            let t778 = t777 * t117;
            let t780 = t216 * t528;
            let t781 = t780 * t117;
            let t783 = t480 * t256;
            let t785 = t219 * t219;
            let t787 = t73 / t785;
            let t788 = t488 * t244;
            let t795 = t244 * t520;
            let t799 = t103 * t524;
            let t809 = f64x8::splat(1.0) / t105 / t474;
            let t813 = f64x8::splat(6.0) * t787 * t222 * t788 + f64x8::splat(7.0) * t487 * t492 * t488 - f64x8::splat(6.0) * t487 * t222 * t795 + f64x8::splat(91.0) / f64x8::splat(12.0) * t221 * t799 * t244 - f64x8::splat(7.0) / f64x8::splat(2.0) * t221 * t492 * t520 + t221 * t222 * t623 + f64x8::splat(1729.0) / f64x8::splat(216.0) * t73 * t104 * t809;
            let t814 = t71 * t813;
            let t815 = t814 * t117;
            let t816 = t47 * t47;
            let t817 = f64x8::splat(1.0) / t816;
            let t818 = t204 * t817;
            let t819 = t818 * t35;
            let t822 = t627 * t437;
            let t825 = t144 * t399;
            let t828 = t434 * t653;
            let t831 = t317 * t177;
            let t836 = t188 * t663;
            let t837 = t2 * t618;
            let t842 = t4 * t627 * t36;
            let t844 = t70 * t174;
            let t846 = t125 * t844 * t177;
            let t848 = t120 * t391;
            let t855 = t173 * t173;
            let t856 = f64x8::splat(1.0) / t855;
            let t857 = t8 * t856;
            let t858 = t393 * t177;
            let t862 = t177 * t399;
            let t869 = -t684 - f64x8::splat(0.3770233333333333) * t665 + f64x8::splat(0.2513488888888889) * t686 - f64x8::splat(0.2932403703703704) * t688;
            let t875 = (-f64x8::splat(7.0) / f64x8::splat(27.0) * t842 - t846 / f64x8::splat(3.0) - t125 * t848 * t393 / f64x8::splat(2.0) + t125 * t386 * t399 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t125 * t857 * t858 + f64x8::splat(3.0) / f64x8::splat(2.0) * t125 * t392 * t862 - t125 * t175 * t869 / f64x8::splat(4.0)) * t79 * t142;
            let t880 = t431 * t431;
            let t881 = f64x8::splat(1.0) / t880;
            let t882 = t881 * t79;
            let t883 = t882 * t81;
            let t885 = f64x8::splat(1.0) / t436 / t192;
            let t886 = t885 * t131;
            let t892 = t46 * t391;
            let t893 = t892 * t132;
            let t897 = t442 * t299;
            let t919 = t47 * t856;
            let t925 = -f64x8::splat(11.0) / f64x8::splat(216.0) * t842 - t846 / f64x8::splat(24.0) - t893 * t75 * t120 * t393 + t897 * t82 * t301 * t177 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t443 * t75 * t70 * t177 + t443 * t75 * t120 * t399 / f64x8::splat(2.0) + t655 * t656 * t36 / f64x8::splat(432.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t198 * t665 + f64x8::splat(4.0) / f64x8::splat(9.0) * t448 * t669 - f64x8::splat(14.0) / f64x8::splat(27.0) * t199 * t672 - f64x8::splat(6.0) * t919 * t858 + f64x8::splat(6.0) * t453 * t862 - t202 * t869;
            let t926 = t925 * t205;
            let t931 = f64x8::splat(1.7251408095085948e-05) * t819 * t295 + f64x8::splat(23.390302462324474) * t435 * t822 + t183 * t825 / f64x8::splat(3.0) - f64x8::splat(0.3384256597539519) * t828 * t822 + f64x8::splat(2.0) / f64x8::splat(9.0) * t183 * t831 + f64x8::splat(2.0) / f64x8::splat(3.0) * t405 * t411 + f64x8::splat(1.508712481235847) * t836 * t837 * t193 + t875 * t184 / f64x8::splat(3.0) - f64x8::splat(9.200750984045839e-05) * t465 * t295 + f64x8::splat(863.1222451360587) * t883 * t668 * t886 + f64x8::splat(0.00041403379428206277) * t926 * t35 + f64x8::splat(0.0008280675885641255) * t458 * t177;
            let t942 = t457 * t462;
            let t943 = t942 * t464;
            let t946 = t177 * t131;
            let t947 = t463 * t946;
            let t950 = t35 * t298;
            let t951 = t463 * t950;
            let t956 = t5 * t226;
            let t957 = t956 * t35;
            let t963 = f64x8::splat(1.0) / t431;
            let t964 = t963 * t79;
            let t965 = t964 * t81;
            let t966 = t193 * t131;
            let t971 = f64x8::splat(1.0) / t431 / t187;
            let t972 = t971 * t79;
            let t973 = t972 * t81;
            let t974 = t437 * t131;
            let t978 = t416 * t653;
            let t979 = t2 * t193;
            let t983 = f64x8::splat(0.00041403379428206277) * t206 * t399 - f64x8::splat(1.005808320823898) * t422 * t82 * t591 * t193 + f64x8::splat(1.1734430409612142) * t190 * t75 * t215 * t193 + f64x8::splat(0.00013801126476068758) * t943 * t129 + f64x8::splat(0.00013801126476068758) * t947 * t129 + f64x8::splat(4.6003754920229193e-05) * t951 * t361 + f64x8::splat(2.0) / f64x8::splat(9.0) * t405 * t408 - f64x8::splat(2.0) / f64x8::splat(27.0) * t183 * t957 - f64x8::splat(0.4609954803776199) * t417 * t627 * t193 + f64x8::splat(0.2514520802059745) * t965 * t668 * t966 - f64x8::splat(29.76947586114024) * t973 * t668 * t974 + f64x8::splat(0.02095434001716454) * t978 * t627 * t979;
            let t986 = t33 * (t931 + t983) * t65;
            let t988 = t149 * t663;
            let t990 = t988 * t837 * t154;
            let t992 = t344 * t653;
            let t993 = t627 * t347;
            let t994 = t992 * t993;
            let t996 = t345 * t993;
            let t998 = t317 * t136;
            let t999 = t143 * t998;
            let t1001 = t144 * t308;
            let t1002 = t143 * t1001;
            let t1004 = t314 * t321;
            let t1008 = t120 * t289;
            let t1015 = t8 * t676;
            let t1027 = (-f64x8::splat(7.0) / f64x8::splat(27.0) * t629 - t633 / f64x8::splat(3.0) - t125 * t1008 * t291 / f64x8::splat(2.0) + t125 * t284 * t308 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t125 * t1015 * t678 + f64x8::splat(3.0) / f64x8::splat(2.0) * t125 * t290 * t681 - t125 * t128 * t690 / f64x8::splat(4.0)) * t79 * t142;
            let t1028 = t1027 * t145;
            let t1030 = t314 * t318;
            let t1032 = t956 * t14;
            let t1033 = t143 * t1032;
            let t1036 = t327 * t627 * t154;
            let t1038 = f64x8::splat(70.0) / f64x8::splat(3.0) * t766 - f64x8::splat(7.0) * t769 + f64x8::splat(3.0) * t771 + f64x8::splat(3.0) * t773 + f64x8::splat(3.0) * t775 + f64x8::splat(70.0) / f64x8::splat(3.0) * t778 - f64x8::splat(7.0) * t781 - f64x8::splat(14.0) * t783 + t815 - t986 / f64x8::splat(24.0) + f64x8::splat(0.15906298270010708) * t990 - f64x8::splat(0.026614487661862786) * t994 + f64x8::splat(1.839461336186415) * t996 + f64x8::splat(0.006909044444444444) * t999 + f64x8::splat(0.010363566666666667) * t1002 + f64x8::splat(0.020727133333333335) * t1004 + f64x8::splat(0.010363566666666667) * t1028 + f64x8::splat(0.006909044444444444) * t1030 - f64x8::splat(0.002303014814814815) * t1033 - f64x8::splat(0.04860257804725494) * t1036;
            let tv3rho30 = v_rho * (t765 + t1038) - t472 / f64x8::splat(8.0) - f64x8::splat(0.15906298270010708) * t339 + f64x8::splat(0.0004845113855772187) * t379 - f64x8::splat(14.0) * t483 + f64x8::splat(3.0) * t538 + f64x8::splat(0.0029070683134633122) * t372 + f64x8::splat(0.0029070683134633122) * t381 + f64x8::splat(0.0310907) * t315 + f64x8::splat(0.0310907) * t322 + f64x8::splat(0.03976574567502677) * t329 - f64x8::splat(1.5050138205161576) * t349 + f64x8::splat(70.0) / f64x8::splat(3.0) * t478 - f64x8::splat(14.0) * t481 + f64x8::splat(3.0) * t530 + f64x8::splat(6.0) * t534 + f64x8::splat(0.010363566666666667) * t319 + f64x8::splat(0.07953149135005354) * t335 + f64x8::splat(3.0) * t533;
            acc_v3rho3 = tv3rho30;
            let t1063 = t476 * t110;
            let t1064 = t1063 * t261;
            let t1066 = t215 * t252;
            let t1067 = t1066 * t117;
            let t1069 = t541 * t546;
            let t1071 = t70 * t528;
            let t1072 = t1071 * t117;
            let t1073 = t70 * t531;
            let t1074 = t1073 * t117;
            let t1075 = t544 * t256;
            let t1077 = t520 * t116;
            let t1078 = t260 * t1077;
            let t1080 = f64x8::splat(1.0) / t263 / t579;
            let t1082 = t103 * t1080 * param_ftilde;
            let t1083 = t1082 * t269;
            let t1085 = t551 * t556;
            let t1088 = t72 * t528 * t555;
            let t1089 = t267 * t1088;
            let t1092 = t72 * t531 * t555;
            let t1093 = t267 * t1092;
            let t1095 = f64x8::splat(70.0) / f64x8::splat(9.0) * t1064 - f64x8::splat(14.0) / f64x8::splat(3.0) * t1067 - f64x8::splat(14.0) / f64x8::splat(3.0) * t1069 + t1072 + t1074 + f64x8::splat(2.0) * t1075 + t1078 - f64x8::splat(63.0) / f64x8::splat(8.0) * t1083 + f64x8::splat(7.0) / f64x8::splat(2.0) * t1085 - t1089 / f64x8::splat(2.0) - t1093 / f64x8::splat(2.0);
            let tv3rho2sigma0 = -f64x8::splat(14.0) / f64x8::splat(3.0) * t542 + f64x8::splat(2.0) * t545 + f64x8::splat(2.0) * t547 + f64x8::splat(7.0) / f64x8::splat(2.0) * t552 - t557 + v_rho * t1095;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t1098 = t549 * param_ftilde * t72;
            let t1099 = t1098 * t565;
            let t1102 = t563 * t252 * t555;
            let t1103 = t562 * t1102;
            let t1108 = f64x8::splat(1.0) / t83 / t579 * t570 * t572;
            let t1109 = t1108 * t575;
            let t1111 = t220 * t110;
            let t1112 = t1111 * t546;
            let t1113 = t573 * t1112;
            let t1116 = t102 * t252 * t555;
            let t1117 = t573 * t1116;
            let tv3rhosigma20 = -t567 + t577 + v_rho * (f64x8::splat(21.0) / f64x8::splat(8.0) * t1099 - f64x8::splat(3.0) / f64x8::splat(4.0) * t1103 - f64x8::splat(7.0) / f64x8::splat(6.0) * t1109 - t1113 / f64x8::splat(4.0) + t1117 / f64x8::splat(4.0));
            acc_v3rhosigma2 = tv3rhosigma20;
            let t1122 = f64x8::splat(1.0) / t103 / v_sigma;
            let t1124 = t1122 * t110 * t116;
            let t1126 = f64x8::splat(3.0) / f64x8::splat(8.0) * t562 * t1124;
            let t1127 = f64x8::splat(1.0) / v_sigma;
            let t1129 = t1127 * t102 * t555;
            let t1131 = f64x8::splat(3.0) / f64x8::splat(8.0) * t573 * t1129;
            let t1132 = t105 * t105;
            let t1133 = t1132 * t1132;
            let t1134 = t1133 * t105;
            let t1137 = t570 * param_ftilde;
            let t1138 = f64x8::splat(1.0) / t1134 / t579 * t1137;
            let t1139 = t572 * t72;
            let t1140 = t1138 * t1139;
            let t1142 = t220 * t563 * t555;
            let t1144 = t1140 * t1142 / f64x8::splat(8.0);
            let tv3sigma30 = v_rho * (t1126 + t1131 - t1144);
            acc_v3sigma3 = tv3sigma30;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v3rho2sigma, ip, m, acc_v3rho2sigma);
        store_add(v3rhosigma2, ip, m, acc_v3rhosigma2);
        store_add(v3sigma3, ip, m, acc_v3sigma3);
        ip += 8;
    }
}
