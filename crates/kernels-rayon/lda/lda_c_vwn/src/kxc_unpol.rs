//! LDA_C_VWN kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn.c`
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
pub fn lda_c_vwn_kxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
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
            let t54 = (simd::cbrt(zeta_threshold));
            let t56 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t54 * zeta_threshold, f64x8::splat(1.0)));
            let t59 = f64x8::splat(M_CBRT2);
            let t60 = t59 - f64x8::splat(1.0);
            let t65 = f64x8::splat(9.0) * t56 - f64x8::splat(9.0);
            let t67 = t33 * (t40 + f64x8::splat(0.31770800474394145) * t44 + f64x8::splat(0.00041403379428206277) * t49) * t65 / f64x8::splat(24.0);
            let tzk0 = t20 + t25 + t31 - t67;
            acc_zk = tzk0;
            let t69 = f64x8::splat(1.0) / t7 / v_rho;
            let t70 = t6 * t69;
            let t74 = t4 * t6;
            let t75 = t14 * t14;
            let t76 = f64x8::splat(1.0) / t75;
            let t77 = t8 * t76;
            let t78 = t4 * t70;
            let t79 = t78 / f64x8::splat(12.0);
            let t80 = f64x8::splat(1.0) / t12;
            let t81 = t80 * t1;
            let t82 = t3 * t6;
            let t84 = t81 * t82 * t69;
            let t86 = -t79 - f64x8::splat(0.31062) * t84;
            let t91 = t1 * t1;
            let t93 = f64x8::splat(1.0) / t3;
            let t94 = (-t4 * t70 * t15 / f64x8::splat(12.0) - t74 * t77 * t86 / f64x8::splat(4.0)) * t91 * t93;
            let t95 = t5 * t7;
            let t96 = t95 * t14;
            let t97 = t94 * t96;
            let t99 = t21 * t21;
            let t100 = f64x8::splat(1.0) / t99;
            let t102 = t100 * t80 * t1;
            let t104 = f64x8::splat(37.8469910464) * t100 + f64x8::splat(1.0);
            let t105 = f64x8::splat(1.0) / t104;
            let t108 = t102 * t82 * t69 * t105;
            let t110 = t27 * t15;
            let t111 = t110 * t80;
            let t114 = t28 * t76;
            let t116 = -t111 * t78 / f64x8::splat(6.0) - t114 * t86;
            let t117 = f64x8::splat(1.0) / t28;
            let t118 = t116 * t117;
            let t119 = t118 * t14;
            let t124 = t35 * t35;
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t8 * t125;
            let t128 = -t79 - f64x8::splat(0.09425583333333333) * t84;
            let t134 = (-t4 * t70 * t36 / f64x8::splat(12.0) - t74 * t126 * t128 / f64x8::splat(4.0)) * t91 * t93;
            let t135 = t95 * t35;
            let t138 = t41 * t41;
            let t139 = f64x8::splat(1.0) / t138;
            let t141 = t139 * t80 * t1;
            let t143 = f64x8::splat(50.7386806551) * t139 + f64x8::splat(1.0);
            let t144 = f64x8::splat(1.0) / t143;
            let t149 = t46 * t36;
            let t150 = t149 * t80;
            let t153 = t47 * t125;
            let t155 = -t150 * t78 / f64x8::splat(6.0) - t153 * t128;
            let t156 = f64x8::splat(1.0) / t47;
            let t157 = t155 * t156;
            let t162 = t33 * (t134 * t135 / f64x8::splat(3.0) + f64x8::splat(0.37717812030896175) * t141 * t82 * t69 * t144 + f64x8::splat(0.00041403379428206277) * t157 * t35) * t65;
            let tvrho0 = t20 + t25 + t31 - t67 + v_rho * (f64x8::splat(0.010363566666666667) * t97 + f64x8::splat(0.03976574567502677) * t108 + f64x8::splat(0.0009690227711544374) * t119 - t162 / f64x8::splat(24.0));
            acc_vrho = tvrho0;
            let t170 = v_rho * v_rho;
            let t172 = f64x8::splat(1.0) / t7 / t170;
            let t173 = t6 * t172;
            let t175 = t4 * t173 * t15;
            let t177 = t69 * t76;
            let t182 = f64x8::splat(1.0) / t75 / t14;
            let t183 = t8 * t182;
            let t184 = t86 * t86;
            let t188 = t4 * t173;
            let t189 = t188 / f64x8::splat(9.0);
            let t191 = f64x8::splat(1.0) / t12 / t10;
            let t192 = t191 * t91;
            let t193 = t3 * t3;
            let t194 = t193 * t5;
            let t195 = t7 * t7;
            let t197 = f64x8::splat(1.0) / t195 / t170;
            let t199 = t192 * t194 * t197;
            let t202 = t81 * t82 * t172;
            let t204 = t189 - f64x8::splat(0.20708) * t199 + f64x8::splat(0.41416) * t202;
            let t210 = (t175 / f64x8::splat(9.0) + t74 * t177 * t86 / f64x8::splat(6.0) + t74 * t183 * t184 / f64x8::splat(2.0) - t74 * t77 * t204 / f64x8::splat(4.0)) * t91 * t93;
            let t211 = t210 * t96;
            let t214 = t5 / t195;
            let t215 = t214 * t14;
            let t216 = t94 * t215;
            let t218 = t95 * t86;
            let t219 = t94 * t218;
            let t221 = t99 * t21;
            let t222 = f64x8::splat(1.0) / t221;
            let t223 = t222 * t1;
            let t224 = t223 * t3;
            let t226 = t224 * t173 * t105;
            let t229 = t100 * t191 * t91;
            let t232 = t229 * t194 * t197 * t105;
            let t236 = t102 * t82 * t172 * t105;
            let t238 = t99 * t99;
            let t240 = f64x8::splat(1.0) / t238 / t21;
            let t241 = t240 * t1;
            let t242 = t241 * t3;
            let t243 = t104 * t104;
            let t244 = f64x8::splat(1.0) / t243;
            let t246 = t242 * t173 * t244;
            let t249 = t27 * t76;
            let t250 = t249 * t81;
            let t251 = t69 * t86;
            let t255 = t110 * t191;
            let t256 = t91 * t193;
            let t257 = t5 * t197;
            let t258 = t256 * t257;
            let t263 = t28 * t182;
            let t267 = t175 / f64x8::splat(72.0) + t250 * t82 * t251 / f64x8::splat(3.0) - t255 * t258 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t111 * t188 + f64x8::splat(2.0) * t263 * t184 - t114 * t204;
            let t268 = t267 * t117;
            let t269 = t268 * t14;
            let t272 = f64x8::splat(1.0) / t28 / t27;
            let t273 = t116 * t272;
            let t274 = t14 * t80;
            let t275 = t273 * t274;
            let t276 = t275 * t78;
            let t278 = t118 * t86;
            let t281 = t4 * t173 * t36;
            let t283 = t69 * t125;
            let t288 = f64x8::splat(1.0) / t124 / t35;
            let t289 = t8 * t288;
            let t290 = t128 * t128;
            let t296 = t189 - f64x8::splat(0.06283722222222222) * t199 + f64x8::splat(0.12567444444444445) * t202;
            let t302 = (t281 / f64x8::splat(9.0) + t74 * t283 * t128 / f64x8::splat(6.0) + t74 * t289 * t290 / f64x8::splat(2.0) - t74 * t126 * t296 / f64x8::splat(4.0)) * t91 * t93;
            let t305 = t214 * t35;
            let t308 = t95 * t128;
            let t311 = t138 * t41;
            let t312 = f64x8::splat(1.0) / t311;
            let t313 = t312 * t1;
            let t314 = t313 * t3;
            let t319 = t139 * t191 * t91;
            let t328 = t138 * t138;
            let t330 = f64x8::splat(1.0) / t328 / t41;
            let t331 = t330 * t1;
            let t332 = t331 * t3;
            let t333 = t143 * t143;
            let t334 = f64x8::splat(1.0) / t333;
            let t339 = t46 * t125;
            let t340 = t339 * t81;
            let t341 = t69 * t128;
            let t345 = t149 * t191;
            let t350 = t47 * t288;
            let t354 = t281 / f64x8::splat(72.0) + t340 * t82 * t341 / f64x8::splat(3.0) - t345 * t258 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t150 * t188 + f64x8::splat(2.0) * t350 * t290 - t153 * t296;
            let t355 = t354 * t156;
            let t359 = f64x8::splat(1.0) / t47 / t46;
            let t360 = t155 * t359;
            let t361 = t35 * t80;
            let t362 = t360 * t361;
            let t369 = t33 * (t302 * t135 / f64x8::splat(3.0) + t134 * t305 / f64x8::splat(9.0) + t134 * t308 / f64x8::splat(3.0) + f64x8::splat(0.12572604010298724) * t314 * t173 * t144 + f64x8::splat(0.2514520802059745) * t319 * t194 * t197 * t144 - f64x8::splat(0.502904160411949) * t141 * t82 * t172 * t144 - f64x8::splat(6.379173398815766) * t332 * t173 * t334 + f64x8::splat(0.00041403379428206277) * t355 * t35 + f64x8::splat(6.900563238034379e-05) * t362 * t78 + f64x8::splat(0.00041403379428206277) * t157 * t128) * t65;
            let t371 = f64x8::splat(0.010363566666666667) * t211 + f64x8::splat(0.003454522222222222) * t216 + f64x8::splat(0.010363566666666667) * t219 + f64x8::splat(0.013255248558342257) * t226 + f64x8::splat(0.026510497116684514) * t232 - f64x8::splat(0.05302099423336903) * t236 - f64x8::splat(0.5016712735053859) * t246 + f64x8::splat(0.0009690227711544374) * t269 + f64x8::splat(0.00016150379519240624) * t276 + f64x8::splat(0.0009690227711544374) * t278 - t369 / f64x8::splat(24.0);
            let tv2rho20 = f64x8::splat(0.020727133333333335) * t97 + f64x8::splat(0.07953149135005354) * t108 + f64x8::splat(0.001938045542308875) * t119 - t162 / f64x8::splat(12.0) + v_rho * t371;
            acc_v2rho2 = tv2rho20;
            let t384 = f64x8::splat(1.0) / t193;
            let t385 = t331 * t384;
            let t386 = t170 * v_rho;
            let t388 = f64x8::splat(1.0) / t7 / t386;
            let t389 = t6 * t388;
            let t390 = t389 * t334;
            let t395 = t214 * t128;
            let t398 = t95 * t296;
            let t404 = t4 * t389 * t36;
            let t406 = t172 * t125;
            let t408 = t74 * t406 * t128;
            let t410 = t69 * t288;
            let t417 = t124 * t124;
            let t418 = f64x8::splat(1.0) / t417;
            let t419 = t8 * t418;
            let t420 = t290 * t128;
            let t424 = t128 * t296;
            let t428 = t4 * t389;
            let t429 = f64x8::splat(7.0) / f64x8::splat(27.0) * t428;
            let t433 = f64x8::splat(1.0) / t12 / t256 / t214 / f64x8::splat(4.0);
            let t434 = t433 * t2;
            let t435 = t170 * t170;
            let t436 = f64x8::splat(1.0) / t435;
            let t437 = t434 * t436;
            let t440 = f64x8::splat(1.0) / t195 / t386;
            let t442 = t192 * t194 * t440;
            let t445 = t81 * t82 * t388;
            let t447 = -t429 - f64x8::splat(0.3770233333333333) * t437 + f64x8::splat(0.2513488888888889) * t442 - f64x8::splat(0.2932403703703704) * t445;
            let t453 = (-f64x8::splat(7.0) / f64x8::splat(27.0) * t404 - t408 / f64x8::splat(3.0) - t74 * t410 * t290 / f64x8::splat(2.0) + t74 * t283 * t296 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t74 * t419 * t420 + f64x8::splat(3.0) / f64x8::splat(2.0) * t74 * t289 * t424 - t74 * t126 * t447 / f64x8::splat(4.0)) * t91 * t93;
            let t456 = t139 * t433;
            let t457 = t2 * t436;
            let t465 = t5 / t195 / v_rho;
            let t466 = t465 * t35;
            let t472 = f64x8::splat(1.0) / t328;
            let t473 = t472 * t91;
            let t474 = t473 * t193;
            let t475 = t5 * t440;
            let t476 = t144 * t80;
            let t481 = f64x8::splat(1.0) / t328 / t138;
            let t482 = t481 * t91;
            let t483 = t482 * t193;
            let t484 = t334 * t80;
            let t488 = -f64x8::splat(0.3384256597539519) * t385 * t390 + f64x8::splat(23.390302462324474) * t332 * t390 + f64x8::splat(2.0) / f64x8::splat(9.0) * t134 * t395 + t134 * t398 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t302 * t308 + t453 * t135 / f64x8::splat(3.0) + f64x8::splat(1.508712481235847) * t456 * t457 * t144 + f64x8::splat(2.0) / f64x8::splat(9.0) * t302 * t305 - f64x8::splat(2.0) / f64x8::splat(27.0) * t134 * t466 - f64x8::splat(0.4609954803776199) * t314 * t389 * t144 + f64x8::splat(0.2514520802059745) * t474 * t475 * t476 - f64x8::splat(29.76947586114024) * t483 * t475 * t484;
            let t489 = t313 * t384;
            let t490 = t2 * t144;
            let t494 = t328 * t328;
            let t495 = f64x8::splat(1.0) / t494;
            let t496 = t495 * t91;
            let t497 = t496 * t193;
            let t499 = f64x8::splat(1.0) / t333 / t143;
            let t500 = t499 * t80;
            let t504 = t47 * t47;
            let t505 = f64x8::splat(1.0) / t504;
            let t506 = t155 * t505;
            let t507 = t506 * t35;
            let t512 = t46 * t288;
            let t513 = t512 * t81;
            let t517 = t339 * t192;
            let t531 = t1 * t384 * t6;
            let t532 = t388 * t2;
            let t538 = t256 * t475;
            let t543 = t47 * t418;
            let t549 = -f64x8::splat(11.0) / f64x8::splat(216.0) * t404 - t408 / f64x8::splat(24.0) - t513 * t82 * t69 * t290 + t517 * t194 * t197 * t128 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t340 * t82 * t172 * t128 + t340 * t82 * t69 * t296 / f64x8::splat(2.0) + t531 * t532 * t36 / f64x8::splat(432.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t149 * t437 + f64x8::splat(4.0) / f64x8::splat(9.0) * t345 * t538 - f64x8::splat(14.0) / f64x8::splat(27.0) * t150 * t428 - f64x8::splat(6.0) * t543 * t420 + f64x8::splat(6.0) * t350 * t424 - t153 * t447;
            let t550 = t549 * t156;
            let t565 = t354 * t359;
            let t566 = t565 * t361;
            let t569 = t128 * t80;
            let t570 = t360 * t569;
            let t573 = t35 * t191;
            let t574 = t360 * t573;
            let t579 = f64x8::splat(0.02095434001716454) * t489 * t389 * t490 + f64x8::splat(863.1222451360587) * t497 * t475 * t500 + f64x8::splat(1.7251408095085948e-05) * t507 * t188 + f64x8::splat(0.00041403379428206277) * t550 * t35 + f64x8::splat(0.0008280675885641255) * t355 * t128 + f64x8::splat(0.00041403379428206277) * t157 * t296 - f64x8::splat(1.005808320823898) * t319 * t194 * t440 * t144 + f64x8::splat(1.1734430409612142) * t141 * t82 * t388 * t144 + f64x8::splat(0.00013801126476068758) * t566 * t78 + f64x8::splat(0.00013801126476068758) * t570 * t78 + f64x8::splat(4.6003754920229193e-05) * t574 * t258 - f64x8::splat(9.200750984045839e-05) * t362 * t188;
            let t582 = t33 * (t488 + t579) * t65;
            let t584 = t100 * t433;
            let t586 = t584 * t457 * t105;
            let t588 = t268 * t86;
            let t590 = t118 * t204;
            let t593 = t4 * t389 * t15;
            let t595 = t172 * t76;
            let t597 = t74 * t595 * t86;
            let t599 = t27 * t182;
            let t600 = t599 * t81;
            let t604 = t249 * t192;
            let t626 = t75 * t75;
            let t627 = f64x8::splat(1.0) / t626;
            let t628 = t28 * t627;
            let t629 = t184 * t86;
            let t632 = t86 * t204;
            let t638 = -t429 - f64x8::splat(1.24248) * t437 + f64x8::splat(0.82832) * t442 - f64x8::splat(0.9663733333333333) * t445;
            let t640 = -f64x8::splat(11.0) / f64x8::splat(216.0) * t593 - t597 / f64x8::splat(24.0) - t600 * t82 * t69 * t184 + t604 * t194 * t197 * t86 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t250 * t82 * t172 * t86 + t250 * t82 * t69 * t204 / f64x8::splat(2.0) + t531 * t532 * t15 / f64x8::splat(432.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t110 * t437 + f64x8::splat(4.0) / f64x8::splat(9.0) * t255 * t538 - f64x8::splat(14.0) / f64x8::splat(27.0) * t111 * t428 - f64x8::splat(6.0) * t628 * t629 + f64x8::splat(6.0) * t263 * t632 - t114 * t638;
            let t641 = t640 * t117;
            let t642 = t641 * t14;
            let t646 = t69 * t182;
            let t653 = t8 * t627;
            let t665 = (-f64x8::splat(7.0) / f64x8::splat(27.0) * t593 - t597 / f64x8::splat(3.0) - t74 * t646 * t184 / f64x8::splat(2.0) + t74 * t177 * t204 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t74 * t653 * t629 + f64x8::splat(3.0) / f64x8::splat(2.0) * t74 * t183 * t632 - t74 * t77 * t638 / f64x8::splat(4.0)) * t91 * t93;
            let t666 = t665 * t96;
            let t668 = t210 * t215;
            let t670 = t465 * t14;
            let t671 = t94 * t670;
            let t674 = t224 * t389 * t105;
            let t676 = t210 * t218;
            let t678 = t214 * t86;
            let t679 = t94 * t678;
            let t681 = t95 * t204;
            let t682 = t94 * t681;
            let t684 = -t582 / f64x8::splat(24.0) + f64x8::splat(0.15906298270010708) * t586 + f64x8::splat(0.001938045542308875) * t588 + f64x8::splat(0.0009690227711544374) * t590 + f64x8::splat(0.0009690227711544374) * t642 + f64x8::splat(0.010363566666666667) * t666 + f64x8::splat(0.006909044444444444) * t668 - f64x8::splat(0.002303014814814815) * t671 - f64x8::splat(0.04860257804725494) * t674 + f64x8::splat(0.020727133333333335) * t676 + f64x8::splat(0.006909044444444444) * t679 + f64x8::splat(0.010363566666666667) * t682;
            let t685 = t241 * t384;
            let t686 = t389 * t244;
            let t687 = t685 * t686;
            let t689 = t242 * t686;
            let t691 = t275 * t188;
            let t693 = t267 * t272;
            let t694 = t693 * t274;
            let t695 = t694 * t78;
            let t697 = t86 * t80;
            let t698 = t273 * t697;
            let t699 = t698 * t78;
            let t701 = t14 * t191;
            let t702 = t273 * t701;
            let t703 = t702 * t258;
            let t705 = f64x8::splat(1.0) / t238;
            let t706 = t705 * t91;
            let t707 = t706 * t193;
            let t708 = t105 * t80;
            let t710 = t707 * t475 * t708;
            let t713 = f64x8::splat(1.0) / t238 / t99;
            let t714 = t713 * t91;
            let t715 = t714 * t193;
            let t716 = t244 * t80;
            let t718 = t715 * t475 * t716;
            let t720 = t223 * t384;
            let t721 = t2 * t105;
            let t723 = t720 * t389 * t721;
            let t725 = t238 * t238;
            let t726 = f64x8::splat(1.0) / t725;
            let t727 = t726 * t91;
            let t728 = t727 * t193;
            let t730 = f64x8::splat(1.0) / t243 / t104;
            let t731 = t730 * t80;
            let t733 = t728 * t475 * t731;
            let t735 = t28 * t28;
            let t736 = f64x8::splat(1.0) / t735;
            let t737 = t116 * t736;
            let t738 = t737 * t14;
            let t739 = t738 * t188;
            let t743 = t229 * t194 * t440 * t105;
            let t747 = t102 * t82 * t388 * t105;
            let t749 = -f64x8::splat(0.026614487661862786) * t687 + f64x8::splat(1.839461336186415) * t689 - f64x8::splat(0.00021533839358987497) * t691 + f64x8::splat(0.0003230075903848125) * t695 + f64x8::splat(0.0003230075903848125) * t699 + f64x8::splat(0.00010766919679493748) * t703 + f64x8::splat(0.026510497116684514) * t710 - f64x8::splat(2.341132609691801) * t718 + f64x8::splat(0.002209208093057043) * t723 + f64x8::splat(50.6313285242518) * t733 + f64x8::splat(4.037594879810156e-05) * t739 - f64x8::splat(0.10604198846673805) * t743 + f64x8::splat(0.12371565321119439) * t747;
            let tv3rho30 = f64x8::splat(0.0310907) * t211 + f64x8::splat(0.010363566666666667) * t216 + f64x8::splat(0.0310907) * t219 + f64x8::splat(0.03976574567502677) * t226 + f64x8::splat(0.07953149135005354) * t232 - f64x8::splat(0.15906298270010708) * t236 - f64x8::splat(1.5050138205161576) * t246 + f64x8::splat(0.0029070683134633122) * t269 + f64x8::splat(0.0004845113855772187) * t276 + f64x8::splat(0.0029070683134633122) * t278 - t369 / f64x8::splat(8.0) + v_rho * (t684 + t749);
            acc_v3rho3 = tv3rho30;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
