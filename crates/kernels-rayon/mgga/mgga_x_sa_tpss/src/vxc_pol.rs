//! MGGA_X_SA_TPSS vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_sa_tpss.c`
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
pub fn mgga_x_sa_tpss_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = ((f64x8::splat(5.0)).sqrt());
            let t30 = f64x8::splat(M_PI) * t29;
            let t31 = (simd::cbrt(v_rho0));
            let t32 = t31 * t31;
            let t34 = f64x8::splat(1.0) / t32 / v_rho0;
            let t36 = v_rho0 * v_rho0;
            let t38 = f64x8::splat(1.0) / t32 / t36;
            let t39 = v_sigma0 * t38;
            let t41 = v_tau0 * t34 - t39 / f64x8::splat(8.0);
            let t42 = f64x8::splat(M_CBRT6);
            let t43 = t41 * t42;
            let t44 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t45 = (simd::cbrt(t44));
            let t46 = t45 * t45;
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t43 * t47;
            let t50 = f64x8::splat(5.0) * t48 + f64x8::splat(9.0);
            let t51 = ((t50).sqrt());
            let t52 = f64x8::splat(5.0) / f64x8::splat(9.0) * t48;
            let t53 = t52 + f64x8::splat(0.348);
            let t54 = (simd::ln(t53));
            let t55 = f64x8::splat(2.413) + t54;
            let t56 = ((t55).sqrt());
            let t57 = f64x8::splat(1.0) / t56;
            let t58 = t51 * t57;
            let t59 = t30 * t58;
            let t61 = v_sigma0 * v_sigma0;
            let t62 = f64x8::splat(1.0) / t36;
            let t63 = t61 * t62;
            let t64 = v_tau0 * v_tau0;
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t63 * t65;
            let t68 = f64x8::splat(1.0) + t66 / f64x8::splat(64.0);
            let t69 = t68 * t68;
            let t70 = f64x8::splat(1.0) / t69;
            let t71 = t65 * t70;
            let t75 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.02485875) * t63 * t71) * t42;
            let t76 = t47 * v_sigma0;
            let t77 = t76 * t38;
            let t80 = t52 - f64x8::splat(1.0);
            let t81 = t47 * t80;
            let t84 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t43 * t81;
            let t85 = ((t84).sqrt());
            let t86 = f64x8::splat(1.0) / t85;
            let t89 = t42 * t47;
            let t90 = t89 * t39;
            let t92 = f64x8::splat(9.0) / f64x8::splat(20.0) * t80 * t86 + t90 / f64x8::splat(36.0);
            let t93 = t92 * t92;
            let t96 = t42 * t42;
            let t98 = f64x8::splat(1.0) / t45 / t44;
            let t99 = t96 * t98;
            let t100 = t36 * t36;
            let t101 = t100 * v_rho0;
            let t103 = f64x8::splat(1.0) / t31 / t101;
            let t104 = t61 * t103;
            let t105 = t99 * t104;
            let t107 = f64x8::splat(162.0) * t66 + f64x8::splat(50.0) * t105;
            let t108 = ((t107).sqrt());
            let t112 = f64x8::splat(1.0) / f64x8::splat(M_PI) * t29;
            let t113 = f64x8::splat(1.0) / t51;
            let t115 = t112 * t113 * t56;
            let t119 = t61 * v_sigma0;
            let t120 = t100 * t100;
            let t121 = f64x8::splat(1.0) / t120;
            let t124 = t75 * t77 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t93 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t92 * t108 + f64x8::splat(25.0) / f64x8::splat(209952.0) * t115 * t105 + f64x8::splat(0.0017218861679299947) * t66 + f64x8::splat(1.5033019185692233e-06) * t119 * t121;
            let t126 = f64x8::splat(1.0) + f64x8::splat(0.05165658503789984) * t90;
            let t127 = t126 * t126;
            let t128 = f64x8::splat(1.0) / t127;
            let t130 = f64x8::splat(2.0) / f64x8::splat(45.0) * t59 + t124 * t128;
            let t131 = f64x8::splat(1.0) / t130;
            let t135 = f64x8::splat(1.0) - f64x8::splat(2.0) / f64x8::splat(45.0) * t30 * t58 * t131;
            let t139 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t30 * t58 * t135;
            let t143 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t139));
            let t144 = (v_rho1).simd_le(dens_threshold);
            let t145 = -t17;
            let t147 = ((t15).select(t12, (t11).select(t16, t145 * t8)));
            let t148 = f64x8::splat(1.0) + t147;
            let t149 = (t148).simd_le(zeta_threshold);
            let t150 = (simd::cbrt(t148));
            let t152 = ((t149).select(t23, t150 * t148));
            let t153 = t152 * t27;
            let t154 = (simd::cbrt(v_rho1));
            let t155 = t154 * t154;
            let t157 = f64x8::splat(1.0) / t155 / v_rho1;
            let t159 = v_rho1 * v_rho1;
            let t161 = f64x8::splat(1.0) / t155 / t159;
            let t162 = v_sigma2 * t161;
            let t164 = v_tau1 * t157 - t162 / f64x8::splat(8.0);
            let t165 = t164 * t42;
            let t166 = t165 * t47;
            let t168 = f64x8::splat(5.0) * t166 + f64x8::splat(9.0);
            let t169 = ((t168).sqrt());
            let t170 = f64x8::splat(5.0) / f64x8::splat(9.0) * t166;
            let t171 = t170 + f64x8::splat(0.348);
            let t172 = (simd::ln(t171));
            let t173 = f64x8::splat(2.413) + t172;
            let t174 = ((t173).sqrt());
            let t175 = f64x8::splat(1.0) / t174;
            let t176 = t169 * t175;
            let t177 = t30 * t176;
            let t179 = v_sigma2 * v_sigma2;
            let t180 = f64x8::splat(1.0) / t159;
            let t181 = t179 * t180;
            let t182 = v_tau1 * v_tau1;
            let t183 = f64x8::splat(1.0) / t182;
            let t184 = t181 * t183;
            let t186 = f64x8::splat(1.0) + t184 / f64x8::splat(64.0);
            let t187 = t186 * t186;
            let t188 = f64x8::splat(1.0) / t187;
            let t189 = t183 * t188;
            let t193 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.02485875) * t181 * t189) * t42;
            let t194 = t47 * v_sigma2;
            let t195 = t194 * t161;
            let t198 = t170 - f64x8::splat(1.0);
            let t199 = t47 * t198;
            let t202 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t165 * t199;
            let t203 = ((t202).sqrt());
            let t204 = f64x8::splat(1.0) / t203;
            let t207 = t89 * t162;
            let t209 = f64x8::splat(9.0) / f64x8::splat(20.0) * t198 * t204 + t207 / f64x8::splat(36.0);
            let t210 = t209 * t209;
            let t213 = t159 * t159;
            let t214 = t213 * v_rho1;
            let t216 = f64x8::splat(1.0) / t154 / t214;
            let t217 = t179 * t216;
            let t218 = t99 * t217;
            let t220 = f64x8::splat(162.0) * t184 + f64x8::splat(50.0) * t218;
            let t221 = ((t220).sqrt());
            let t224 = f64x8::splat(1.0) / t169;
            let t226 = t112 * t224 * t174;
            let t230 = t179 * v_sigma2;
            let t231 = t213 * t213;
            let t232 = f64x8::splat(1.0) / t231;
            let t235 = t193 * t195 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t210 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t209 * t221 + f64x8::splat(25.0) / f64x8::splat(209952.0) * t226 * t218 + f64x8::splat(0.0017218861679299947) * t184 + f64x8::splat(1.5033019185692233e-06) * t230 * t232;
            let t237 = f64x8::splat(1.0) + f64x8::splat(0.05165658503789984) * t207;
            let t238 = t237 * t237;
            let t239 = f64x8::splat(1.0) / t238;
            let t241 = f64x8::splat(2.0) / f64x8::splat(45.0) * t177 + t235 * t239;
            let t242 = f64x8::splat(1.0) / t241;
            let t246 = f64x8::splat(1.0) - f64x8::splat(2.0) / f64x8::splat(45.0) * t30 * t176 * t242;
            let t250 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t30 * t176 * t246;
            let t254 = ((t144).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t153 * t250));
            let tzk0 = t143 + t254;
            acc_zk = tzk0;
            let t255 = t7 * t7;
            let t256 = f64x8::splat(1.0) / t255;
            let t257 = t17 * t256;
            let t259 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t257)));
            let t262 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t259));
            let t263 = t262 * t27;
            let t267 = t27 * t27;
            let t268 = f64x8::splat(1.0) / t267;
            let t269 = t26 * t268;
            let t272 = t6 * t269 * t139 / f64x8::splat(8.0);
            let t273 = t113 * t57;
            let t274 = t30 * t273;
            let t277 = t36 * v_rho0;
            let t279 = f64x8::splat(1.0) / t32 / t277;
            let t280 = v_sigma0 * t279;
            let t282 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t38 + t280 / f64x8::splat(3.0);
            let t283 = t135 * t282;
            let t288 = f64x8::splat(1.0) / t56 / t55;
            let t289 = t51 * t288;
            let t290 = t30 * t289;
            let t291 = f64x8::splat(1.0) / t53;
            let t292 = t89 * t291;
            let t296 = t131 * t282;
            let t303 = t30 * t51;
            let t304 = t130 * t130;
            let t305 = f64x8::splat(1.0) / t304;
            let t306 = t57 * t305;
            let t307 = t30 * t113;
            let t312 = t282 * t42;
            let t313 = t47 * t291;
            let t317 = f64x8::splat(1.0) / t277;
            let t318 = t61 * t317;
            let t321 = t61 * t61;
            let t322 = f64x8::splat(1.0) / t101;
            let t323 = t321 * t322;
            let t324 = t64 * t64;
            let t325 = f64x8::splat(1.0) / t324;
            let t327 = f64x8::splat(1.0) / t69 / t68;
            let t328 = t325 * t327;
            let t332 = (-f64x8::splat(0.0497175) * t318 * t71 + f64x8::splat(0.001553671875) * t323 * t328) * t42;
            let t335 = t76 * t279;
            let t338 = t47 * t86;
            let t342 = f64x8::splat(1.0) / t85 / t84;
            let t343 = t80 * t342;
            let t346 = t41 * t96;
            let t347 = t98 * t282;
            let t350 = f64x8::splat(0.2222222222222222) * t312 * t81 + f64x8::splat(0.12345679012345678) * t346 * t347;
            let t355 = t312 * t338 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t343 * t350 - f64x8::splat(2.0) / f64x8::splat(27.0) * t89 * t280;
            let t360 = f64x8::splat(1.0) / t108;
            let t361 = t92 * t360;
            let t362 = t318 * t65;
            let t364 = t100 * t36;
            let t366 = f64x8::splat(1.0) / t31 / t364;
            let t367 = t61 * t366;
            let t368 = t99 * t367;
            let t370 = -f64x8::splat(324.0) * t362 - f64x8::splat(800.0) / f64x8::splat(3.0) * t368;
            let t373 = t44 * t44;
            let t376 = f64x8::splat(1.0) / t373 / f64x8::splat(M_PI) * t29;
            let t378 = f64x8::splat(1.0) / t51 / t50;
            let t379 = t376 * t378;
            let t380 = t56 * t61;
            let t381 = t103 * t282;
            let t385 = t376 * t273;
            let t386 = t282 * t291;
            let t393 = t120 * v_rho0;
            let t394 = f64x8::splat(1.0) / t393;
            let t397 = t332 * t77 / f64x8::splat(24.0) - t75 * t335 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t92 * t355 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t355 * t108 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t361 * t370 - f64x8::splat(125.0) / f64x8::splat(69984.0) * t379 * t380 * t381 + f64x8::splat(125.0) / f64x8::splat(629856.0) * t385 * t104 * t386 - f64x8::splat(25.0) / f64x8::splat(39366.0) * t115 * t368 - f64x8::splat(0.0034437723358599895) * t362 - f64x8::splat(1.2026415348553787e-05) * t119 * t394;
            let t400 = f64x8::splat(1.0) / t127 / t126;
            let t401 = t124 * t400;
            let t402 = t401 * t42;
            let t405 = t307 * t57 * t282 * t89 / f64x8::splat(9.0) - t290 * t312 * t313 / f64x8::splat(81.0) + t397 * t128 + f64x8::splat(0.27550178686879917) * t402 * t335;
            let t409 = -t274 * t296 * t89 / f64x8::splat(9.0) + t290 * t296 * t292 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t303 * t306 * t405;
            let t413 = t274 * t283 * t89 / f64x8::splat(9.0) - t290 * t283 * t292 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t30 * t58 * t409;
            let t418 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t263 * t139 - t272 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t413));
            let t419 = t145 * t256;
            let t421 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t419)));
            let t424 = ((t149).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t150 * t421));
            let t425 = t424 * t27;
            let t429 = t152 * t268;
            let t432 = t6 * t429 * t250 / f64x8::splat(8.0);
            let t434 = ((t144).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t425 * t250 - t432));
            let tvrho0 = t143 + t254 + t7 * (t418 + t434);
            acc_vrho_0 = tvrho0;
            let t438 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t257)));
            let t441 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t438));
            let t442 = t441 * t27;
            let t447 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t442 * t139 - t272));
            let t449 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t419)));
            let t452 = ((t149).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t150 * t449));
            let t453 = t452 * t27;
            let t457 = t224 * t175;
            let t458 = t30 * t457;
            let t461 = t159 * v_rho1;
            let t463 = f64x8::splat(1.0) / t155 / t461;
            let t464 = v_sigma2 * t463;
            let t466 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t161 + t464 / f64x8::splat(3.0);
            let t467 = t246 * t466;
            let t472 = f64x8::splat(1.0) / t174 / t173;
            let t473 = t169 * t472;
            let t474 = t30 * t473;
            let t475 = f64x8::splat(1.0) / t171;
            let t476 = t89 * t475;
            let t480 = t242 * t466;
            let t487 = t30 * t169;
            let t488 = t241 * t241;
            let t489 = f64x8::splat(1.0) / t488;
            let t490 = t175 * t489;
            let t491 = t30 * t224;
            let t496 = t466 * t42;
            let t497 = t47 * t475;
            let t501 = f64x8::splat(1.0) / t461;
            let t502 = t179 * t501;
            let t505 = t179 * t179;
            let t506 = f64x8::splat(1.0) / t214;
            let t507 = t505 * t506;
            let t508 = t182 * t182;
            let t509 = f64x8::splat(1.0) / t508;
            let t511 = f64x8::splat(1.0) / t187 / t186;
            let t512 = t509 * t511;
            let t516 = (-f64x8::splat(0.0497175) * t502 * t189 + f64x8::splat(0.001553671875) * t507 * t512) * t42;
            let t519 = t194 * t463;
            let t522 = t47 * t204;
            let t526 = f64x8::splat(1.0) / t203 / t202;
            let t527 = t198 * t526;
            let t530 = t164 * t96;
            let t531 = t98 * t466;
            let t534 = f64x8::splat(0.2222222222222222) * t496 * t199 + f64x8::splat(0.12345679012345678) * t530 * t531;
            let t539 = t496 * t522 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t527 * t534 - f64x8::splat(2.0) / f64x8::splat(27.0) * t89 * t464;
            let t544 = f64x8::splat(1.0) / t221;
            let t545 = t209 * t544;
            let t546 = t502 * t183;
            let t548 = t213 * t159;
            let t550 = f64x8::splat(1.0) / t154 / t548;
            let t551 = t179 * t550;
            let t552 = t99 * t551;
            let t554 = -f64x8::splat(324.0) * t546 - f64x8::splat(800.0) / f64x8::splat(3.0) * t552;
            let t558 = f64x8::splat(1.0) / t169 / t168;
            let t559 = t376 * t558;
            let t560 = t174 * t179;
            let t561 = t216 * t466;
            let t565 = t376 * t457;
            let t566 = t466 * t475;
            let t573 = t231 * v_rho1;
            let t574 = f64x8::splat(1.0) / t573;
            let t577 = t516 * t195 / f64x8::splat(24.0) - t193 * t519 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t209 * t539 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t539 * t221 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t545 * t554 - f64x8::splat(125.0) / f64x8::splat(69984.0) * t559 * t560 * t561 + f64x8::splat(125.0) / f64x8::splat(629856.0) * t565 * t217 * t566 - f64x8::splat(25.0) / f64x8::splat(39366.0) * t226 * t552 - f64x8::splat(0.0034437723358599895) * t546 - f64x8::splat(1.2026415348553787e-05) * t230 * t574;
            let t580 = f64x8::splat(1.0) / t238 / t237;
            let t581 = t235 * t580;
            let t582 = t581 * t42;
            let t585 = t491 * t175 * t466 * t89 / f64x8::splat(9.0) - t474 * t496 * t497 / f64x8::splat(81.0) + t577 * t239 + f64x8::splat(0.27550178686879917) * t582 * t519;
            let t589 = -t458 * t480 * t89 / f64x8::splat(9.0) + t474 * t480 * t476 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t487 * t490 * t585;
            let t593 = t458 * t467 * t89 / f64x8::splat(9.0) - t474 * t467 * t476 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t30 * t176 * t589;
            let t598 = ((t144).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t453 * t250 - t432 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t153 * t593));
            let tvrho1 = t143 + t254 + t7 * (t447 + t598);
            acc_vrho_1 = tvrho1;
            let t601 = t135 * t38;
            let t603 = t274 * t601 * t89;
            let t606 = t290 * t601 * t292;
            let t608 = t131 * t38;
            let t610 = t274 * t608 * t89;
            let t613 = t290 * t608 * t292;
            let t615 = t57 * t38;
            let t617 = t307 * t615 * t89;
            let t619 = t38 * t42;
            let t621 = t290 * t619 * t313;
            let t623 = v_sigma0 * t62;
            let t626 = f64x8::splat(1.0) / t100;
            let t627 = t119 * t626;
            let t631 = (f64x8::splat(0.0497175) * t623 * t71 - f64x8::splat(0.001553671875) * t627 * t328) * t42;
            let t634 = t47 * t38;
            let t637 = t619 * t338;
            let t639 = t619 * t81;
            let t642 = t346 * t98 * t38;
            let t644 = -f64x8::splat(0.027777777777777776) * t639 - f64x8::splat(0.015432098765432098) * t642;
            let t647 = t619 * t47;
            let t649 = -t637 / f64x8::splat(32.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t343 * t644 + t647 / f64x8::splat(36.0);
            let t654 = t623 * t65;
            let t656 = v_sigma0 * t103;
            let t657 = t99 * t656;
            let t659 = f64x8::splat(324.0) * t654 + f64x8::splat(100.0) * t657;
            let t663 = t379 * t380 * t121;
            let t665 = t376 * t113;
            let t666 = t57 * t61;
            let t667 = t121 * t291;
            let t669 = t665 * t666 * t667;
            let t674 = t61 * t121;
            let t676 = t631 * t77 / f64x8::splat(24.0) + t75 * t634 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t92 * t649 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t649 * t108 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t361 * t659 + f64x8::splat(125.0) / f64x8::splat(559872.0) * t663 - f64x8::splat(125.0) / f64x8::splat(5038848.0) * t669 + f64x8::splat(25.0) / f64x8::splat(104976.0) * t115 * t657 + f64x8::splat(0.0034437723358599895) * t654 + f64x8::splat(4.50990575570767e-06) * t674;
            let t680 = -t617 / f64x8::splat(72.0) + t621 / f64x8::splat(648.0) + t676 * t128 - f64x8::splat(0.10331317007579968) * t401 * t647;
            let t684 = t610 / f64x8::splat(72.0) - t613 / f64x8::splat(648.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t303 * t306 * t680;
            let t688 = -t603 / f64x8::splat(72.0) + t606 / f64x8::splat(648.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t30 * t58 * t684;
            let t692 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t688));
            let tvsigma0 = t7 * t692;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t693 = t246 * t161;
            let t695 = t458 * t693 * t89;
            let t698 = t474 * t693 * t476;
            let t700 = t242 * t161;
            let t702 = t458 * t700 * t89;
            let t705 = t474 * t700 * t476;
            let t707 = t175 * t161;
            let t709 = t491 * t707 * t89;
            let t711 = t161 * t42;
            let t713 = t474 * t711 * t497;
            let t715 = v_sigma2 * t180;
            let t718 = f64x8::splat(1.0) / t213;
            let t719 = t230 * t718;
            let t723 = (f64x8::splat(0.0497175) * t715 * t189 - f64x8::splat(0.001553671875) * t719 * t512) * t42;
            let t726 = t47 * t161;
            let t729 = t711 * t522;
            let t731 = t711 * t199;
            let t734 = t530 * t98 * t161;
            let t736 = -f64x8::splat(0.027777777777777776) * t731 - f64x8::splat(0.015432098765432098) * t734;
            let t739 = t711 * t47;
            let t741 = -t729 / f64x8::splat(32.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t527 * t736 + t739 / f64x8::splat(36.0);
            let t746 = t715 * t183;
            let t748 = v_sigma2 * t216;
            let t749 = t99 * t748;
            let t751 = f64x8::splat(324.0) * t746 + f64x8::splat(100.0) * t749;
            let t755 = t559 * t560 * t232;
            let t757 = t376 * t224;
            let t758 = t175 * t179;
            let t759 = t232 * t475;
            let t761 = t757 * t758 * t759;
            let t766 = t179 * t232;
            let t768 = t723 * t195 / f64x8::splat(24.0) + t193 * t726 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t209 * t741 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t741 * t221 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t545 * t751 + f64x8::splat(125.0) / f64x8::splat(559872.0) * t755 - f64x8::splat(125.0) / f64x8::splat(5038848.0) * t761 + f64x8::splat(25.0) / f64x8::splat(104976.0) * t226 * t749 + f64x8::splat(0.0034437723358599895) * t746 + f64x8::splat(4.50990575570767e-06) * t766;
            let t772 = -t709 / f64x8::splat(72.0) + t713 / f64x8::splat(648.0) + t768 * t239 - f64x8::splat(0.10331317007579968) * t581 * t739;
            let t776 = t702 / f64x8::splat(72.0) - t705 / f64x8::splat(648.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t487 * t490 * t772;
            let t780 = -t695 / f64x8::splat(72.0) + t698 / f64x8::splat(648.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t30 * t176 * t776;
            let t784 = ((t144).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t153 * t780));
            let tvsigma2 = t7 * t784;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t785 = t135 * t34;
            let t792 = t131 * t34;
            let t799 = t57 * t34;
            let t803 = t34 * t42;
            let t807 = t64 * v_tau0;
            let t808 = f64x8::splat(1.0) / t807;
            let t809 = t808 * t70;
            let t812 = t321 * t626;
            let t813 = t324 * v_tau0;
            let t814 = f64x8::splat(1.0) / t813;
            let t815 = t814 * t327;
            let t819 = (-f64x8::splat(0.0497175) * t63 * t809 + f64x8::splat(0.001553671875) * t812 * t815) * t42;
            let t829 = f64x8::splat(0.2222222222222222) * t803 * t81 + f64x8::splat(0.12345679012345678) * t346 * t98 * t34;
            let t832 = t803 * t338 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t343 * t829;
            let t837 = t63 * t808;
            let t840 = t100 * t277;
            let t841 = f64x8::splat(1.0) / t840;
            let t845 = t841 * t291;
            let t850 = t819 * t77 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t92 * t832 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t832 * t108 + f64x8::splat(73.0) / f64x8::splat(600.0) * t361 * t837 - f64x8::splat(125.0) / f64x8::splat(69984.0) * t379 * t380 * t841 + f64x8::splat(125.0) / f64x8::splat(629856.0) * t665 * t666 * t845 - f64x8::splat(0.0034437723358599895) * t837;
            let t852 = t307 * t799 * t89 / f64x8::splat(9.0) - t290 * t803 * t313 / f64x8::splat(81.0) + t850 * t128;
            let t856 = -t274 * t792 * t89 / f64x8::splat(9.0) + t290 * t792 * t292 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t303 * t306 * t852;
            let t860 = t274 * t785 * t89 / f64x8::splat(9.0) - t290 * t785 * t292 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t30 * t58 * t856;
            let t864 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t860));
            let tvtau0 = t7 * t864;
            acc_vtau_0 = tvtau0;
            let t865 = t246 * t157;
            let t872 = t242 * t157;
            let t879 = t175 * t157;
            let t883 = t157 * t42;
            let t887 = t182 * v_tau1;
            let t888 = f64x8::splat(1.0) / t887;
            let t889 = t888 * t188;
            let t892 = t505 * t718;
            let t893 = t508 * v_tau1;
            let t894 = f64x8::splat(1.0) / t893;
            let t895 = t894 * t511;
            let t899 = (-f64x8::splat(0.0497175) * t181 * t889 + f64x8::splat(0.001553671875) * t892 * t895) * t42;
            let t909 = f64x8::splat(0.2222222222222222) * t883 * t199 + f64x8::splat(0.12345679012345678) * t530 * t98 * t157;
            let t912 = t883 * t522 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t527 * t909;
            let t917 = t181 * t888;
            let t920 = t213 * t461;
            let t921 = f64x8::splat(1.0) / t920;
            let t925 = t921 * t475;
            let t930 = t899 * t195 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t209 * t912 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t912 * t221 + f64x8::splat(73.0) / f64x8::splat(600.0) * t545 * t917 - f64x8::splat(125.0) / f64x8::splat(69984.0) * t559 * t560 * t921 + f64x8::splat(125.0) / f64x8::splat(629856.0) * t757 * t758 * t925 - f64x8::splat(0.0034437723358599895) * t917;
            let t932 = t491 * t879 * t89 / f64x8::splat(9.0) - t474 * t883 * t497 / f64x8::splat(81.0) + t930 * t239;
            let t936 = -t458 * t872 * t89 / f64x8::splat(9.0) + t474 * t872 * t476 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t487 * t490 * t932;
            let t940 = t458 * t865 * t89 / f64x8::splat(9.0) - t474 * t865 * t476 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(45.0) * t30 * t176 * t936;
            let t944 = ((t144).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t153 * t940));
            let tvtau1 = t7 * t944;
            acc_vtau_1 = tvtau1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
