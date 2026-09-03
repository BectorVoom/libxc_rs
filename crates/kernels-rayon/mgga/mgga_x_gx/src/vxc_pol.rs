//! MGGA_X_GX vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gx.c`
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
pub fn mgga_x_gx_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c0: f64,
    param_c1: f64,
    param_alphainf: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c0 = f64x8::splat(param_c0);
    let param_c1 = f64x8::splat(param_c1);
    let param_alphainf = f64x8::splat(param_alphainf);
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
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t3 * t3;
            let t32 = f64x8::splat(M_CBRT4);
            let t34 = f64x8::splat(8.0) / f64x8::splat(27.0) * t29 * t30 * t32;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / v_rho0;
            let t40 = v_rho0 * v_rho0;
            let t42 = f64x8::splat(1.0) / t36 / t40;
            let t45 = v_tau0 * t38 - v_sigma0 * t42 / f64x8::splat(8.0);
            let t46 = f64x8::splat(M_CBRT6);
            let t48 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t49 = (simd::cbrt(t48));
            let t50 = t49 * t49;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = t45 * t46 * t51;
            let t54 = t46 * t51;
            let t57 = param_c0 + f64x8::splat(5.0) / f64x8::splat(9.0) * param_c1 * t45 * t54;
            let t58 = param_c0 + param_c1 - f64x8::splat(1.0);
            let t62 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(9.0) * t58 * t45 * t54;
            let t63 = f64x8::splat(1.0) / t62;
            let t65 = f64x8::splat(1.0) - t34;
            let t66 = t57 * t63 * t65;
            let t69 = t34 + f64x8::splat(5.0) / f64x8::splat(9.0) * t52 * t66;
            let t70 = f64x8::splat(5.0) / f64x8::splat(9.0) * t52;
            let t71 = f64x8::splat(1.0) - t70;
            let t72 = ((t71).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t74 = f64x8::splat(1.0) - param_alphainf;
            let t75 = t74 * t71;
            let t76 = f64x8::splat(1.0) + t70;
            let t77 = f64x8::splat(1.0) / t76;
            let t79 = t75 * t77 + f64x8::splat(1.0);
            let t80 = -t71;
            let t81 = ((t80).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t83 = t69 * t72 + t79 * t81;
            let t87 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t83));
            let t88 = (v_rho1).simd_le(dens_threshold);
            let t89 = -t17;
            let t91 = ((t15).select(t12, (t11).select(t16, t89 * t8)));
            let t92 = f64x8::splat(1.0) + t91;
            let t93 = (t92).simd_le(zeta_threshold);
            let t94 = (simd::cbrt(t92));
            let t96 = ((t93).select(t23, t94 * t92));
            let t97 = t96 * t27;
            let t98 = (simd::cbrt(v_rho1));
            let t99 = t98 * t98;
            let t101 = f64x8::splat(1.0) / t99 / v_rho1;
            let t103 = v_rho1 * v_rho1;
            let t105 = f64x8::splat(1.0) / t99 / t103;
            let t108 = v_tau1 * t101 - v_sigma2 * t105 / f64x8::splat(8.0);
            let t110 = t108 * t46 * t51;
            let t114 = param_c0 + f64x8::splat(5.0) / f64x8::splat(9.0) * param_c1 * t108 * t54;
            let t118 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(9.0) * t58 * t108 * t54;
            let t119 = f64x8::splat(1.0) / t118;
            let t121 = t114 * t119 * t65;
            let t124 = t34 + f64x8::splat(5.0) / f64x8::splat(9.0) * t110 * t121;
            let t125 = f64x8::splat(5.0) / f64x8::splat(9.0) * t110;
            let t126 = f64x8::splat(1.0) - t125;
            let t127 = ((t126).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t129 = t74 * t126;
            let t130 = f64x8::splat(1.0) + t125;
            let t131 = f64x8::splat(1.0) / t130;
            let t133 = t129 * t131 + f64x8::splat(1.0);
            let t134 = -t126;
            let t135 = ((t134).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t137 = t124 * t127 + t133 * t135;
            let t141 = ((t88).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t97 * t137));
            let tzk0 = t87 + t141;
            acc_zk = tzk0;
            let t142 = t7 * t7;
            let t143 = f64x8::splat(1.0) / t142;
            let t144 = t17 * t143;
            let t146 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t144)));
            let t149 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t146));
            let t150 = t149 * t27;
            let t154 = t27 * t27;
            let t155 = f64x8::splat(1.0) / t154;
            let t156 = t26 * t155;
            let t159 = t6 * t156 * t83 / f64x8::splat(8.0);
            let t162 = t40 * v_rho0;
            let t164 = f64x8::splat(1.0) / t36 / t162;
            let t167 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t42 + v_sigma0 * t164 / f64x8::splat(3.0);
            let t169 = t167 * t46 * t51;
            let t172 = t46 * t46;
            let t173 = t45 * t172;
            let t175 = f64x8::splat(1.0) / t49 / t48;
            let t176 = t173 * t175;
            let t177 = param_c1 * t167;
            let t178 = t63 * t65;
            let t179 = t177 * t178;
            let t182 = t175 * t57;
            let t183 = t173 * t182;
            let t184 = t62 * t62;
            let t185 = f64x8::splat(1.0) / t184;
            let t186 = t185 * t65;
            let t188 = t186 * t58 * t167;
            let t191 = f64x8::splat(5.0) / f64x8::splat(9.0) * t169 * t66 + f64x8::splat(25.0) / f64x8::splat(81.0) * t176 * t179 - f64x8::splat(25.0) / f64x8::splat(81.0) * t183 * t188;
            let t193 = f64x8::splat(0.0);
            let t194 = t69 * t193;
            let t198 = t54 * t77;
            let t200 = t76 * t76;
            let t201 = f64x8::splat(1.0) / t200;
            let t202 = t75 * t201;
            let t205 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t74 * t167 * t198 - f64x8::splat(5.0) / f64x8::splat(9.0) * t202 * t169;
            let t207 = t79 * t193;
            let t210 = t191 * t72 - f64x8::splat(5.0) / f64x8::splat(9.0) * t194 * t169 + t205 * t81 + f64x8::splat(5.0) / f64x8::splat(9.0) * t207 * t169;
            let t215 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t150 * t83 - t159 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t210));
            let t216 = t89 * t143;
            let t218 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t216)));
            let t221 = ((t93).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t94 * t218));
            let t222 = t221 * t27;
            let t226 = t96 * t155;
            let t229 = t6 * t226 * t137 / f64x8::splat(8.0);
            let t231 = ((t88).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t222 * t137 - t229));
            let tvrho0 = t87 + t141 + t7 * (t215 + t231);
            acc_vrho_0 = tvrho0;
            let t235 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t144)));
            let t238 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t235));
            let t239 = t238 * t27;
            let t244 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t239 * t83 - t159));
            let t246 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t216)));
            let t249 = ((t93).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t94 * t246));
            let t250 = t249 * t27;
            let t256 = t103 * v_rho1;
            let t258 = f64x8::splat(1.0) / t99 / t256;
            let t261 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t105 + v_sigma2 * t258 / f64x8::splat(3.0);
            let t263 = t261 * t46 * t51;
            let t266 = t108 * t172;
            let t267 = t266 * t175;
            let t269 = t119 * t65;
            let t270 = param_c1 * t261 * t269;
            let t273 = t175 * t114;
            let t274 = t266 * t273;
            let t275 = t118 * t118;
            let t276 = f64x8::splat(1.0) / t275;
            let t277 = t276 * t65;
            let t279 = t277 * t58 * t261;
            let t282 = f64x8::splat(5.0) / f64x8::splat(9.0) * t263 * t121 + f64x8::splat(25.0) / f64x8::splat(81.0) * t267 * t270 - f64x8::splat(25.0) / f64x8::splat(81.0) * t274 * t279;
            let t284 = f64x8::splat(0.0);
            let t285 = t124 * t284;
            let t289 = t54 * t131;
            let t291 = t130 * t130;
            let t292 = f64x8::splat(1.0) / t291;
            let t293 = t129 * t292;
            let t296 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t74 * t261 * t289 - f64x8::splat(5.0) / f64x8::splat(9.0) * t293 * t263;
            let t298 = t133 * t284;
            let t301 = t282 * t127 - f64x8::splat(5.0) / f64x8::splat(9.0) * t285 * t263 + t296 * t135 + f64x8::splat(5.0) / f64x8::splat(9.0) * t298 * t263;
            let t306 = ((t88).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t250 * t137 - t229 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t97 * t301));
            let tvrho1 = t87 + t141 + t7 * (t244 + t306);
            acc_vrho_1 = tvrho1;
            let t310 = t42 * t46 * t51;
            let t311 = t310 * t66;
            let t313 = param_c1 * t42;
            let t315 = t176 * t313 * t178;
            let t319 = t183 * t186 * t58 * t42;
            let t321 = -f64x8::splat(5.0) / f64x8::splat(72.0) * t311 - f64x8::splat(25.0) / f64x8::splat(648.0) * t315 + f64x8::splat(25.0) / f64x8::splat(648.0) * t319;
            let t323 = t194 * t310;
            let t325 = t74 * t42;
            let t326 = t325 * t198;
            let t327 = t202 * t310;
            let t329 = f64x8::splat(5.0) / f64x8::splat(72.0) * t326 + f64x8::splat(5.0) / f64x8::splat(72.0) * t327;
            let t331 = t207 * t310;
            let t333 = t321 * t72 + f64x8::splat(5.0) / f64x8::splat(72.0) * t323 + t329 * t81 - f64x8::splat(5.0) / f64x8::splat(72.0) * t331;
            let t337 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t333));
            let tvsigma0 = t7 * t337;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t339 = t105 * t46 * t51;
            let t340 = t339 * t121;
            let t342 = param_c1 * t105;
            let t344 = t267 * t342 * t269;
            let t348 = t274 * t277 * t58 * t105;
            let t350 = -f64x8::splat(5.0) / f64x8::splat(72.0) * t340 - f64x8::splat(25.0) / f64x8::splat(648.0) * t344 + f64x8::splat(25.0) / f64x8::splat(648.0) * t348;
            let t352 = t285 * t339;
            let t354 = t74 * t105;
            let t355 = t354 * t289;
            let t356 = t293 * t339;
            let t358 = f64x8::splat(5.0) / f64x8::splat(72.0) * t355 + f64x8::splat(5.0) / f64x8::splat(72.0) * t356;
            let t360 = t298 * t339;
            let t362 = t350 * t127 + f64x8::splat(5.0) / f64x8::splat(72.0) * t352 + t358 * t135 - f64x8::splat(5.0) / f64x8::splat(72.0) * t360;
            let t366 = ((t88).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t97 * t362));
            let tvsigma2 = t7 * t366;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t368 = t38 * t46 * t51;
            let t371 = param_c1 * t38;
            let t379 = f64x8::splat(5.0) / f64x8::splat(9.0) * t368 * t66 + f64x8::splat(25.0) / f64x8::splat(81.0) * t176 * t371 * t178 - f64x8::splat(25.0) / f64x8::splat(81.0) * t183 * t186 * t58 * t38;
            let t383 = t74 * t38;
            let t387 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t383 * t198 - f64x8::splat(5.0) / f64x8::splat(9.0) * t202 * t368;
            let t391 = t379 * t72 - f64x8::splat(5.0) / f64x8::splat(9.0) * t194 * t368 + t387 * t81 + f64x8::splat(5.0) / f64x8::splat(9.0) * t207 * t368;
            let t395 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t391));
            let tvtau0 = t7 * t395;
            acc_vtau_0 = tvtau0;
            let t397 = t101 * t46 * t51;
            let t400 = param_c1 * t101;
            let t408 = f64x8::splat(5.0) / f64x8::splat(9.0) * t397 * t121 + f64x8::splat(25.0) / f64x8::splat(81.0) * t267 * t400 * t269 - f64x8::splat(25.0) / f64x8::splat(81.0) * t274 * t277 * t58 * t101;
            let t412 = t74 * t101;
            let t416 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t412 * t289 - f64x8::splat(5.0) / f64x8::splat(9.0) * t293 * t397;
            let t420 = t408 * t127 - f64x8::splat(5.0) / f64x8::splat(9.0) * t285 * t397 + t416 * t135 + f64x8::splat(5.0) / f64x8::splat(9.0) * t298 * t397;
            let t424 = ((t88).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t97 * t420));
            let tvtau1 = t7 * t424;
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
