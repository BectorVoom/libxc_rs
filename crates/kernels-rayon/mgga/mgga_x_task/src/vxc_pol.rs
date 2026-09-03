//! MGGA_X_TASK vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_task.c`
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
pub fn mgga_x_task_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_task_c: f64,
    param_task_bnu_0: f64,
    param_task_bnu_1: f64,
    param_task_bnu_2: f64,
    param_task_bnu_3: f64,
    param_task_bnu_4: f64,
    param_task_anu_0: f64,
    param_task_anu_1: f64,
    param_task_anu_2: f64,
    param_task_h0x: f64,
    param_task_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_task_c = f64x8::splat(param_task_c);
    let param_task_bnu_0 = f64x8::splat(param_task_bnu_0);
    let param_task_bnu_1 = f64x8::splat(param_task_bnu_1);
    let param_task_bnu_2 = f64x8::splat(param_task_bnu_2);
    let param_task_bnu_3 = f64x8::splat(param_task_bnu_3);
    let param_task_bnu_4 = f64x8::splat(param_task_bnu_4);
    let param_task_anu_0 = f64x8::splat(param_task_anu_0);
    let param_task_anu_1 = f64x8::splat(param_task_anu_1);
    let param_task_anu_2 = f64x8::splat(param_task_anu_2);
    let param_task_h0x = f64x8::splat(param_task_h0x);
    let param_task_d = f64x8::splat(param_task_d);
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
            let t20 = t19 + f64x8::splat(1.0);
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = f64x8::splat(M_CBRT6);
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t34 = t29 / t32;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t38 = t37 * t35;
            let t39 = f64x8::splat(1.0) / t38;
            let t42 = t34 * v_sigma0 * t39 / f64x8::splat(24.0);
            let t43 = (f64x8::splat(0.0)).simd_lt(t42);
            let t44 = ((t43).select(t42, f64x8::splat(0.0)));
            let t45 = ((t44).sqrt().sqrt());
            let t48 = (simd::exp(-param_task_c / t45));
            let t50 = ((t43).select(f64x8::splat(1.0) - t48, f64x8::splat(0.0)));
            let t52 = v_tau0 * v_tau0;
            let t53 = t52 * t52;
            let t54 = t53 * t29;
            let t55 = param_task_bnu_0;
            let t56 = param_task_bnu_1;
            let t57 = param_task_bnu_2;
            let t58 = param_task_bnu_3;
            let t59 = param_task_bnu_4;
            let t60 = t55 + t56 + t57 + t58 + t59;
            let t61 = v_rho0 * v_tau0;
            let t65 = f64x8::splat(1.0) / v_rho0;
            let t67 = f64x8::splat(1.0) / v_tau0;
            let t69 = (f64x8::splat(0.0)).simd_lt((f64x8::splat(0.9999999999) * t61 - f64x8::splat(0.125) * v_sigma0) * t65 * t67);
            let t71 = f64x8::splat(8.0) * t61 - v_sigma0;
            let t72 = t71 * t65;
            let t75 = ((t69).select(t72 * t67 / f64x8::splat(8.0), f64x8::splat(1e-10)));
            let t76 = t75 * t75;
            let t77 = t76 * t76;
            let t78 = t60 * t77;
            let t81 = t56 / f64x8::splat(2.0);
            let t82 = f64x8::splat(7.0) / f64x8::splat(2.0) * t58;
            let t83 = f64x8::splat(7.0) * t59;
            let t85 = t4 * f64x8::splat(M_PI);
            let t86 = (t55 + t81 - t57 - t82 - t83) * t85;
            let t87 = t52 * v_tau0;
            let t88 = t37 * v_rho0;
            let t89 = t87 * t88;
            let t90 = t76 * t75;
            let t94 = t29 * t29;
            let t95 = t4 * t4;
            let t96 = t95 * t30;
            let t97 = t94 * t96;
            let t98 = t35 * v_rho0;
            let t99 = t36 * t98;
            let t100 = t97 * t99;
            let t103 = t55 - f64x8::splat(5.0) / f64x8::splat(3.0) * t57 + f64x8::splat(35.0) / f64x8::splat(3.0) * t59;
            let t104 = t52 * t103;
            let t105 = t104 * t76;
            let t108 = t30 * t30;
            let t110 = t108 * (t55 - t81 - t57 + t82 - t83);
            let t111 = t110 * t29;
            let t112 = t35 * t35;
            let t113 = t112 * v_rho0;
            let t114 = t113 * v_tau0;
            let t119 = t37 * t112 * t35;
            let t121 = t4 * t108 * f64x8::splat(M_PI);
            let t122 = t119 * t121;
            let t123 = t55 - t56 + t57 - t58 + t59;
            let t126 = f64x8::splat(14580.0) * t111 * t114 * t75 + f64x8::splat(27000.0) * t86 * t89 * t90 + f64x8::splat(12150.0) * t100 * t105 + f64x8::splat(6561.0) * t122 * t123 + f64x8::splat(3750.0) * t54 * t78;
            let t127 = t88 * t85;
            let t129 = v_tau0 * t29;
            let t132 = f64x8::splat(5.0) * t129 * t75 + f64x8::splat(9.0) * t127;
            let t133 = t132 * t132;
            let t134 = t133 * t133;
            let t135 = f64x8::splat(1.0) / t134;
            let t137 = f64x8::splat(1.0) - t126 * t135;
            let t138 = param_task_anu_0;
            let t139 = param_task_anu_1;
            let t140 = param_task_anu_2;
            let t142 = t96 * (t138 - t139 + t140);
            let t146 = t29 * t85;
            let t148 = t138 - f64x8::splat(3.0) * t140;
            let t151 = f64x8::splat(48.0) * t146 * t148 * t38;
            let t153 = t138 + t139 + t140;
            let t154 = v_sigma0 * t94 * t153;
            let t157 = f64x8::splat(576.0) * t142 * t36 * t113 + (t151 + t154) * v_sigma0;
            let t161 = t29 * v_sigma0 + f64x8::splat(24.0) * t85 * t38;
            let t162 = t161 * t161;
            let t163 = f64x8::splat(1.0) / t162;
            let t165 = t157 * t163 - param_task_h0x;
            let t166 = t137 * t165;
            let t167 = (simd::pow(t50, param_task_d));
            let t168 = t166 * t167;
            let t169 = param_task_h0x * t50 + t168;
            let t173 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t169));
            let t174 = (v_rho1).simd_le(dens_threshold);
            let t175 = -t17;
            let t177 = ((t15).select(t12, (t11).select(t16, t175 * t8)));
            let t178 = t177 + f64x8::splat(1.0);
            let t179 = (t178).simd_le(zeta_threshold);
            let t180 = (simd::cbrt(t178));
            let t182 = ((t179).select(t23, t180 * t178));
            let t183 = t182 * t27;
            let t184 = v_rho1 * v_rho1;
            let t185 = (simd::cbrt(v_rho1));
            let t186 = t185 * t185;
            let t187 = t186 * t184;
            let t188 = f64x8::splat(1.0) / t187;
            let t191 = t34 * v_sigma2 * t188 / f64x8::splat(24.0);
            let t192 = (f64x8::splat(0.0)).simd_lt(t191);
            let t193 = ((t192).select(t191, f64x8::splat(0.0)));
            let t194 = ((t193).sqrt().sqrt());
            let t197 = (simd::exp(-param_task_c / t194));
            let t199 = ((t192).select(f64x8::splat(1.0) - t197, f64x8::splat(0.0)));
            let t201 = v_tau1 * v_tau1;
            let t202 = t201 * t201;
            let t203 = t202 * t29;
            let t204 = v_rho1 * v_tau1;
            let t208 = f64x8::splat(1.0) / v_rho1;
            let t210 = f64x8::splat(1.0) / v_tau1;
            let t212 = (f64x8::splat(0.0)).simd_lt((f64x8::splat(0.9999999999) * t204 - f64x8::splat(0.125) * v_sigma2) * t208 * t210);
            let t214 = f64x8::splat(8.0) * t204 - v_sigma2;
            let t215 = t214 * t208;
            let t218 = ((t212).select(t215 * t210 / f64x8::splat(8.0), f64x8::splat(1e-10)));
            let t219 = t218 * t218;
            let t220 = t219 * t219;
            let t221 = t60 * t220;
            let t224 = t201 * v_tau1;
            let t225 = t186 * v_rho1;
            let t226 = t224 * t225;
            let t227 = t219 * t218;
            let t231 = t184 * v_rho1;
            let t232 = t185 * t231;
            let t233 = t97 * t232;
            let t234 = t201 * t103;
            let t235 = t234 * t219;
            let t238 = t184 * t184;
            let t239 = t238 * v_rho1;
            let t240 = t239 * v_tau1;
            let t245 = t186 * t238 * t184;
            let t246 = t245 * t121;
            let t249 = f64x8::splat(14580.0) * t111 * t240 * t218 + f64x8::splat(27000.0) * t86 * t226 * t227 + f64x8::splat(6561.0) * t246 * t123 + f64x8::splat(3750.0) * t203 * t221 + f64x8::splat(12150.0) * t233 * t235;
            let t250 = t225 * t85;
            let t252 = v_tau1 * t29;
            let t255 = f64x8::splat(5.0) * t252 * t218 + f64x8::splat(9.0) * t250;
            let t256 = t255 * t255;
            let t257 = t256 * t256;
            let t258 = f64x8::splat(1.0) / t257;
            let t260 = f64x8::splat(1.0) - t249 * t258;
            let t266 = f64x8::splat(48.0) * t146 * t148 * t187;
            let t268 = v_sigma2 * t94 * t153;
            let t271 = f64x8::splat(576.0) * t142 * t185 * t239 + (t266 + t268) * v_sigma2;
            let t275 = f64x8::splat(24.0) * t85 * t187 + t29 * v_sigma2;
            let t276 = t275 * t275;
            let t277 = f64x8::splat(1.0) / t276;
            let t279 = t271 * t277 - param_task_h0x;
            let t280 = t260 * t279;
            let t281 = (simd::pow(t199, param_task_d));
            let t282 = t280 * t281;
            let t283 = param_task_h0x * t199 + t282;
            let t287 = ((t174).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t183 * t283));
            let tzk0 = t173 + t287;
            acc_zk = tzk0;
            let t288 = t7 * t7;
            let t289 = f64x8::splat(1.0) / t288;
            let t290 = t17 * t289;
            let t292 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t290)));
            let t295 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t292));
            let t296 = t295 * t27;
            let t300 = t27 * t27;
            let t301 = f64x8::splat(1.0) / t300;
            let t302 = t26 * t301;
            let t305 = t6 * t302 * t169 / f64x8::splat(8.0);
            let t308 = param_task_c / t45 / t44;
            let t309 = t37 * t98;
            let t310 = f64x8::splat(1.0) / t309;
            let t314 = ((t43).select(-t34 * v_sigma0 * t310 / f64x8::splat(9.0), f64x8::splat(0.0)));
            let t315 = t314 * t48;
            let t318 = ((t43).select(-t308 * t315 / f64x8::splat(4.0), f64x8::splat(0.0)));
            let t320 = t60 * t90;
            let t321 = f64x8::splat(1.0) / t35;
            let t322 = t71 * t321;
            let t326 = ((t69).select(t65 - t322 * t67 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t327 = t320 * t326;
            let t330 = t87 * t37;
            let t334 = t86 * t87;
            let t335 = t88 * t76;
            let t336 = t335 * t326;
            let t339 = t36 * t35;
            let t340 = t97 * t339;
            let t343 = t75 * t326;
            let t344 = t104 * t343;
            let t347 = t112 * v_tau0;
            let t354 = t37 * t113;
            let t358 = f64x8::splat(14580.0) * t111 * t114 * t326 + f64x8::splat(72900.0) * t111 * t347 * t75 + f64x8::splat(43740.0) * t354 * t121 * t123 + f64x8::splat(45000.0) * t86 * t330 * t90 + f64x8::splat(24300.0) * t100 * t344 + f64x8::splat(40500.0) * t340 * t105 + f64x8::splat(15000.0) * t54 * t327 + f64x8::splat(81000.0) * t334 * t336;
            let t361 = f64x8::splat(1.0) / t134 / t132;
            let t362 = t126 * t361;
            let t363 = t37 * t85;
            let t367 = f64x8::splat(5.0) * t129 * t326 + f64x8::splat(15.0) * t363;
            let t370 = -t358 * t135 + f64x8::splat(4.0) * t362 * t367;
            let t372 = t370 * t165 * t167;
            let t376 = t148 * t88;
            let t380 = f64x8::splat(3072.0) * t142 * t36 * t112 + f64x8::splat(128.0) * t146 * t376 * v_sigma0;
            let t383 = f64x8::splat(1.0) / t162 / t161;
            let t384 = t157 * t383;
            let t387 = -f64x8::splat(128.0) * t384 * t127 + t380 * t163;
            let t389 = t137 * t387 * t167;
            let t391 = f64x8::splat(1.0) / t50;
            let t392 = param_task_d * t318 * t391;
            let t394 = t168 * t392 + param_task_h0x * t318 + t372 + t389;
            let t399 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t296 * t169 - t305 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t394));
            let t400 = t175 * t289;
            let t402 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t400)));
            let t405 = ((t179).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t180 * t402));
            let t406 = t405 * t27;
            let t410 = t182 * t301;
            let t413 = t6 * t410 * t283 / f64x8::splat(8.0);
            let t415 = ((t174).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t406 * t283 - t413));
            let tvrho0 = t173 + t287 + t7 * (t399 + t415);
            acc_vrho_0 = tvrho0;
            let t419 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t290)));
            let t422 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t419));
            let t423 = t422 * t27;
            let t428 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t423 * t169 - t305));
            let t430 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t400)));
            let t433 = ((t179).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t180 * t430));
            let t434 = t433 * t27;
            let t440 = param_task_c / t194 / t193;
            let t441 = t186 * t231;
            let t442 = f64x8::splat(1.0) / t441;
            let t446 = ((t192).select(-t34 * v_sigma2 * t442 / f64x8::splat(9.0), f64x8::splat(0.0)));
            let t447 = t446 * t197;
            let t450 = ((t192).select(-t440 * t447 / f64x8::splat(4.0), f64x8::splat(0.0)));
            let t452 = t60 * t227;
            let t453 = f64x8::splat(1.0) / t184;
            let t454 = t214 * t453;
            let t458 = ((t212).select(t208 - t454 * t210 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t459 = t452 * t458;
            let t462 = t224 * t186;
            let t466 = t86 * t224;
            let t467 = t225 * t219;
            let t468 = t467 * t458;
            let t471 = t185 * t184;
            let t472 = t97 * t471;
            let t475 = t218 * t458;
            let t476 = t234 * t475;
            let t479 = t238 * v_tau1;
            let t486 = t186 * t239;
            let t490 = f64x8::splat(72900.0) * t111 * t479 * t218 + f64x8::splat(14580.0) * t111 * t240 * t458 + f64x8::splat(43740.0) * t486 * t121 * t123 + f64x8::splat(45000.0) * t86 * t462 * t227 + f64x8::splat(15000.0) * t203 * t459 + f64x8::splat(24300.0) * t233 * t476 + f64x8::splat(40500.0) * t472 * t235 + f64x8::splat(81000.0) * t466 * t468;
            let t493 = f64x8::splat(1.0) / t257 / t255;
            let t494 = t249 * t493;
            let t495 = t186 * t85;
            let t499 = f64x8::splat(5.0) * t252 * t458 + f64x8::splat(15.0) * t495;
            let t502 = -t490 * t258 + f64x8::splat(4.0) * t494 * t499;
            let t504 = t502 * t279 * t281;
            let t508 = t148 * t225;
            let t512 = f64x8::splat(3072.0) * t142 * t185 * t238 + f64x8::splat(128.0) * t146 * t508 * v_sigma2;
            let t515 = f64x8::splat(1.0) / t276 / t275;
            let t516 = t271 * t515;
            let t519 = -f64x8::splat(128.0) * t516 * t250 + t512 * t277;
            let t521 = t260 * t519 * t281;
            let t523 = f64x8::splat(1.0) / t199;
            let t524 = param_task_d * t450 * t523;
            let t526 = t282 * t524 + param_task_h0x * t450 + t504 + t521;
            let t531 = ((t174).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t434 * t283 - t413 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t183 * t526));
            let tvrho1 = t173 + t287 + t7 * (t428 + t531);
            acc_vrho_1 = tvrho1;
            let t536 = ((t43).select(t34 * t39 / f64x8::splat(24.0), f64x8::splat(0.0)));
            let t537 = t536 * t48;
            let t540 = ((t43).select(-t308 * t537 / f64x8::splat(4.0), f64x8::splat(0.0)));
            let t542 = t65 * t67;
            let t544 = ((t69).select(-t542 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t545 = t320 * t544;
            let t548 = t335 * t544;
            let t551 = t75 * t544;
            let t552 = t104 * t551;
            let t558 = f64x8::splat(14580.0) * t111 * t114 * t544 + f64x8::splat(24300.0) * t100 * t552 + f64x8::splat(81000.0) * t334 * t548 + f64x8::splat(15000.0) * t54 * t545;
            let t560 = t129 * t544;
            let t563 = -t558 * t135 + f64x8::splat(20.0) * t362 * t560;
            let t565 = t563 * t165 * t167;
            let t567 = f64x8::splat(2.0) * t154 + t151;
            let t571 = t567 * t163 - f64x8::splat(2.0) * t384 * t29;
            let t573 = t137 * t571 * t167;
            let t574 = param_task_d * t540;
            let t575 = t574 * t391;
            let t577 = t168 * t575 + param_task_h0x * t540 + t565 + t573;
            let t581 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t577));
            let tvsigma0 = t7 * t581;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t584 = ((t192).select(t34 * t188 / f64x8::splat(24.0), f64x8::splat(0.0)));
            let t585 = t584 * t197;
            let t588 = ((t192).select(-t440 * t585 / f64x8::splat(4.0), f64x8::splat(0.0)));
            let t590 = t208 * t210;
            let t592 = ((t212).select(-t590 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t593 = t452 * t592;
            let t596 = t467 * t592;
            let t599 = t218 * t592;
            let t600 = t234 * t599;
            let t606 = f64x8::splat(14580.0) * t111 * t240 * t592 + f64x8::splat(15000.0) * t203 * t593 + f64x8::splat(24300.0) * t233 * t600 + f64x8::splat(81000.0) * t466 * t596;
            let t608 = t252 * t592;
            let t611 = -t606 * t258 + f64x8::splat(20.0) * t494 * t608;
            let t613 = t611 * t279 * t281;
            let t615 = f64x8::splat(2.0) * t268 + t266;
            let t619 = t615 * t277 - f64x8::splat(2.0) * t516 * t29;
            let t621 = t260 * t619 * t281;
            let t622 = param_task_d * t588;
            let t623 = t622 * t523;
            let t625 = t282 * t623 + param_task_h0x * t588 + t613 + t621;
            let t629 = ((t174).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t183 * t625));
            let tvsigma2 = t7 * t629;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t630 = t6 * t26;
            let t631 = t87 * t29;
            let t634 = f64x8::splat(1.0) / t52;
            let t638 = ((t69).select(t67 - t72 * t634 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t639 = t320 * t638;
            let t642 = t52 * t88;
            let t646 = t335 * t638;
            let t649 = v_tau0 * t103;
            let t650 = t649 * t76;
            let t653 = t75 * t638;
            let t654 = t104 * t653;
            let t657 = t29 * t113;
            let t664 = f64x8::splat(14580.0) * t110 * t657 * t75 + f64x8::splat(14580.0) * t111 * t114 * t638 + f64x8::splat(81000.0) * t86 * t642 * t90 + f64x8::splat(24300.0) * t100 * t650 + f64x8::splat(24300.0) * t100 * t654 + f64x8::splat(81000.0) * t334 * t646 + f64x8::splat(15000.0) * t54 * t639 + f64x8::splat(15000.0) * t631 * t78;
            let t669 = f64x8::splat(5.0) * t129 * t638 + f64x8::splat(5.0) * t29 * t75;
            let t672 = -t664 * t135 + f64x8::splat(4.0) * t362 * t669;
            let t673 = t27 * t672;
            let t674 = t165 * t167;
            let t675 = t673 * t674;
            let t678 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t630 * t675));
            let tvtau0 = t7 * t678;
            acc_vtau_0 = tvtau0;
            let t679 = t6 * t182;
            let t680 = t224 * t29;
            let t683 = f64x8::splat(1.0) / t201;
            let t687 = ((t212).select(t210 - t215 * t683 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t688 = t452 * t687;
            let t691 = t201 * t225;
            let t695 = t467 * t687;
            let t698 = v_tau1 * t103;
            let t699 = t698 * t219;
            let t702 = t218 * t687;
            let t703 = t234 * t702;
            let t706 = t29 * t239;
            let t713 = f64x8::splat(14580.0) * t110 * t706 * t218 + f64x8::splat(14580.0) * t111 * t240 * t687 + f64x8::splat(81000.0) * t86 * t691 * t227 + f64x8::splat(15000.0) * t203 * t688 + f64x8::splat(15000.0) * t680 * t221 + f64x8::splat(24300.0) * t233 * t699 + f64x8::splat(24300.0) * t233 * t703 + f64x8::splat(81000.0) * t466 * t695;
            let t718 = f64x8::splat(5.0) * t29 * t218 + f64x8::splat(5.0) * t252 * t687;
            let t721 = -t713 * t258 + f64x8::splat(4.0) * t494 * t718;
            let t722 = t27 * t721;
            let t723 = t279 * t281;
            let t724 = t722 * t723;
            let t727 = ((t174).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t679 * t724));
            let tvtau1 = t7 * t727;
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
