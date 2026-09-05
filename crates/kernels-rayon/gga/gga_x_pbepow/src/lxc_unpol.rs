//! GGA_X_PBEPOW lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbepow.c`
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
pub fn gga_x_pbepow_lxc_unpol(
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
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t20 * t24;
            let t26 = t25 * v_sigma;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_rho * v_rho;
            let t30 = t18 * t18;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t33 = t28 * t32;
            let t34 = v_sigma * t28;
            let t38 = f64x8::splat(0.9146457198521546) * t25 * t34 * t32 + f64x8::splat(0.804);
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t33 * t39;
            let t41 = t26 * t40;
            let t42 = (simd::pow(t41, f64x8::splat(100.0)));
            let t44 = f64x8::splat(0.0001334414156799501) * t42 - f64x8::splat(1.0);
            let t45 = t33 * t44;
            let t48 = f64x8::splat(1.0) - f64x8::splat(0.009146457198521547) * t26 * t45;
            let t52 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t48));
            let tzk0 = f64x8::splat(2.0) * t52;
            acc_zk = tzk0;
            let t54 = t17 / t30;
            let t58 = t29 * v_rho;
            let t60 = f64x8::splat(1.0) / t30 / t58;
            let t61 = t28 * t60;
            let t62 = t61 * t44;
            let t65 = (simd::pow(t41, f64x8::splat(99.0)));
            let t66 = t61 * t39;
            let t69 = t20 * t20;
            let t72 = t69 / t22 / t21;
            let t73 = v_sigma * v_sigma;
            let t74 = t72 * t73;
            let t75 = t29 * t29;
            let t76 = t75 * t29;
            let t78 = f64x8::splat(1.0) / t18 / t76;
            let t80 = t38 * t38;
            let t81 = f64x8::splat(1.0) / t80;
            let t82 = t27 * t78 * t81;
            let t85 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t26 * t66 + f64x8::splat(4.8781105058781575) * t74 * t82;
            let t86 = t65 * t85;
            let t90 = f64x8::splat(0.024390552529390788) * t26 * t62 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t86;
            let t95 = ((t2).select(f64x8::splat(0.0), -t6 * t54 * t48 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t90));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t95 + f64x8::splat(2.0) * t52;
            acc_vrho = tvrho0;
            let t102 = t75 * v_rho;
            let t106 = t27 / t18 / t102 * t81;
            let t109 = t25 * t40 - f64x8::splat(1.8292914397043092) * t72 * v_sigma * t106;
            let t110 = t65 * t109;
            let t114 = -f64x8::splat(0.009146457198521547) * t25 * t45 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t110;
            let t118 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t114));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t118;
            acc_vsigma = tvsigma0;
            let t123 = t17 / t30 / v_rho;
            let t131 = f64x8::splat(1.0) / t30 / t75;
            let t132 = t28 * t131;
            let t133 = t132 * t44;
            let t139 = (simd::pow(t41, f64x8::splat(98.0)));
            let t140 = t85 * t85;
            let t141 = t139 * t140;
            let t145 = t132 * t39;
            let t148 = t75 * t58;
            let t150 = f64x8::splat(1.0) / t18 / t148;
            let t152 = t27 * t150 * t81;
            let t155 = t73 * v_sigma;
            let t156 = t75 * t75;
            let t157 = t156 * t29;
            let t158 = f64x8::splat(1.0) / t157;
            let t161 = f64x8::splat(1.0) / t80 / t38;
            let t164 = f64x8::splat(88.0) / f64x8::splat(9.0) * t26 * t145 - f64x8::splat(43.90299455290342) * t74 * t152 + f64x8::splat(2.931467096752081) * t155 * t158 * t161;
            let t165 = t65 * t164;
            let t169 = -f64x8::splat(0.08943202594109956) * t26 * t133 + f64x8::splat(0.0006509419717476189) * t26 * t61 * t86 - f64x8::splat(0.012083110350565177) * t26 * t33 * t141 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t165;
            let t174 = ((t2).select(f64x8::splat(0.0), t6 * t123 * t48 / f64x8::splat(12.0) - t6 * t54 * t90 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t169));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t174 + f64x8::splat(4.0) * t95;
            acc_v2rho2 = tv2rho20;
            let t182 = t25 * t28;
            let t183 = t32 * t65;
            let t190 = t25 * t34;
            let t191 = t32 * t139;
            let t192 = t109 * t85;
            let t193 = t191 * t192;
            let t198 = t72 * t27;
            let t203 = t156 * v_rho;
            let t204 = f64x8::splat(1.0) / t203;
            let t208 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t25 * t66 + f64x8::splat(14.634331517634473) * t198 * t78 * t81 * v_sigma - f64x8::splat(1.0993001612820303) * t73 * t204 * t161;
            let t209 = t65 * t208;
            let t213 = f64x8::splat(0.024390552529390788) * t25 * t62 - f64x8::splat(0.00012205161970267855) * t182 * t183 * t85 + f64x8::splat(0.00032547098587380947) * t26 * t61 * t110 - f64x8::splat(0.012083110350565177) * t190 * t193 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t209;
            let t218 = ((t2).select(f64x8::splat(0.0), -t6 * t54 * t114 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t213));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t218 + f64x8::splat(2.0) * t118;
            acc_v2rhosigma = tv2rhosigma0;
            let t224 = t109 * t109;
            let t225 = t139 * t224;
            let t231 = f64x8::splat(1.0) / t156;
            let t235 = -f64x8::splat(3.6585828794086184) * t72 * t106 + f64x8::splat(0.4122375604807614) * v_sigma * t231 * t161;
            let t236 = t65 * t235;
            let t240 = -f64x8::splat(0.0002441032394053571) * t182 * t183 * t109 - f64x8::splat(0.012083110350565177) * t26 * t33 * t225 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t236;
            let t244 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t240));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t244;
            acc_v2sigma2 = tv2sigma20;
            let t247 = t17 * t32;
            let t259 = t28 / t30 / t102;
            let t260 = t259 * t44;
            let t272 = (simd::pow(t41, f64x8::splat(97.0)));
            let t273 = t140 * t85;
            let t274 = t272 * t273;
            let t278 = t85 * t164;
            let t279 = t191 * t278;
            let t282 = t259 * t39;
            let t286 = f64x8::splat(1.0) / t18 / t156;
            let t291 = t156 * t58;
            let t292 = f64x8::splat(1.0) / t291;
            let t296 = t73 * t73;
            let t297 = t156 * t102;
            let t299 = f64x8::splat(1.0) / t30 / t297;
            let t301 = t80 * t80;
            let t302 = f64x8::splat(1.0) / t301;
            let t306 = -f64x8::splat(1232.0) / f64x8::splat(27.0) * t26 * t282 + f64x8::splat(369.65237388987816) * t74 * t27 * t286 * t81 - f64x8::splat(55.697874838289536) * t155 * t292 * t161 + f64x8::splat(21.450030663453703) * t296 * t299 * t302 * t182;
            let t307 = t65 * t306;
            let t311 = f64x8::splat(0.41734945439179794) * t26 * t260 - f64x8::splat(0.003580180844611904) * t26 * t132 * t86 + f64x8::splat(0.09666488280452142) * t26 * t61 * t141 + f64x8::splat(0.0009764129576214284) * t26 * t61 * t165 - f64x8::splat(1.1841448143553872) * t26 * t33 * t274 - f64x8::splat(0.036249331051695526) * t190 * t279 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t307;
            let t316 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t247 * t48 + t6 * t123 * t90 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t54 * t169 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t311));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t316 + f64x8::splat(6.0) * t174;
            acc_v3rho3 = tv3rho30;
            let t328 = t60 * t65;
            let t341 = t60 * t139;
            let t342 = t341 * t192;
            let t348 = t32 * t272;
            let t349 = t109 * t140;
            let t350 = t348 * t349;
            let t353 = t208 * t85;
            let t354 = t191 * t353;
            let t357 = t109 * t164;
            let t358 = t191 * t357;
            let t367 = t158 * t161;
            let t370 = t156 * t75;
            let t372 = f64x8::splat(1.0) / t30 / t370;
            let t377 = f64x8::splat(88.0) / f64x8::splat(9.0) * t25 * t145 - f64x8::splat(105.69239429402676) * t198 * t150 * t81 * v_sigma + f64x8::splat(18.688102741794516) * t367 * t73 - f64x8::splat(8.043761498795138) * t155 * t372 * t302 * t182;
            let t378 = t65 * t377;
            let t382 = -f64x8::splat(0.08943202594109956) * t25 * t133 + f64x8::splat(0.0006509419717476189) * t182 * t328 * t85 - f64x8::splat(0.012083110350565177) * t182 * t191 * t140 - f64x8::splat(0.00012205161970267855) * t182 * t183 * t164 - f64x8::splat(0.0011933936148706347) * t26 * t132 * t110 + f64x8::splat(0.06444325520301428) * t190 * t342 + f64x8::splat(0.0006509419717476189) * t26 * t61 * t209 - f64x8::splat(1.1841448143553872) * t190 * t350 - f64x8::splat(0.024166220701130354) * t190 * t354 - f64x8::splat(0.012083110350565177) * t190 * t358 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t378;
            let t387 = ((t2).select(f64x8::splat(0.0), t6 * t123 * t114 / f64x8::splat(12.0) - t6 * t54 * t213 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t382));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t387 + f64x8::splat(4.0) * t218;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t404 = t224 * t85;
            let t405 = t348 * t404;
            let t408 = t109 * t208;
            let t409 = t191 * t408;
            let t415 = t235 * t85;
            let t416 = t191 * t415;
            let t421 = t204 * t161;
            let t425 = f64x8::splat(1.0) / t30 / t291;
            let t430 = f64x8::splat(19.51244202351263) * t72 * t82 - f64x8::splat(5.496500806410151) * t421 * v_sigma + f64x8::splat(3.016410562048177) * t73 * t425 * t302 * t182;
            let t431 = t65 * t430;
            let t435 = f64x8::splat(0.0006509419717476189) * t182 * t328 * t109 - f64x8::splat(0.024166220701130354) * t182 * t193 - f64x8::splat(0.0002441032394053571) * t182 * t183 * t208 + f64x8::splat(0.03222162760150714) * t26 * t61 * t225 - f64x8::splat(1.1841448143553872) * t190 * t405 - f64x8::splat(0.024166220701130354) * t190 * t409 + f64x8::splat(0.00032547098587380947) * t26 * t61 * t236 - f64x8::splat(0.012083110350565177) * t190 * t416 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t431;
            let t440 = ((t2).select(f64x8::splat(0.0), -t6 * t54 * t240 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t435));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t440 + f64x8::splat(2.0) * t244;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t449 = t224 * t109;
            let t450 = t272 * t449;
            let t454 = t109 * t235;
            let t455 = t191 * t454;
            let t461 = f64x8::splat(1.0) / t30 / t157;
            let t466 = f64x8::splat(1.2367126814422842) * t231 * t161 - f64x8::splat(1.1311539607680663) * v_sigma * t461 * t302 * t182;
            let t467 = t65 * t466;
            let t471 = -f64x8::splat(0.036249331051695526) * t182 * t191 * t224 - f64x8::splat(0.0003661548591080356) * t182 * t183 * t235 - f64x8::splat(1.1841448143553872) * t26 * t33 * t450 - f64x8::splat(0.036249331051695526) * t190 * t455 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t467;
            let t475 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t471));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t475;
            acc_v3sigma3 = tv3sigma30;
            let t493 = t28 / t30 / t76;
            let t515 = (simd::pow(t41, f64x8::splat(96.0)));
            let t516 = t140 * t140;
            let t525 = t164 * t164;
            let t547 = t156 * t76;
            let t555 = t156 * t156;
            let t561 = f64x8::splat(1.0) / t301 / t38;
            let t570 = -f64x8::splat(2.3649802415535217) * t26 * t493 * t44 + f64x8::splat(0.022276680810918513) * t26 * t259 * t86 - f64x8::splat(0.708875807233157) * t26 * t132 * t141 - f64x8::splat(0.007160361689223808) * t26 * t132 * t165 + f64x8::splat(12.630878019790797) * t26 * t61 * t274 + f64x8::splat(0.38665953121808566) * t190 * t341 * t278 + f64x8::splat(0.0013018839434952379) * t26 * t61 * t307 - f64x8::splat(114.86204699247257) * t26 * t33 * t515 * t516 - f64x8::splat(7.104868886132324) * t190 * t348 * t140 * t164 - f64x8::splat(0.036249331051695526) * t26 * t33 * t139 * t525 - f64x8::splat(0.04833244140226071) * t190 * t191 * t85 * t306 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t65 * (f64x8::splat(20944.0) / f64x8::splat(81.0) * t26 * t493 * t39 - f64x8::splat(3303.022824757944) * t74 * t27 / t18 / t203 * t81 + f64x8::splat(834.8166854417315) * t155 / t370 * t161 - f64x8::splat(700.7010016728209) * t296 / t30 / t547 * t302 * t182 + f64x8::splat(418.54247972320854) * t296 * v_sigma / t18 / t555 / v_rho * t561 * t198);
            let t575 = ((t2).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t60 * t48 - f64x8::splat(5.0) / f64x8::splat(9.0) * t6 * t247 * t90 + t6 * t123 * t169 / f64x8::splat(2.0) - t6 * t54 * t311 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t570));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t575 + f64x8::splat(8.0) * t316;
            acc_v4rho4 = tv4rho40;
            let t661 = t131 * t65;
            let t668 = t60 * t272;
            let t672 = t32 * t515;
            let t681 = -f64x8::splat(0.003580180844611904) * t26 * t132 * t209 + f64x8::splat(0.09666488280452142) * t182 * t341 * t140 - f64x8::splat(1.1841448143553872) * t182 * t348 * t273 - f64x8::splat(0.036249331051695526) * t182 * t279 - f64x8::splat(0.00012205161970267855) * t182 * t183 * t306 + f64x8::splat(0.41734945439179794) * t25 * t260 - f64x8::splat(0.003580180844611904) * t182 * t661 * t85 + f64x8::splat(0.0009764129576214284) * t182 * t328 * t164 + f64x8::splat(9.473158514843098) * t190 * t668 * t349 - f64x8::splat(114.86204699247257) * t190 * t672 * t109 * t273 - f64x8::splat(3.552434443066162) * t190 * t348 * t192 * t164;
            let t687 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t247 * t114 + t6 * t123 * t213 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t54 * t382 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-f64x8::splat(0.00012205161970267855) * t26 * t33 * t65 * (-f64x8::splat(1232.0) / f64x8::splat(27.0) * t25 * t282 + f64x8::splat(822.7746386581159) * t198 * t286 * t81 * v_sigma - f64x8::splat(250.3961478475736) * t292 * t161 * t73 + f64x8::splat(238.63159113092243) * t299 * t302 * t155 * t182 - f64x8::splat(156.9534298962032) * t296 / t18 / t555 * t561 * t198) - f64x8::splat(0.036249331051695526) * t190 * t191 * t377 * t85 - f64x8::splat(0.036249331051695526) * t190 * t191 * t208 * t164 - f64x8::splat(0.012083110350565177) * t190 * t191 * t109 * t306 - f64x8::splat(3.552434443066162) * t190 * t348 * t208 * t140 + f64x8::splat(0.19332976560904283) * t190 * t341 * t353 + f64x8::splat(0.09666488280452142) * t190 * t341 * t357 + f64x8::splat(0.005569170202729628) * t26 * t259 * t110 - f64x8::splat(0.3544379036165785) * t190 * t131 * t139 * t192 + f64x8::splat(0.0009764129576214284) * t26 * t61 * t378 + t681)));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t687 + f64x8::splat(6.0) * t387;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t719 = t208 * t208;
            let t737 = -f64x8::splat(1.1841448143553872) * t190 * t348 * t235 * t140 - f64x8::splat(0.0011933936148706347) * t26 * t132 * t236 + f64x8::splat(0.06444325520301428) * t190 * t341 * t415 - f64x8::splat(0.024166220701130354) * t190 * t191 * t430 * t85 - f64x8::splat(0.012083110350565177) * t190 * t191 * t235 * t164 - f64x8::splat(1.1841448143553872) * t190 * t348 * t224 * t164 - f64x8::splat(0.024166220701130354) * t26 * t33 * t139 * t719 - f64x8::splat(0.024166220701130354) * t190 * t191 * t109 * t377 - f64x8::splat(0.11814596787219284) * t26 * t132 * t225 + f64x8::splat(6.315439009895399) * t190 * t668 * t404 + f64x8::splat(0.12888651040602855) * t190 * t341 * t408;
            let t786 = -f64x8::splat(2.3682896287107744) * t182 * t350 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t65 * (-f64x8::splat(123.57879948224667) * t72 * t152 + f64x8::splat(61.19437564469969) * t367 * v_sigma - f64x8::splat(75.41026405120442) * t372 * t302 * t73 * t182 + f64x8::splat(58.8575362110762) * t155 / t18 / t156 / t148 * t561 * t198) + f64x8::splat(0.0006509419717476189) * t26 * t61 * t431 - f64x8::splat(0.0023867872297412694) * t182 * t661 * t109 + f64x8::splat(0.12888651040602855) * t182 * t342 - f64x8::splat(0.04833244140226071) * t182 * t354 - f64x8::splat(0.024166220701130354) * t182 * t358 - f64x8::splat(0.0002441032394053571) * t182 * t183 * t377 + f64x8::splat(0.0013018839434952379) * t182 * t328 * t208 - f64x8::splat(114.86204699247257) * t190 * t672 * t224 * t140 - f64x8::splat(4.736579257421549) * t190 * t348 * t192 * t208;
            let t792 = ((t2).select(f64x8::splat(0.0), t6 * t123 * t240 / f64x8::splat(12.0) - t6 * t54 * t435 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t737 + t786)));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t792 + f64x8::splat(4.0) * t440;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t864 = f64x8::splat(0.09666488280452142) * t182 * t341 * t224 - f64x8::splat(3.552434443066162) * t182 * t405 - f64x8::splat(0.07249866210339105) * t182 * t409 + f64x8::splat(0.0009764129576214284) * t182 * t328 * t235 - f64x8::splat(0.036249331051695526) * t182 * t416 - f64x8::splat(0.0003661548591080356) * t182 * t183 * t430 + f64x8::splat(3.1577195049476994) * t26 * t61 * t450 - f64x8::splat(114.86204699247257) * t190 * t672 * t449 * t85 - f64x8::splat(3.552434443066162) * t190 * t348 * t224 * t208 + f64x8::splat(0.09666488280452142) * t190 * t341 * t454 - f64x8::splat(3.552434443066162) * t190 * t348 * t454 * t85 - f64x8::splat(0.036249331051695526) * t190 * t191 * t208 * t235 - f64x8::splat(0.036249331051695526) * t190 * t191 * t109 * t430 + f64x8::splat(0.00032547098587380947) * t26 * t61 * t467 - f64x8::splat(0.012083110350565177) * t190 * t191 * t466 * t85 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t65 * (-f64x8::splat(9.893701451538274) * t421 + f64x8::splat(21.114873934337236) * t425 * t302 * t20 * t24 * v_sigma * t28 - f64x8::splat(22.071576079153576) * t73 / t18 / t547 * t561 * t198);
            let t869 = ((t2).select(f64x8::splat(0.0), -t6 * t54 * t471 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t864));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t869 + f64x8::splat(2.0) * t475;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t880 = t224 * t224;
            let t889 = t235 * t235;
            let t916 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-f64x8::splat(4.736579257421549) * t182 * t348 * t449 - f64x8::splat(0.1449973242067821) * t182 * t455 - f64x8::splat(0.0004882064788107142) * t182 * t183 * t466 - f64x8::splat(114.86204699247257) * t26 * t33 * t515 * t880 - f64x8::splat(7.104868886132324) * t190 * t348 * t224 * t235 - f64x8::splat(0.036249331051695526) * t26 * t33 * t139 * t889 - f64x8::splat(0.04833244140226071) * t190 * t191 * t109 * t466 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t65 * (-f64x8::splat(4.524615843072265) * t461 * t302 * t182 + f64x8::splat(8.27684102968259) * v_sigma / t18 / t297 * t561 * t198))));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t916;
            acc_v4sigma4 = tv4sigma40;
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
        store_add(v4rho4, ip, m, acc_v4rho4);
        store_add(v4rho3sigma, ip, m, acc_v4rho3sigma);
        store_add(v4rho2sigma2, ip, m, acc_v4rho2sigma2);
        store_add(v4rhosigma3, ip, m, acc_v4rhosigma3);
        store_add(v4sigma4, ip, m, acc_v4sigma4);
        ip += 8;
    }
}
