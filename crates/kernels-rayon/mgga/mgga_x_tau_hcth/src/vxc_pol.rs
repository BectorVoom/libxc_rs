//! MGGA_X_TAU_HCTH vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tau_hcth.c`
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
pub fn mgga_x_tau_hcth_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_cx_local_1: f64,
    param_cx_local_2: f64,
    param_cx_local_3: f64,
    param_cx_nlocal_1: f64,
    param_cx_nlocal_2: f64,
    param_cx_nlocal_3: f64,
    param_cx_nlocal_0: f64,
    param_cx_local_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_cx_local_1 = f64x8::splat(param_cx_local_1);
    let param_cx_local_2 = f64x8::splat(param_cx_local_2);
    let param_cx_local_3 = f64x8::splat(param_cx_local_3);
    let param_cx_nlocal_1 = f64x8::splat(param_cx_nlocal_1);
    let param_cx_nlocal_2 = f64x8::splat(param_cx_nlocal_2);
    let param_cx_nlocal_3 = f64x8::splat(param_cx_nlocal_3);
    let param_cx_nlocal_0 = f64x8::splat(param_cx_nlocal_0);
    let param_cx_local_0 = f64x8::splat(param_cx_local_0);
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
            let t29 = param_cx_local_0;
            let t30 = param_cx_local_1;
            let t31 = t30 * v_sigma0;
            let t32 = v_rho0 * v_rho0;
            let t33 = (simd::cbrt(v_rho0));
            let t34 = t33 * t33;
            let t36 = f64x8::splat(1.0) / t34 / t32;
            let t39 = f64x8::splat(1.0) + f64x8::splat(0.004) * v_sigma0 * t36;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t36 * t40;
            let t44 = param_cx_local_2;
            let t45 = v_sigma0 * v_sigma0;
            let t46 = t44 * t45;
            let t47 = t32 * t32;
            let t48 = t47 * v_rho0;
            let t50 = f64x8::splat(1.0) / t33 / t48;
            let t51 = t39 * t39;
            let t52 = f64x8::splat(1.0) / t51;
            let t53 = t50 * t52;
            let t56 = param_cx_local_3;
            let t57 = t45 * v_sigma0;
            let t58 = t56 * t57;
            let t59 = t47 * t47;
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t51 * t39;
            let t62 = f64x8::splat(1.0) / t61;
            let t63 = t60 * t62;
            let t66 = param_cx_nlocal_0;
            let t67 = param_cx_nlocal_1;
            let t68 = t67 * v_sigma0;
            let t71 = param_cx_nlocal_2;
            let t72 = t71 * t45;
            let t75 = param_cx_nlocal_3;
            let t76 = t75 * t57;
            let t79 = t66 + f64x8::splat(0.004) * t68 * t41 + f64x8::splat(1.6e-05) * t72 * t53 + f64x8::splat(6.4e-08) * t76 * t63;
            let t80 = f64x8::splat(M_CBRT6);
            let t81 = t80 * t80;
            let t82 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t83 = (simd::cbrt(t82));
            let t84 = t83 * t83;
            let t86 = f64x8::splat(3.0) / f64x8::splat(10.0) * t81 * t84;
            let t88 = f64x8::splat(1.0) / t34 / v_rho0;
            let t89 = v_tau0 * t88;
            let t90 = t86 - t89;
            let t91 = t86 + t89;
            let t92 = f64x8::splat(1.0) / t91;
            let t94 = t90 * t90;
            let t95 = t94 * t90;
            let t96 = t91 * t91;
            let t97 = t96 * t91;
            let t98 = f64x8::splat(1.0) / t97;
            let t101 = t94 * t94;
            let t102 = t101 * t90;
            let t103 = t96 * t96;
            let t105 = f64x8::splat(1.0) / t103 / t91;
            let t107 = t102 * t105 + t90 * t92 - f64x8::splat(2.0) * t95 * t98;
            let t109 = t29 + f64x8::splat(0.004) * t31 * t41 + f64x8::splat(1.6e-05) * t46 * t53 + f64x8::splat(6.4e-08) * t58 * t63 + t79 * t107;
            let t113 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t109));
            let t114 = (v_rho1).simd_le(dens_threshold);
            let t115 = -t17;
            let t117 = ((t15).select(t12, (t11).select(t16, t115 * t8)));
            let t118 = f64x8::splat(1.0) + t117;
            let t119 = (t118).simd_le(zeta_threshold);
            let t120 = (simd::cbrt(t118));
            let t122 = ((t119).select(t23, t120 * t118));
            let t123 = t122 * t27;
            let t124 = t30 * v_sigma2;
            let t125 = v_rho1 * v_rho1;
            let t126 = (simd::cbrt(v_rho1));
            let t127 = t126 * t126;
            let t129 = f64x8::splat(1.0) / t127 / t125;
            let t132 = f64x8::splat(1.0) + f64x8::splat(0.004) * v_sigma2 * t129;
            let t133 = f64x8::splat(1.0) / t132;
            let t134 = t129 * t133;
            let t137 = v_sigma2 * v_sigma2;
            let t138 = t44 * t137;
            let t139 = t125 * t125;
            let t140 = t139 * v_rho1;
            let t142 = f64x8::splat(1.0) / t126 / t140;
            let t143 = t132 * t132;
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t142 * t144;
            let t148 = t137 * v_sigma2;
            let t149 = t56 * t148;
            let t150 = t139 * t139;
            let t151 = f64x8::splat(1.0) / t150;
            let t152 = t143 * t132;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t151 * t153;
            let t157 = t67 * v_sigma2;
            let t160 = t71 * t137;
            let t163 = t75 * t148;
            let t166 = t66 + f64x8::splat(0.004) * t157 * t134 + f64x8::splat(1.6e-05) * t160 * t145 + f64x8::splat(6.4e-08) * t163 * t154;
            let t168 = f64x8::splat(1.0) / t127 / v_rho1;
            let t169 = v_tau1 * t168;
            let t170 = t86 - t169;
            let t171 = t86 + t169;
            let t172 = f64x8::splat(1.0) / t171;
            let t174 = t170 * t170;
            let t175 = t174 * t170;
            let t176 = t171 * t171;
            let t177 = t176 * t171;
            let t178 = f64x8::splat(1.0) / t177;
            let t181 = t174 * t174;
            let t182 = t181 * t170;
            let t183 = t176 * t176;
            let t185 = f64x8::splat(1.0) / t183 / t171;
            let t187 = t170 * t172 - f64x8::splat(2.0) * t175 * t178 + t182 * t185;
            let t189 = t29 + f64x8::splat(0.004) * t124 * t134 + f64x8::splat(1.6e-05) * t138 * t145 + f64x8::splat(6.4e-08) * t149 * t154 + t166 * t187;
            let t193 = ((t114).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t123 * t189));
            let tzk0 = t113 + t193;
            acc_zk = tzk0;
            let t194 = t7 * t7;
            let t195 = f64x8::splat(1.0) / t194;
            let t196 = t17 * t195;
            let t198 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t196)));
            let t201 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t198));
            let t202 = t201 * t27;
            let t206 = t27 * t27;
            let t207 = f64x8::splat(1.0) / t206;
            let t208 = t26 * t207;
            let t211 = t6 * t208 * t109 / f64x8::splat(8.0);
            let t212 = t32 * v_rho0;
            let t214 = f64x8::splat(1.0) / t34 / t212;
            let t215 = t214 * t40;
            let t218 = t30 * t45;
            let t219 = t47 * t32;
            let t221 = f64x8::splat(1.0) / t33 / t219;
            let t222 = t221 * t52;
            let t227 = t44 * t57;
            let t228 = t59 * v_rho0;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t229 * t62;
            let t235 = t45 * t45;
            let t236 = t56 * t235;
            let t237 = t59 * t212;
            let t239 = f64x8::splat(1.0) / t34 / t237;
            let t240 = t51 * t51;
            let t241 = f64x8::splat(1.0) / t240;
            let t242 = t239 * t241;
            let t247 = t67 * t45;
            let t252 = t71 * t57;
            let t257 = t75 * t235;
            let t260 = -f64x8::splat(0.010666666666666666) * t68 * t215 + f64x8::splat(4.266666666666667e-05) * t247 * t222 - f64x8::splat(8.533333333333334e-05) * t72 * t222 + f64x8::splat(3.413333333333333e-07) * t252 * t230 - f64x8::splat(5.12e-07) * t76 * t230 + f64x8::splat(2.048e-09) * t257 * t242;
            let t262 = v_tau0 * t36;
            let t265 = f64x8::splat(1.0) / t96;
            let t266 = t90 * t265;
            let t269 = t94 * t98;
            let t272 = f64x8::splat(1.0) / t103;
            let t273 = t95 * t272;
            let t276 = t101 * t105;
            let t280 = f64x8::splat(1.0) / t103 / t96;
            let t281 = t102 * t280;
            let t284 = f64x8::splat(5.0) / f64x8::splat(3.0) * t262 * t92 + f64x8::splat(5.0) / f64x8::splat(3.0) * t266 * t262 - f64x8::splat(10.0) * t269 * t262 - f64x8::splat(10.0) * t273 * t262 + f64x8::splat(25.0) / f64x8::splat(3.0) * t276 * t262 + f64x8::splat(25.0) / f64x8::splat(3.0) * t281 * t262;
            let t286 = -f64x8::splat(0.010666666666666666) * t31 * t215 + f64x8::splat(4.266666666666667e-05) * t218 * t222 - f64x8::splat(8.533333333333334e-05) * t46 * t222 + f64x8::splat(3.413333333333333e-07) * t227 * t230 - f64x8::splat(5.12e-07) * t58 * t230 + f64x8::splat(2.048e-09) * t236 * t242 + t260 * t107 + t79 * t284;
            let t291 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t202 * t109 - t211 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t286));
            let t292 = t115 * t195;
            let t294 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t292)));
            let t297 = ((t119).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t120 * t294));
            let t298 = t297 * t27;
            let t302 = t122 * t207;
            let t305 = t6 * t302 * t189 / f64x8::splat(8.0);
            let t307 = ((t114).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t298 * t189 - t305));
            let tvrho0 = t113 + t193 + t7 * (t291 + t307);
            acc_vrho_0 = tvrho0;
            let t311 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t196)));
            let t314 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t311));
            let t315 = t314 * t27;
            let t320 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t315 * t109 - t211));
            let t322 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t292)));
            let t325 = ((t119).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t120 * t322));
            let t326 = t325 * t27;
            let t330 = t125 * v_rho1;
            let t332 = f64x8::splat(1.0) / t127 / t330;
            let t333 = t332 * t133;
            let t336 = t30 * t137;
            let t337 = t139 * t125;
            let t339 = f64x8::splat(1.0) / t126 / t337;
            let t340 = t339 * t144;
            let t345 = t44 * t148;
            let t346 = t150 * v_rho1;
            let t347 = f64x8::splat(1.0) / t346;
            let t348 = t347 * t153;
            let t353 = t137 * t137;
            let t354 = t56 * t353;
            let t355 = t150 * t330;
            let t357 = f64x8::splat(1.0) / t127 / t355;
            let t358 = t143 * t143;
            let t359 = f64x8::splat(1.0) / t358;
            let t360 = t357 * t359;
            let t365 = t67 * t137;
            let t370 = t71 * t148;
            let t375 = t75 * t353;
            let t378 = -f64x8::splat(0.010666666666666666) * t157 * t333 + f64x8::splat(4.266666666666667e-05) * t365 * t340 - f64x8::splat(8.533333333333334e-05) * t160 * t340 + f64x8::splat(3.413333333333333e-07) * t370 * t348 - f64x8::splat(5.12e-07) * t163 * t348 + f64x8::splat(2.048e-09) * t375 * t360;
            let t380 = v_tau1 * t129;
            let t383 = f64x8::splat(1.0) / t176;
            let t384 = t170 * t383;
            let t387 = t174 * t178;
            let t390 = f64x8::splat(1.0) / t183;
            let t391 = t175 * t390;
            let t394 = t181 * t185;
            let t398 = f64x8::splat(1.0) / t183 / t176;
            let t399 = t182 * t398;
            let t402 = f64x8::splat(5.0) / f64x8::splat(3.0) * t380 * t172 + f64x8::splat(5.0) / f64x8::splat(3.0) * t384 * t380 - f64x8::splat(10.0) * t387 * t380 - f64x8::splat(10.0) * t391 * t380 + f64x8::splat(25.0) / f64x8::splat(3.0) * t394 * t380 + f64x8::splat(25.0) / f64x8::splat(3.0) * t399 * t380;
            let t404 = -f64x8::splat(0.010666666666666666) * t124 * t333 + f64x8::splat(4.266666666666667e-05) * t336 * t340 - f64x8::splat(8.533333333333334e-05) * t138 * t340 + f64x8::splat(3.413333333333333e-07) * t345 * t348 - f64x8::splat(5.12e-07) * t149 * t348 + f64x8::splat(2.048e-09) * t354 * t360 + t378 * t187 + t166 * t402;
            let t409 = ((t114).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t326 * t189 - t305 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t123 * t404));
            let tvrho1 = t113 + t193 + t7 * (t320 + t409);
            acc_vrho_1 = tvrho1;
            let t417 = t44 * v_sigma0;
            let t422 = t56 * t45;
            let t425 = t59 * t32;
            let t427 = f64x8::splat(1.0) / t34 / t425;
            let t428 = t427 * t241;
            let t436 = t71 * v_sigma0;
            let t441 = t75 * t45;
            let t446 = f64x8::splat(0.004) * t67 * t36 * t40 - f64x8::splat(1.6e-05) * t68 * t53 + f64x8::splat(3.2e-05) * t436 * t53 - f64x8::splat(1.28e-07) * t72 * t63 + f64x8::splat(1.92e-07) * t441 * t63 - f64x8::splat(7.68e-10) * t76 * t428;
            let t448 = f64x8::splat(0.004) * t30 * t36 * t40 - f64x8::splat(1.6e-05) * t31 * t53 + f64x8::splat(3.2e-05) * t417 * t53 - f64x8::splat(1.28e-07) * t46 * t63 + f64x8::splat(1.92e-07) * t422 * t63 - f64x8::splat(7.68e-10) * t58 * t428 + t446 * t107;
            let t452 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t448));
            let tvsigma0 = t7 * t452;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t458 = t44 * v_sigma2;
            let t463 = t56 * t137;
            let t466 = t150 * t125;
            let t468 = f64x8::splat(1.0) / t127 / t466;
            let t469 = t468 * t359;
            let t477 = t71 * v_sigma2;
            let t482 = t75 * t137;
            let t487 = f64x8::splat(0.004) * t67 * t129 * t133 - f64x8::splat(1.6e-05) * t157 * t145 + f64x8::splat(3.2e-05) * t477 * t145 - f64x8::splat(1.28e-07) * t160 * t154 + f64x8::splat(1.92e-07) * t482 * t154 - f64x8::splat(7.68e-10) * t163 * t469;
            let t489 = f64x8::splat(0.004) * t30 * t129 * t133 - f64x8::splat(1.6e-05) * t124 * t145 + f64x8::splat(3.2e-05) * t458 * t145 - f64x8::splat(1.28e-07) * t138 * t154 + f64x8::splat(1.92e-07) * t463 * t154 - f64x8::splat(7.68e-10) * t149 * t469 + t487 * t187;
            let t493 = ((t114).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t123 * t489));
            let tvsigma2 = t7 * t493;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t494 = t6 * t26;
            let t495 = t27 * t79;
            let t506 = -t266 * t88 + f64x8::splat(6.0) * t269 * t88 + f64x8::splat(6.0) * t273 * t88 - f64x8::splat(5.0) * t276 * t88 - f64x8::splat(5.0) * t281 * t88 - t88 * t92;
            let t507 = t495 * t506;
            let t510 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t494 * t507));
            let tvtau0 = t7 * t510;
            acc_vtau_0 = tvtau0;
            let t511 = t6 * t122;
            let t512 = t27 * t166;
            let t523 = -t168 * t172 - t384 * t168 + f64x8::splat(6.0) * t387 * t168 + f64x8::splat(6.0) * t391 * t168 - f64x8::splat(5.0) * t394 * t168 - f64x8::splat(5.0) * t399 * t168;
            let t524 = t512 * t523;
            let t527 = ((t114).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t511 * t524));
            let tvtau1 = t7 * t527;
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
