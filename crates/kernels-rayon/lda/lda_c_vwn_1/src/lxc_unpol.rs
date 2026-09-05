//! LDA_C_VWN_1 lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_1.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_vwn_1_lxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
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
            let t21 = t12 + f64x8::splat(3.72744);
            let t24 = (simd::atan(f64x8::splat(6.15199081975908) / t21));
            let t26 = t12 / f64x8::splat(2.0);
            let t27 = t26 + f64x8::splat(0.10498);
            let t28 = t27 * t27;
            let t30 = (simd::ln(t28 * t15));
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t38 = f64x8::splat(2.0) * t36 - f64x8::splat(2.0);
            let t39 = f64x8::splat(M_CBRT2);
            let t42 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t44 = -t38 * t42 + f64x8::splat(1.0);
            let t45 = (f64x8::splat(0.0310907) * t19 + f64x8::splat(0.038783294878113016) * t24 + f64x8::splat(0.0009690227711544374) * t30) * t44;
            let t47 = t11 + f64x8::splat(3.53021) * t12 + f64x8::splat(18.0578);
            let t48 = f64x8::splat(1.0) / t47;
            let t52 = (simd::ln(t4 * t9 * t48 / f64x8::splat(4.0)));
            let t54 = t12 + f64x8::splat(7.06042);
            let t57 = (simd::atan(f64x8::splat(4.730926909560113) / t54));
            let t59 = t26 + f64x8::splat(0.325);
            let t60 = t59 * t59;
            let t62 = (simd::ln(t60 * t48));
            let t66 = (f64x8::splat(0.01554535) * t52 + f64x8::splat(0.05249139316978094) * t57 + f64x8::splat(0.0022478670955426118) * t62) * t38 * t42;
            let tzk0 = t45 + t66;
            acc_zk = tzk0;
            let t68 = f64x8::splat(1.0) / t7 / v_rho;
            let t69 = t6 * t68;
            let t73 = t4 * t6;
            let t74 = t14 * t14;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t8 * t75;
            let t77 = t4 * t69;
            let t78 = t77 / f64x8::splat(12.0);
            let t79 = f64x8::splat(1.0) / t12;
            let t80 = t79 * t1;
            let t81 = t3 * t6;
            let t83 = t80 * t81 * t68;
            let t85 = -t78 - f64x8::splat(0.31062) * t83;
            let t90 = t1 * t1;
            let t92 = f64x8::splat(1.0) / t3;
            let t93 = (-t4 * t69 * t15 / f64x8::splat(12.0) - t73 * t76 * t85 / f64x8::splat(4.0)) * t90 * t92;
            let t94 = t5 * t7;
            let t95 = t94 * t14;
            let t98 = t21 * t21;
            let t99 = f64x8::splat(1.0) / t98;
            let t101 = t99 * t79 * t1;
            let t103 = f64x8::splat(37.8469910464) * t99 + f64x8::splat(1.0);
            let t104 = f64x8::splat(1.0) / t103;
            let t109 = t27 * t15;
            let t110 = t109 * t79;
            let t113 = t28 * t75;
            let t115 = -t110 * t77 / f64x8::splat(6.0) - t113 * t85;
            let t116 = f64x8::splat(1.0) / t28;
            let t117 = t115 * t116;
            let t121 = (f64x8::splat(0.010363566666666667) * t93 * t95 + f64x8::splat(0.03976574567502677) * t101 * t81 * t68 * t104 + f64x8::splat(0.0009690227711544374) * t117 * t14) * t44;
            let t125 = t47 * t47;
            let t126 = f64x8::splat(1.0) / t125;
            let t127 = t8 * t126;
            let t129 = -t78 - f64x8::splat(0.5883683333333334) * t83;
            let t135 = (-t4 * t69 * t48 / f64x8::splat(12.0) - t73 * t127 * t129 / f64x8::splat(4.0)) * t90 * t92;
            let t136 = t94 * t47;
            let t139 = t54 * t54;
            let t140 = f64x8::splat(1.0) / t139;
            let t142 = t140 * t79 * t1;
            let t144 = f64x8::splat(22.3816694236) * t140 + f64x8::splat(1.0);
            let t145 = f64x8::splat(1.0) / t144;
            let t150 = t59 * t48;
            let t151 = t150 * t79;
            let t154 = t60 * t126;
            let t156 = -t151 * t77 / f64x8::splat(6.0) - t154 * t129;
            let t157 = f64x8::splat(1.0) / t60;
            let t158 = t156 * t157;
            let t163 = (f64x8::splat(0.005181783333333334) * t135 * t136 + f64x8::splat(0.041388824077869424) * t142 * t81 * t68 * t145 + f64x8::splat(0.0022478670955426118) * t158 * t47) * t38 * t42;
            let tvrho0 = t45 + t66 + v_rho * (t121 + t163);
            acc_vrho = tvrho0;
            let t168 = v_rho * v_rho;
            let t170 = f64x8::splat(1.0) / t7 / t168;
            let t171 = t6 * t170;
            let t173 = t4 * t171 * t15;
            let t175 = t68 * t75;
            let t180 = f64x8::splat(1.0) / t74 / t14;
            let t181 = t8 * t180;
            let t182 = t85 * t85;
            let t186 = t4 * t171;
            let t187 = t186 / f64x8::splat(9.0);
            let t189 = f64x8::splat(1.0) / t12 / t10;
            let t190 = t189 * t90;
            let t191 = t3 * t3;
            let t192 = t191 * t5;
            let t193 = t7 * t7;
            let t195 = f64x8::splat(1.0) / t193 / t168;
            let t197 = t190 * t192 * t195;
            let t200 = t80 * t81 * t170;
            let t202 = t187 - f64x8::splat(0.20708) * t197 + f64x8::splat(0.41416) * t200;
            let t208 = (t173 / f64x8::splat(9.0) + t73 * t175 * t85 / f64x8::splat(6.0) + t73 * t181 * t182 / f64x8::splat(2.0) - t73 * t76 * t202 / f64x8::splat(4.0)) * t90 * t92;
            let t212 = t5 / t193;
            let t213 = t212 * t14;
            let t216 = t94 * t85;
            let t219 = t98 * t21;
            let t221 = f64x8::splat(1.0) / t219 * t1;
            let t222 = t221 * t3;
            let t227 = t99 * t189 * t90;
            let t236 = t98 * t98;
            let t238 = f64x8::splat(1.0) / t236 / t21;
            let t239 = t238 * t1;
            let t240 = t239 * t3;
            let t241 = t103 * t103;
            let t242 = f64x8::splat(1.0) / t241;
            let t247 = t27 * t75;
            let t248 = t247 * t80;
            let t249 = t68 * t85;
            let t253 = t109 * t189;
            let t254 = t90 * t191;
            let t255 = t5 * t195;
            let t256 = t254 * t255;
            let t261 = t28 * t180;
            let t265 = t173 / f64x8::splat(72.0) + t248 * t81 * t249 / f64x8::splat(3.0) - t253 * t256 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t110 * t186 + f64x8::splat(2.0) * t261 * t182 - t113 * t202;
            let t266 = t265 * t116;
            let t270 = f64x8::splat(1.0) / t28 / t27;
            let t271 = t115 * t270;
            let t272 = t14 * t79;
            let t273 = t271 * t272;
            let t279 = (f64x8::splat(0.010363566666666667) * t208 * t95 + f64x8::splat(0.003454522222222222) * t93 * t213 + f64x8::splat(0.010363566666666667) * t93 * t216 + f64x8::splat(0.013255248558342257) * t222 * t171 * t104 + f64x8::splat(0.026510497116684514) * t227 * t192 * t195 * t104 - f64x8::splat(0.05302099423336903) * t101 * t81 * t170 * t104 - f64x8::splat(0.5016712735053859) * t240 * t171 * t242 + f64x8::splat(0.0009690227711544374) * t266 * t14 + f64x8::splat(0.00016150379519240624) * t273 * t77 + f64x8::splat(0.0009690227711544374) * t117 * t85) * t44;
            let t281 = t4 * t171 * t48;
            let t283 = t68 * t126;
            let t288 = f64x8::splat(1.0) / t125 / t47;
            let t289 = t8 * t288;
            let t290 = t129 * t129;
            let t296 = t187 - f64x8::splat(0.39224555555555557) * t197 + f64x8::splat(0.7844911111111111) * t200;
            let t302 = (t281 / f64x8::splat(9.0) + t73 * t283 * t129 / f64x8::splat(6.0) + t73 * t289 * t290 / f64x8::splat(2.0) - t73 * t127 * t296 / f64x8::splat(4.0)) * t90 * t92;
            let t305 = t212 * t47;
            let t308 = t94 * t129;
            let t311 = t139 * t54;
            let t313 = f64x8::splat(1.0) / t311 * t1;
            let t314 = t313 * t3;
            let t319 = t140 * t189 * t90;
            let t328 = t139 * t139;
            let t330 = f64x8::splat(1.0) / t328 / t54;
            let t331 = t330 * t1;
            let t332 = t331 * t3;
            let t333 = t144 * t144;
            let t334 = f64x8::splat(1.0) / t333;
            let t339 = t59 * t126;
            let t340 = t339 * t80;
            let t341 = t68 * t129;
            let t345 = t150 * t189;
            let t350 = t60 * t288;
            let t354 = t281 / f64x8::splat(72.0) + t340 * t81 * t341 / f64x8::splat(3.0) - t345 * t256 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t151 * t186 + f64x8::splat(2.0) * t350 * t290 - t154 * t296;
            let t355 = t354 * t157;
            let t359 = f64x8::splat(1.0) / t60 / t59;
            let t360 = t156 * t359;
            let t361 = t47 * t79;
            let t362 = t360 * t361;
            let t369 = (f64x8::splat(0.005181783333333334) * t302 * t136 + f64x8::splat(0.001727261111111111) * t135 * t305 + f64x8::splat(0.005181783333333334) * t135 * t308 + f64x8::splat(0.013796274692623142) * t314 * t171 * t145 + f64x8::splat(0.027592549385246284) * t319 * t192 * t195 * t145 - f64x8::splat(0.05518509877049257) * t142 * t81 * t170 * t145 - f64x8::splat(0.3087836594474698) * t332 * t171 * t334 + f64x8::splat(0.0022478670955426118) * t355 * t47 + f64x8::splat(0.00037464451592376865) * t362 * t77 + f64x8::splat(0.0022478670955426118) * t158 * t129) * t38 * t42;
            let tv2rho20 = f64x8::splat(2.0) * t121 + f64x8::splat(2.0) * t163 + v_rho * (t279 + t369);
            acc_v2rho2 = tv2rho20;
            let t374 = t168 * v_rho;
            let t376 = f64x8::splat(1.0) / t7 / t374;
            let t377 = t6 * t376;
            let t379 = t4 * t377 * t15;
            let t381 = t170 * t75;
            let t383 = t73 * t381 * t85;
            let t385 = t68 * t180;
            let t392 = t74 * t74;
            let t393 = f64x8::splat(1.0) / t392;
            let t394 = t8 * t393;
            let t395 = t182 * t85;
            let t399 = t85 * t202;
            let t403 = t4 * t377;
            let t404 = f64x8::splat(7.0) / f64x8::splat(27.0) * t403;
            let t408 = f64x8::splat(1.0) / t12 / t254 / t212 / f64x8::splat(4.0);
            let t409 = t408 * t2;
            let t410 = t168 * t168;
            let t411 = f64x8::splat(1.0) / t410;
            let t412 = t409 * t411;
            let t415 = f64x8::splat(1.0) / t193 / t374;
            let t417 = t190 * t192 * t415;
            let t420 = t80 * t81 * t376;
            let t422 = -t404 - f64x8::splat(1.24248) * t412 + f64x8::splat(0.82832) * t417 - f64x8::splat(0.9663733333333333) * t420;
            let t428 = (-f64x8::splat(7.0) / f64x8::splat(27.0) * t379 - t383 / f64x8::splat(3.0) - t73 * t385 * t182 / f64x8::splat(2.0) + t73 * t175 * t202 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t73 * t394 * t395 + f64x8::splat(3.0) / f64x8::splat(2.0) * t73 * t181 * t399 - t73 * t76 * t422 / f64x8::splat(4.0)) * t90 * t92;
            let t435 = t5 / t193 / v_rho;
            let t436 = t435 * t14;
            let t442 = f64x8::splat(1.0) / t236;
            let t443 = t442 * t90;
            let t444 = t443 * t191;
            let t445 = t5 * t415;
            let t446 = t104 * t79;
            let t451 = f64x8::splat(1.0) / t236 / t98;
            let t452 = t451 * t90;
            let t453 = t452 * t191;
            let t454 = t242 * t79;
            let t458 = f64x8::splat(1.0) / t191;
            let t459 = t221 * t458;
            let t460 = t2 * t104;
            let t464 = t236 * t236;
            let t465 = f64x8::splat(1.0) / t464;
            let t466 = t465 * t90;
            let t467 = t466 * t191;
            let t469 = f64x8::splat(1.0) / t241 / t103;
            let t470 = t469 * t79;
            let t474 = t28 * t28;
            let t475 = f64x8::splat(1.0) / t474;
            let t476 = t115 * t475;
            let t477 = t476 * t14;
            let t484 = t27 * t180;
            let t485 = t484 * t80;
            let t489 = t247 * t190;
            let t503 = t1 * t458 * t6;
            let t504 = t376 * t2;
            let t510 = t254 * t445;
            let t515 = t28 * t393;
            let t521 = -f64x8::splat(11.0) / f64x8::splat(216.0) * t379 - t383 / f64x8::splat(24.0) - t485 * t81 * t68 * t182 + t489 * t192 * t195 * t85 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t248 * t81 * t170 * t85 + t248 * t81 * t68 * t202 / f64x8::splat(2.0) + t503 * t504 * t15 / f64x8::splat(432.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t109 * t412 + f64x8::splat(4.0) / f64x8::splat(9.0) * t253 * t510 - f64x8::splat(14.0) / f64x8::splat(27.0) * t110 * t403 - f64x8::splat(6.0) * t515 * t395 + f64x8::splat(6.0) * t261 * t399 - t113 * t422;
            let t522 = t521 * t116;
            let t527 = f64x8::splat(0.010363566666666667) * t428 * t95 + f64x8::splat(0.006909044444444444) * t208 * t213 - f64x8::splat(0.002303014814814815) * t93 * t436 - f64x8::splat(0.04860257804725494) * t222 * t377 * t104 + f64x8::splat(0.026510497116684514) * t444 * t445 * t446 - f64x8::splat(2.341132609691801) * t453 * t445 * t454 + f64x8::splat(0.002209208093057043) * t459 * t377 * t460 + f64x8::splat(50.6313285242518) * t467 * t445 * t470 + f64x8::splat(4.037594879810156e-05) * t477 * t186 - f64x8::splat(0.00021533839358987497) * t273 * t186 + f64x8::splat(0.0009690227711544374) * t522 * t14 + f64x8::splat(0.001938045542308875) * t266 * t85;
            let t538 = t265 * t270;
            let t539 = t538 * t272;
            let t542 = t85 * t79;
            let t543 = t271 * t542;
            let t546 = t14 * t189;
            let t547 = t271 * t546;
            let t550 = t99 * t408;
            let t551 = t2 * t411;
            let t557 = t212 * t85;
            let t560 = t94 * t202;
            let t563 = t239 * t458;
            let t564 = t377 * t242;
            let t569 = f64x8::splat(0.0009690227711544374) * t117 * t202 - f64x8::splat(0.10604198846673805) * t227 * t192 * t415 * t104 + f64x8::splat(0.12371565321119439) * t101 * t81 * t376 * t104 + f64x8::splat(0.0003230075903848125) * t539 * t77 + f64x8::splat(0.0003230075903848125) * t543 * t77 + f64x8::splat(0.00010766919679493748) * t547 * t256 + f64x8::splat(0.15906298270010708) * t550 * t551 * t104 + f64x8::splat(0.020727133333333335) * t208 * t216 + f64x8::splat(0.006909044444444444) * t93 * t557 + f64x8::splat(0.010363566666666667) * t93 * t560 - f64x8::splat(0.026614487661862786) * t563 * t564 + f64x8::splat(1.839461336186415) * t240 * t564;
            let t571 = (t527 + t569) * t44;
            let t574 = t435 * t47;
            let t580 = f64x8::splat(1.0) / t328;
            let t581 = t580 * t90;
            let t582 = t581 * t191;
            let t583 = t145 * t79;
            let t588 = f64x8::splat(1.0) / t328 / t139;
            let t589 = t588 * t90;
            let t590 = t589 * t191;
            let t591 = t334 * t79;
            let t595 = t313 * t458;
            let t596 = t2 * t145;
            let t600 = t328 * t328;
            let t601 = f64x8::splat(1.0) / t600;
            let t602 = t601 * t90;
            let t603 = t602 * t191;
            let t605 = f64x8::splat(1.0) / t333 / t144;
            let t606 = t605 * t79;
            let t610 = t60 * t60;
            let t611 = f64x8::splat(1.0) / t610;
            let t612 = t156 * t611;
            let t613 = t612 * t47;
            let t619 = t4 * t377 * t48;
            let t621 = t170 * t126;
            let t623 = t73 * t621 * t129;
            let t625 = t59 * t288;
            let t626 = t625 * t80;
            let t630 = t339 * t190;
            let t652 = t125 * t125;
            let t653 = f64x8::splat(1.0) / t652;
            let t654 = t60 * t653;
            let t655 = t290 * t129;
            let t658 = t129 * t296;
            let t664 = -t404 - f64x8::splat(2.3534733333333335) * t412 + f64x8::splat(1.5689822222222223) * t417 - f64x8::splat(1.8304792592592594) * t420;
            let t666 = -f64x8::splat(11.0) / f64x8::splat(216.0) * t619 - t623 / f64x8::splat(24.0) - t626 * t81 * t68 * t290 + t630 * t192 * t195 * t129 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t340 * t81 * t170 * t129 + t340 * t81 * t68 * t296 / f64x8::splat(2.0) + t503 * t504 * t48 / f64x8::splat(432.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t150 * t412 + f64x8::splat(4.0) / f64x8::splat(9.0) * t345 * t510 - f64x8::splat(14.0) / f64x8::splat(27.0) * t151 * t403 - f64x8::splat(6.0) * t654 * t655 + f64x8::splat(6.0) * t350 * t658 - t154 * t664;
            let t667 = t666 * t157;
            let t674 = f64x8::splat(0.003454522222222222) * t302 * t305 - f64x8::splat(0.0011515074074074075) * t135 * t574 - f64x8::splat(0.050586340539618184) * t314 * t377 * t145 + f64x8::splat(0.027592549385246284) * t582 * t445 * t583 - f64x8::splat(1.4409904107548592) * t590 * t445 * t591 + f64x8::splat(0.00229937911543719) * t595 * t377 * t596 + f64x8::splat(18.429583437767338) * t603 * t445 * t606 + f64x8::splat(9.366112898094216e-05) * t613 * t186 - f64x8::splat(0.0004995260212316916) * t362 * t186 + f64x8::splat(0.0022478670955426118) * t667 * t47 + f64x8::splat(0.0044957341910852235) * t355 * t129 + f64x8::splat(0.0022478670955426118) * t158 * t296;
            let t683 = t354 * t359;
            let t684 = t683 * t361;
            let t687 = t129 * t79;
            let t688 = t360 * t687;
            let t691 = t47 * t189;
            let t692 = t360 * t691;
            let t695 = t140 * t408;
            let t701 = t68 * t288;
            let t708 = t8 * t653;
            let t720 = (-f64x8::splat(7.0) / f64x8::splat(27.0) * t619 - t623 / f64x8::splat(3.0) - t73 * t701 * t290 / f64x8::splat(2.0) + t73 * t283 * t296 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t73 * t708 * t655 + f64x8::splat(3.0) / f64x8::splat(2.0) * t73 * t289 * t658 - t73 * t127 * t664 / f64x8::splat(4.0)) * t90 * t92;
            let t725 = t212 * t129;
            let t728 = t94 * t296;
            let t731 = t331 * t458;
            let t732 = t377 * t334;
            let t737 = -f64x8::splat(0.11037019754098513) * t319 * t192 * t415 * t145 + f64x8::splat(0.12876523046448266) * t142 * t81 * t376 * t145 + f64x8::splat(0.0007492890318475373) * t684 * t77 + f64x8::splat(0.0007492890318475373) * t688 * t77 + f64x8::splat(0.0002497630106158458) * t692 * t256 + f64x8::splat(0.1655552963114777) * t695 * t551 * t145 + f64x8::splat(0.005181783333333334) * t720 * t136 + f64x8::splat(0.010363566666666667) * t302 * t308 + f64x8::splat(0.003454522222222222) * t135 * t725 + f64x8::splat(0.005181783333333334) * t135 * t728 - f64x8::splat(0.01638148191568975) * t731 * t732 + f64x8::splat(1.1322067513073895) * t332 * t732;
            let t740 = (t674 + t737) * t38 * t42;
            let tv3rho30 = f64x8::splat(3.0) * t279 + f64x8::splat(3.0) * t369 + v_rho * (t571 + t740);
            acc_v3rho3 = tv3rho30;
            let t764 = t192 * t415 * t79;
            let t773 = t92 * t5;
            let t776 = f64x8::splat(1.0) / t193 / t410;
            let t777 = t776 * t2;
            let t782 = t410 * v_rho;
            let t783 = f64x8::splat(1.0) / t782;
            let t798 = t2 * t783;
            let t802 = f64x8::splat(0.0005024562517097083) * t273 * t403 + f64x8::splat(0.0009690227711544374) * t538 * t542 * t77 + f64x8::splat(0.0004845113855772187) * t271 * t202 * t79 * t77 + f64x8::splat(0.0004845113855772187) * t521 * t270 * t272 * t77 + f64x8::splat(5.383459839746874e-05) * t115 / t474 / t27 * t14 * t90 * t764 - f64x8::splat(0.000646015180769625) * t539 * t186 - f64x8::splat(0.000646015180769625) * t543 * t186 - f64x8::splat(0.00043067678717974994) * t547 * t510 + f64x8::splat(0.004418416186114086) * t443 * t773 * t777 * t446 + f64x8::splat(101.2626570485036) * t465 * t2 * t783 * t469 * t189 + f64x8::splat(0.05302099423336903) * t442 * t2 * t783 * t104 * t189 - f64x8::splat(4.682265219383602) * t451 * t2 * t783 * t242 * t189 - f64x8::splat(1.2725038616008566) * t550 * t798 * t104;
            let t806 = f64x8::splat(1.0) / t7 / t410;
            let t807 = t6 * t806;
            let t809 = t4 * t807 * t15;
            let t813 = t73 * t376 * t75 * t85;
            let t817 = t73 * t170 * t180 * t182;
            let t820 = t73 * t381 * t202;
            let t833 = f64x8::splat(1.0) / t392 / t14;
            let t835 = t182 * t182;
            let t839 = t182 * t202;
            let t843 = t202 * t202;
            let t847 = t85 * t422;
            let t851 = t4 * t807;
            let t852 = f64x8::splat(70.0) / f64x8::splat(81.0) * t851;
            let t857 = f64x8::splat(1.0) / t12 / t2 * v_rho / f64x8::splat(48.0);
            let t858 = t857 * t2;
            let t860 = f64x8::splat(1.0) / t7 / t782;
            let t862 = t858 * t860 * t73;
            let t864 = t409 * t783;
            let t867 = t190 * t192 * t776;
            let t870 = t80 * t81 * t806;
            let t872 = t852 - f64x8::splat(1.0354) * t862 + f64x8::splat(9.93984) * t864 - f64x8::splat(3.6814222222222224) * t867 + f64x8::splat(3.2212444444444444) * t870;
            let t876 = f64x8::splat(70.0) / f64x8::splat(81.0) * t809 + f64x8::splat(28.0) / f64x8::splat(27.0) * t813 + f64x8::splat(4.0) / f64x8::splat(3.0) * t817 - f64x8::splat(2.0) / f64x8::splat(3.0) * t820 + f64x8::splat(2.0) * t73 * t68 * t393 * t395 - f64x8::splat(2.0) * t73 * t385 * t399 + t73 * t175 * t422 / f64x8::splat(3.0) + f64x8::splat(6.0) * t73 * t8 * t833 * t835 - f64x8::splat(9.0) * t73 * t394 * t839 + f64x8::splat(3.0) / f64x8::splat(2.0) * t73 * t181 * t843 + f64x8::splat(2.0) * t73 * t181 * t847 - t73 * t76 * t872 / f64x8::splat(4.0);
            let t892 = f64x8::splat(1.0) / t464 / t21;
            let t896 = t90 * t92 * t5;
            let t902 = t241 * t241;
            let t911 = f64x8::splat(1.0) / t236 / t219;
            let t921 = t807 * t242;
            let t924 = f64x8::splat(0.010363566666666667) * t428 * t213 + f64x8::splat(0.010363566666666667) * t876 * t90 * t92 * t95 - f64x8::splat(0.006909044444444444) * t208 * t436 + f64x8::splat(0.003838358024691358) * t93 * t255 * t14 + f64x8::splat(0.20435174860777647) * t222 * t807 * t104 + f64x8::splat(0.0310907) * t428 * t216 + f64x8::splat(18.80252782320349) * t892 * t776 * t469 * t896 - f64x8::splat(609.9592304352594) / t464 / t219 * t776 / t902 * t896 - f64x8::splat(0.006909044444444444) * t93 * t435 * t85 - f64x8::splat(0.10645795064745114) * t911 * t776 * t242 * t896 + f64x8::splat(0.020727133333333335) * t208 * t557 + f64x8::splat(0.000646015180769625) * t271 * t14 * t412 + f64x8::splat(0.1951729095203271) * t563 * t921;
            let t950 = t860 * t1 * t81;
            let t982 = -f64x8::splat(36.0) * t515 * t839 + f64x8::splat(8.0) * t261 * t847 - f64x8::splat(4.0) / f64x8::splat(3.0) * t248 * t81 * t170 * t202 + f64x8::splat(2.0) / f64x8::splat(3.0) * t248 * t81 * t68 * t422 - f64x8::splat(5.0) / f64x8::splat(9.0) * t109 * t858 * t950 - f64x8::splat(4.0) / f64x8::splat(3.0) * t484 * t190 * t192 * t195 * t182 + f64x8::splat(2.0) / f64x8::splat(3.0) * t489 * t192 * t195 * t202 + f64x8::splat(4.0) * t27 * t393 * t80 * t81 * t68 * t395 + f64x8::splat(8.0) / f64x8::splat(3.0) * t485 * t81 * t170 * t182 - f64x8::splat(16.0) / f64x8::splat(9.0) * t489 * t192 * t415 * t85 + f64x8::splat(56.0) / f64x8::splat(27.0) * t248 * t81 * t376 * t85 + f64x8::splat(16.0) / f64x8::splat(3.0) * t109 * t864;
            let t988 = t806 * t2;
            let t1001 = t5 * t776;
            let t1002 = t254 * t1001;
            let t1015 = f64x8::splat(185.0) / f64x8::splat(864.0) * t809 + f64x8::splat(8.0) / f64x8::splat(3.0) * t247 * t408 * t551 * t85 - f64x8::splat(11.0) / f64x8::splat(648.0) * t503 * t988 * t15 + f64x8::splat(11.0) / f64x8::splat(54.0) * t813 + t817 / f64x8::splat(6.0) - t820 / f64x8::splat(12.0) + f64x8::splat(6.0) * t261 * t843 + f64x8::splat(24.0) * t28 * t833 * t835 - t113 * t872 - f64x8::splat(160.0) / f64x8::splat(81.0) * t253 * t1002 + f64x8::splat(140.0) / f64x8::splat(81.0) * t110 * t851 - t503 * t504 * t75 * t85 / f64x8::splat(108.0) - f64x8::splat(4.0) * t485 * t81 * t249 * t202;
            let t1036 = t458 * t6 * t504;
            let t1040 = t2 * t860;
            let t1052 = -f64x8::splat(7.734098799874699) * t240 * t921 + f64x8::splat(0.010363566666666667) * t93 * t94 * t422 + f64x8::splat(0.0310907) * t208 * t560 + f64x8::splat(0.010363566666666667) * t93 * t212 * t202 + f64x8::splat(0.0009690227711544374) * (t982 + t1015) * t116 * t14 + f64x8::splat(0.0029070683134633122) * t522 * t85 + f64x8::splat(0.0029070683134633122) * t266 * t202 + f64x8::splat(0.0009690227711544374) * t117 * t422 + f64x8::splat(0.0003230075903848125) * t538 * t546 * t256 + f64x8::splat(0.0003230075903848125) * t271 * t85 * t189 * t256 + f64x8::splat(6.729324799683593e-06) * t476 * t14 * t1 * t1036 + f64x8::splat(0.13255248558342256) * t99 * t857 * t1040 * t104 * t1 * t81 - f64x8::splat(2.341132609691801) * t911 * t2 * t776 * t242 * t90 * t773;
            let t1054 = t1001 * t454;
            let t1084 = t1001 * t470;
            let t1100 = -f64x8::splat(0.12420094242202633) * t452 * t92 * t1054 + f64x8::splat(67.50843803233573) * t892 * t2 * t776 * t469 * t90 * t773 + f64x8::splat(0.00012112784639430468) * t265 * t475 * t14 * t186 + f64x8::splat(0.00012112784639430468) * t476 * t85 * t186 - f64x8::splat(0.19441031218901977) * t444 * t1001 * t446 + f64x8::splat(17.16830580440654) * t453 * t1054 + f64x8::splat(0.017673664744456342) * t238 * t2 * t776 * t104 * t90 * t773 - f64x8::splat(0.01620085934908498) * t459 * t807 * t460 - f64x8::splat(371.29640917784656) * t467 * t1084 + f64x8::splat(2.686075403314784) * t466 * t92 * t1084 + f64x8::splat(0.4712977265188358) * t227 * t192 * t776 * t104 - f64x8::splat(0.41238551070398133) * t101 * t81 * t806 * t104 - f64x8::splat(0.00014804514559303906) * t477 * t403;
            let t1112 = t1001 * t606;
            let t1157 = f64x8::splat(0.49053421129326724) * t319 * t192 * t776 * t145 - f64x8::splat(0.42921743488160885) * t142 * t81 * t806 * t145 - f64x8::splat(135.15027854362714) * t603 * t1112 - f64x8::splat(0.0003434241395967879) * t613 * t403 - f64x8::splat(0.016862113513206062) * t595 * t807 * t596 + f64x8::splat(0.05518509877049257) * t580 * t2 * t783 * t145 * t189 - f64x8::splat(2.8819808215097185) * t588 * t2 * t783 * t334 * t189 - f64x8::splat(1.3244423704918216) * t695 * t798 * t145 + f64x8::splat(36.859166875534676) * t601 * t2 * t783 * t605 * t189 + f64x8::splat(0.0001248815053079229) * t156 / t610 / t59 * t47 * t90 * t764 - f64x8::splat(0.0014985780636950746) * t684 * t186 + f64x8::splat(1.561018816349036e-05) * t612 * t47 * t1 * t1036 + f64x8::splat(0.13796274692623142) * t140 * t857 * t1040 * t145 * t1 * t81;
            let t1197 = f64x8::splat(0.0007492890318475373) * t683 * t691 * t256 + f64x8::splat(0.0022478670955426118) * t683 * t687 * t77 + f64x8::splat(0.0011239335477713059) * t360 * t296 * t79 * t77 + f64x8::splat(0.0011239335477713059) * t666 * t359 * t361 * t77 - f64x8::splat(0.0014985780636950746) * t688 * t186 - f64x8::splat(0.0009990520424633831) * t692 * t510 + f64x8::splat(0.00459875823087438) * t581 * t773 * t777 * t583 + f64x8::splat(0.0011655607162072803) * t362 * t403 + f64x8::splat(0.0007492890318475373) * t360 * t129 * t189 * t256 + f64x8::splat(0.005181783333333334) * t720 * t305 - f64x8::splat(0.003454522222222222) * t135 * t435 * t129 + f64x8::splat(0.001919179012345679) * t135 * t255 * t47 + f64x8::splat(0.2126925681779401) * t314 * t807 * t145;
            let t1204 = t4 * t807 * t48;
            let t1208 = t73 * t376 * t126 * t129;
            let t1212 = t73 * t170 * t288 * t290;
            let t1215 = t73 * t621 * t296;
            let t1228 = f64x8::splat(1.0) / t652 / t47;
            let t1230 = t290 * t290;
            let t1234 = t290 * t296;
            let t1238 = t296 * t296;
            let t1242 = t129 * t664;
            let t1250 = t852 - f64x8::splat(1.9612277777777778) * t862 + f64x8::splat(18.82778666666667) * t864 - f64x8::splat(6.973254320987654) * t867 + f64x8::splat(6.101597530864198) * t870;
            let t1254 = f64x8::splat(70.0) / f64x8::splat(81.0) * t1204 + f64x8::splat(28.0) / f64x8::splat(27.0) * t1208 + f64x8::splat(4.0) / f64x8::splat(3.0) * t1212 - f64x8::splat(2.0) / f64x8::splat(3.0) * t1215 + f64x8::splat(2.0) * t73 * t68 * t653 * t655 - f64x8::splat(2.0) * t73 * t701 * t658 + t73 * t283 * t664 / f64x8::splat(3.0) + f64x8::splat(6.0) * t73 * t8 * t1228 * t1230 - f64x8::splat(9.0) * t73 * t708 * t1234 + f64x8::splat(3.0) / f64x8::splat(2.0) * t73 * t289 * t1238 + f64x8::splat(2.0) * t73 * t289 * t1242 - t73 * t127 * t1250 / f64x8::splat(4.0);
            let t1262 = t807 * t334;
            let t1268 = f64x8::splat(1.0) / t328 / t311;
            let t1274 = f64x8::splat(1.0) / t600 / t54;
            let t1282 = t333 * t333;
            let t1297 = -f64x8::splat(0.003454522222222222) * t302 * t574 + f64x8::splat(0.010363566666666667) * t302 * t725 + f64x8::splat(0.005181783333333334) * t1254 * t90 * t92 * t136 + f64x8::splat(0.0014985780636950746) * t360 * t47 * t412 + f64x8::splat(0.12013086738172483) * t731 * t1262 - f64x8::splat(4.76041474981516) * t332 * t1262 - f64x8::splat(0.065525927662759) * t1268 * t776 * t334 * t896 + f64x8::splat(6.844038374238793) * t1274 * t776 * t605 * t896 - f64x8::splat(131.2980037839818) / t600 / t311 * t776 / t1282 * t896 + f64x8::splat(0.005181783333333334) * t135 * t94 * t664 + f64x8::splat(0.01554535) * t302 * t728 + f64x8::splat(0.01554535) * t720 * t308 + f64x8::splat(0.005181783333333334) * t135 * t212 * t296;
            let t1334 = f64x8::splat(8.0) * t350 * t1242 - f64x8::splat(36.0) * t654 * t1234 + f64x8::splat(8.0) / f64x8::splat(3.0) * t339 * t408 * t551 * t129 - f64x8::splat(11.0) / f64x8::splat(648.0) * t503 * t988 * t48 - t503 * t504 * t126 * t129 / f64x8::splat(108.0) + f64x8::splat(140.0) / f64x8::splat(81.0) * t151 * t851 - f64x8::splat(160.0) / f64x8::splat(81.0) * t345 * t1002 + f64x8::splat(16.0) / f64x8::splat(3.0) * t150 * t864 + f64x8::splat(185.0) / f64x8::splat(864.0) * t1204 + f64x8::splat(56.0) / f64x8::splat(27.0) * t340 * t81 * t376 * t129 + f64x8::splat(4.0) * t59 * t653 * t80 * t81 * t68 * t655 + f64x8::splat(8.0) / f64x8::splat(3.0) * t626 * t81 * t170 * t290;
            let t1372 = -f64x8::splat(16.0) / f64x8::splat(9.0) * t630 * t192 * t415 * t129 - f64x8::splat(4.0) / f64x8::splat(3.0) * t625 * t190 * t192 * t195 * t290 + f64x8::splat(2.0) / f64x8::splat(3.0) * t630 * t192 * t195 * t296 - f64x8::splat(4.0) / f64x8::splat(3.0) * t340 * t81 * t170 * t296 + f64x8::splat(2.0) / f64x8::splat(3.0) * t340 * t81 * t68 * t664 - f64x8::splat(5.0) / f64x8::splat(9.0) * t150 * t858 * t950 - t1215 / f64x8::splat(12.0) + f64x8::splat(11.0) / f64x8::splat(54.0) * t1208 + t1212 / f64x8::splat(6.0) + f64x8::splat(24.0) * t60 * t1228 * t1230 + f64x8::splat(6.0) * t350 * t1238 - t154 * t1250 - f64x8::splat(4.0) * t626 * t81 * t341 * t296;
            let t1386 = t1001 * t591;
            let t1420 = f64x8::splat(0.0022478670955426118) * (t1334 + t1372) * t157 * t47 + f64x8::splat(0.006743601286627835) * t667 * t129 + f64x8::splat(0.006743601286627835) * t355 * t296 + f64x8::splat(0.0022478670955426118) * t158 * t664 - f64x8::splat(0.20234536215847274) * t582 * t1001 * t583 + f64x8::splat(10.567263012202302) * t590 * t1386 + f64x8::splat(0.01839503292349752) * t330 * t2 * t776 * t145 * t90 * t773 - f64x8::splat(1.4409904107548592) * t1268 * t2 * t776 * t334 * t90 * t773 - f64x8::splat(0.07644691560655217) * t589 * t92 * t1386 + f64x8::splat(24.572777917023114) * t1274 * t2 * t776 * t605 * t90 * t773 + f64x8::splat(0.00028098338694282647) * t354 * t611 * t47 * t186 + f64x8::splat(0.00028098338694282647) * t612 * t129 * t186 + f64x8::splat(0.9777197677483991) * t602 * t92 * t1112;
            let tv4rho40 = f64x8::splat(4.0) * t571 + f64x8::splat(4.0) * t740 + v_rho * ((t802 + t924 + t1052 + t1100) * t44 + (t1157 + t1197 + t1297 + t1420) * t38 * t42);
            acc_v4rho4 = tv4rho40;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho4.into(); v4rho4[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
