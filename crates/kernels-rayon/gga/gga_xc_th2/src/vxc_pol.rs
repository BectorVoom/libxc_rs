//! GGA_XC_TH2 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th2.c`
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

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_xc_th2_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = (simd::pow(v_rho0, f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t4 = (simd::pow(v_rho1, f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t7 = (simd::pow(v_rho0, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t8 = t7 * v_rho0;
            let t10 = (simd::pow(v_rho1, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t11 = t10 * v_rho1;
            let t13 = (simd::cbrt(v_rho0));
            let t14 = t13 * v_rho0;
            let t16 = (simd::cbrt(v_rho1));
            let t17 = t16 * v_rho1;
            let t19 = ((v_rho0).sqrt());
            let t20 = t19 * v_rho0;
            let t22 = ((v_rho1).sqrt());
            let t23 = t22 * v_rho1;
            let t25 = t13 * t13;
            let t26 = t25 * v_rho0;
            let t28 = t16 * t16;
            let t29 = t28 * v_rho1;
            let t31 = t1 * t1;
            let t32 = t31 * t31;
            let t33 = t32 * t1;
            let t35 = t4 * t4;
            let t36 = t35 * t35;
            let t37 = t36 * t4;
            let t39 = v_rho0 * t33 + v_rho1 * t37;
            let t40 = ((v_sigma0).sqrt());
            let t41 = f64x8::splat(1.0) / t14;
            let t42 = t40 * t41;
            let t43 = v_rho0 - v_rho1;
            let t44 = v_rho0 + v_rho1;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t43 * t45;
            let t47 = f64x8::splat(1.0) + t46;
            let t48 = (t47).simd_le(zeta_threshold);
            let t49 = (simd::cbrt(zeta_threshold));
            let t50 = t49 * zeta_threshold;
            let t51 = (simd::cbrt(t47));
            let t53 = ((t48).select(t50, t51 * t47));
            let t54 = f64x8::splat(M_CBRT2);
            let t55 = t54 * t54;
            let t56 = t53 * t55;
            let t58 = ((v_sigma2).sqrt());
            let t59 = f64x8::splat(1.0) / t17;
            let t60 = t58 * t59;
            let t61 = f64x8::splat(1.0) - t46;
            let t62 = (t61).simd_le(zeta_threshold);
            let t63 = (simd::cbrt(t61));
            let t65 = ((t62).select(t50, t63 * t61));
            let t66 = t65 * t55;
            let t69 = t42 * t56 / f64x8::splat(4.0) + t60 * t66 / f64x8::splat(4.0);
            let t72 = t20 + t23;
            let t75 = f64x8::splat(0.678831) * t1 * v_rho0 + f64x8::splat(0.678831) * t4 * v_rho1 - f64x8::splat(1.75821) * t8 - f64x8::splat(1.75821) * t11 + f64x8::splat(1.27676) * t14 + f64x8::splat(1.27676) * t17 - f64x8::splat(1.60789) * t20 - f64x8::splat(1.60789) * t23 + f64x8::splat(0.36561) * t26 + f64x8::splat(0.36561) * t29 - f64x8::splat(0.0906635) * t39 * t69 + f64x8::splat(0.0734865) * t72 * t69;
            let t76 = t26 + t29;
            let t79 = t7 * t7;
            let t80 = t79 * t79;
            let t81 = t80 * t7;
            let t82 = t81 * v_rho0;
            let t83 = t10 * t10;
            let t84 = t83 * t83;
            let t85 = t84 * t10;
            let t86 = t85 * v_rho1;
            let t87 = t82 + t86;
            let t90 = v_rho0 * v_rho0;
            let t92 = f64x8::splat(1.0) / t25 / t90;
            let t93 = v_sigma0 * t92;
            let t94 = t53 * t53;
            let t95 = t94 * t54;
            let t96 = t93 * t95;
            let t97 = v_rho1 * v_rho1;
            let t99 = f64x8::splat(1.0) / t28 / t97;
            let t100 = v_sigma2 * t99;
            let t101 = t65 * t65;
            let t102 = t101 * t54;
            let t103 = t100 * t102;
            let t105 = t96 / f64x8::splat(8.0) + t103 / f64x8::splat(8.0);
            let t110 = t90 + t97;
            let t116 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t117 = t44 * t44;
            let t118 = (simd::cbrt(t44));
            let t119 = t118 * t118;
            let t121 = f64x8::splat(1.0) / t119 / t117;
            let t122 = t116 * t121;
            let t123 = t96 / f64x8::splat(4.0) + t103 / f64x8::splat(4.0) - t122;
            let t130 = t8 + t11;
            let t131 = t43 * t43;
            let t132 = t130 * t131;
            let t133 = f64x8::splat(1.0) / t117;
            let t136 = t14 + t17;
            let t137 = t136 * t131;
            let t140 = t72 * t131;
            let t143 = (simd::pow_5_3(v_rho0));
            let t144 = (simd::pow_5_3(v_rho1));
            let t145 = t143 + t144;
            let t146 = t145 * t131;
            let t149 = f64x8::splat(0.0735705) * t76 * t69 - f64x8::splat(0.03584585) * t87 * t69 - f64x8::splat(0.02035835) * t76 * t105 + f64x8::splat(0.01073125) * t87 * t105 - f64x8::splat(0.000384078) * t110 * t105 + f64x8::splat(0.0310377) * t76 * t123 - f64x8::splat(0.0720326) * t87 * t123 + f64x8::splat(0.0446562) * t110 * t123 - f64x8::splat(0.266802) * t132 * t133 + f64x8::splat(1.50822) * t137 * t133 - f64x8::splat(1.94515) * t140 * t133 + f64x8::splat(0.679078) * t146 * t133;
            let tzk0 = (t75 + t149) * t45;
            acc_zk = tzk0;
            let t157 = f64x8::splat(1.0) / t13 / t90;
            let t158 = t40 * t157;
            let t161 = t43 * t133;
            let t162 = t45 - t161;
            let t165 = ((t48).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t51 * t162));
            let t166 = t165 * t55;
            let t169 = -t162;
            let t172 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t169));
            let t173 = t172 * t55;
            let t176 = -t158 * t56 / f64x8::splat(3.0) + t42 * t166 / f64x8::splat(4.0) + t60 * t173 / f64x8::splat(4.0);
            let t193 = t90 * v_rho0;
            let t195 = f64x8::splat(1.0) / t25 / t193;
            let t196 = v_sigma0 * t195;
            let t197 = t196 * t95;
            let t199 = t53 * t54;
            let t200 = t199 * t165;
            let t201 = t93 * t200;
            let t203 = t65 * t54;
            let t204 = t203 * t172;
            let t205 = t100 * t204;
            let t207 = -t197 / f64x8::splat(3.0) + t201 / f64x8::splat(4.0) + t205 / f64x8::splat(4.0);
            let t218 = f64x8::splat(0.73540025) * t1 + f64x8::splat(0.60935) * t25 - f64x8::splat(2.051245) * t7 + f64x8::splat(1.7023466666666667) * t13 - f64x8::splat(2.411835) * t19 - f64x8::splat(0.0906635) * t39 * t176 - f64x8::splat(0.12843995833333333) * t33 * t69 + f64x8::splat(0.0734865) * t72 * t176 + f64x8::splat(0.11022975) * t19 * t69 + f64x8::splat(0.0735705) * t76 * t176 + f64x8::splat(0.1226175) * t25 * t69 - f64x8::splat(0.03584585) * t87 * t176 - f64x8::splat(0.06571739166666667) * t81 * t69 - f64x8::splat(0.02035835) * t76 * t207 - f64x8::splat(0.03393058333333333) * t25 * t105 + f64x8::splat(0.01073125) * t87 * t207 + f64x8::splat(0.019673958333333335) * t81 * t105 - f64x8::splat(0.000384078) * t110 * t207;
            let t224 = t117 * t44;
            let t226 = f64x8::splat(1.0) / t119 / t224;
            let t227 = t116 * t226;
            let t228 = f64x8::splat(8.0) / f64x8::splat(3.0) * t227;
            let t229 = -f64x8::splat(2.0) / f64x8::splat(3.0) * t197 + t201 / f64x8::splat(2.0) + t205 / f64x8::splat(2.0) + t228;
            let t242 = t130 * t43;
            let t244 = f64x8::splat(0.533604) * t242 * t133;
            let t245 = f64x8::splat(1.0) / t224;
            let t247 = f64x8::splat(0.533604) * t132 * t245;
            let t248 = t136 * t43;
            let t250 = f64x8::splat(3.01644) * t248 * t133;
            let t252 = f64x8::splat(3.01644) * t137 * t245;
            let t253 = t72 * t43;
            let t255 = f64x8::splat(3.8903) * t253 * t133;
            let t257 = f64x8::splat(3.8903) * t140 * t245;
            let t258 = t145 * t43;
            let t260 = f64x8::splat(1.358156) * t258 * t133;
            let t262 = f64x8::splat(1.358156) * t146 * t245;
            let t263 = t7 * t131;
            let t266 = t13 * t131;
            let t269 = t19 * t131;
            let t272 = (simd::pow_2_3(v_rho0));
            let t273 = t272 * t131;
            let t276 = -f64x8::splat(0.000768156) * v_rho0 * t105 + f64x8::splat(0.0310377) * t76 * t229 + f64x8::splat(0.0517295) * t25 * t123 - f64x8::splat(0.0720326) * t87 * t229 - f64x8::splat(0.13205976666666666) * t81 * t123 + f64x8::splat(0.0446562) * t110 * t229 + f64x8::splat(0.0893124) * v_rho0 * t123 - t244 + t247 + t250 - t252 - t255 + t257 + t260 - t262 - f64x8::splat(0.311269) * t263 * t133 + f64x8::splat(2.01096) * t266 * t133 - f64x8::splat(2.917725) * t269 * t133 + f64x8::splat(1.1317966666666666) * t273 * t133;
            let tvrho0 = t218 + t276;
            acc_vrho_0 = tvrho0;
            let t282 = -t45 - t161;
            let t285 = ((t48).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t51 * t282));
            let t286 = t285 * t55;
            let t290 = f64x8::splat(1.0) / t16 / t97;
            let t291 = t58 * t290;
            let t294 = -t282;
            let t297 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t294));
            let t298 = t297 * t55;
            let t301 = t42 * t286 / f64x8::splat(4.0) - t291 * t66 / f64x8::splat(3.0) + t60 * t298 / f64x8::splat(4.0);
            let t318 = t199 * t285;
            let t319 = t93 * t318;
            let t321 = t97 * v_rho1;
            let t323 = f64x8::splat(1.0) / t28 / t321;
            let t324 = v_sigma2 * t323;
            let t325 = t324 * t102;
            let t327 = t203 * t297;
            let t328 = t100 * t327;
            let t330 = t319 / f64x8::splat(4.0) - t325 / f64x8::splat(3.0) + t328 / f64x8::splat(4.0);
            let t341 = f64x8::splat(0.73540025) * t4 + f64x8::splat(0.60935) * t28 - f64x8::splat(2.411835) * t22 - f64x8::splat(2.051245) * t10 + f64x8::splat(1.7023466666666667) * t16 - f64x8::splat(0.0906635) * t39 * t301 - f64x8::splat(0.12843995833333333) * t37 * t69 + f64x8::splat(0.0734865) * t72 * t301 + f64x8::splat(0.11022975) * t22 * t69 + f64x8::splat(0.0735705) * t76 * t301 + f64x8::splat(0.1226175) * t28 * t69 - f64x8::splat(0.03584585) * t87 * t301 - f64x8::splat(0.06571739166666667) * t85 * t69 - f64x8::splat(0.02035835) * t76 * t330 - f64x8::splat(0.03393058333333333) * t28 * t105 + f64x8::splat(0.01073125) * t87 * t330 + f64x8::splat(0.019673958333333335) * t85 * t105 - f64x8::splat(0.000384078) * t110 * t330;
            let t347 = t319 / f64x8::splat(2.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t325 + t328 / f64x8::splat(2.0) + t228;
            let t360 = t10 * t131;
            let t363 = t16 * t131;
            let t366 = t22 * t131;
            let t369 = (simd::pow_2_3(v_rho1));
            let t370 = t369 * t131;
            let t373 = -f64x8::splat(0.000768156) * v_rho1 * t105 + f64x8::splat(0.0310377) * t76 * t347 + f64x8::splat(0.0517295) * t28 * t123 - f64x8::splat(0.0720326) * t87 * t347 - f64x8::splat(0.13205976666666666) * t85 * t123 + f64x8::splat(0.0446562) * t110 * t347 + f64x8::splat(0.0893124) * v_rho1 * t123 + t244 + t247 - t250 - t252 + t255 + t257 - t260 - t262 - f64x8::splat(0.311269) * t360 * t133 + f64x8::splat(2.01096) * t363 * t133 - f64x8::splat(2.917725) * t366 * t133 + f64x8::splat(1.1317966666666666) * t370 * t133;
            let tvrho1 = t341 + t373;
            acc_vrho_1 = tvrho1;
            let t374 = f64x8::splat(1.0) / t40;
            let t375 = t39 * t374;
            let t377 = t41 * t53 * t55;
            let t380 = t72 * t374;
            let t383 = t76 * t374;
            let t386 = t87 * t374;
            let t389 = t76 * t92;
            let t392 = t87 * t92;
            let t395 = t110 * t92;
            let t399 = t92 * t94 * t54;
            let t401 = t399 / f64x8::splat(4.0) - t121;
            let tvsigma0 = -f64x8::splat(0.0113329375) * t375 * t377 + f64x8::splat(0.0091858125) * t380 * t377 + f64x8::splat(0.0091963125) * t383 * t377 - f64x8::splat(0.00448073125) * t386 * t377 - f64x8::splat(0.00254479375) * t389 * t95 + f64x8::splat(0.00134140625) * t392 * t95 - f64x8::splat(4.800975e-05) * t395 * t95 + f64x8::splat(0.0310377) * t76 * t401 - f64x8::splat(0.0720326) * t87 * t401 + f64x8::splat(0.0446562) * t110 * t401;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = -f64x8::splat(0.0620754) * t76 * t121 + f64x8::splat(0.1440652) * t87 * t121 - f64x8::splat(0.0893124) * t110 * t121;
            acc_vsigma_1 = tvsigma1;
            let t414 = f64x8::splat(1.0) / t58;
            let t415 = t39 * t414;
            let t417 = t59 * t65 * t55;
            let t420 = t72 * t414;
            let t423 = t76 * t414;
            let t426 = t87 * t414;
            let t429 = t76 * t99;
            let t432 = t87 * t99;
            let t435 = t110 * t99;
            let t439 = t99 * t101 * t54;
            let t441 = t439 / f64x8::splat(4.0) - t121;
            let tvsigma2 = -f64x8::splat(0.0113329375) * t415 * t417 + f64x8::splat(0.0091858125) * t420 * t417 + f64x8::splat(0.0091963125) * t423 * t417 - f64x8::splat(0.00448073125) * t426 * t417 - f64x8::splat(0.00254479375) * t429 * t102 + f64x8::splat(0.00134140625) * t432 * t102 - f64x8::splat(4.800975e-05) * t435 * t102 + f64x8::splat(0.0310377) * t76 * t441 - f64x8::splat(0.0720326) * t87 * t441 + f64x8::splat(0.0446562) * t110 * t441;
            acc_vsigma_2 = tvsigma2;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
