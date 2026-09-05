//! GGA_C_REVTCA lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_revtca.c`
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
pub fn gga_c_revtca_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
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
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
        let mut acc_v4rho3sigma = V_ZERO;
        let mut acc_v4rho2sigma2 = V_ZERO;
        let mut acc_v4rhosigma3 = V_ZERO;
        let mut acc_v4sigma4 = V_ZERO;
        {
            let t2 = (simd::cbrt(zeta_threshold));
            let t3 = t2 * t2;
            let t4 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t3, f64x8::splat(1.0)));
            let t5 = t4 * t4;
            let t6 = t5 * t4;
            let t7 = f64x8::splat(M_CBRT3);
            let t9 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t10 = t7 * t9;
            let t11 = f64x8::splat(M_CBRT4);
            let t12 = t11 * t11;
            let t13 = (simd::cbrt(v_rho));
            let t18 = f64x8::splat(4.88827) + f64x8::splat(0.79425925) * t10 * t12 / t13;
            let t19 = (simd::atan(t18));
            let t21 = -f64x8::splat(0.655868) * t19 + f64x8::splat(0.897889);
            let t22 = t6 * t21;
            let t23 = t7 * t7;
            let t24 = t22 * t23;
            let t25 = f64x8::splat(1.0) / t9;
            let t26 = t25 * t11;
            let t27 = f64x8::splat(M_CBRT6);
            let t28 = t27 * t27;
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = t28 * t31;
            let t33 = f64x8::splat(M_CBRT2);
            let t34 = ((v_sigma).sqrt());
            let t35 = t33 * t34;
            let t37 = f64x8::splat(1.0) / t13 / v_rho;
            let t39 = t32 * t35 * t37;
            let t40 = (simd::pow(t39, f64x8::splat(2.3)));
            let t42 = f64x8::splat(1.0) + f64x8::splat(0.004712150703442276) * t40;
            let t43 = f64x8::splat(1.0) / t42;
            let t46 = t24 * t26 * t13 * t43;
            let tzk0 = t46 / f64x8::splat(3.0);
            acc_zk = tzk0;
            let t48 = t18 * t18;
            let t49 = t48 + f64x8::splat(1.0);
            let t50 = f64x8::splat(1.0) / t49;
            let t51 = t6 * t50;
            let t55 = f64x8::splat(1.0) / v_rho * t6;
            let t57 = t23 * t25;
            let t58 = t57 * t11;
            let t60 = t42 * t42;
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = (simd::pow(t39, f64x8::splat(1.3)));
            let t63 = t61 * t62;
            let t64 = t63 * t28;
            let t65 = t31 * t33;
            let t66 = t65 * t34;
            let t67 = t64 * t66;
            let tvrho0 = f64x8::splat(4.0) / f64x8::splat(9.0) * t46 + f64x8::splat(0.6945723010386666) * t51 * t43 + f64x8::splat(0.004816865163518771) * t55 * t21 * t58 * t67;
            acc_vrho = tvrho0;
            let t70 = t22 * t58;
            let t71 = f64x8::splat(1.0) / t34;
            let t72 = t65 * t71;
            let tvsigma0 = -f64x8::splat(0.001806324436319539) * t70 * t64 * t72;
            acc_vsigma = tvsigma0;
            let t76 = t50 * t43;
            let t79 = t13 * t13;
            let t85 = v_rho * v_rho;
            let t86 = f64x8::splat(1.0) / t85;
            let t91 = t49 * t49;
            let t92 = f64x8::splat(1.0) / t91;
            let t93 = t6 * t92;
            let t94 = t43 * t18;
            let t102 = f64x8::splat(1.0) / t13 / t85;
            let t107 = t85 * v_rho;
            let t109 = f64x8::splat(1.0) / t13 / t107;
            let t110 = t109 * t6;
            let t112 = t110 * t21 * t58;
            let t114 = f64x8::splat(1.0) / t60 / t42;
            let t115 = (simd::pow(t39, f64x8::splat(2.6)));
            let t116 = t114 * t115;
            let t117 = t116 * t27;
            let t118 = t30 * t30;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = t33 * t33;
            let t121 = t119 * t120;
            let t122 = t121 * v_sigma;
            let t123 = t117 * t122;
            let t126 = (simd::pow(t39, f64x8::splat(0.3)));
            let t127 = t61 * t126;
            let t128 = t127 * t27;
            let t129 = t128 * t122;
            let tv2rho20 = f64x8::splat(0.9260964013848889) * t55 * t76 + f64x8::splat(4.0) / f64x8::splat(27.0) * t24 * t26 / t79 * t43 + f64x8::splat(0.0016056217211729237) * t24 * t26 * t86 * t67 + f64x8::splat(0.3677803165958304) * t93 * t94 * t10 * t12 * t37 + f64x8::splat(0.020073966722509357) * t51 * t63 * t32 * t35 * t102 + f64x8::splat(0.0008352788401267458) * t112 * t123 - f64x8::splat(0.05009539770059522) * t112 * t129;
            acc_v2rho2 = tv2rho20;
            let t133 = t50 * t61;
            let t134 = t6 * t37 * t133;
            let t135 = t62 * t28;
            let t136 = t135 * t72;
            let t139 = t121 * t102;
            let tv2rhosigma0 = -f64x8::splat(0.0037638687604705044) * t134 * t136 - f64x8::splat(0.0003132295650475297) * t70 * t117 * t139 + f64x8::splat(0.018785774137723206) * t70 * t128 * t139;
            acc_v2rhosigma = tv2rhosigma0;
            let t147 = t24 * t26 * t114;
            let t148 = t115 * t27;
            let t149 = t148 * t119;
            let t150 = f64x8::splat(1.0) / v_sigma;
            let t151 = t120 * t150;
            let t152 = t151 * t37;
            let t157 = t24 * t26 * t61;
            let t158 = t126 * t27;
            let t159 = t158 * t119;
            let t163 = t34 * v_sigma;
            let t164 = f64x8::splat(1.0) / t163;
            let t165 = t65 * t164;
            let tv2sigma20 = f64x8::splat(0.00011746108689282363) * t147 * t149 * t152 - f64x8::splat(0.007044665301646202) * t157 * t159 * t152 + f64x8::splat(0.0009031622181597695) * t70 * t64 * t165;
            acc_v2sigma2 = tv2sigma20;
            let t169 = t6 * t102;
            let t170 = t92 * t43;
            let t172 = t18 * t7;
            let t173 = t9 * t12;
            let t174 = t172 * t173;
            let t184 = f64x8::splat(1.0) / t91 / t49;
            let t185 = t6 * t184;
            let t188 = t9 * t9;
            let t189 = t23 * t188;
            let t191 = f64x8::splat(1.0) / t79 / t85;
            let t193 = t189 * t11 * t191;
            let t199 = t85 * t85;
            let t200 = t199 * v_rho;
            let t202 = f64x8::splat(1.0) / t79 / t200;
            let t203 = t202 * t6;
            let t204 = t21 * t23;
            let t205 = t204 * t25;
            let t206 = t203 * t205;
            let t207 = t11 * t114;
            let t208 = (simd::pow(t39, f64x8::splat(1.6)));
            let t209 = f64x8::splat(1.0) / t29;
            let t210 = t208 * t209;
            let t212 = t207 * t210 * t163;
            let t216 = (simd::pow(t39, -f64x8::splat(0.7)));
            let t217 = t216 * t209;
            let t219 = t11 * t61 * t217 * t163;
            let t225 = t110 * t133;
            let t226 = t135 * t66;
            let t231 = t120 * v_sigma;
            let t233 = f64x8::splat(1.0) / t79 / t199;
            let t235 = t27 * t119 * t231 * t233;
            let t241 = t203 * t204;
            let t242 = t60 * t60;
            let t243 = f64x8::splat(1.0) / t242;
            let t244 = (simd::pow(t39, f64x8::splat(3.9)));
            let t245 = t243 * t244;
            let t247 = t26 * t245 * t163;
            let t250 = t114 * t208;
            let t251 = t250 * t163;
            let t252 = t26 * t251;
            let t255 = t93 * t61;
            let t259 = f64x8::splat(1.0) / t79 / t107;
            let t262 = t32 * t35;
            let t266 = f64x8::splat(1.0) / t107;
            let t272 = f64x8::splat(1.0) / t13 / t199;
            let t274 = t24 * t26 * t272;
            let tv3rho30 = -f64x8::splat(1e-20) * t169 * t170 * t174 - f64x8::splat(8.0) / f64x8::splat(81.0) * t24 * t26 / t79 / v_rho * t43 + f64x8::splat(1.5579355649288897) * t185 * t43 * t48 * t193 - f64x8::splat(0.6173976009232592) * t6 * t86 * t76 - f64x8::splat(0.03474759974927263) * t206 * t212 + f64x8::splat(0.24045790896285704) * t206 * t219 - f64x8::splat(0.38948389123222243) * t93 * t43 * t193 - f64x8::splat(0.030110950083764035) * t225 * t226 + f64x8::splat(0.0052214539139616815) * t51 * t116 * t235 - f64x8::splat(0.313153880871146) * t51 * t127 * t235 + f64x8::splat(4.402708977978636e-05) * t241 * t247 - f64x8::splat(0.0017603339676632507) * t241 * t252 + f64x8::splat(0.01594393375354524) * t255 * t172 * t9 * t12 * t259 * t62 * t262 - f64x8::splat(0.0010704144807819492) * t24 * t26 * t266 * t67 - f64x8::splat(0.0025058365203802376) * t274 * t123 + f64x8::splat(0.15028619310178565) * t274 * t129;
            acc_v3rho3 = tv3rho30;
            let t279 = t169 * t133;
            let t282 = t6 * t191;
            let t284 = t282 * t92 * t64;
            let t285 = t72 * t174;
            let t288 = t6 * t259;
            let t289 = t50 * t114;
            let t290 = t288 * t289;
            let t291 = t148 * t121;
            let t294 = t288 * t133;
            let t295 = t158 * t121;
            let t298 = t22 * t57;
            let t299 = t11 * t243;
            let t306 = t209 * t233 * t34;
            let t310 = t121 * t109;
            let t319 = t61 * t216;
            let tv3rho2sigma0 = f64x8::splat(0.005018491680627339) * t279 * t136 - f64x8::splat(0.001992991719193155) * t284 * t285 - f64x8::splat(0.0013053634784904204) * t290 * t291 + f64x8::splat(0.0782884702177865) * t294 * t295 - f64x8::splat(1.6510158667419884e-05) * t298 * t299 * t244 * t233 * t34 + f64x8::splat(0.013030349905977234) * t70 * t250 * t306 + f64x8::splat(0.0007308689851109025) * t70 * t117 * t310 + f64x8::splat(0.000660125237873719) * t298 * t207 * t208 * t233 * t34 - f64x8::splat(0.0901717158610714) * t70 * t319 * t306 - f64x8::splat(0.043833472988020816) * t70 * t128 * t310;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t326 = t282 * t289;
            let t327 = t121 * t150;
            let t328 = t148 * t327;
            let t331 = t244 * t71;
            let t336 = t209 * t71;
            let t337 = t336 * t259;
            let t345 = t282 * t133;
            let t346 = t158 * t327;
            let t349 = t208 * t71;
            let t357 = t135 * t165;
            let tv3rhosigma20 = f64x8::splat(0.0002447556522169538) * t326 * t328 + f64x8::splat(6.191309500282457e-06) * t298 * t299 * t331 * t259 - f64x8::splat(0.004886381214741463) * t70 * t250 * t337 + f64x8::splat(1e-23) * t147 * t149 * t151 * t102 - f64x8::splat(0.014679088165834967) * t345 * t346 - f64x8::splat(0.00024754696420264467) * t298 * t207 * t349 * t259 + f64x8::splat(0.03381439344790177) * t70 * t319 * t337 + f64x8::splat(0.0018819343802352522) * t134 * t357;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t360 = t244 * t164;
            let t365 = t209 * t164;
            let t366 = t365 * t191;
            let t370 = v_sigma * v_sigma;
            let t371 = f64x8::splat(1.0) / t370;
            let t372 = t120 * t371;
            let t373 = t372 * t37;
            let t377 = t208 * t164;
            let t389 = f64x8::splat(1.0) / t34 / t370;
            let t390 = t65 * t389;
            let tv3sigma30 = -f64x8::splat(2.3217410626059214e-06) * t298 * t299 * t360 * t191 + f64x8::splat(0.0018323929555280486) * t70 * t250 * t366 - f64x8::splat(0.00017619163033923545) * t147 * t149 * t373 + f64x8::splat(9.283011157599174e-05) * t298 * t207 * t377 * t191 - f64x8::splat(0.012680397542963165) * t70 * t319 * t366 + f64x8::splat(0.010566997952469305) * t157 * t159 * t373 - f64x8::splat(0.0013547433272396543) * t70 * t64 * t390;
            acc_v3sigma3 = tv3sigma30;
            let t396 = t48 * t23;
            let t397 = t188 * t11;
            let t398 = t396 * t397;
            let t407 = t91 * t91;
            let t412 = f64x8::splat(1.0) / t199;
            let t421 = f64x8::splat(1.0) / t199 / t107;
            let t422 = t163 * t421;
            let t434 = t288 * t92;
            let t439 = t6 * t233;
            let t451 = f64x8::splat(1.0) / t200;
            let t457 = t199 * t199;
            let t461 = f64x8::splat(1.0) / t457 * t6 * t21 * t58;
            let t462 = (simd::pow(t39, f64x8::splat(0.6)));
            let t463 = t114 * t462;
            let t466 = t370 * t28 * t65;
            let t470 = (simd::pow(t39, -f64x8::splat(1.7)));
            let t478 = t199 * t85;
            let t479 = f64x8::splat(1.0) / t478;
            let t481 = t231 * t479 * t174;
            let t489 = f64x8::splat(1.0) / t79 / t478 * t6;
            let t490 = t489 * t205;
            let t493 = -f64x8::splat(4.154494839810372) * t288 * t184 * t43 * t398 - f64x8::splat(0.32691583697407145) * t110 * t170 * t174 + f64x8::splat(1.0289960015387654) * t6 * t266 * t76 + f64x8::splat(9.453075836052559) * t6 / t407 * t43 * t48 * t18 * t412 - f64x8::splat(4.726537918026279) * t185 * t94 * t412 - f64x8::splat(0.21721248282080596) * t51 * t114 * t210 * t422 + f64x8::splat(1.5031386281815007) * t51 * t61 * t217 * t422 + f64x8::splat(40.0) / f64x8::splat(243.0) * t24 * t26 * t191 * t43 + f64x8::splat(1.038623709952593) * t434 * t43 * t23 * t397 - f64x8::splat(0.07440502418321111) * t439 * t92 * t61 * t18 * t7 * t173 * t62 * t262 + f64x8::splat(0.09005238659655486) * t185 * t61 * t396 * t188 * t11 * t451 * t62 * t262 + f64x8::splat(0.07412821279844826) * t461 * t463 * t209 * t466 + f64x8::splat(0.22442738169866658) * t461 * t61 * t470 * t209 * t466 + f64x8::splat(0.005529584092817026) * t93 * t114 * t149 * t481 - f64x8::splat(0.331633822073741) * t255 * t159 * t481 + f64x8::splat(0.30114586449369607) * t490 * t212;
            let t496 = t489 * t204;
            let t519 = t421 * t6 * t50;
            let t529 = (simd::pow(t39, f64x8::splat(2.9)));
            let t530 = t243 * t529;
            let t532 = t32 * t33;
            let t540 = t189 * t11;
            let t548 = f64x8::splat(1.0) / t242 / t42;
            let t549 = (simd::pow(t39, f64x8::splat(5.2)));
            let t550 = t548 * t549;
            let t558 = t24 * t26 / t13 / t200;
            let t563 = -f64x8::splat(2.0839685443447613) * t490 * t219 - f64x8::splat(0.0003815681114248151) * t496 * t247 + f64x8::splat(0.015256227719748173) * t496 * t252 + f64x8::splat(0.08921762987781937) * t6 * t272 * t133 * t226 - f64x8::splat(0.03480969275974454) * t203 * t289 * t148 * t122 + f64x8::splat(2.0876925391409733) * t203 * t133 * t158 * t122 + f64x8::splat(0.0003669599646765861) * t51 * t243 * t244 * t163 * t421 - f64x8::splat(0.022008225861295017) * t519 * t251 + f64x8::splat(0.050766594995289835) * t519 * t319 * t163 + f64x8::splat(0.0017840241346365818) * t24 * t26 * t412 * t67 - f64x8::splat(0.00045788173370977814) * t461 * t530 * t370 * t532 + f64x8::splat(0.004459512718080236) * t461 * t463 * t370 * t532 - f64x8::splat(0.022513096649138715) * t255 * t540 * t451 * t62 * t28 * t66 + f64x8::splat(2.544870660136395e-06) * t461 * t550 * t370 * t532 + f64x8::splat(0.01067300740161953) * t558 * t123 - f64x8::splat(0.6401078595076056) * t558 * t129;
            let tv4rho40 = t493 + t563;
            acc_v4rho4 = tv4rho40;
            let t565 = t6 * t479 * t50;
            let t566 = t209 * t34;
            let t603 = t6 * t412;
            let t609 = f64x8::splat(0.05430312070520149) * t565 * t250 * t566 - f64x8::splat(0.37578465704537517) * t565 * t319 * t566 - f64x8::splat(0.019037473123233686) * t565 * t319 * t34 + f64x8::splat(0.0068775705816546935) * t565 * t250 * t34 - f64x8::splat(0.00010320749006528984) * t565 * t245 * t34 + f64x8::splat(0.0071794991316973126) * t439 * t289 * t291 - f64x8::splat(0.4305865861978257) * t439 * t133 * t295 - f64x8::splat(0.011709813921463792) * t225 * t136 + f64x8::splat(0.00011557111067193919) * t298 * t299 * t244 * t202 * t34 - f64x8::splat(0.004620876665116033) * t298 * t207 * t208 * t202 * t34 + f64x8::splat(0.00797196687677262) * t434 * t64 * t285 - f64x8::splat(0.008442411243427018) * t603 * t184 * t64 * t72 * t398;
            let t610 = t462 * t209;
            let t613 = v_sigma * t28 * t65;
            let t617 = t470 * t209;
            let t622 = t6 * t451;
            let t623 = t622 * t92;
            let t627 = t121 * t18 * t10 * t12;
            let t633 = t603 * t92;
            let t639 = t24 * t26 * t548;
            let t646 = t24 * t26 * t243;
            let t652 = t121 * t272;
            let t665 = t209 * t202 * t34;
            let t672 = -f64x8::splat(0.0277980797994181) * t147 * t610 * t421 * t613 - f64x8::splat(0.08416026813699996) * t157 * t617 * t421 * t613 - f64x8::splat(0.0010367970174031925) * t623 * t117 * t627 + f64x8::splat(0.06218134163882644) * t623 * t128 * t627 + f64x8::splat(0.0021106028108567546) * t633 * t64 * t72 * t540 - f64x8::splat(9.54326497551148e-07) * t639 * t549 * t421 * v_sigma * t532 + f64x8::splat(0.0001717056501411668) * t646 * t529 * t421 * v_sigma * t532 - f64x8::splat(0.002436229950369675) * t70 * t117 * t652 - f64x8::splat(0.0016723172692800882) * t147 * t462 * t421 * v_sigma * t532 + f64x8::splat(0.14611157662673604) * t70 * t128 * t652 - f64x8::splat(0.09121244934184064) * t70 * t250 * t665 + f64x8::splat(0.6312020110274997) * t70 * t319 * t665;
            let tv4rho3sigma0 = t609 + t672;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t673 = t622 * t50;
            let t705 = t327 * t174;
            let t734 = t336 * t233;
            let t745 = f64x8::splat(0.00012959962717539906) * t633 * t117 * t705 - f64x8::splat(2.3333333333333332e-23) * t147 * t149 * t151 * t109 - f64x8::splat(0.007772667704853305) * t633 * t128 * t705 + f64x8::splat(0.0009964958595965775) * t284 * t165 * t174 + f64x8::splat(0.010424279924781787) * t147 * t610 * t479 * t532 + f64x8::splat(0.03156010055137499) * t157 * t617 * t479 * t532 + f64x8::splat(3.578724365816805e-07) * t70 * t550 * t479 * t532 - f64x8::splat(6.438961880293756e-05) * t70 * t530 * t479 * t532 + f64x8::splat(0.017916731120718697) * t70 * t250 * t734 + f64x8::splat(0.0006271189759800331) * t70 * t463 * t479 * t532 - f64x8::splat(0.12398610930897316) * t70 * t319 * t734;
            let tv4rho2sigma20 = f64x8::splat(0.07045962319600785) * t673 * t319 * t336 - f64x8::splat(0.01018183513222528) * t673 * t250 * t336 + f64x8::splat(0.007139052421212633) * t673 * t319 * t71 - f64x8::splat(0.002063271174496408) * t673 * t250 * t71 + f64x8::splat(2.580187251632246e-05) * t673 * t245 * t71 - f64x8::splat(0.0003263408696226051) * t290 * t328 - f64x8::splat(2.270146816770234e-05) * t298 * t299 * t331 * t233 + f64x8::splat(0.019572117554446624) * t294 * t346 + f64x8::splat(0.0009076722020763637) * t298 * t207 * t349 * t233 - f64x8::splat(0.0025092458403136696) * t279 * t357 + t745;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t754 = t121 * t371;
            let t764 = t603 * t50;
            let t776 = t451 * t28 * t65;
            let t803 = t365 * t259;
            let tv4rhosigma30 = -f64x8::splat(3.0956547501412286e-06) * t298 * t299 * t360 * t259 + f64x8::splat(0.00012377348210132233) * t298 * t207 * t377 * t259 - f64x8::splat(0.00036713347832543076) * t326 * t148 * t754 + f64x8::splat(0.022018632248752452) * t345 * t158 * t754 - f64x8::splat(0.0028229015703528783) * t134 * t135 * t390 - f64x8::splat(0.0026771446579547374) * t764 * t319 * t164 + f64x8::splat(0.0005802950178271148) * t764 * t250 * t164 - f64x8::splat(4.837851096810462e-06) * t764 * t245 * t164 - f64x8::splat(0.01183503770676562) * t157 * t617 * t150 * t776 - f64x8::splat(1e-21) * t157 * t159 * t372 * t102 - f64x8::splat(0.00390910497179317) * t147 * t610 * t150 * t776 - f64x8::splat(1.3420216371813018e-07) * t639 * t549 * t150 * t451 * t532 + f64x8::splat(2.414610705110158e-05) * t646 * t529 * t150 * t451 * t532 - f64x8::splat(0.00023516961599251242) * t147 * t462 * t150 * t451 * t532 + f64x8::splat(0.0024431906073707314) * t70 * t250 * t803 - f64x8::splat(0.016907196723950885) * t70 * t319 * t803;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t827 = t412 * t28 * t65;
            let t832 = t209 * t389 * t191;
            let t836 = t370 * v_sigma;
            let t839 = t120 / t836 * t37;
            let tv4sigma40 = f64x8::splat(5.0325811394298826e-08) * t639 * t549 * t371 * t412 * t532 - f64x8::splat(9.054790144163093e-06) * t646 * t529 * t371 * t412 * t532 + f64x8::splat(6.965223187817764e-06) * t298 * t299 * t244 * t389 * t191 + f64x8::splat(0.0014659143644224389) * t147 * t610 * t371 * t827 - f64x8::splat(0.005497178866584146) * t70 * t250 * t832 + f64x8::splat(0.00044047907584808863) * t147 * t149 * t839 + f64x8::splat(8.818860599719215e-05) * t147 * t462 * t371 * t412 * t532 - f64x8::splat(0.0002784903347279752) * t298 * t207 * t208 * t389 * t191 + f64x8::splat(0.004438139140037108) * t157 * t617 * t371 * t827 + f64x8::splat(0.0380411926288895) * t70 * t319 * t832 - f64x8::splat(0.02641749488117326) * t157 * t159 * t839 + f64x8::splat(0.003386858318099136) * t70 * t64 * t65 / t34 / t836;
            acc_v4sigma4 = tv4sigma40;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho2sigma.into(); v3rho2sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rhosigma2.into(); v3rhosigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3sigma3.into(); v3sigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho4.into(); v4rho4[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho3sigma.into(); v4rho3sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho2sigma2.into(); v4rho2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rhosigma3.into(); v4rhosigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4sigma4.into(); v4sigma4[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
