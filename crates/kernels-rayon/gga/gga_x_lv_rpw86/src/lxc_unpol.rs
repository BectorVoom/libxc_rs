//! GGA_X_LV_RPW86 lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lv_rpw86.c`
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
pub fn gga_x_lv_rpw86_lxc_unpol(
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
            let t25 = t20 / t23;
            let t26 = f64x8::splat(M_CBRT2);
            let t27 = t26 * t26;
            let t28 = v_sigma * t27;
            let t29 = v_rho * v_rho;
            let t30 = t18 * t18;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t34 = t25 * t28 * t32;
            let t36 = f64x8::splat(1.0) + f64x8::splat(0.003931018518518519) * t34;
            let t37 = v_sigma * v_sigma;
            let t38 = t37 * v_sigma;
            let t39 = t29 * t29;
            let t40 = t39 * t39;
            let t41 = f64x8::splat(1.0) / t40;
            let t42 = t38 * t41;
            let t43 = f64x8::splat(3.881824540052514e-07) * t42;
            let t44 = f64x8::splat(1.0) + t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t48 = t20 * t20;
            let t51 = t48 / t22 / t21;
            let t52 = t37 * t26;
            let t53 = t39 * v_rho;
            let t55 = f64x8::splat(1.0) / t18 / t53;
            let t60 = f64x8::splat(1.0) + f64x8::splat(0.077125) * t34 + f64x8::splat(0.06017361111111111) * t51 * t52 * t55 + f64x8::splat(2.905130394988796e-06) * t42;
            let t61 = (simd::pow(t60, f64x8::splat(1.0) / f64x8::splat(15.0)));
            let t62 = f64x8::splat(1.15) + t43;
            let t63 = f64x8::splat(1.0) / t62;
            let t64 = t61 * t63;
            let t67 = t36 * t45 + f64x8::splat(3.881824540052514e-07) * t42 * t64;
            let t71 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t67));
            let tzk0 = f64x8::splat(2.0) * t71;
            acc_zk = tzk0;
            let t73 = t17 / t30;
            let t77 = t25 * v_sigma;
            let t78 = t29 * v_rho;
            let t80 = f64x8::splat(1.0) / t30 / t78;
            let t81 = t27 * t80;
            let t82 = t81 * t45;
            let t85 = t44 * t44;
            let t86 = f64x8::splat(1.0) / t85;
            let t87 = t36 * t86;
            let t88 = t40 * v_rho;
            let t89 = f64x8::splat(1.0) / t88;
            let t90 = t38 * t89;
            let t95 = t61 * t61;
            let t96 = t95 * t95;
            let t98 = t96 * t96;
            let t99 = t98 * t96 * t95;
            let t100 = f64x8::splat(1.0) / t99;
            let t101 = t100 * t63;
            let t105 = t39 * t29;
            let t107 = f64x8::splat(1.0) / t18 / t105;
            let t112 = -f64x8::splat(0.20566666666666666) * t25 * t28 * t80 - f64x8::splat(0.32092592592592595) * t51 * t52 * t107 - f64x8::splat(2.324104315991037e-05) * t90;
            let t113 = t101 * t112;
            let t116 = t37 * t37;
            let t117 = t116 * t37;
            let t118 = t40 * t40;
            let t120 = f64x8::splat(1.0) / t118 / v_rho;
            let t121 = t117 * t120;
            let t122 = t62 * t62;
            let t123 = f64x8::splat(1.0) / t122;
            let t124 = t61 * t123;
            let t127 = -f64x8::splat(0.010482716049382716) * t77 * t82 + f64x8::splat(3.1054596320420114e-06) * t87 * t90 - f64x8::splat(3.1054596320420114e-06) * t90 * t64 + f64x8::splat(2.5878830267016762e-08) * t42 * t113 + f64x8::splat(1.205484940780313e-12) * t121 * t124;
            let t132 = ((t2).select(f64x8::splat(0.0), -t6 * t73 * t67 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t127));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t132 + f64x8::splat(2.0) * t71;
            acc_vrho = tvrho0;
            let t135 = t27 * t32;
            let t139 = t37 * t41;
            let t146 = v_sigma * t26;
            let t151 = f64x8::splat(0.077125) * t25 * t135 + f64x8::splat(0.12034722222222222) * t51 * t146 * t55 + f64x8::splat(8.715391184966388e-06) * t139;
            let t152 = t101 * t151;
            let t155 = t116 * v_sigma;
            let t156 = f64x8::splat(1.0) / t118;
            let t157 = t155 * t156;
            let t160 = f64x8::splat(0.003931018518518519) * t25 * t135 * t45 - f64x8::splat(1.1645473620157543e-06) * t87 * t139 + f64x8::splat(1.1645473620157543e-06) * t139 * t64 + f64x8::splat(2.5878830267016762e-08) * t42 * t152 - f64x8::splat(4.5205685279261743e-13) * t157 * t124;
            let t164 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t160));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t164;
            acc_vsigma = tvsigma0;
            let t169 = t17 / t30 / v_rho;
            let t177 = f64x8::splat(1.0) / t30 / t39;
            let t178 = t27 * t177;
            let t179 = t178 * t45;
            let t182 = t25 * t116;
            let t183 = t40 * t39;
            let t185 = f64x8::splat(1.0) / t30 / t183;
            let t187 = t27 * t185 * t86;
            let t191 = f64x8::splat(1.0) / t85 / t44;
            let t192 = t36 * t191;
            let t193 = t118 * t29;
            let t194 = f64x8::splat(1.0) / t193;
            let t195 = t117 * t194;
            let t198 = t40 * t29;
            let t199 = f64x8::splat(1.0) / t198;
            let t200 = t38 * t199;
            let t210 = f64x8::splat(1.0) / t99 / t60;
            let t211 = t210 * t63;
            let t212 = t112 * t112;
            let t213 = t211 * t212;
            let t216 = t100 * t123;
            let t217 = t216 * t112;
            let t225 = f64x8::splat(1.0) / t18 / t39 / t78;
            let t230 = f64x8::splat(0.7541111111111111) * t25 * t28 * t177 + f64x8::splat(2.032530864197531) * t51 * t52 * t225 + f64x8::splat(0.00020916938843919332) * t200;
            let t231 = t101 * t230;
            let t234 = t116 * t116;
            let t235 = t234 * v_sigma;
            let t236 = t118 * t198;
            let t237 = f64x8::splat(1.0) / t236;
            let t238 = t235 * t237;
            let t240 = f64x8::splat(1.0) / t122 / t62;
            let t241 = t61 * t240;
            let t244 = f64x8::splat(0.03843662551440329) * t77 * t179 - f64x8::splat(6.510730305103387e-08) * t182 * t187 + f64x8::splat(1.9287759052485008e-11) * t192 * t195 - f64x8::splat(2.79491366883781e-05) * t87 * t200 + f64x8::splat(2.79491366883781e-05) * t200 * t64 - f64x8::splat(4.140612842722682e-07) * t90 * t113 - f64x8::splat(3.013712351950783e-11) * t195 * t124 - f64x8::splat(2.4153574915882312e-08) * t42 * t213 + f64x8::splat(1.6073132543737508e-13) * t121 * t217 + f64x8::splat(2.5878830267016762e-08) * t42 * t231 + f64x8::splat(7.487169641255634e-18) * t238 * t241;
            let t249 = ((t2).select(f64x8::splat(0.0), t6 * t169 * t67 / f64x8::splat(12.0) - t6 * t73 * t127 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t244));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t249 + f64x8::splat(4.0) * t132;
            acc_v2rho2 = tv2rho20;
            let t257 = t25 * t27;
            let t258 = t40 * t78;
            let t260 = f64x8::splat(1.0) / t30 / t258;
            let t261 = t260 * t86;
            let t265 = t155 * t120;
            let t268 = t37 * t89;
            let t279 = t42 * t210;
            let t280 = t63 * t151;
            let t281 = t280 * t112;
            let t284 = t216 * t151;
            let t293 = -f64x8::splat(0.20566666666666666) * t25 * t81 - f64x8::splat(0.6418518518518519) * t51 * t146 * t107 - f64x8::splat(6.97231294797311e-05) * t268;
            let t294 = t101 * t293;
            let t300 = f64x8::splat(1.0) / t118 / t88;
            let t301 = t234 * t300;
            let t304 = -f64x8::splat(0.010482716049382716) * t25 * t82 + f64x8::splat(2.4415238644137703e-08) * t257 * t261 * t38 - f64x8::splat(7.232909644681879e-12) * t192 * t265 + f64x8::splat(9.316378896126034e-06) * t87 * t268 - f64x8::splat(9.316378896126034e-06) * t268 * t64 + f64x8::splat(7.763649080105028e-08) * t139 * t113 + f64x8::splat(1.0849364467022818e-11) * t265 * t124 - f64x8::splat(2.070306421361341e-07) * t90 * t152 - f64x8::splat(2.4153574915882312e-08) * t279 * t281 + f64x8::splat(8.036566271868754e-14) * t121 * t284 + f64x8::splat(2.5878830267016762e-08) * t42 * t294 - f64x8::splat(3.0137123519507825e-14) * t157 * t217 - f64x8::splat(2.8076886154708627e-18) * t301 * t241;
            let t309 = ((t2).select(f64x8::splat(0.0), -t6 * t73 * t160 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t304));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t309 + f64x8::splat(2.0) * t164;
            acc_v2rhosigma = tv2rhosigma0;
            let t313 = f64x8::splat(1.0) / t30 / t198;
            let t314 = t313 * t86;
            let t318 = t116 * t156;
            let t321 = v_sigma * t41;
            let t330 = t151 * t151;
            let t331 = t211 * t330;
            let t340 = f64x8::splat(0.12034722222222222) * t51 * t26 * t55 + f64x8::splat(1.7430782369932776e-05) * t321;
            let t341 = t101 * t340;
            let t344 = t116 * t38;
            let t346 = f64x8::splat(1.0) / t118 / t40;
            let t347 = t344 * t346;
            let t350 = -f64x8::splat(9.155714491551638e-09) * t257 * t314 * t37 + f64x8::splat(2.7123411167557045e-12) * t192 * t318 - f64x8::splat(2.3290947240315086e-06) * t87 * t321 + f64x8::splat(2.3290947240315086e-06) * t321 * t64 + f64x8::splat(1.5527298160210056e-07) * t139 * t152 - f64x8::splat(3.6164548223409394e-12) * t318 * t124 - f64x8::splat(2.4153574915882312e-08) * t42 * t331 - f64x8::splat(6.027424703901565e-14) * t157 * t284 + f64x8::splat(2.5878830267016762e-08) * t42 * t341 + f64x8::splat(1.0528832308015735e-18) * t347 * t241;
            let t354 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t350));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t354;
            acc_v2sigma2 = tv2sigma20;
            let t357 = t17 * t32;
            let t367 = t60 * t60;
            let t369 = f64x8::splat(1.0) / t99 / t367;
            let t370 = t369 * t63;
            let t371 = t212 * t112;
            let t372 = t370 * t371;
            let t375 = t63 * t112;
            let t376 = t375 * t230;
            let t379 = t40 * t53;
            let t381 = f64x8::splat(1.0) / t30 / t379;
            let t386 = t25 * t344;
            let t389 = f64x8::splat(1.0) / t30 / t118 / t53;
            let t396 = t210 * t123;
            let t397 = t396 * t212;
            let t400 = t216 * t230;
            let t403 = t100 * t240;
            let t404 = t403 * t112;
            let t408 = f64x8::splat(1.0) / t30 / t53;
            let t413 = f64x8::splat(1.0) / t18 / t40;
            let t417 = f64x8::splat(1.0) / t258;
            let t418 = t38 * t417;
            let t420 = -f64x8::splat(3.519185185185185) * t25 * t28 * t408 - f64x8::splat(14.90522633744856) * t51 * t52 * t413 - f64x8::splat(0.0020916938843919333) * t418;
            let t421 = t101 * t420;
            let t424 = t118 * t78;
            let t425 = f64x8::splat(1.0) / t424;
            let t426 = t117 * t425;
            let t439 = t27 * t408;
            let t440 = t439 * t45;
            let t443 = t85 * t85;
            let t444 = f64x8::splat(1.0) / t443;
            let t445 = t36 * t444;
            let t446 = t118 * t258;
            let t447 = f64x8::splat(1.0) / t446;
            let t448 = t235 * t447;
            let t455 = t234 * t116;
            let t456 = t118 * t118;
            let t458 = f64x8::splat(1.0) / t456 / t78;
            let t459 = t455 * t458;
            let t460 = t122 * t122;
            let t461 = f64x8::splat(1.0) / t460;
            let t462 = t61 * t461;
            let t465 = f64x8::splat(4.669691150403914e-08) * t42 * t372 - f64x8::splat(7.246072474764693e-08) * t279 * t376 + f64x8::splat(1.2370387579696436e-06) * t182 * t27 * t381 * t86 - f64x8::splat(6.065643041283341e-13) * t386 * t27 * t389 * t191 + f64x8::splat(5.796857979811754e-07) * t90 * t213 - f64x8::splat(2.2502385561232512e-13) * t121 * t397 + f64x8::splat(2.410969881560626e-13) * t121 * t400 + f64x8::splat(1.4974339282511268e-18) * t238 * t404 + f64x8::splat(2.5878830267016762e-08) * t42 * t421 - f64x8::splat(5.207694944170953e-10) * t192 * t426 + f64x8::splat(0.00027949136688378104) * t87 * t418 - f64x8::splat(0.00027949136688378104) * t418 * t64 + f64x8::splat(5.58982733767562e-06) * t200 * t113 - f64x8::splat(6.210919264084022e-07) * t90 * t231 - f64x8::splat(6.027424703901566e-12) * t195 * t217 - f64x8::splat(0.17937091906721536) * t77 * t440 + f64x8::splat(1.7969207139013521e-16) * t445 * t448 + f64x8::splat(6.292631390873234e-10) * t426 * t124 - f64x8::splat(3.8184565170403735e-16) * t448 * t241 + f64x8::splat(6.975330923750952e-23) * t459 * t462;
            let t470 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t357 * t67 + t6 * t169 * t127 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t73 * t244 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t465));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t470 + f64x8::splat(6.0) * t249;
            acc_v3rho3 = tv3rho30;
            let t480 = t42 * t369;
            let t481 = t280 * t212;
            let t484 = t90 * t210;
            let t487 = t63 * t293;
            let t488 = t487 * t112;
            let t491 = t280 * t230;
            let t494 = t121 * t210;
            let t495 = t123 * t151;
            let t496 = t495 * t112;
            let t503 = t118 * t39;
            let t505 = f64x8::splat(1.0) / t30 / t503;
            let t524 = f64x8::splat(4.669691150403914e-08) * t480 * t481 + f64x8::splat(3.86457198654117e-07) * t484 * t281 - f64x8::splat(4.8307149831764624e-08) * t279 * t488 - f64x8::splat(2.4153574915882312e-08) * t279 * t491 - f64x8::splat(1.5001590374155006e-13) * t494 * t496 - f64x8::splat(4.150590569503409e-07) * t257 * t185 * t86 * t38 + f64x8::splat(2.274616140481253e-13) * t257 * t505 * t191 * t117 - f64x8::splat(3.0137123519507825e-14) * t157 * t400 - f64x8::splat(3.743584820627817e-19) * t301 * t404 - f64x8::splat(1.2421838528168045e-06) * t268 * t113 + f64x8::splat(2.812798195154064e-14) * t157 * t397 + f64x8::splat(0.03843662551440329) * t25 * t179 - f64x8::splat(7.246072474764693e-08) * t139 * t213 + f64x8::splat(7.763649080105028e-08) * t139 * t231;
            let t531 = t216 * t293;
            let t534 = t403 * t151;
            let t542 = t37 * t199;
            let t544 = f64x8::splat(0.7541111111111111) * t25 * t178 + f64x8::splat(4.065061728395062) * t51 * t146 * t225 + f64x8::splat(0.00062750816531758) * t542;
            let t545 = t101 * t544;
            let t550 = t155 * t194;
            let t553 = t234 * t237;
            let t556 = t234 * t38;
            let t558 = f64x8::splat(1.0) / t456 / t29;
            let t559 = t556 * t558;
            let t570 = f64x8::splat(1.4465819289363758e-12) * t265 * t217 - f64x8::splat(4.140612842722682e-07) * t90 * t294 - f64x8::splat(2.0091415679671886e-12) * t195 * t284 + f64x8::splat(1.6073132543737508e-13) * t121 * t531 + f64x8::splat(4.991446427503756e-19) * t238 * t534 + f64x8::splat(2.5878830267016762e-08) * t42 * t545 + f64x8::splat(1.863275779225207e-06) * t200 * t152 - f64x8::splat(2.1337083451811542e-10) * t550 * t124 + f64x8::splat(1.3757674215807229e-16) * t553 * t241 - f64x8::splat(2.615749096406607e-23) * t559 * t462 + f64x8::splat(1.8082274111704696e-10) * t192 * t550 - f64x8::splat(8.384741006513431e-05) * t87 * t542 + f64x8::splat(8.384741006513431e-05) * t542 * t64 - f64x8::splat(6.73845267713007e-17) * t445 * t553;
            let t571 = t524 + t570;
            let t576 = ((t2).select(f64x8::splat(0.0), t6 * t169 * t160 / f64x8::splat(12.0) - t6 * t73 * t304 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t571));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t576 + f64x8::splat(4.0) * t309;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t588 = t280 * t293;
            let t591 = t157 * t210;
            let t599 = f64x8::splat(1.0) / t30 / t424 * t191;
            let t603 = t139 * t210;
            let t606 = t63 * t330;
            let t607 = t606 * t112;
            let t610 = t63 * t340;
            let t611 = t610 * t112;
            let t620 = f64x8::splat(1.5527298160210056e-07) * t321 * t113 - f64x8::splat(1.2421838528168045e-06) * t268 * t152 + f64x8::splat(1.932285993270585e-07) * t90 * t331 - f64x8::splat(4.8307149831764624e-08) * t279 * t588 + f64x8::splat(5.625596390308128e-14) * t591 * t496 + f64x8::splat(1.220761932206885e-07) * t257 * t261 * t37 - f64x8::splat(8.529810526804699e-14) * t257 * t599 * t155 - f64x8::splat(1.4492144949529385e-07) * t603 * t281 + f64x8::splat(4.669691150403914e-08) * t480 * t607 - f64x8::splat(2.4153574915882312e-08) * t279 * t611 + f64x8::splat(1.5527298160210056e-07) * t139 * t294 + f64x8::splat(1.4465819289363758e-12) * t265 * t284 - f64x8::splat(2.410969881560626e-13) * t318 * t217;
            let t621 = t396 * t330;
            let t631 = v_sigma * t89;
            let t633 = -f64x8::splat(0.6418518518518519) * t51 * t26 * t107 - f64x8::splat(0.0001394462589594622) * t631;
            let t634 = t101 * t633;
            let t637 = t216 * t340;
            let t644 = t344 * t300;
            let t647 = t116 * t120;
            let t652 = t234 * t37;
            let t654 = f64x8::splat(1.0) / t456 / v_rho;
            let t655 = t652 * t654;
            let t664 = -f64x8::splat(7.500795187077503e-14) * t121 * t621 - f64x8::splat(6.027424703901565e-14) * t157 * t531 - f64x8::splat(3.743584820627817e-19) * t301 * t534 + f64x8::splat(2.5878830267016762e-08) * t42 * t634 + f64x8::splat(8.036566271868754e-14) * t121 * t637 + f64x8::splat(7.019221538677157e-20) * t347 * t404 - f64x8::splat(2.070306421361341e-07) * t90 * t341 + f64x8::splat(2.5269197539237764e-17) * t445 * t644 + f64x8::splat(6.509618680213691e-11) * t647 * t124 - f64x8::splat(4.773070646300467e-17) * t644 * t241 + f64x8::splat(9.809059111524776e-24) * t655 * t462 - f64x8::splat(5.786327715745503e-11) * t192 * t647 + f64x8::splat(1.863275779225207e-05) * t87 * t631 - f64x8::splat(1.863275779225207e-05) * t631 * t64;
            let t665 = t620 + t664;
            let t670 = ((t2).select(f64x8::splat(0.0), -t6 * t73 * t350 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t665));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t670 + f64x8::splat(2.0) * t354;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t673 = t38 * t156;
            let t676 = t117 * t346;
            let t681 = f64x8::splat(1.0) / t456;
            let t682 = t235 * t681;
            let t694 = f64x8::splat(1.0) / t30 / t193 * t191;
            let t700 = t330 * t151;
            let t701 = t370 * t700;
            let t704 = t280 * t340;
            let t724 = -f64x8::splat(1.7178160406119463e-11) * t673 * t124 + f64x8::splat(1.5793248462023603e-17) * t676 * t241 + f64x8::splat(4.510882583727985e-13) * t673 * t101 - f64x8::splat(3.678397166821791e-24) * t682 * t462 - f64x8::splat(9.475949077214161e-18) * t445 * t676 - f64x8::splat(2.3290947240315086e-06) * t87 * t41 + f64x8::splat(2.3290947240315086e-06) * t41 * t61 * t63 + f64x8::splat(3.198678947551762e-14) * t257 * t694 * t116 - f64x8::splat(2.173821742429408e-07) * t139 * t331 + f64x8::splat(4.669691150403914e-08) * t42 * t701 - f64x8::splat(7.246072474764693e-08) * t279 * t704 + f64x8::splat(1.627404670053423e-11) * t192 * t673 + f64x8::splat(4.6581894480630173e-07) * t321 * t152 + f64x8::splat(2.3290947240315087e-07) * t139 * t341 - f64x8::splat(7.232909644681879e-13) * t318 * t284 + f64x8::splat(8.438394585462192e-14) * t157 * t621 - f64x8::splat(9.041137055852349e-14) * t157 * t637 + f64x8::splat(2.105766461603147e-19) * t347 * t534 - f64x8::splat(2.7467143474654916e-08) * t257 * t314 * v_sigma;
            let t728 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t724));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t728;
            acc_v3sigma3 = tv3sigma30;
            let t746 = t40 * t105;
            let t754 = f64x8::splat(1.0) / t30 / t105;
            let t761 = t210 * t240;
            let t762 = t761 * t212;
            let t765 = t403 * t230;
            let t768 = t100 * t461;
            let t769 = t768 * t112;
            let t781 = t38 / t183;
            let t793 = t369 * t123;
            let t794 = t793 * t371;
            let t797 = t230 * t230;
            let t809 = f64x8::splat(2.3187431919247016e-06) * t484 * t376 - f64x8::splat(2.0393054144540498e-05) * t182 * t27 / t30 / t746 * t86 + f64x8::splat(1.0164352080475538) * t77 * t27 * t754 * t45 + f64x8::splat(1.1251192780616255e-11) * t195 * t397 - f64x8::splat(2.7952099994021032e-18) * t238 * t762 + f64x8::splat(2.9948678565022535e-18) * t238 * t765 + f64x8::splat(1.8600882463335872e-23) * t459 * t769 + f64x8::splat(2.5878830267016762e-08) * t42 * t101 * (f64x8::splat(19.94204938271605) * t25 * t28 * t754 + f64x8::splat(124.210219478738) * t51 * t52 / t18 / t88 + f64x8::splat(0.023008632728311267) * t781) - f64x8::splat(7.453103116900827e-05) * t418 * t113 + f64x8::splat(1.117965467535124e-05) * t200 * t231 + f64x8::splat(1.678035037566196e-10) * t426 * t217 + f64x8::splat(5.80061494467327e-13) * t121 * t794 - f64x8::splat(7.246072474764693e-08) * t42 * t211 * t797 - f64x8::splat(1.0434344363661158e-05) * t200 * t213 - f64x8::splat(1.2054849407803132e-11) * t195 * t400 - f64x8::splat(1.0182550712107662e-16) * t448 * t404 - f64x8::splat(8.281225685445364e-07) * t90 * t421;
            let t810 = t216 * t420;
            let t817 = f64x8::splat(1.0) / t99 / t367 / t60;
            let t818 = t817 * t63;
            let t819 = t212 * t212;
            let t831 = t123 * t112 * t230;
            let t851 = t455 / t456 / t39;
            let t860 = t61 / t460 / t62;
            let t863 = t118 * t183;
            let t865 = t235 / t863;
            let t869 = t117 / t503;
            let t880 = t36 / t443 / t44;
            let t885 = f64x8::splat(3.2146265087475015e-13) * t121 * t810 - f64x8::splat(1.4943011681292524e-06) * t90 * t372 - f64x8::splat(1.369776070785148e-07) * t42 * t818 * t819 + f64x8::splat(2.801814690242348e-07) * t480 * t63 * t212 * t230 - f64x8::splat(9.661429966352925e-08) * t279 * t375 * t420 - f64x8::splat(9.000954224493005e-13) * t494 * t831 + f64x8::splat(2.628445317889448e-11) * t386 * t27 / t30 / t118 / t105 * t191 - f64x8::splat(7.534643842832781e-18) * t25 * t652 * t27 / t30 / t118 / t746 * t444 - f64x8::splat(5.9987845944258184e-21) * t851 * t462 + f64x8::splat(8.664643441537158e-28) * t234 * t344 / t456 / t183 * t860 - f64x8::splat(9.703371855067301e-15) * t445 * t865 + f64x8::splat(1.163051870864846e-08) * t192 * t869 - f64x8::splat(0.003074405035721591) * t87 * t781 + f64x8::splat(0.003074405035721591) * t781 * t64 - f64x8::splat(1.282394880002097e-08) * t869 * t124 + f64x8::splat(2.2321058956003046e-21) * t880 * t851 + f64x8::splat(1.421813514874445e-14) * t865 * t241;
            let t891 = ((t2).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t80 * t67 - f64x8::splat(5.0) / f64x8::splat(9.0) * t6 * t357 * t127 + t6 * t169 * t244 / f64x8::splat(2.0) - t6 * t73 * t465 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t809 + t885)));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t891 + f64x8::splat(8.0) * t470;
            acc_v4rho4 = tv4rho40;
            let t906 = t403 * t293;
            let t909 = t768 * t151;
            let t917 = t37 * t417;
            let t935 = t90 * t369;
            let t941 = t121 * t369;
            let t942 = t495 * t212;
            let t945 = f64x8::splat(2.1698728934045634e-12) * t265 * t400 + f64x8::splat(1.4974339282511268e-18) * t238 * t906 + f64x8::splat(4.650220615833968e-24) * t459 * t909 + f64x8::splat(2.5878830267016762e-08) * t42 * t101 * (-f64x8::splat(3.519185185185185) * t25 * t439 - f64x8::splat(29.81045267489712) * t51 * t146 * t413 - f64x8::splat(0.0062750816531758) * t917) - f64x8::splat(3.0137123519507825e-14) * t157 * t810 - f64x8::splat(5.615377230941725e-19) * t301 * t765 + f64x8::splat(5.58982733767562e-06) * t200 * t294 + f64x8::splat(4.19508759391549e-11) * t426 * t284 + f64x8::splat(5.241018748878944e-19) * t301 * t762 + f64x8::splat(7.763649080105028e-08) * t139 * t421 - f64x8::splat(1.1207258760969393e-06) * t935 * t481 + f64x8::splat(1.400907345121174e-07) * t480 * t487 * t212 + f64x8::splat(4.350461208504952e-13) * t941 * t942;
            let t948 = t240 * t151 * t112;
            let t962 = t123 * t293 * t112;
            let t968 = t495 * t230;
            let t980 = t42 * t817;
            let t990 = -f64x8::splat(1.3976049997010516e-18) * t238 * t210 * t948 + f64x8::splat(5.625596390308128e-12) * t195 * t210 * t496 - f64x8::splat(7.246072474764693e-08) * t279 * t63 * t544 * t112 - f64x8::splat(7.246072474764693e-08) * t279 * t487 * t230 - f64x8::splat(4.5004771122465024e-13) * t494 * t962 - f64x8::splat(2.4153574915882312e-08) * t279 * t280 * t420 - f64x8::splat(2.2502385561232512e-13) * t494 * t968 - f64x8::splat(2.173821742429408e-07) * t603 * t376 - f64x8::splat(5.217172181830579e-06) * t200 * t210 * t281 + f64x8::splat(1.1593715959623508e-06) * t484 * t488 + f64x8::splat(5.796857979811754e-07) * t484 * t491 - f64x8::splat(1.369776070785148e-07) * t980 * t280 * t371 + f64x8::splat(8.438394585462192e-14) * t591 * t831 + f64x8::splat(6.255726701486838e-06) * t257 * t381 * t86 * t38;
            let t1015 = t216 * t544;
            let t1026 = -f64x8::splat(9.174285099941055e-12) * t257 * t389 * t191 * t117 + f64x8::splat(2.8254914410622928e-18) * t257 / t30 / t118 / t379 * t444 * t235 - f64x8::splat(5.231498192813214e-24) * t559 * t769 + f64x8::splat(1.6769482013026862e-05) * t542 * t113 + f64x8::splat(2.7515348431614454e-17) * t553 * t404 - f64x8::splat(6.210919264084022e-07) * t90 * t545 - f64x8::splat(6.027424703901566e-12) * t195 * t531 - f64x8::splat(2.5456376780269155e-17) * t448 * t534 + f64x8::splat(2.410969881560626e-13) * t121 * t1015 - f64x8::splat(0.17937091906721536) * t25 * t440 + f64x8::splat(1.7390573939435264e-06) * t268 * t213 + f64x8::splat(1.400907345121174e-07) * t139 * t372 - f64x8::splat(1.863275779225207e-06) * t268 * t231;
            let t1035 = t234 * t447;
            let t1038 = t155 * t425;
            let t1041 = t556 * t458;
            let t1064 = -f64x8::splat(4.267416690362308e-11) * t550 * t217 - f64x8::splat(1.863275779225207e-05) * t418 * t152 - f64x8::splat(2.025214700510926e-12) * t265 * t397 - f64x8::splat(5.43807651063119e-14) * t157 * t794 + f64x8::splat(3.436610865336336e-15) * t445 * t1035 + f64x8::splat(4.101059768534625e-09) * t1038 * t124 - f64x8::splat(8.370397108501143e-22) * t880 * t1041 - f64x8::splat(4.902224322612126e-15) * t1035 * t241 + f64x8::splat(2.171071750017484e-21) * t1041 * t462 - f64x8::splat(3.249241290576434e-28) * t234 * t117 / t456 / t258 * t860 - f64x8::splat(3.77557883452394e-09) * t192 * t1038 + f64x8::splat(0.000838474100651343) * t87 * t917 - f64x8::splat(0.000838474100651343) * t917 * t64 + f64x8::splat(1.400907345121174e-07) * t480 * t280 * t230 * t112;
            let t1071 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t357 * t160 + t6 * t169 * t304 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t73 * t571 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t945 + t990 + t1026 + t1064)));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t1071 + f64x8::splat(6.0) * t576;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t1082 = t123 * t330 * t112;
            let t1085 = t495 * t293;
            let t1101 = t123 * t340 * t112;
            let t1123 = f64x8::splat(2.900307472336635e-13) * t941 * t1082 - f64x8::splat(3.0003180748310013e-13) * t494 * t1085 + f64x8::splat(1.1251192780616256e-13) * t591 * t962 + f64x8::splat(6.988024998505258e-19) * t301 * t210 * t948 - f64x8::splat(4.8307149831764624e-08) * t279 * t63 * t633 * t112 - f64x8::splat(2.4153574915882312e-08) * t279 * t610 * t230 - f64x8::splat(1.5001590374155006e-13) * t494 * t1101 - f64x8::splat(2.898428989905877e-07) * t603 * t488 - f64x8::splat(2.7002862673479015e-12) * t265 * t210 * t496 + f64x8::splat(2.3187431919247016e-06) * t268 * t210 * t281 - f64x8::splat(1.4492144949529385e-07) * t603 * t491 - f64x8::splat(7.471505840646262e-07) * t935 * t607 + f64x8::splat(7.72914397308234e-07) * t484 * t588 + f64x8::splat(4.669691150403914e-08) * t480 * t606 * t230 + f64x8::splat(3.86457198654117e-07) * t484 * t611;
            let t1144 = t157 * t369;
            let t1147 = t139 * t369;
            let t1168 = -f64x8::splat(1.369776070785148e-07) * t980 * t606 * t212 + f64x8::splat(4.669691150403914e-08) * t480 * t610 * t212 + f64x8::splat(3.0422990878936757e-12) * t25 * t155 * t27 * t505 * t191 - f64x8::splat(1.6195441633944676e-06) * t25 * t37 * t187 - f64x8::splat(1.0595592903983598e-18) * t257 / t30 / t863 * t444 * t234 - f64x8::splat(1.087615302126238e-13) * t1144 * t942 + f64x8::splat(2.801814690242348e-07) * t1147 * t481 + f64x8::splat(2.2502385561232512e-13) * t318 * t397 + f64x8::splat(1.875198796769376e-12) * t195 * t621 - f64x8::splat(2.484367705633609e-06) * t268 * t294 - f64x8::splat(2.844944460241539e-11) * t550 * t284 + f64x8::splat(8.679491573618254e-12) * t647 * t217 + f64x8::splat(1.117965467535124e-05) * t542 * t152 - f64x8::splat(1.7390573939435264e-06) * t200 * t331 + f64x8::splat(1.863275779225207e-06) * t200 * t341 - f64x8::splat(4.140612842722682e-07) * t90 * t634;
            let t1182 = t761 * t330;
            let t1194 = v_sigma * t199;
            let t1200 = t216 * t633;
            let t1203 = t403 * t340;
            let t1212 = -f64x8::splat(6.55127343609868e-20) * t347 * t762 - f64x8::splat(1.4492144949529385e-07) * t321 * t213 + f64x8::splat(2.8931638578727516e-12) * t265 * t531 + f64x8::splat(1.8343565621076303e-17) * t553 * t534 - f64x8::splat(2.410969881560626e-13) * t318 * t400 - f64x8::splat(6.364094195067289e-18) * t644 * t404 - f64x8::splat(4.658683332336839e-19) * t238 * t1182 - f64x8::splat(6.027424703901565e-14) * t157 * t1015 - f64x8::splat(7.487169641255634e-19) * t301 * t906 - f64x8::splat(3.487665461875476e-24) * t559 * t909 + f64x8::splat(2.5878830267016762e-08) * t42 * t101 * (f64x8::splat(4.065061728395062) * t51 * t26 * t225 + f64x8::splat(0.00125501633063516) * t1194) + f64x8::splat(1.6073132543737508e-13) * t121 * t1200 + f64x8::splat(4.991446427503756e-19) * t238 * t1203 + f64x8::splat(7.019221538677157e-20) * t347 * t765 + f64x8::splat(1.3078745482033035e-24) * t655 * t769 - f64x8::splat(2.484367705633609e-06) * t631 * t113;
            let t1219 = t293 * t293;
            let t1228 = t344 * t237;
            let t1231 = t652 * t558;
            let t1240 = t116 * t194;
            let t1257 = f64x8::splat(1.5527298160210056e-07) * t321 * t231 - f64x8::splat(2.0091415679671886e-12) * t195 * t637 + f64x8::splat(1.5527298160210056e-07) * t139 * t545 - f64x8::splat(4.8307149831764624e-08) * t42 * t211 * t1219 - f64x8::splat(4.8307149831764624e-08) * t279 * t280 * t544 + f64x8::splat(5.625596390308128e-14) * t591 * t968 + f64x8::splat(1.5975748222029209e-15) * t1228 * t241 - f64x8::splat(7.683762970694408e-22) * t1231 * t462 + f64x8::splat(1.2184654839661627e-28) * t234 * t155 / t456 / t198 * t860 + f64x8::splat(1.0994022659916456e-09) * t192 * t1240 - f64x8::splat(0.00016769482013026863) * t87 * t1194 + f64x8::splat(0.00016769482013026863) * t1194 * t64 - f64x8::splat(1.1708061526513498e-15) * t445 * t1228 - f64x8::splat(1.1644984527937824e-09) * t1240 * t124 + f64x8::splat(3.1388989156879283e-22) * t880 * t1231 + f64x8::splat(1.8678764601615655e-07) * t480 * t280 * t112 * t293;
            let t1264 = ((t2).select(f64x8::splat(0.0), t6 * t169 * t350 / f64x8::splat(12.0) - t6 * t73 * t665 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t1123 + t1168 + t1212 + t1257)));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t1264 + f64x8::splat(4.0) * t670;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t1276 = t120 * t38;
            let t1279 = t41 * t100;
            let t1282 = t117 * t300;
            let t1285 = t235 * t654;
            let t1303 = -f64x8::splat(1.863275779225207e-06) * t268 * t341 - f64x8::splat(3.735752920323131e-07) * t90 * t701 - f64x8::splat(3.726551558450414e-06) * t631 * t152 - f64x8::splat(2.748505664979114e-10) * t192 * t1276 + f64x8::splat(1.5527298160210056e-07) * t1279 * t375 - f64x8::splat(4.857301304764592e-16) * t1282 * t241 + f64x8::splat(2.6484459601116894e-22) * t1285 * t462 + f64x8::splat(1.4008363768648626e-18) * t1282 * t216 - f64x8::splat(4.5692455648731104e-29) * t455 / t456 / t88 * t860 - f64x8::splat(1.1770870933829732e-22) * t880 * t1285 + f64x8::splat(2.8208347614259326e-10) * t1276 * t124 - f64x8::splat(7.217412133964776e-12) * t1276 * t101 + f64x8::splat(3.7903796308856647e-16) * t445 * t1282;
            let t1322 = t495 * t340;
            let t1343 = -f64x8::splat(1.631422953189357e-13) * t1144 * t1082 + f64x8::splat(1.6876789170924384e-13) * t591 * t1085 - f64x8::splat(9.382791579485169e-13) * t257 * t599 * t116 + f64x8::splat(5.796857979811754e-07) * t484 * t704 + f64x8::splat(1.400907345121174e-07) * t480 * t606 * t293 - f64x8::splat(7.246072474764693e-08) * t279 * t487 * t340 - f64x8::splat(7.246072474764693e-08) * t279 * t280 * t633 - f64x8::splat(2.2502385561232512e-13) * t494 * t1322 - f64x8::splat(4.347643484858816e-07) * t321 * t210 * t281 - f64x8::splat(2.173821742429408e-07) * t603 * t611 + f64x8::splat(6.750715668369754e-13) * t318 * t210 * t496 + f64x8::splat(3.973347338993849e-19) * t386 * t27 / t30 / t446 * t444 + f64x8::splat(3.1739810237379015e-07) * t77 * t27 * t260 * t86;
            let t1367 = t793 * t700;
            let t1376 = f64x8::splat(4.202722035363522e-07) * t1147 * t607 - f64x8::splat(4.347643484858816e-07) * t603 * t588 - f64x8::splat(1.369776070785148e-07) * t980 * t63 * t700 * t112 + f64x8::splat(8.438394585462192e-14) * t591 * t1101 - f64x8::splat(1.965382030829604e-19) * t347 * t210 * t948 + f64x8::splat(1.0528832308015735e-18) * t676 * t404 - f64x8::splat(4.2101570781461193e-13) * t673 * t211 * t112 - f64x8::splat(2.452264777881194e-25) * t682 * t769 - f64x8::splat(2.025214700510926e-12) * t265 * t621 + f64x8::splat(1.4501537361683174e-13) * t121 * t1367 + f64x8::splat(1.7390573939435264e-06) * t268 * t331 + f64x8::splat(2.3290947240315087e-07) * t139 * t634 + f64x8::splat(2.1698728934045634e-12) * t265 * t637;
            let t1406 = -f64x8::splat(7.232909644681879e-13) * t318 * t531 - f64x8::splat(9.546141292600933e-18) * t644 * t534 + f64x8::splat(5.241018748878944e-19) * t301 * t1182 - f64x8::splat(9.041137055852349e-14) * t157 * t1200 - f64x8::splat(5.615377230941725e-19) * t301 * t1203 + f64x8::splat(2.105766461603147e-19) * t347 * t906 + f64x8::splat(1.9618118223049554e-24) * t655 * t909 - f64x8::splat(1.1452106937412974e-12) * t673 * t217 + f64x8::splat(4.6581894480630173e-07) * t321 * t294 + f64x8::splat(1.3019237360427382e-11) * t647 * t284 + f64x8::splat(1.863275779225207e-05) * t87 * t89 - f64x8::splat(1.863275779225207e-05) * t89 * t61 * t63 + f64x8::splat(1.400907345121174e-07) * t480 * t280 * t340 * t112;
            let t1413 = ((t2).select(f64x8::splat(0.0), -t6 * t73 * t724 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t1303 + t1343 + t1376 + t1406)));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t1413 + f64x8::splat(2.0) * t728;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t1431 = t330 * t330;
            let t1435 = t340 * t340;
            let t1453 = -f64x8::splat(4.5808427749651894e-12) * t673 * t284 + f64x8::splat(4.211532923206294e-18) * t676 * t534 - f64x8::splat(1.6840628312584477e-12) * t673 * t211 * t151 - f64x8::splat(9.809059111524777e-25) * t682 * t909 - f64x8::splat(3.662285796620655e-08) * t25 * t27 * t313 * t86 + f64x8::splat(1.3501431336739507e-12) * t318 * t621 - f64x8::splat(1.369776070785148e-07) * t42 * t818 * t1431 - f64x8::splat(7.246072474764693e-08) * t42 * t211 * t1435 - f64x8::splat(2.175230604252476e-13) * t157 * t1367 + f64x8::splat(9.316378896126035e-07) * t321 * t341 - f64x8::splat(1.4465819289363758e-12) * t318 * t637 - f64x8::splat(3.930764061659208e-19) * t347 * t1182 + f64x8::splat(4.211532923206294e-19) * t347 * t1203 - f64x8::splat(8.695286969717632e-07) * t321 * t331 + f64x8::splat(5.603629380484696e-07) * t139 * t701;
            let t1454 = t155 * t346;
            let t1462 = t37 * t156;
            let t1469 = t234 * t681;
            let t1496 = -f64x8::splat(2.101254565297294e-18) * t1454 * t216 + f64x8::splat(1.7134670868274165e-29) * t556 / t456 / t40 * t860 - f64x8::splat(5.424682233511409e-11) * t1462 * t124 + f64x8::splat(5.4130591004735826e-12) * t1462 * t101 - f64x8::splat(1.1371138892656995e-16) * t445 * t1454 + f64x8::splat(4.414076600186149e-23) * t880 * t1469 + f64x8::splat(5.424682233511409e-11) * t192 * t1462 + f64x8::splat(6.210919264084022e-07) * t1279 * t280 + f64x8::splat(1.347690535426014e-16) * t1454 * t241 - f64x8::splat(8.828153200372298e-23) * t1469 * t462 + f64x8::splat(2.5589431580414095e-13) * t257 * t694 * t38 - f64x8::splat(8.695286969717632e-07) * t603 * t704 + f64x8::splat(2.801814690242348e-07) * t480 * t606 * t340 + f64x8::splat(3.375357834184877e-13) * t591 * t1322 - f64x8::splat(1.4900052521226934e-19) * t257 / t30 / t236 * t444 * t117;
            let t1501 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t1453 + t1496)));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t1501;
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
