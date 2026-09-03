//! MGGA_X_FT98 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_ft98.c`
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
pub fn mgga_x_ft98_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_a: f64,
    param_a1: f64,
    param_a2: f64,
    param_b: f64,
    param_b1: f64,
    param_b2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_a1 = f64x8::splat(param_a1);
    let param_a2 = f64x8::splat(param_a2);
    let param_b = f64x8::splat(param_b);
    let param_b1 = f64x8::splat(param_b1);
    let param_b2 = f64x8::splat(param_b2);
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
            let t5 = f64x8::splat(1.0) / t4;
            let t6 = t3 * t5;
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
            let t30 = v_rho0 * v_rho0;
            let t31 = (simd::cbrt(v_rho0));
            let t32 = t31 * t31;
            let t34 = f64x8::splat(1.0) / t32 / t30;
            let t36 = param_a1 * v_sigma0 * t34 + f64x8::splat(1.0);
            let t37 = ((t36).sqrt());
            let t38 = param_a * t37;
            let t39 = param_b1 * v_sigma0;
            let t41 = t39 * t34 + f64x8::splat(1.0);
            let t42 = ((t41).sqrt().sqrt());
            let t43 = t42 * t42;
            let t44 = t43 * t42;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t45 * v_sigma0;
            let t49 = v_sigma0 * t34;
            let t51 = f64x8::splat(1.0) / t32 / v_rho0;
            let t53 = -v_lapl0 * t51 + t49;
            let t54 = t53 * t53;
            let t55 = param_a2 * t54;
            let t56 = f64x8::splat(1.0) + t49;
            let t57 = t56 * t56;
            let t58 = f64x8::splat(1.0) / t57;
            let t61 = param_b * (t55 * t58 + f64x8::splat(1.0));
            let t62 = param_b2 * param_b2;
            let t64 = ((t62 + f64x8::splat(1.0)).sqrt());
            let t65 = t64 - param_b2;
            let t66 = v_sigma0 * v_sigma0;
            let t67 = t30 * t30;
            let t68 = t67 * v_rho0;
            let t70 = f64x8::splat(1.0) / t31 / t68;
            let t71 = t66 * t70;
            let t72 = v_lapl0 * v_lapl0;
            let t73 = t30 * v_rho0;
            let t75 = f64x8::splat(1.0) / t31 / t73;
            let t76 = t72 * t75;
            let t77 = t71 - t76 - param_b2;
            let t78 = ((f64x8::splat(f64::EPSILON)).sqrt().sqrt());
            let t79 = f64x8::splat(1.0) / t78;
            let t80 = (t77).simd_lt(-t79);
            let t83 = f64x8::splat(2.0) * param_b2;
            let t86 = t77 * t77;
            let t87 = t86 * t77;
            let t88 = f64x8::splat(1.0) / t87;
            let t90 = t86 * t86;
            let t91 = t90 * t77;
            let t92 = f64x8::splat(1.0) / t91;
            let t97 = (((f64x8::splat(0.0)).simd_lt(t77)).select(t77, -t77));
            let t98 = (t97).simd_lt(t78);
            let t101 = t90 * t86;
            let t103 = t90 * t90;
            let t106 = (-t79).simd_lt(t77);
            let t107 = ((t106).select(t77, -t79));
            let t108 = t107 * t107;
            let t109 = f64x8::splat(1.0) + t108;
            let t110 = ((t109).sqrt());
            let t111 = t107 + t110;
            let t113 = ((t80).select(-f64x8::splat(2.0) * t71 + f64x8::splat(2.0) * t76 + t83 - f64x8::splat(1.0) / t77 / f64x8::splat(2.0) + t88 / f64x8::splat(8.0) - t92 / f64x8::splat(16.0), (t98).select(f64x8::splat(1.0) - t71 + t76 + param_b2 + t86 / f64x8::splat(2.0) - t90 / f64x8::splat(8.0) + t101 / f64x8::splat(16.0) - f64x8::splat(5.0) / f64x8::splat(128.0) * t103, f64x8::splat(1.0) / t111)));
            let t115 = t65 * t113 + f64x8::splat(1.0);
            let t116 = f64x8::splat(M_CBRT2);
            let t117 = t116 - f64x8::splat(1.0);
            let t118 = t117 * t65;
            let t120 = t118 * t113 + f64x8::splat(1.0);
            let t121 = t120 * t120;
            let t122 = t121 * t120;
            let t123 = f64x8::splat(1.0) / t122;
            let t124 = t115 * t123;
            let t125 = t124 * t54;
            let t127 = t38 * t46 * t34 + t61 * t125 + f64x8::splat(1.0);
            let t128 = t3 * t3;
            let t129 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t130 = (simd::cbrt(t129));
            let t131 = t130 * t130;
            let t132 = t128 * t131;
            let t133 = f64x8::splat(M_CBRT4);
            let t134 = t132 * t133;
            let t135 = param_b * v_sigma0;
            let t139 = f64x8::splat(1.0) + f64x8::splat(81.0) / f64x8::splat(4.0) * t134 * t135 * t34;
            let t140 = f64x8::splat(1.0) / t139;
            let t141 = t127 * t140;
            let t142 = ((t141).sqrt());
            let t146 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t142));
            let t147 = (v_rho1).simd_le(dens_threshold);
            let t148 = -t17;
            let t150 = ((t15).select(t12, (t11).select(t16, t148 * t8)));
            let t151 = f64x8::splat(1.0) + t150;
            let t152 = (t151).simd_le(zeta_threshold);
            let t153 = (simd::cbrt(t151));
            let t155 = ((t152).select(t23, t153 * t151));
            let t156 = t155 * t27;
            let t157 = param_a1 * v_sigma2;
            let t158 = v_rho1 * v_rho1;
            let t159 = (simd::cbrt(v_rho1));
            let t160 = t159 * t159;
            let t162 = f64x8::splat(1.0) / t160 / t158;
            let t164 = t157 * t162 + f64x8::splat(1.0);
            let t165 = ((t164).sqrt());
            let t166 = param_a * t165;
            let t169 = param_b1 * v_sigma2 * t162 + f64x8::splat(1.0);
            let t170 = ((t169).sqrt().sqrt());
            let t171 = t170 * t170;
            let t172 = t171 * t170;
            let t173 = f64x8::splat(1.0) / t172;
            let t174 = t173 * v_sigma2;
            let t177 = v_sigma2 * t162;
            let t179 = f64x8::splat(1.0) / t160 / v_rho1;
            let t181 = -v_lapl1 * t179 + t177;
            let t182 = t181 * t181;
            let t183 = param_a2 * t182;
            let t184 = f64x8::splat(1.0) + t177;
            let t185 = t184 * t184;
            let t186 = f64x8::splat(1.0) / t185;
            let t189 = param_b * (t183 * t186 + f64x8::splat(1.0));
            let t190 = v_sigma2 * v_sigma2;
            let t191 = t158 * t158;
            let t192 = t191 * v_rho1;
            let t194 = f64x8::splat(1.0) / t159 / t192;
            let t195 = t190 * t194;
            let t196 = v_lapl1 * v_lapl1;
            let t197 = t158 * v_rho1;
            let t199 = f64x8::splat(1.0) / t159 / t197;
            let t200 = t196 * t199;
            let t201 = t195 - t200 - param_b2;
            let t202 = (t201).simd_lt(-t79);
            let t207 = t201 * t201;
            let t208 = t207 * t201;
            let t209 = f64x8::splat(1.0) / t208;
            let t211 = t207 * t207;
            let t212 = t211 * t201;
            let t213 = f64x8::splat(1.0) / t212;
            let t218 = (((f64x8::splat(0.0)).simd_lt(t201)).select(t201, -t201));
            let t219 = (t218).simd_lt(t78);
            let t222 = t211 * t207;
            let t224 = t211 * t211;
            let t227 = (-t79).simd_lt(t201);
            let t228 = ((t227).select(t201, -t79));
            let t229 = t228 * t228;
            let t230 = f64x8::splat(1.0) + t229;
            let t231 = ((t230).sqrt());
            let t232 = t228 + t231;
            let t234 = ((t202).select(-f64x8::splat(2.0) * t195 + f64x8::splat(2.0) * t200 + t83 - f64x8::splat(1.0) / t201 / f64x8::splat(2.0) + t209 / f64x8::splat(8.0) - t213 / f64x8::splat(16.0), (t219).select(f64x8::splat(1.0) - t195 + t200 + param_b2 + t207 / f64x8::splat(2.0) - t211 / f64x8::splat(8.0) + t222 / f64x8::splat(16.0) - f64x8::splat(5.0) / f64x8::splat(128.0) * t224, f64x8::splat(1.0) / t232)));
            let t236 = t65 * t234 + f64x8::splat(1.0);
            let t238 = t118 * t234 + f64x8::splat(1.0);
            let t239 = t238 * t238;
            let t240 = t239 * t238;
            let t241 = f64x8::splat(1.0) / t240;
            let t242 = t236 * t241;
            let t243 = t242 * t182;
            let t245 = t166 * t174 * t162 + t189 * t243 + f64x8::splat(1.0);
            let t246 = param_b * v_sigma2;
            let t250 = f64x8::splat(1.0) + f64x8::splat(81.0) / f64x8::splat(4.0) * t134 * t246 * t162;
            let t251 = f64x8::splat(1.0) / t250;
            let t252 = t245 * t251;
            let t253 = ((t252).sqrt());
            let t257 = ((t147).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t156 * t253));
            let tzk0 = t146 + t257;
            acc_zk = tzk0;
            let t258 = t7 * t7;
            let t259 = f64x8::splat(1.0) / t258;
            let t260 = t17 * t259;
            let t262 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t260)));
            let t265 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t262));
            let t266 = t265 * t27;
            let t270 = t27 * t27;
            let t271 = f64x8::splat(1.0) / t270;
            let t272 = t26 * t271;
            let t275 = t6 * t272 * t142 / f64x8::splat(8.0);
            let t276 = t6 * t26;
            let t277 = f64x8::splat(1.0) / t142;
            let t278 = t27 * t277;
            let t280 = param_a / t37;
            let t281 = t280 * t45;
            let t282 = t67 * t30;
            let t284 = f64x8::splat(1.0) / t31 / t282;
            let t285 = t66 * t284;
            let t290 = f64x8::splat(1.0) / t44 / t41;
            let t291 = t38 * t290;
            let t296 = f64x8::splat(1.0) / t32 / t73;
            let t300 = param_a2 * t53;
            let t301 = v_sigma0 * t296;
            let t305 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t301 + f64x8::splat(5.0) / f64x8::splat(3.0) * v_lapl0 * t34;
            let t310 = f64x8::splat(1.0) / t57 / t56;
            let t311 = t310 * v_sigma0;
            let t312 = t311 * t296;
            let t316 = param_b * (f64x8::splat(2.0) * t300 * t58 * t305 + f64x8::splat(16.0) / f64x8::splat(3.0) * t55 * t312);
            let t318 = t61 * t65;
            let t321 = f64x8::splat(1.0) / t31 / t67;
            let t322 = t72 * t321;
            let t324 = f64x8::splat(1.0) / t86;
            let t325 = f64x8::splat(16.0) / f64x8::splat(3.0) * t285;
            let t326 = f64x8::splat(10.0) / f64x8::splat(3.0) * t322;
            let t327 = -t325 + t326;
            let t330 = f64x8::splat(1.0) / t90;
            let t333 = f64x8::splat(1.0) / t101;
            let t342 = t90 * t87;
            let t346 = t111 * t111;
            let t347 = f64x8::splat(1.0) / t346;
            let t348 = ((t106).select(t327, f64x8::splat(0.0)));
            let t349 = f64x8::splat(1.0) / t110;
            let t350 = t349 * t107;
            let t352 = t350 * t348 + t348;
            let t354 = ((t80).select(f64x8::splat(32.0) / f64x8::splat(3.0) * t285 - f64x8::splat(20.0) / f64x8::splat(3.0) * t322 + t324 * t327 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t330 * t327 + f64x8::splat(5.0) / f64x8::splat(16.0) * t333 * t327, (t98).select(t325 - t326 + t77 * t327 - t87 * t327 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t327 - f64x8::splat(5.0) / f64x8::splat(16.0) * t342 * t327, -t347 * t352)));
            let t355 = t354 * t123;
            let t356 = t355 * t54;
            let t358 = t121 * t121;
            let t359 = f64x8::splat(1.0) / t358;
            let t360 = t115 * t359;
            let t361 = t61 * t360;
            let t362 = t54 * t117;
            let t363 = t65 * t354;
            let t364 = t362 * t363;
            let t367 = t61 * t115;
            let t368 = t123 * t53;
            let t369 = t368 * t305;
            let t372 = -f64x8::splat(4.0) / f64x8::splat(3.0) * t281 * t285 * param_a1 + f64x8::splat(2.0) * t291 * t285 * param_b1 - f64x8::splat(8.0) / f64x8::splat(3.0) * t38 * t46 * t296 + t316 * t125 + t318 * t356 - f64x8::splat(3.0) * t361 * t364 + f64x8::splat(2.0) * t367 * t369;
            let t374 = t139 * t139;
            let t375 = f64x8::splat(1.0) / t374;
            let t376 = t127 * t375;
            let t377 = t376 * t132;
            let t378 = t133 * param_b;
            let t379 = t378 * t301;
            let t382 = t372 * t140 + f64x8::splat(54.0) * t377 * t379;
            let t383 = t278 * t382;
            let t387 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t266 * t142 - t275 - f64x8::splat(3.0) / f64x8::splat(16.0) * t276 * t383));
            let t388 = t148 * t259;
            let t390 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t388)));
            let t393 = ((t152).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t153 * t390));
            let t394 = t393 * t27;
            let t398 = t155 * t271;
            let t401 = t6 * t398 * t253 / f64x8::splat(8.0);
            let t403 = ((t147).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t394 * t253 - t401));
            let tvrho0 = t146 + t257 + t7 * (t387 + t403);
            acc_vrho_0 = tvrho0;
            let t407 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t260)));
            let t410 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t407));
            let t411 = t410 * t27;
            let t416 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t411 * t142 - t275));
            let t418 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t388)));
            let t421 = ((t152).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t153 * t418));
            let t422 = t421 * t27;
            let t426 = t6 * t155;
            let t427 = f64x8::splat(1.0) / t253;
            let t428 = t27 * t427;
            let t430 = param_a / t165;
            let t431 = t430 * t173;
            let t432 = t191 * t158;
            let t434 = f64x8::splat(1.0) / t159 / t432;
            let t435 = t190 * t434;
            let t440 = f64x8::splat(1.0) / t172 / t169;
            let t441 = t166 * t440;
            let t446 = f64x8::splat(1.0) / t160 / t197;
            let t450 = param_a2 * t181;
            let t451 = v_sigma2 * t446;
            let t455 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t451 + f64x8::splat(5.0) / f64x8::splat(3.0) * v_lapl1 * t162;
            let t460 = f64x8::splat(1.0) / t185 / t184;
            let t461 = t460 * v_sigma2;
            let t462 = t461 * t446;
            let t466 = param_b * (f64x8::splat(2.0) * t450 * t186 * t455 + f64x8::splat(16.0) / f64x8::splat(3.0) * t183 * t462);
            let t468 = t189 * t65;
            let t471 = f64x8::splat(1.0) / t159 / t191;
            let t472 = t196 * t471;
            let t474 = f64x8::splat(1.0) / t207;
            let t475 = f64x8::splat(16.0) / f64x8::splat(3.0) * t435;
            let t476 = f64x8::splat(10.0) / f64x8::splat(3.0) * t472;
            let t477 = -t475 + t476;
            let t480 = f64x8::splat(1.0) / t211;
            let t483 = f64x8::splat(1.0) / t222;
            let t492 = t211 * t208;
            let t496 = t232 * t232;
            let t497 = f64x8::splat(1.0) / t496;
            let t498 = ((t227).select(t477, f64x8::splat(0.0)));
            let t499 = f64x8::splat(1.0) / t231;
            let t500 = t499 * t228;
            let t502 = t500 * t498 + t498;
            let t504 = ((t202).select(f64x8::splat(32.0) / f64x8::splat(3.0) * t435 - f64x8::splat(20.0) / f64x8::splat(3.0) * t472 + t474 * t477 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t480 * t477 + f64x8::splat(5.0) / f64x8::splat(16.0) * t483 * t477, (t219).select(t475 - t476 + t201 * t477 - t208 * t477 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t212 * t477 - f64x8::splat(5.0) / f64x8::splat(16.0) * t492 * t477, -t497 * t502)));
            let t505 = t504 * t241;
            let t506 = t505 * t182;
            let t508 = t239 * t239;
            let t509 = f64x8::splat(1.0) / t508;
            let t510 = t236 * t509;
            let t511 = t189 * t510;
            let t512 = t182 * t117;
            let t513 = t65 * t504;
            let t514 = t512 * t513;
            let t517 = t189 * t236;
            let t518 = t241 * t181;
            let t519 = t518 * t455;
            let t522 = -f64x8::splat(4.0) / f64x8::splat(3.0) * t431 * t435 * param_a1 + f64x8::splat(2.0) * t441 * t435 * param_b1 - f64x8::splat(8.0) / f64x8::splat(3.0) * t166 * t174 * t446 + t466 * t243 + t468 * t506 - f64x8::splat(3.0) * t511 * t514 + f64x8::splat(2.0) * t517 * t519;
            let t524 = t250 * t250;
            let t525 = f64x8::splat(1.0) / t524;
            let t526 = t245 * t525;
            let t527 = t526 * t132;
            let t528 = t378 * t451;
            let t531 = t522 * t251 + f64x8::splat(54.0) * t527 * t528;
            let t532 = t428 * t531;
            let t536 = ((t147).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t422 * t253 - t401 - f64x8::splat(3.0) / f64x8::splat(16.0) * t426 * t532));
            let tvrho1 = t146 + t257 + t7 * (t416 + t536);
            acc_vrho_1 = tvrho1;
            let t539 = v_sigma0 * t70;
            let t548 = t58 * t34;
            let t550 = t310 * t34;
            let t554 = param_b * (f64x8::splat(2.0) * t300 * t548 - f64x8::splat(2.0) * t55 * t550);
            let t557 = t324 * v_sigma0;
            let t559 = t330 * v_sigma0;
            let t562 = t333 * v_sigma0;
            let t566 = f64x8::splat(2.0) * t539;
            let t567 = t77 * v_sigma0;
            let t570 = t87 * v_sigma0;
            let t572 = t91 * v_sigma0;
            let t575 = t342 * v_sigma0;
            let t579 = ((t106).select(t566, f64x8::splat(0.0)));
            let t581 = t350 * t579 + t579;
            let t583 = ((t80).select(-f64x8::splat(4.0) * t539 + t557 * t70 - f64x8::splat(3.0) / f64x8::splat(4.0) * t559 * t70 + f64x8::splat(5.0) / f64x8::splat(8.0) * t562 * t70, (t98).select(-t566 + f64x8::splat(2.0) * t567 * t70 - t570 * t70 + f64x8::splat(3.0) / f64x8::splat(4.0) * t572 * t70 - f64x8::splat(5.0) / f64x8::splat(8.0) * t575 * t70, -t347 * t581)));
            let t584 = t583 * t123;
            let t585 = t584 * t54;
            let t587 = t65 * t583;
            let t588 = t362 * t587;
            let t591 = t368 * t34;
            let t592 = t367 * t591;
            let t594 = t281 * t539 * param_a1 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t291 * t539 * param_b1 + t38 * t45 * t34 + t554 * t125 + t318 * t585 - f64x8::splat(3.0) * t361 * t588 + f64x8::splat(2.0) * t592;
            let t596 = t376 * t128;
            let t597 = t131 * t133;
            let t598 = param_b * t34;
            let t599 = t597 * t598;
            let t602 = t594 * t140 - f64x8::splat(81.0) / f64x8::splat(4.0) * t596 * t599;
            let t603 = t278 * t602;
            let t606 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(16.0) * t276 * t603));
            let tvsigma0 = t7 * t606;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t607 = v_sigma2 * t194;
            let t616 = t186 * t162;
            let t618 = t460 * t162;
            let t622 = param_b * (-f64x8::splat(2.0) * t183 * t618 + f64x8::splat(2.0) * t450 * t616);
            let t625 = t474 * v_sigma2;
            let t627 = t480 * v_sigma2;
            let t630 = t483 * v_sigma2;
            let t634 = f64x8::splat(2.0) * t607;
            let t635 = t201 * v_sigma2;
            let t638 = t208 * v_sigma2;
            let t640 = t212 * v_sigma2;
            let t643 = t492 * v_sigma2;
            let t647 = ((t227).select(t634, f64x8::splat(0.0)));
            let t649 = t500 * t647 + t647;
            let t651 = ((t202).select(-f64x8::splat(4.0) * t607 + t625 * t194 - f64x8::splat(3.0) / f64x8::splat(4.0) * t627 * t194 + f64x8::splat(5.0) / f64x8::splat(8.0) * t630 * t194, (t219).select(-t634 + f64x8::splat(2.0) * t635 * t194 - t638 * t194 + f64x8::splat(3.0) / f64x8::splat(4.0) * t640 * t194 - f64x8::splat(5.0) / f64x8::splat(8.0) * t643 * t194, -t497 * t649)));
            let t652 = t651 * t241;
            let t653 = t652 * t182;
            let t655 = t65 * t651;
            let t656 = t512 * t655;
            let t659 = t518 * t162;
            let t660 = t517 * t659;
            let t662 = t431 * t607 * param_a1 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t441 * t607 * param_b1 + t166 * t173 * t162 + t622 * t243 + t468 * t653 - f64x8::splat(3.0) * t511 * t656 + f64x8::splat(2.0) * t660;
            let t664 = t526 * t128;
            let t665 = param_b * t162;
            let t666 = t597 * t665;
            let t669 = t662 * t251 - f64x8::splat(81.0) / f64x8::splat(4.0) * t664 * t666;
            let t670 = t428 * t669;
            let t673 = ((t147).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(16.0) * t426 * t670));
            let tvsigma2 = t7 * t673;
            acc_vsigma_2 = tvsigma2;
            let t674 = param_b * param_a2;
            let t675 = t54 * t53;
            let t676 = t674 * t675;
            let t677 = t58 * t51;
            let t678 = t677 * t124;
            let t681 = v_lapl0 * t75;
            let t683 = t324 * v_lapl0;
            let t685 = t330 * v_lapl0;
            let t688 = t333 * v_lapl0;
            let t692 = f64x8::splat(2.0) * t681;
            let t693 = t77 * v_lapl0;
            let t696 = t87 * v_lapl0;
            let t698 = t91 * v_lapl0;
            let t701 = t342 * v_lapl0;
            let t705 = ((t106).select(-t692, f64x8::splat(0.0)));
            let t707 = t350 * t705 + t705;
            let t709 = ((t80).select(f64x8::splat(4.0) * t681 - t683 * t75 + f64x8::splat(3.0) / f64x8::splat(4.0) * t685 * t75 - f64x8::splat(5.0) / f64x8::splat(8.0) * t688 * t75, (t98).select(t692 - f64x8::splat(2.0) * t693 * t75 + t696 * t75 - f64x8::splat(3.0) / f64x8::splat(4.0) * t698 * t75 + f64x8::splat(5.0) / f64x8::splat(8.0) * t701 * t75, -t347 * t707)));
            let t710 = t709 * t123;
            let t711 = t710 * t54;
            let t713 = t65 * t709;
            let t714 = t362 * t713;
            let t717 = t368 * t51;
            let t720 = t318 * t711 - f64x8::splat(3.0) * t361 * t714 - f64x8::splat(2.0) * t367 * t717 - f64x8::splat(2.0) * t676 * t678;
            let t721 = t720 * t140;
            let t722 = t278 * t721;
            let t725 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(16.0) * t276 * t722));
            let tvlapl0 = t7 * t725;
            acc_vlapl_0 = tvlapl0;
            let t726 = t182 * t181;
            let t727 = t674 * t726;
            let t728 = t186 * t179;
            let t729 = t728 * t242;
            let t732 = v_lapl1 * t199;
            let t734 = t474 * v_lapl1;
            let t736 = t480 * v_lapl1;
            let t739 = t483 * v_lapl1;
            let t743 = f64x8::splat(2.0) * t732;
            let t744 = t201 * v_lapl1;
            let t747 = t208 * v_lapl1;
            let t749 = t212 * v_lapl1;
            let t752 = t492 * v_lapl1;
            let t756 = ((t227).select(-t743, f64x8::splat(0.0)));
            let t758 = t500 * t756 + t756;
            let t760 = ((t202).select(f64x8::splat(4.0) * t732 - t734 * t199 + f64x8::splat(3.0) / f64x8::splat(4.0) * t736 * t199 - f64x8::splat(5.0) / f64x8::splat(8.0) * t739 * t199, (t219).select(t743 - f64x8::splat(2.0) * t744 * t199 + t747 * t199 - f64x8::splat(3.0) / f64x8::splat(4.0) * t749 * t199 + f64x8::splat(5.0) / f64x8::splat(8.0) * t752 * t199, -t497 * t758)));
            let t761 = t760 * t241;
            let t762 = t761 * t182;
            let t764 = t65 * t760;
            let t765 = t512 * t764;
            let t768 = t518 * t179;
            let t771 = t468 * t762 - f64x8::splat(3.0) * t511 * t765 - f64x8::splat(2.0) * t517 * t768 - f64x8::splat(2.0) * t727 * t729;
            let t772 = t771 * t251;
            let t773 = t428 * t772;
            let t776 = ((t147).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(16.0) * t426 * t773));
            let tvlapl1 = t7 * t776;
            acc_vlapl_1 = tvlapl1;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau_0 = tvtau0;
            let tvtau1 = f64x8::splat(0.0);
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
