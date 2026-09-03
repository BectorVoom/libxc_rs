//! MGGA_X_VT84 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_vt84.c`
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
pub fn mgga_x_vt84_vxc_pol(
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
            let t29 = v_sigma0 * v_sigma0;
            let t30 = t29 * v_sigma0;
            let t31 = v_rho0 * v_rho0;
            let t32 = t31 * v_rho0;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t30 * t33;
            let t35 = v_tau0 * v_tau0;
            let t36 = t35 * v_tau0;
            let t37 = f64x8::splat(1.0) / t36;
            let t38 = f64x8::splat(1.0) / t31;
            let t39 = t29 * t38;
            let t40 = f64x8::splat(1.0) / t35;
            let t41 = t39 * t40;
            let t43 = f64x8::splat(1.0) + t41 / f64x8::splat(64.0);
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t37 * t45;
            let t50 = f64x8::splat(M_CBRT6);
            let t51 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.00419826171875) * t34 * t46) * t50;
            let t52 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t53 = (simd::cbrt(t52));
            let t54 = t53 * t53;
            let t55 = f64x8::splat(1.0) / t54;
            let t56 = t55 * v_sigma0;
            let t57 = (simd::cbrt(v_rho0));
            let t58 = t57 * t57;
            let t60 = f64x8::splat(1.0) / t58 / t31;
            let t61 = t56 * t60;
            let t65 = f64x8::splat(1.0) / t58 / v_rho0;
            let t67 = v_sigma0 * t60;
            let t69 = v_tau0 * t65 - t67 / f64x8::splat(8.0);
            let t70 = t69 * t50;
            let t73 = f64x8::splat(5.0) / f64x8::splat(9.0) * t70 * t55 - f64x8::splat(1.0);
            let t74 = t55 * t73;
            let t77 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t70 * t74;
            let t78 = ((t77).sqrt());
            let t79 = f64x8::splat(1.0) / t78;
            let t82 = t50 * t55;
            let t83 = t82 * t67;
            let t85 = f64x8::splat(9.0) / f64x8::splat(20.0) * t73 * t79 + t83 / f64x8::splat(36.0);
            let t86 = t85 * t85;
            let t89 = t50 * t50;
            let t91 = f64x8::splat(1.0) / t53 / t52;
            let t92 = t89 * t91;
            let t93 = t31 * t31;
            let t94 = t93 * v_rho0;
            let t96 = f64x8::splat(1.0) / t57 / t94;
            let t98 = t92 * t29 * t96;
            let t100 = f64x8::splat(162.0) * t41 + f64x8::splat(50.0) * t98;
            let t101 = ((t100).sqrt());
            let t106 = t93 * t93;
            let t107 = f64x8::splat(1.0) / t106;
            let t110 = t51 * t61 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t86 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t85 * t101 + f64x8::splat(2.6505934954444615e-05) * t98 + f64x8::splat(0.0019577914932045744) * t41 + f64x8::splat(1.0930269815274441e-06) * t30 * t107;
            let t112 = f64x8::splat(1.0) + f64x8::splat(0.05873374479613724) * t83;
            let t113 = t112 * t112;
            let t114 = f64x8::splat(1.0) / t113;
            let t115 = t110 * t114;
            let t117 = (simd::exp(-f64x8::splat(0.0001863) * t115));
            let t118 = f64x8::splat(1.0) + t115;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = t117 * t119;
            let t122 = t110 * t110;
            let t123 = t113 * t113;
            let t124 = f64x8::splat(1.0) / t123;
            let t127 = (simd::exp(-f64x8::splat(0.00150903) * t122 * t124));
            let t128 = f64x8::splat(1.0) - t127;
            let t129 = f64x8::splat(1.0) / t110;
            let t132 = f64x8::splat(10.0) / f64x8::splat(81.0) * t129 * t113 - f64x8::splat(1.0);
            let t134 = t115 * t120 + t128 * t132 + f64x8::splat(1.0);
            let t138 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t134));
            let t139 = (v_rho1).simd_le(dens_threshold);
            let t140 = -t17;
            let t142 = ((t15).select(t12, (t11).select(t16, t140 * t8)));
            let t143 = f64x8::splat(1.0) + t142;
            let t144 = (t143).simd_le(zeta_threshold);
            let t145 = (simd::cbrt(t143));
            let t147 = ((t144).select(t23, t145 * t143));
            let t148 = t147 * t27;
            let t149 = v_sigma2 * v_sigma2;
            let t150 = t149 * v_sigma2;
            let t151 = v_rho1 * v_rho1;
            let t152 = t151 * v_rho1;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t150 * t153;
            let t155 = v_tau1 * v_tau1;
            let t156 = t155 * v_tau1;
            let t157 = f64x8::splat(1.0) / t156;
            let t158 = f64x8::splat(1.0) / t151;
            let t159 = t149 * t158;
            let t160 = f64x8::splat(1.0) / t155;
            let t161 = t159 * t160;
            let t163 = f64x8::splat(1.0) + t161 / f64x8::splat(64.0);
            let t164 = t163 * t163;
            let t165 = f64x8::splat(1.0) / t164;
            let t166 = t157 * t165;
            let t170 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.00419826171875) * t154 * t166) * t50;
            let t171 = t55 * v_sigma2;
            let t172 = (simd::cbrt(v_rho1));
            let t173 = t172 * t172;
            let t175 = f64x8::splat(1.0) / t173 / t151;
            let t176 = t171 * t175;
            let t180 = f64x8::splat(1.0) / t173 / v_rho1;
            let t182 = v_sigma2 * t175;
            let t184 = v_tau1 * t180 - t182 / f64x8::splat(8.0);
            let t185 = t184 * t50;
            let t188 = f64x8::splat(5.0) / f64x8::splat(9.0) * t185 * t55 - f64x8::splat(1.0);
            let t189 = t55 * t188;
            let t192 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t185 * t189;
            let t193 = ((t192).sqrt());
            let t194 = f64x8::splat(1.0) / t193;
            let t197 = t82 * t182;
            let t199 = f64x8::splat(9.0) / f64x8::splat(20.0) * t188 * t194 + t197 / f64x8::splat(36.0);
            let t200 = t199 * t199;
            let t203 = t151 * t151;
            let t204 = t203 * v_rho1;
            let t206 = f64x8::splat(1.0) / t172 / t204;
            let t208 = t92 * t149 * t206;
            let t210 = f64x8::splat(162.0) * t161 + f64x8::splat(50.0) * t208;
            let t211 = ((t210).sqrt());
            let t216 = t203 * t203;
            let t217 = f64x8::splat(1.0) / t216;
            let t220 = t170 * t176 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t200 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t199 * t211 + f64x8::splat(2.6505934954444615e-05) * t208 + f64x8::splat(0.0019577914932045744) * t161 + f64x8::splat(1.0930269815274441e-06) * t150 * t217;
            let t222 = f64x8::splat(1.0) + f64x8::splat(0.05873374479613724) * t197;
            let t223 = t222 * t222;
            let t224 = f64x8::splat(1.0) / t223;
            let t225 = t220 * t224;
            let t227 = (simd::exp(-f64x8::splat(0.0001863) * t225));
            let t228 = f64x8::splat(1.0) + t225;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t227 * t229;
            let t232 = t220 * t220;
            let t233 = t223 * t223;
            let t234 = f64x8::splat(1.0) / t233;
            let t237 = (simd::exp(-f64x8::splat(0.00150903) * t232 * t234));
            let t238 = f64x8::splat(1.0) - t237;
            let t239 = f64x8::splat(1.0) / t220;
            let t242 = f64x8::splat(10.0) / f64x8::splat(81.0) * t239 * t223 - f64x8::splat(1.0);
            let t244 = t225 * t230 + t238 * t242 + f64x8::splat(1.0);
            let t248 = ((t139).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t148 * t244));
            let tzk0 = t138 + t248;
            acc_zk = tzk0;
            let t249 = t7 * t7;
            let t250 = f64x8::splat(1.0) / t249;
            let t251 = t17 * t250;
            let t253 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t251)));
            let t256 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t253));
            let t257 = t256 * t27;
            let t261 = t27 * t27;
            let t262 = f64x8::splat(1.0) / t261;
            let t263 = t26 * t262;
            let t266 = t6 * t263 * t134 / f64x8::splat(8.0);
            let t267 = f64x8::splat(1.0) / t93;
            let t268 = t30 * t267;
            let t271 = t29 * t29;
            let t272 = t271 * v_sigma0;
            let t273 = t93 * t31;
            let t274 = f64x8::splat(1.0) / t273;
            let t275 = t272 * t274;
            let t276 = t35 * t35;
            let t277 = t276 * v_tau0;
            let t278 = f64x8::splat(1.0) / t277;
            let t280 = f64x8::splat(1.0) / t44 / t43;
            let t281 = t278 * t280;
            let t285 = (-f64x8::splat(0.01259478515625) * t268 * t46 + f64x8::splat(0.000262391357421875) * t275 * t281) * t50;
            let t289 = f64x8::splat(1.0) / t58 / t32;
            let t290 = t56 * t289;
            let t295 = v_sigma0 * t289;
            let t297 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t60 + t295 / f64x8::splat(3.0);
            let t298 = t297 * t50;
            let t299 = t55 * t79;
            let t303 = f64x8::splat(1.0) / t78 / t77;
            let t304 = t73 * t303;
            let t307 = t69 * t89;
            let t308 = t91 * t297;
            let t311 = f64x8::splat(0.2222222222222222) * t298 * t74 + f64x8::splat(0.12345679012345678) * t307 * t308;
            let t314 = t82 * t295;
            let t316 = t298 * t299 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t304 * t311 - f64x8::splat(2.0) / f64x8::splat(27.0) * t314;
            let t321 = f64x8::splat(1.0) / t101;
            let t322 = t85 * t321;
            let t323 = t29 * t33;
            let t324 = t323 * t40;
            let t327 = f64x8::splat(1.0) / t57 / t273;
            let t329 = t92 * t29 * t327;
            let t331 = -f64x8::splat(324.0) * t324 - f64x8::splat(800.0) / f64x8::splat(3.0) * t329;
            let t336 = t106 * v_rho0;
            let t337 = f64x8::splat(1.0) / t336;
            let t340 = t285 * t61 / f64x8::splat(24.0) - t51 * t290 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t85 * t316 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t316 * t101 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t322 * t331 - f64x8::splat(0.0001413649864237046) * t329 - f64x8::splat(0.003915582986409149) * t324 - f64x8::splat(8.744215852219553e-06) * t30 * t337;
            let t341 = t340 * t114;
            let t343 = t113 * t112;
            let t344 = f64x8::splat(1.0) / t343;
            let t345 = t110 * t344;
            let t346 = t345 * t120;
            let t350 = t345 * t50;
            let t351 = t350 * t290;
            let t353 = -f64x8::splat(0.0001863) * t341 - f64x8::splat(5.835784882944196e-05) * t351;
            let t354 = t353 * t117;
            let t355 = t354 * t119;
            let t357 = t118 * t118;
            let t358 = f64x8::splat(1.0) / t357;
            let t359 = t117 * t358;
            let t361 = t341 + f64x8::splat(0.3132466389127319) * t351;
            let t362 = t359 * t361;
            let t364 = t110 * t124;
            let t367 = t123 * t112;
            let t368 = f64x8::splat(1.0) / t367;
            let t369 = t122 * t368;
            let t370 = t369 * t50;
            let t373 = -f64x8::splat(0.00301806) * t364 * t340 - f64x8::splat(0.0009453971510369597) * t370 * t290;
            let t374 = t373 * t127;
            let t375 = t374 * t132;
            let t376 = f64x8::splat(1.0) / t122;
            let t377 = t376 * t113;
            let t380 = t129 * t112;
            let t381 = t380 * t50;
            let t384 = -f64x8::splat(10.0) / f64x8::splat(81.0) * t377 * t340 - f64x8::splat(0.0386724245571274) * t381 * t290;
            let t386 = t341 * t120 + f64x8::splat(0.3132466389127319) * t346 * t314 + t115 * t355 - t115 * t362 - t375 + t128 * t384;
            let t391 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t257 * t134 - t266 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t386));
            let t392 = t140 * t250;
            let t394 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t392)));
            let t397 = ((t144).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t145 * t394));
            let t398 = t397 * t27;
            let t402 = t147 * t262;
            let t405 = t6 * t402 * t244 / f64x8::splat(8.0);
            let t407 = ((t139).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t398 * t244 - t405));
            let tvrho0 = t138 + t248 + t7 * (t391 + t407);
            acc_vrho_0 = tvrho0;
            let t411 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t251)));
            let t414 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t411));
            let t415 = t414 * t27;
            let t420 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t415 * t134 - t266));
            let t422 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t392)));
            let t425 = ((t144).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t145 * t422));
            let t426 = t425 * t27;
            let t430 = f64x8::splat(1.0) / t203;
            let t431 = t150 * t430;
            let t434 = t149 * t149;
            let t435 = t434 * v_sigma2;
            let t436 = t203 * t151;
            let t437 = f64x8::splat(1.0) / t436;
            let t438 = t435 * t437;
            let t439 = t155 * t155;
            let t440 = t439 * v_tau1;
            let t441 = f64x8::splat(1.0) / t440;
            let t443 = f64x8::splat(1.0) / t164 / t163;
            let t444 = t441 * t443;
            let t448 = (-f64x8::splat(0.01259478515625) * t431 * t166 + f64x8::splat(0.000262391357421875) * t438 * t444) * t50;
            let t452 = f64x8::splat(1.0) / t173 / t152;
            let t453 = t171 * t452;
            let t458 = v_sigma2 * t452;
            let t460 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t175 + t458 / f64x8::splat(3.0);
            let t461 = t460 * t50;
            let t462 = t55 * t194;
            let t466 = f64x8::splat(1.0) / t193 / t192;
            let t467 = t188 * t466;
            let t470 = t184 * t89;
            let t471 = t91 * t460;
            let t474 = f64x8::splat(0.2222222222222222) * t461 * t189 + f64x8::splat(0.12345679012345678) * t470 * t471;
            let t477 = t82 * t458;
            let t479 = t461 * t462 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t467 * t474 - f64x8::splat(2.0) / f64x8::splat(27.0) * t477;
            let t484 = f64x8::splat(1.0) / t211;
            let t485 = t199 * t484;
            let t486 = t149 * t153;
            let t487 = t486 * t160;
            let t490 = f64x8::splat(1.0) / t172 / t436;
            let t492 = t92 * t149 * t490;
            let t494 = -f64x8::splat(324.0) * t487 - f64x8::splat(800.0) / f64x8::splat(3.0) * t492;
            let t499 = t216 * v_rho1;
            let t500 = f64x8::splat(1.0) / t499;
            let t503 = t448 * t176 / f64x8::splat(24.0) - t170 * t453 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t199 * t479 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t479 * t211 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t485 * t494 - f64x8::splat(0.0001413649864237046) * t492 - f64x8::splat(0.003915582986409149) * t487 - f64x8::splat(8.744215852219553e-06) * t150 * t500;
            let t504 = t503 * t224;
            let t506 = t223 * t222;
            let t507 = f64x8::splat(1.0) / t506;
            let t508 = t220 * t507;
            let t509 = t508 * t230;
            let t513 = t508 * t50;
            let t514 = t513 * t453;
            let t516 = -f64x8::splat(0.0001863) * t504 - f64x8::splat(5.835784882944196e-05) * t514;
            let t517 = t516 * t227;
            let t518 = t517 * t229;
            let t520 = t228 * t228;
            let t521 = f64x8::splat(1.0) / t520;
            let t522 = t227 * t521;
            let t524 = t504 + f64x8::splat(0.3132466389127319) * t514;
            let t525 = t522 * t524;
            let t527 = t220 * t234;
            let t530 = t233 * t222;
            let t531 = f64x8::splat(1.0) / t530;
            let t532 = t232 * t531;
            let t533 = t532 * t50;
            let t536 = -f64x8::splat(0.00301806) * t527 * t503 - f64x8::splat(0.0009453971510369597) * t533 * t453;
            let t537 = t536 * t237;
            let t538 = t537 * t242;
            let t539 = f64x8::splat(1.0) / t232;
            let t540 = t539 * t223;
            let t543 = t239 * t222;
            let t544 = t543 * t50;
            let t547 = -f64x8::splat(10.0) / f64x8::splat(81.0) * t540 * t503 - f64x8::splat(0.0386724245571274) * t544 * t453;
            let t549 = t504 * t230 + f64x8::splat(0.3132466389127319) * t509 * t477 + t225 * t518 - t225 * t525 - t538 + t238 * t547;
            let t554 = ((t139).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t426 * t244 - t405 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t148 * t549));
            let tvrho1 = t138 + t248 + t7 * (t420 + t554);
            acc_vrho_1 = tvrho1;
            let t559 = f64x8::splat(1.0) / t94;
            let t560 = t271 * t559;
            let t564 = (f64x8::splat(0.01259478515625) * t323 * t46 - f64x8::splat(0.000262391357421875) * t560 * t281) * t50;
            let t567 = t55 * t60;
            let t570 = t60 * t50;
            let t571 = t570 * t299;
            let t573 = t570 * t74;
            let t576 = t307 * t91 * t60;
            let t578 = -f64x8::splat(0.027777777777777776) * t573 - f64x8::splat(0.015432098765432098) * t576;
            let t581 = t570 * t55;
            let t583 = -t571 / f64x8::splat(32.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t304 * t578 + t581 / f64x8::splat(36.0);
            let t588 = v_sigma0 * t38;
            let t589 = t588 * t40;
            let t592 = t92 * v_sigma0 * t96;
            let t594 = f64x8::splat(324.0) * t589 + f64x8::splat(100.0) * t592;
            let t601 = t564 * t61 / f64x8::splat(24.0) + t51 * t567 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t85 * t583 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t583 * t101 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t322 * t594 + f64x8::splat(5.301186990888923e-05) * t592 + f64x8::splat(0.003915582986409149) * t589 + f64x8::splat(3.2790809445823326e-06) * t29 * t107;
            let t602 = t601 * t114;
            let t604 = t345 * t117;
            let t605 = t119 * t60;
            let t606 = t605 * t82;
            let t610 = t345 * t581;
            let t612 = -f64x8::splat(0.0001863) * t602 + f64x8::splat(2.1884193311040734e-05) * t610;
            let t613 = t612 * t117;
            let t614 = t613 * t119;
            let t617 = t602 - f64x8::splat(0.11746748959227447) * t610;
            let t618 = t359 * t617;
            let t624 = -f64x8::splat(0.00301806) * t364 * t601 + f64x8::splat(0.0003545239316388599) * t369 * t581;
            let t625 = t624 * t127;
            let t626 = t625 * t132;
            let t631 = -f64x8::splat(10.0) / f64x8::splat(81.0) * t377 * t601 + f64x8::splat(0.014502159208922774) * t380 * t581;
            let t633 = t602 * t120 - f64x8::splat(0.11746748959227447) * t604 * t606 + t115 * t614 - t115 * t618 - t626 + t128 * t631;
            let t637 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t633));
            let tvsigma0 = t7 * t637;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t640 = f64x8::splat(1.0) / t204;
            let t641 = t434 * t640;
            let t645 = (f64x8::splat(0.01259478515625) * t486 * t166 - f64x8::splat(0.000262391357421875) * t641 * t444) * t50;
            let t648 = t55 * t175;
            let t651 = t175 * t50;
            let t652 = t651 * t462;
            let t654 = t651 * t189;
            let t657 = t470 * t91 * t175;
            let t659 = -f64x8::splat(0.027777777777777776) * t654 - f64x8::splat(0.015432098765432098) * t657;
            let t662 = t651 * t55;
            let t664 = -t652 / f64x8::splat(32.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t467 * t659 + t662 / f64x8::splat(36.0);
            let t669 = v_sigma2 * t158;
            let t670 = t669 * t160;
            let t673 = t92 * v_sigma2 * t206;
            let t675 = f64x8::splat(324.0) * t670 + f64x8::splat(100.0) * t673;
            let t682 = t645 * t176 / f64x8::splat(24.0) + t170 * t648 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t199 * t664 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t664 * t211 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t485 * t675 + f64x8::splat(5.301186990888923e-05) * t673 + f64x8::splat(0.003915582986409149) * t670 + f64x8::splat(3.2790809445823326e-06) * t149 * t217;
            let t683 = t682 * t224;
            let t685 = t508 * t227;
            let t686 = t229 * t175;
            let t687 = t686 * t82;
            let t691 = t508 * t662;
            let t693 = -f64x8::splat(0.0001863) * t683 + f64x8::splat(2.1884193311040734e-05) * t691;
            let t694 = t693 * t227;
            let t695 = t694 * t229;
            let t698 = t683 - f64x8::splat(0.11746748959227447) * t691;
            let t699 = t522 * t698;
            let t705 = -f64x8::splat(0.00301806) * t527 * t682 + f64x8::splat(0.0003545239316388599) * t532 * t662;
            let t706 = t705 * t237;
            let t707 = t706 * t242;
            let t712 = -f64x8::splat(10.0) / f64x8::splat(81.0) * t540 * t682 + f64x8::splat(0.014502159208922774) * t543 * t662;
            let t714 = t683 * t230 - f64x8::splat(0.11746748959227447) * t685 * t687 + t225 * t695 - t225 * t699 - t707 + t238 * t712;
            let t718 = ((t139).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t148 * t714));
            let tvsigma2 = t7 * t718;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t719 = f64x8::splat(1.0) / t276;
            let t720 = t719 * t45;
            let t723 = t272 * t559;
            let t724 = t276 * t35;
            let t725 = f64x8::splat(1.0) / t724;
            let t726 = t725 * t280;
            let t730 = (-f64x8::splat(0.01259478515625) * t34 * t720 + f64x8::splat(0.000262391357421875) * t723 * t726) * t50;
            let t733 = t65 * t50;
            let t741 = f64x8::splat(0.2222222222222222) * t733 * t74 + f64x8::splat(0.12345679012345678) * t307 * t91 * t65;
            let t744 = t733 * t299 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t304 * t741;
            let t749 = t39 * t37;
            let t753 = t730 * t61 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t85 * t744 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t744 * t101 + f64x8::splat(73.0) / f64x8::splat(600.0) * t322 * t749 - f64x8::splat(0.003915582986409149) * t749;
            let t754 = t753 * t114;
            let t756 = t753 * t117;
            let t757 = t756 * t119;
            let t760 = t359 * t753;
            let t762 = t753 * t127;
            let t763 = t762 * t132;
            let t766 = t128 * t376;
            let t767 = t113 * t753;
            let t770 = t754 * t120 - f64x8::splat(0.0001863) * t364 * t757 - t364 * t760 + f64x8::splat(0.00301806) * t364 * t763 - f64x8::splat(10.0) / f64x8::splat(81.0) * t766 * t767;
            let t774 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t770));
            let tvtau0 = t7 * t774;
            acc_vtau_0 = tvtau0;
            let t775 = f64x8::splat(1.0) / t439;
            let t776 = t775 * t165;
            let t779 = t435 * t640;
            let t780 = t439 * t155;
            let t781 = f64x8::splat(1.0) / t780;
            let t782 = t781 * t443;
            let t786 = (-f64x8::splat(0.01259478515625) * t154 * t776 + f64x8::splat(0.000262391357421875) * t779 * t782) * t50;
            let t789 = t180 * t50;
            let t797 = f64x8::splat(0.2222222222222222) * t789 * t189 + f64x8::splat(0.12345679012345678) * t470 * t91 * t180;
            let t800 = t789 * t462 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(40.0) * t467 * t797;
            let t805 = t159 * t157;
            let t809 = t786 * t176 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t199 * t800 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t800 * t211 + f64x8::splat(73.0) / f64x8::splat(600.0) * t485 * t805 - f64x8::splat(0.003915582986409149) * t805;
            let t810 = t809 * t224;
            let t812 = t809 * t227;
            let t813 = t812 * t229;
            let t816 = t522 * t809;
            let t818 = t809 * t237;
            let t819 = t818 * t242;
            let t822 = t238 * t539;
            let t823 = t223 * t809;
            let t826 = t810 * t230 - f64x8::splat(0.0001863) * t527 * t813 - t527 * t816 + f64x8::splat(0.00301806) * t527 * t819 - f64x8::splat(10.0) / f64x8::splat(81.0) * t822 * t823;
            let t830 = ((t139).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t148 * t826));
            let tvtau1 = t7 * t830;
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
