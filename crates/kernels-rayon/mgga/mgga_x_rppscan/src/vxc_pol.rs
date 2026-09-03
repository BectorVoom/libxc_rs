//! MGGA_X_RPPSCAN vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rppscan.c`
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
pub fn mgga_x_rppscan_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c2: f64,
    param_d: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_eta = f64x8::splat(param_eta);
    let param_k1 = f64x8::splat(param_k1);
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
            let t27 = t6 * t26;
            let t28 = (simd::cbrt(t7));
            let t29 = f64x8::splat(M_CBRT6);
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t29 * t33;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t38 = t37 * t35;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = v_sigma0 * t39;
            let t41 = t34 * t40;
            let t45 = f64x8::splat(100.0) / f64x8::splat(6561.0) / param_k1 - f64x8::splat(73.0) / f64x8::splat(648.0);
            let t46 = t29 * t29;
            let t47 = t45 * t46;
            let t48 = t31 * t30;
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = t47 * t49;
            let t51 = v_sigma0 * v_sigma0;
            let t52 = t35 * t35;
            let t53 = t52 * v_rho0;
            let t55 = f64x8::splat(1.0) / t36 / t53;
            let t56 = t51 * t55;
            let t57 = t45 * t29;
            let t58 = t33 * v_sigma0;
            let t59 = t58 * t39;
            let t62 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t57 * t59));
            let t66 = ((f64x8::splat(146.0)).sqrt());
            let t67 = t66 * t29;
            let t70 = t37 * v_rho0;
            let t71 = f64x8::splat(1.0) / t70;
            let t74 = v_tau0 * t71 - t40 / f64x8::splat(8.0);
            let t76 = f64x8::splat(3.0) / f64x8::splat(10.0) * t46 * t32;
            let t77 = param_eta * v_sigma0;
            let t80 = t76 + t77 * t39 / f64x8::splat(8.0);
            let t81 = f64x8::splat(1.0) / t80;
            let t82 = t74 * t81;
            let t83 = f64x8::splat(1.0) - t82;
            let t85 = t83 * t83;
            let t87 = (simd::exp(-t85 / f64x8::splat(2.0)));
            let t90 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t59 + t66 * t83 * t87 / f64x8::splat(100.0);
            let t91 = t90 * t90;
            let t92 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t41 + t50 * t56 * t62 / f64x8::splat(576.0) + t91;
            let t97 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t92);
            let t98 = (t82).simd_le(f64x8::splat(2.5));
            let t99 = (f64x8::splat(2.5)).simd_lt(t82);
            let t100 = ((t99).select(f64x8::splat(2.5), t82));
            let t102 = t100 * t100;
            let t104 = t102 * t100;
            let t106 = t102 * t102;
            let t108 = t106 * t100;
            let t110 = t106 * t102;
            let t115 = ((t99).select(t82, f64x8::splat(2.5)));
            let t116 = f64x8::splat(1.0) - t115;
            let t119 = (simd::exp(param_c2 / t116));
            let t121 = ((t98).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t100 - f64x8::splat(0.4445555) * t102 - f64x8::splat(0.663086601049) * t104 + f64x8::splat(1.45129704449) * t106 - f64x8::splat(0.887998041597) * t108 + f64x8::splat(0.234528941479) * t110 - f64x8::splat(0.023185843322) * t106 * t104, -param_d * t119));
            let t122 = f64x8::splat(1.0) - t121;
            let t125 = t97 * t122 + f64x8::splat(1.174) * t121;
            let t126 = t28 * t125;
            let t127 = ((f64x8::splat(3.0)).sqrt());
            let t128 = f64x8::splat(1.0) / t31;
            let t129 = t46 * t128;
            let t130 = ((v_sigma0).sqrt());
            let t131 = t36 * v_rho0;
            let t132 = f64x8::splat(1.0) / t131;
            let t134 = t129 * t130 * t132;
            let t135 = ((t134).sqrt());
            let t139 = (simd::exp(-f64x8::splat(9.8958) * t127 / t135));
            let t140 = f64x8::splat(1.0) - t139;
            let t141 = t126 * t140;
            let t144 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t141));
            let t145 = (v_rho1).simd_le(dens_threshold);
            let t146 = -t17;
            let t148 = ((t15).select(t12, (t11).select(t16, t146 * t8)));
            let t149 = f64x8::splat(1.0) + t148;
            let t150 = (t149).simd_le(zeta_threshold);
            let t151 = (simd::cbrt(t149));
            let t153 = ((t150).select(t23, t151 * t149));
            let t154 = t6 * t153;
            let t155 = v_rho1 * v_rho1;
            let t156 = (simd::cbrt(v_rho1));
            let t157 = t156 * t156;
            let t158 = t157 * t155;
            let t159 = f64x8::splat(1.0) / t158;
            let t160 = v_sigma2 * t159;
            let t161 = t34 * t160;
            let t163 = v_sigma2 * v_sigma2;
            let t164 = t155 * t155;
            let t165 = t164 * v_rho1;
            let t167 = f64x8::splat(1.0) / t156 / t165;
            let t168 = t163 * t167;
            let t169 = t33 * v_sigma2;
            let t170 = t169 * t159;
            let t173 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t57 * t170));
            let t179 = t157 * v_rho1;
            let t180 = f64x8::splat(1.0) / t179;
            let t183 = v_tau1 * t180 - t160 / f64x8::splat(8.0);
            let t184 = param_eta * v_sigma2;
            let t187 = t76 + t184 * t159 / f64x8::splat(8.0);
            let t188 = f64x8::splat(1.0) / t187;
            let t189 = t183 * t188;
            let t190 = f64x8::splat(1.0) - t189;
            let t192 = t190 * t190;
            let t194 = (simd::exp(-t192 / f64x8::splat(2.0)));
            let t197 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t170 + t66 * t190 * t194 / f64x8::splat(100.0);
            let t198 = t197 * t197;
            let t199 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t161 + t50 * t168 * t173 / f64x8::splat(576.0) + t198;
            let t204 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t199);
            let t205 = (t189).simd_le(f64x8::splat(2.5));
            let t206 = (f64x8::splat(2.5)).simd_lt(t189);
            let t207 = ((t206).select(f64x8::splat(2.5), t189));
            let t209 = t207 * t207;
            let t211 = t209 * t207;
            let t213 = t209 * t209;
            let t215 = t213 * t207;
            let t217 = t213 * t209;
            let t222 = ((t206).select(t189, f64x8::splat(2.5)));
            let t223 = f64x8::splat(1.0) - t222;
            let t226 = (simd::exp(param_c2 / t223));
            let t228 = ((t205).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t207 - f64x8::splat(0.4445555) * t209 - f64x8::splat(0.663086601049) * t211 + f64x8::splat(1.45129704449) * t213 - f64x8::splat(0.887998041597) * t215 + f64x8::splat(0.234528941479) * t217 - f64x8::splat(0.023185843322) * t213 * t211, -param_d * t226));
            let t229 = f64x8::splat(1.0) - t228;
            let t232 = t204 * t229 + f64x8::splat(1.174) * t228;
            let t233 = t28 * t232;
            let t234 = ((v_sigma2).sqrt());
            let t235 = t156 * v_rho1;
            let t236 = f64x8::splat(1.0) / t235;
            let t238 = t129 * t234 * t236;
            let t239 = ((t238).sqrt());
            let t243 = (simd::exp(-f64x8::splat(9.8958) * t127 / t239));
            let t244 = f64x8::splat(1.0) - t243;
            let t245 = t233 * t244;
            let t248 = ((t145).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t154 * t245));
            let tzk0 = t144 + t248;
            acc_zk = tzk0;
            let t249 = t7 * t7;
            let t250 = f64x8::splat(1.0) / t249;
            let t251 = t17 * t250;
            let t253 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t251)));
            let t256 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t253));
            let t257 = t6 * t256;
            let t260 = t28 * t28;
            let t261 = f64x8::splat(1.0) / t260;
            let t262 = t261 * t125;
            let t263 = t262 * t140;
            let t265 = t27 * t263 / f64x8::splat(8.0);
            let t266 = param_k1 * param_k1;
            let t267 = t92 * t92;
            let t269 = t266 / t267;
            let t270 = t35 * v_rho0;
            let t272 = f64x8::splat(1.0) / t37 / t270;
            let t273 = v_sigma0 * t272;
            let t276 = t52 * t35;
            let t278 = f64x8::splat(1.0) / t36 / t276;
            let t283 = t45 * t45;
            let t284 = t30 * t30;
            let t285 = f64x8::splat(1.0) / t284;
            let t286 = t283 * t285;
            let t287 = t51 * v_sigma0;
            let t288 = t52 * t52;
            let t289 = t288 * v_rho0;
            let t290 = f64x8::splat(1.0) / t289;
            let t301 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t39 + t273 / f64x8::splat(3.0);
            let t303 = t80 * t80;
            let t304 = f64x8::splat(1.0) / t303;
            let t305 = t74 * t304;
            let t306 = t77 * t272;
            let t309 = -t301 * t81 - t305 * t306 / f64x8::splat(3.0);
            let t313 = t66 * t85;
            let t314 = t309 * t87;
            let t317 = -f64x8::splat(7.0) / f64x8::splat(4860.0) * t67 * t58 * t272 + t66 * t309 * t87 / f64x8::splat(100.0) - t313 * t314 / f64x8::splat(100.0);
            let t320 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t34 * t273 - t50 * t51 * t278 * t62 / f64x8::splat(108.0) + f64x8::splat(3.0) / f64x8::splat(320.0) * t286 * t287 * t290 * t62 + f64x8::splat(2.0) * t90 * t317;
            let t321 = t320 * t122;
            let t323 = -t309;
            let t324 = ((t99).select(f64x8::splat(0.0), t323));
            let t326 = t100 * t324;
            let t328 = t102 * t324;
            let t330 = t104 * t324;
            let t332 = t106 * t324;
            let t334 = t108 * t324;
            let t339 = param_d * param_c2;
            let t340 = t116 * t116;
            let t341 = f64x8::splat(1.0) / t340;
            let t342 = ((t99).select(t323, f64x8::splat(0.0)));
            let t346 = ((t98).select(-f64x8::splat(0.667) * t324 - f64x8::splat(0.889111) * t326 - f64x8::splat(1.989259803147) * t328 + f64x8::splat(5.80518817796) * t330 - f64x8::splat(4.439990207985) * t332 + f64x8::splat(1.407173648874) * t334 - f64x8::splat(0.162300903254) * t110 * t324, -t339 * t341 * t342 * t119));
            let t349 = t269 * t321 - t97 * t346 + f64x8::splat(1.174) * t346;
            let t350 = t28 * t349;
            let t351 = t350 * t140;
            let t354 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t355 = t354 * t354;
            let t356 = t355 * t355;
            let t357 = t356 * t354;
            let t358 = t357 * t26;
            let t360 = f64x8::splat(1.0) / t135 / t134;
            let t361 = t126 * t360;
            let t362 = t358 * t361;
            let t364 = f64x8::splat(1.0) / t36 / t35;
            let t367 = t129 * t130 * t364 * t139;
            let t371 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t257 * t141 - t265 - f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t351 - f64x8::splat(1.6891736332904388) * t362 * t367));
            let t372 = t146 * t250;
            let t374 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t372)));
            let t377 = ((t150).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t151 * t374));
            let t378 = t6 * t377;
            let t381 = t261 * t232;
            let t382 = t381 * t244;
            let t384 = t154 * t382 / f64x8::splat(8.0);
            let t386 = ((t145).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t378 * t245 - t384));
            let tvrho0 = t144 + t248 + t7 * (t371 + t386);
            acc_vrho_0 = tvrho0;
            let t390 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t251)));
            let t393 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t390));
            let t394 = t6 * t393;
            let t398 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t394 * t141 - t265));
            let t400 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t372)));
            let t403 = ((t150).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t151 * t400));
            let t404 = t6 * t403;
            let t407 = t199 * t199;
            let t409 = t266 / t407;
            let t410 = t155 * v_rho1;
            let t412 = f64x8::splat(1.0) / t157 / t410;
            let t413 = v_sigma2 * t412;
            let t416 = t164 * t155;
            let t418 = f64x8::splat(1.0) / t156 / t416;
            let t423 = t163 * v_sigma2;
            let t424 = t164 * t164;
            let t425 = t424 * v_rho1;
            let t426 = f64x8::splat(1.0) / t425;
            let t437 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t159 + t413 / f64x8::splat(3.0);
            let t439 = t187 * t187;
            let t440 = f64x8::splat(1.0) / t439;
            let t441 = t183 * t440;
            let t442 = t184 * t412;
            let t445 = -t437 * t188 - t441 * t442 / f64x8::splat(3.0);
            let t449 = t66 * t192;
            let t450 = t445 * t194;
            let t453 = -f64x8::splat(7.0) / f64x8::splat(4860.0) * t67 * t169 * t412 + t66 * t445 * t194 / f64x8::splat(100.0) - t449 * t450 / f64x8::splat(100.0);
            let t456 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t34 * t413 - t50 * t163 * t418 * t173 / f64x8::splat(108.0) + f64x8::splat(3.0) / f64x8::splat(320.0) * t286 * t423 * t426 * t173 + f64x8::splat(2.0) * t197 * t453;
            let t457 = t456 * t229;
            let t459 = -t445;
            let t460 = ((t206).select(f64x8::splat(0.0), t459));
            let t462 = t207 * t460;
            let t464 = t209 * t460;
            let t466 = t211 * t460;
            let t468 = t213 * t460;
            let t470 = t215 * t460;
            let t475 = t223 * t223;
            let t476 = f64x8::splat(1.0) / t475;
            let t477 = ((t206).select(t459, f64x8::splat(0.0)));
            let t481 = ((t205).select(-f64x8::splat(0.667) * t460 - f64x8::splat(0.889111) * t462 - f64x8::splat(1.989259803147) * t464 + f64x8::splat(5.80518817796) * t466 - f64x8::splat(4.439990207985) * t468 + f64x8::splat(1.407173648874) * t470 - f64x8::splat(0.162300903254) * t217 * t460, -t339 * t476 * t477 * t226));
            let t484 = t409 * t457 - t204 * t481 + f64x8::splat(1.174) * t481;
            let t485 = t28 * t484;
            let t486 = t485 * t244;
            let t489 = t357 * t153;
            let t491 = f64x8::splat(1.0) / t239 / t238;
            let t492 = t233 * t491;
            let t493 = t489 * t492;
            let t495 = f64x8::splat(1.0) / t156 / t155;
            let t498 = t129 * t234 * t495 * t243;
            let t502 = ((t145).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t404 * t245 - t384 - f64x8::splat(3.0) / f64x8::splat(8.0) * t154 * t486 - f64x8::splat(1.6891736332904388) * t493 * t498));
            let tvrho1 = t144 + t248 + t7 * (t398 + t502);
            acc_vrho_1 = tvrho1;
            let t511 = f64x8::splat(1.0) / t288;
            let t519 = t39 * t81;
            let t520 = param_eta * t39;
            let t523 = t305 * t520 / f64x8::splat(8.0) + t519 / f64x8::splat(8.0);
            let t524 = t66 * t523;
            let t527 = t523 * t87;
            let t530 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t33 * t39 + t524 * t87 / f64x8::splat(100.0) - t313 * t527 / f64x8::splat(100.0);
            let t533 = f64x8::splat(5.0) / f64x8::splat(972.0) * t34 * t39 + t50 * v_sigma0 * t55 * t62 / f64x8::splat(288.0) - f64x8::splat(9.0) / f64x8::splat(2560.0) * t286 * t51 * t511 * t62 + f64x8::splat(2.0) * t90 * t530;
            let t534 = t533 * t122;
            let t536 = -t523;
            let t537 = ((t99).select(f64x8::splat(0.0), t536));
            let t539 = t100 * t537;
            let t541 = t102 * t537;
            let t543 = t104 * t537;
            let t545 = t106 * t537;
            let t547 = t108 * t537;
            let t552 = ((t99).select(t536, f64x8::splat(0.0)));
            let t556 = ((t98).select(-f64x8::splat(0.667) * t537 - f64x8::splat(0.889111) * t539 - f64x8::splat(1.989259803147) * t541 + f64x8::splat(5.80518817796) * t543 - f64x8::splat(4.439990207985) * t545 + f64x8::splat(1.407173648874) * t547 - f64x8::splat(0.162300903254) * t110 * t537, -t339 * t341 * t552 * t119));
            let t559 = t269 * t534 - t97 * t556 + f64x8::splat(1.174) * t556;
            let t560 = t28 * t559;
            let t561 = t560 * t140;
            let t564 = f64x8::splat(1.0) / t130;
            let t567 = t129 * t564 * t132 * t139;
            let t571 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t561 + f64x8::splat(0.6334401124839145) * t362 * t567));
            let tvsigma0 = t7 * t571;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t578 = f64x8::splat(1.0) / t424;
            let t586 = t159 * t188;
            let t587 = param_eta * t159;
            let t590 = t441 * t587 / f64x8::splat(8.0) + t586 / f64x8::splat(8.0);
            let t591 = t66 * t590;
            let t594 = t590 * t194;
            let t597 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t33 * t159 + t591 * t194 / f64x8::splat(100.0) - t449 * t594 / f64x8::splat(100.0);
            let t600 = f64x8::splat(5.0) / f64x8::splat(972.0) * t34 * t159 + t50 * v_sigma2 * t167 * t173 / f64x8::splat(288.0) - f64x8::splat(9.0) / f64x8::splat(2560.0) * t286 * t163 * t578 * t173 + f64x8::splat(2.0) * t197 * t597;
            let t601 = t600 * t229;
            let t603 = -t590;
            let t604 = ((t206).select(f64x8::splat(0.0), t603));
            let t606 = t207 * t604;
            let t608 = t209 * t604;
            let t610 = t211 * t604;
            let t612 = t213 * t604;
            let t614 = t215 * t604;
            let t619 = ((t206).select(t603, f64x8::splat(0.0)));
            let t623 = ((t205).select(-f64x8::splat(0.667) * t604 - f64x8::splat(0.889111) * t606 - f64x8::splat(1.989259803147) * t608 + f64x8::splat(5.80518817796) * t610 - f64x8::splat(4.439990207985) * t612 + f64x8::splat(1.407173648874) * t614 - f64x8::splat(0.162300903254) * t217 * t604, -t339 * t476 * t619 * t226));
            let t626 = t409 * t601 - t204 * t623 + f64x8::splat(1.174) * t623;
            let t627 = t28 * t626;
            let t628 = t627 * t244;
            let t631 = f64x8::splat(1.0) / t234;
            let t634 = t129 * t631 * t236 * t243;
            let t638 = ((t145).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t154 * t628 + f64x8::splat(0.6334401124839145) * t493 * t634));
            let tvsigma2 = t7 * t638;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t639 = t66 * t71;
            let t640 = t81 * t87;
            let t642 = t71 * t81;
            let t646 = t313 * t642 * t87 / f64x8::splat(100.0) - t639 * t640 / f64x8::splat(100.0);
            let t647 = t90 * t646;
            let t651 = ((t99).select(f64x8::splat(0.0), t642));
            let t653 = t100 * t651;
            let t655 = t102 * t651;
            let t657 = t104 * t651;
            let t659 = t106 * t651;
            let t661 = t108 * t651;
            let t666 = ((t99).select(t642, f64x8::splat(0.0)));
            let t670 = ((t98).select(-f64x8::splat(0.667) * t651 - f64x8::splat(0.889111) * t653 - f64x8::splat(1.989259803147) * t655 + f64x8::splat(5.80518817796) * t657 - f64x8::splat(4.439990207985) * t659 + f64x8::splat(1.407173648874) * t661 - f64x8::splat(0.162300903254) * t110 * t651, -t339 * t341 * t666 * t119));
            let t673 = f64x8::splat(2.0) * t269 * t647 * t122 - t97 * t670 + f64x8::splat(1.174) * t670;
            let t674 = t28 * t673;
            let t675 = t674 * t140;
            let t678 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t675));
            let tvtau0 = t7 * t678;
            acc_vtau_0 = tvtau0;
            let t679 = t66 * t180;
            let t680 = t188 * t194;
            let t682 = t180 * t188;
            let t686 = t449 * t682 * t194 / f64x8::splat(100.0) - t679 * t680 / f64x8::splat(100.0);
            let t687 = t197 * t686;
            let t691 = ((t206).select(f64x8::splat(0.0), t682));
            let t693 = t207 * t691;
            let t695 = t209 * t691;
            let t697 = t211 * t691;
            let t699 = t213 * t691;
            let t701 = t215 * t691;
            let t706 = ((t206).select(t682, f64x8::splat(0.0)));
            let t710 = ((t205).select(-f64x8::splat(0.667) * t691 - f64x8::splat(0.889111) * t693 - f64x8::splat(1.989259803147) * t695 + f64x8::splat(5.80518817796) * t697 - f64x8::splat(4.439990207985) * t699 + f64x8::splat(1.407173648874) * t701 - f64x8::splat(0.162300903254) * t217 * t691, -t339 * t476 * t706 * t226));
            let t713 = f64x8::splat(2.0) * t409 * t687 * t229 - t204 * t710 + f64x8::splat(1.174) * t710;
            let t714 = t28 * t713;
            let t715 = t714 * t244;
            let t718 = ((t145).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t154 * t715));
            let tvtau1 = t7 * t718;
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
