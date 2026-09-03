//! MGGA_X_TASK exc pol kernel — explicit SIMD (bit-exact).
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
pub fn mgga_x_task_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
