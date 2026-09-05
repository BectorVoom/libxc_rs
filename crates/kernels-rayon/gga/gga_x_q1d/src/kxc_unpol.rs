//! GGA_X_Q1D kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_q1d.c`
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
pub fn gga_x_q1d_kxc_unpol(
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
            let t26 = f64x8::splat(M_CBRT2);
            let t27 = t26 * t26;
            let t28 = v_sigma * t27;
            let t29 = v_rho * v_rho;
            let t30 = t18 * t18;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t33 = t28 * t32;
            let t34 = t25 * t33;
            let t36 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t34;
            let t38 = f64x8::splat(0.646416) / t36;
            let t40 = t20 * t20;
            let t42 = f64x8::splat(1.0) / t22 / t21;
            let t43 = t40 * t42;
            let t44 = v_sigma * v_sigma;
            let t45 = t44 * t26;
            let t46 = t29 * t29;
            let t47 = t46 * v_rho;
            let t49 = f64x8::splat(1.0) / t18 / t47;
            let t52 = t43 * t45 * t49 / f64x8::splat(288.0);
            let t53 = t34 / f64x8::splat(24.0) + t52;
            let t54 = t21 * t21;
            let t55 = f64x8::splat(1.0) / t54;
            let t56 = t44 * v_sigma;
            let t57 = t55 * t56;
            let t58 = t46 * t46;
            let t59 = f64x8::splat(1.0) / t58;
            let t62 = f64x8::splat(1.0) + t52 + t57 * t59 / f64x8::splat(576.0);
            let t63 = f64x8::splat(1.0) / t62;
            let t64 = t53 * t63;
            let t66 = (f64x8::splat(1.804) - t38) * t20;
            let t67 = t66 * t24;
            let t70 = -t67 * t33 / f64x8::splat(24.0) + f64x8::splat(0.06525);
            let t72 = f64x8::splat(1.804) - t38 + t64 * t70;
            let t76 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t72));
            let tzk0 = f64x8::splat(2.0) * t76;
            acc_zk = tzk0;
            let t78 = t17 / t30;
            let t82 = t36 * t36;
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t83 * t20;
            let t85 = t84 * t24;
            let t86 = t29 * v_rho;
            let t88 = f64x8::splat(1.0) / t30 / t86;
            let t89 = t28 * t88;
            let t94 = t46 * t29;
            let t96 = f64x8::splat(1.0) / t18 / t94;
            let t97 = t45 * t96;
            let t99 = t43 * t97 / f64x8::splat(54.0);
            let t100 = -t25 * t89 / f64x8::splat(9.0) - t99;
            let t101 = t100 * t63;
            let t103 = t62 * t62;
            let t104 = f64x8::splat(1.0) / t103;
            let t105 = t53 * t104;
            let t106 = t58 * v_rho;
            let t107 = f64x8::splat(1.0) / t106;
            let t110 = -t99 - t57 * t107 / f64x8::splat(72.0);
            let t111 = t70 * t110;
            let t113 = t83 * t40;
            let t114 = t113 * t42;
            let t119 = f64x8::splat(0.0007389300411522634) * t114 * t97 + t67 * t89 / f64x8::splat(9.0);
            let t121 = -f64x8::splat(0.00886716049382716) * t85 * t89 + t101 * t70 - t105 * t111 + t64 * t119;
            let t126 = ((t2).select(f64x8::splat(0.0), -t6 * t78 * t72 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t121));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t126 + f64x8::splat(2.0) * t76;
            acc_vrho = tvrho0;
            let t129 = t24 * t27;
            let t130 = t129 * t32;
            let t137 = v_sigma * t26 * t49;
            let t139 = t43 * t137 / f64x8::splat(144.0);
            let t140 = t25 * t27 * t32 / f64x8::splat(24.0) + t139;
            let t141 = t140 * t63;
            let t143 = t55 * t44;
            let t146 = t139 + t143 * t59 / f64x8::splat(192.0);
            let t147 = t70 * t146;
            let t153 = -f64x8::splat(0.00027709876543209876) * t114 * t137 - t66 * t130 / f64x8::splat(24.0);
            let t155 = f64x8::splat(0.0033251851851851854) * t84 * t130 + t141 * t70 - t105 * t147 + t64 * t153;
            let t159 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t155));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t159;
            acc_vsigma = tvsigma0;
            let t164 = t17 / t30 / v_rho;
            let t172 = f64x8::splat(1.0) / t82 / t36;
            let t173 = t172 * t40;
            let t174 = t173 * t42;
            let t175 = t46 * t86;
            let t177 = f64x8::splat(1.0) / t18 / t175;
            let t178 = t45 * t177;
            let t182 = f64x8::splat(1.0) / t30 / t46;
            let t183 = t28 * t182;
            let t189 = f64x8::splat(19.0) / f64x8::splat(162.0) * t43 * t178;
            let t190 = f64x8::splat(11.0) / f64x8::splat(27.0) * t25 * t183 + t189;
            let t191 = t190 * t63;
            let t193 = t100 * t104;
            let t199 = f64x8::splat(1.0) / t103 / t62;
            let t200 = t53 * t199;
            let t201 = t110 * t110;
            let t202 = t70 * t201;
            let t205 = t119 * t110;
            let t208 = t58 * t29;
            let t209 = f64x8::splat(1.0) / t208;
            let t212 = t189 + t57 * t209 / f64x8::splat(8.0);
            let t213 = t70 * t212;
            let t215 = t172 * t55;
            let t223 = f64x8::splat(0.00024326914935053937) * t215 * t56 * t209 - f64x8::splat(0.006650370370370371) * t114 * t178 - f64x8::splat(11.0) / f64x8::splat(27.0) * t67 * t183;
            let t225 = -f64x8::splat(0.00048653829870107875) * t174 * t178 + f64x8::splat(0.03251292181069959) * t85 * t183 + t191 * t70 - f64x8::splat(2.0) * t193 * t111 + f64x8::splat(2.0) * t101 * t119 + f64x8::splat(2.0) * t200 * t202 - f64x8::splat(2.0) * t105 * t205 - t105 * t213 + t64 * t223;
            let t230 = ((t2).select(f64x8::splat(0.0), t6 * t164 * t72 / f64x8::splat(12.0) - t6 * t78 * t121 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t225));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t230 + f64x8::splat(4.0) * t126;
            acc_v2rho2 = tv2rho20;
            let t236 = t26 * t96;
            let t237 = t236 * v_sigma;
            let t240 = t129 * t88;
            let t247 = t43 * t237 / f64x8::splat(27.0);
            let t248 = -t25 * t27 * t88 / f64x8::splat(9.0) - t247;
            let t249 = t248 * t63;
            let t251 = t140 * t104;
            let t255 = t147 * t110;
            let t258 = t119 * t146;
            let t262 = -t247 - t143 * t107 / f64x8::splat(24.0);
            let t263 = t70 * t262;
            let t266 = t153 * t110;
            let t275 = -f64x8::splat(9.122593100645226e-05) * t215 * t107 * t44 + f64x8::splat(0.00221679012345679) * t114 * t237 + t66 * t240 / f64x8::splat(9.0);
            let t277 = f64x8::splat(0.00018245186201290453) * t174 * t237 - f64x8::splat(0.00886716049382716) * t84 * t240 + t249 * t70 - t251 * t111 + t141 * t119 - t193 * t147 + f64x8::splat(2.0) * t200 * t255 - t105 * t258 - t105 * t263 + t101 * t153 - t105 * t266 + t64 * t275;
            let t282 = ((t2).select(f64x8::splat(0.0), -t6 * t78 * t155 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t277));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t282 + f64x8::splat(2.0) * t159;
            acc_v2rhosigma = tv2rhosigma0;
            let t285 = t42 * t26;
            let t286 = t285 * t49;
            let t289 = t43 * t26;
            let t290 = t49 * t63;
            let t298 = t146 * t146;
            let t299 = t70 * t298;
            let t302 = t153 * t146;
            let t306 = t43 * t26 * t49;
            let t308 = t55 * v_sigma;
            let t311 = t306 / f64x8::splat(144.0) + t308 * t59 / f64x8::splat(96.0);
            let t312 = t70 * t311;
            let t319 = f64x8::splat(3.42097241274196e-05) * t215 * t59 * v_sigma - f64x8::splat(0.0005541975308641975) * t113 * t286;
            let t321 = -f64x8::splat(6.84194482548392e-05) * t173 * t286 + t289 * t290 * t70 / f64x8::splat(144.0) - f64x8::splat(2.0) * t251 * t147 + f64x8::splat(2.0) * t141 * t153 + f64x8::splat(2.0) * t200 * t299 - f64x8::splat(2.0) * t105 * t302 - t105 * t312 + t64 * t319;
            let t325 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t321));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t325;
            acc_v2sigma2 = tv2sigma20;
            let t328 = t17 * t32;
            let t338 = t100 * t199;
            let t341 = t103 * t103;
            let t342 = f64x8::splat(1.0) / t341;
            let t343 = t53 * t342;
            let t344 = t201 * t110;
            let t345 = t70 * t344;
            let t348 = t111 * t212;
            let t352 = f64x8::splat(1.0) / t18 / t58;
            let t353 = t45 * t352;
            let t357 = f64x8::splat(1.0) / t30 / t47;
            let t358 = t28 * t357;
            let t361 = t82 * t82;
            let t363 = f64x8::splat(1.0) / t361 * t55;
            let t364 = t58 * t86;
            let t365 = f64x8::splat(1.0) / t364;
            let t366 = t56 * t365;
            let t369 = t190 * t104;
            let t376 = t119 * t201;
            let t379 = t223 * t110;
            let t382 = t119 * t212;
            let t386 = f64x8::splat(209.0) / f64x8::splat(243.0) * t43 * t353;
            let t389 = -t386 - f64x8::splat(5.0) / f64x8::splat(4.0) * t57 * t365;
            let t390 = t70 * t389;
            let t394 = -f64x8::splat(154.0) / f64x8::splat(81.0) * t25 * t358 - t386;
            let t395 = t394 * t63;
            let t401 = t44 * t44;
            let t402 = t363 * t401;
            let t403 = t58 * t47;
            let t405 = f64x8::splat(1.0) / t30 / t403;
            let t416 = f64x8::splat(1.0011076104960468e-05) * t402 * t405 * t20 * t129 - f64x8::splat(0.004622113837660248) * t215 * t366 + f64x8::splat(0.0559944764517604) * t114 * t353 + f64x8::splat(154.0) / f64x8::splat(81.0) * t67 * t358;
            let t418 = f64x8::splat(6.0) * t338 * t202 - f64x8::splat(6.0) * t343 * t345 + f64x8::splat(6.0) * t200 * t348 + f64x8::splat(0.005351921285711866) * t174 * t353 - f64x8::splat(0.15172696844993142) * t85 * t358 - f64x8::splat(0.00024026582651905123) * t363 * t366 - f64x8::splat(3.0) * t369 * t111 - f64x8::splat(6.0) * t193 * t205 - f64x8::splat(3.0) * t193 * t213 + f64x8::splat(6.0) * t200 * t376 - f64x8::splat(3.0) * t105 * t379 - f64x8::splat(3.0) * t105 * t382 - t105 * t390 + t395 * t70 + f64x8::splat(3.0) * t191 * t119 + f64x8::splat(3.0) * t101 * t223 + t64 * t416;
            let t423 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t328 * t72 + t6 * t164 * t121 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t78 * t225 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t418));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t423 + f64x8::splat(6.0) * t230;
            acc_v3rho3 = tv3rho30;
            let t435 = t258 * t110;
            let t438 = t263 * t110;
            let t441 = t147 * t212;
            let t444 = t129 * t182;
            let t447 = t147 * t201;
            let t450 = t26 * t177;
            let t451 = t450 * v_sigma;
            let t457 = t58 * t46;
            let t459 = f64x8::splat(1.0) / t30 / t457;
            let t460 = t363 * t459;
            let t462 = t56 * t20 * t129;
            let t465 = t209 * t44;
            let t472 = -f64x8::splat(3.7541535393601755e-06) * t460 * t462 + f64x8::splat(0.0015508408271096886) * t215 * t465 - f64x8::splat(0.016010150891632373) * t114 * t451 - f64x8::splat(11.0) / f64x8::splat(27.0) * t66 * t444;
            let t478 = f64x8::splat(19.0) / f64x8::splat(81.0) * t43 * t451;
            let t479 = f64x8::splat(11.0) / f64x8::splat(27.0) * t25 * t27 * t182 + t478;
            let t480 = t479 * t63;
            let t485 = t223 * t146;
            let t487 = f64x8::splat(4.0) * t338 * t255 + f64x8::splat(4.0) * t200 * t435 + f64x8::splat(4.0) * t200 * t438 + f64x8::splat(2.0) * t200 * t441 + f64x8::splat(0.03251292181069959) * t84 * t444 - f64x8::splat(6.0) * t343 * t447 - f64x8::splat(0.0016420667581161408) * t174 * t451 + t191 * t153 + f64x8::splat(2.0) * t101 * t275 + t64 * t472 + t480 * t70 + f64x8::splat(2.0) * t249 * t119 + t141 * t223 - t105 * t485;
            let t488 = t119 * t262;
            let t493 = t478 + f64x8::splat(3.0) / f64x8::splat(8.0) * t143 * t209;
            let t494 = t70 * t493;
            let t498 = t275 * t110;
            let t501 = t153 * t212;
            let t503 = t153 * t201;
            let t506 = t140 * t199;
            let t511 = t248 * t104;
            let t522 = -f64x8::splat(2.0) * t105 * t488 - t105 * t494 - f64x8::splat(2.0) * t193 * t266 - f64x8::splat(2.0) * t105 * t498 - t105 * t501 + f64x8::splat(2.0) * t200 * t503 + f64x8::splat(2.0) * t506 * t202 + f64x8::splat(9.009968494464422e-05) * t363 * t465 - f64x8::splat(2.0) * t511 * t111 - f64x8::splat(2.0) * t251 * t205 - t251 * t213 - t369 * t147 - f64x8::splat(2.0) * t193 * t258 - f64x8::splat(2.0) * t193 * t263;
            let t523 = t487 + t522;
            let t528 = ((t2).select(f64x8::splat(0.0), t6 * t164 * t155 / f64x8::splat(12.0) - t6 * t78 * t277 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t523));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t528 + f64x8::splat(4.0) * t282;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t537 = t96 * t63;
            let t543 = t299 * t110;
            let t546 = t147 * t262;
            let t549 = t302 * t110;
            let t552 = t312 * t110;
            let t555 = t285 * t96;
            let t564 = f64x8::splat(1.0) / t30 / t364;
            let t565 = t363 * t564;
            let t567 = t44 * t20 * t129;
            let t570 = t107 * v_sigma;
            let t575 = f64x8::splat(1.407807577260066e-06) * t565 * t567 - f64x8::splat(0.00045612965503226134) * t215 * t570 + f64x8::splat(0.0029557201646090536) * t113 * t555;
            let t577 = t119 * t311;
            let t579 = t289 * t290 * t119 / f64x8::splat(144.0) - t289 * t537 * t70 / f64x8::splat(27.0) + f64x8::splat(4.0) * t506 * t255 - f64x8::splat(6.0) * t343 * t543 + f64x8::splat(4.0) * t200 * t546 + f64x8::splat(4.0) * t200 * t549 + f64x8::splat(2.0) * t200 * t552 + f64x8::splat(0.00036490372402580906) * t173 * t555 + f64x8::splat(2.0) * t249 * t153 + f64x8::splat(2.0) * t141 * t275 + t101 * t319 + t64 * t575 - t105 * t577;
            let t584 = -t43 * t236 / f64x8::splat(27.0) - t308 * t107 / f64x8::splat(12.0);
            let t585 = t70 * t584;
            let t587 = t319 * t110;
            let t601 = t119 * t298;
            let t606 = t275 * t146;
            let t609 = t153 * t262;
            let t613 = t49 * t104;
            let t617 = -t105 * t585 - t105 * t587 - f64x8::splat(3.378738185424158e-05) * t363 * t570 - f64x8::splat(2.0) * t511 * t147 - f64x8::splat(2.0) * t251 * t258 - f64x8::splat(2.0) * t251 * t263 - f64x8::splat(2.0) * t251 * t266 + f64x8::splat(2.0) * t338 * t299 + f64x8::splat(2.0) * t200 * t601 - f64x8::splat(2.0) * t193 * t302 - f64x8::splat(2.0) * t105 * t606 - f64x8::splat(2.0) * t105 * t609 - t193 * t312 - t289 * t613 * t111 / f64x8::splat(144.0);
            let t618 = t579 + t617;
            let t623 = ((t2).select(f64x8::splat(0.0), -t6 * t78 * t321 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t618));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t623 + f64x8::splat(2.0) * t325;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t629 = t70 * t55;
            let t630 = t629 * t59;
            let t635 = t298 * t146;
            let t636 = t70 * t635;
            let t639 = t147 * t311;
            let t649 = t153 * t298;
            let t652 = t319 * t146;
            let t655 = t153 * t311;
            let t663 = f64x8::splat(1.0) / t30 / t208;
            let t664 = t363 * t663;
            let t666 = v_sigma * t20 * t129;
            let t671 = -f64x8::splat(5.279278414725247e-07) * t664 * t666 + f64x8::splat(0.0001026291723822588) * t215 * t59;
            let t673 = t289 * t290 * t153 / f64x8::splat(48.0) - t105 * t630 / f64x8::splat(96.0) + f64x8::splat(6.0) * t506 * t299 - f64x8::splat(6.0) * t343 * t636 + f64x8::splat(6.0) * t200 * t639 - t289 * t613 * t147 / f64x8::splat(48.0) - f64x8::splat(6.0) * t251 * t302 - f64x8::splat(3.0) * t251 * t312 + f64x8::splat(6.0) * t200 * t649 - f64x8::splat(3.0) * t105 * t652 - f64x8::splat(3.0) * t105 * t655 + f64x8::splat(1.2670268195340592e-05) * t363 * t59 + f64x8::splat(3.0) * t141 * t319 + t64 * t671;
            let t677 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t673));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t677;
            acc_v3sigma3 = tv3sigma30;
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
        ip += 8;
    }
}
