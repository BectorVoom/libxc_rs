//! GGA_X_SSB_SW vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ssb_sw.c`
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
pub fn gga_x_ssb_sw_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_A = f64x8::splat(param_A);
    let param_B = f64x8::splat(param_B);
    let param_C = f64x8::splat(param_C);
    let param_D = f64x8::splat(param_D);
    let param_E = f64x8::splat(param_E);
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
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = param_B * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t29 * t33;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t40 = v_sigma0 * t39;
            let t41 = param_C * t28;
            let t42 = t33 * v_sigma0;
            let t46 = f64x8::splat(1.0) + t41 * t42 * t39 / f64x8::splat(24.0);
            let t47 = f64x8::splat(1.0) / t46;
            let t51 = param_D * t28;
            let t52 = t51 * t33;
            let t53 = t28 * t28;
            let t54 = param_E * t53;
            let t56 = f64x8::splat(1.0) / t31 / t30;
            let t57 = v_sigma0 * v_sigma0;
            let t59 = t35 * t35;
            let t60 = t59 * v_rho0;
            let t62 = f64x8::splat(1.0) / t36 / t60;
            let t66 = f64x8::splat(1.0) + t54 * t56 * t57 * t62 / f64x8::splat(576.0);
            let t67 = f64x8::splat(1.0) / t66;
            let t71 = param_A + t34 * t40 * t47 / f64x8::splat(24.0) - t52 * t40 * t67 / f64x8::splat(24.0);
            let t75 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t71));
            let t76 = (v_rho1).simd_le(dens_threshold);
            let t77 = -t16;
            let t79 = ((t14).select(t11, (t10).select(t15, t77 * t7)));
            let t80 = f64x8::splat(1.0) + t79;
            let t81 = (t80).simd_le(zeta_threshold);
            let t82 = (simd::cbrt(t80));
            let t84 = ((t81).select(t22, t82 * t80));
            let t85 = t84 * t26;
            let t86 = v_rho1 * v_rho1;
            let t87 = (simd::cbrt(v_rho1));
            let t88 = t87 * t87;
            let t90 = f64x8::splat(1.0) / t88 / t86;
            let t91 = v_sigma2 * t90;
            let t92 = t33 * v_sigma2;
            let t96 = f64x8::splat(1.0) + t41 * t92 * t90 / f64x8::splat(24.0);
            let t97 = f64x8::splat(1.0) / t96;
            let t101 = v_sigma2 * v_sigma2;
            let t103 = t86 * t86;
            let t104 = t103 * v_rho1;
            let t106 = f64x8::splat(1.0) / t87 / t104;
            let t110 = f64x8::splat(1.0) + t54 * t56 * t101 * t106 / f64x8::splat(576.0);
            let t111 = f64x8::splat(1.0) / t110;
            let t115 = param_A + t34 * t91 * t97 / f64x8::splat(24.0) - t52 * t91 * t111 / f64x8::splat(24.0);
            let t119 = ((t76).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t85 * t115));
            let tzk0 = t75 + t119;
            acc_zk = tzk0;
            let t120 = t6 * t6;
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t16 * t121;
            let t124 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t122)));
            let t127 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t124));
            let t128 = t127 * t26;
            let t132 = t26 * t26;
            let t133 = f64x8::splat(1.0) / t132;
            let t134 = t25 * t133;
            let t137 = t5 * t134 * t71 / f64x8::splat(8.0);
            let t138 = t35 * v_rho0;
            let t140 = f64x8::splat(1.0) / t37 / t138;
            let t141 = v_sigma0 * t140;
            let t146 = param_B * t53 * t56;
            let t147 = t59 * t35;
            let t149 = f64x8::splat(1.0) / t36 / t147;
            let t151 = t46 * t46;
            let t152 = f64x8::splat(1.0) / t151;
            let t153 = t152 * param_C;
            let t160 = t30 * t30;
            let t161 = f64x8::splat(1.0) / t160;
            let t162 = param_D * t161;
            let t163 = t57 * v_sigma0;
            let t164 = t162 * t163;
            let t165 = t59 * t59;
            let t166 = t165 * v_rho0;
            let t167 = f64x8::splat(1.0) / t166;
            let t168 = t66 * t66;
            let t169 = f64x8::splat(1.0) / t168;
            let t171 = t167 * t169 * param_E;
            let t174 = -t34 * t141 * t47 / f64x8::splat(9.0) + t146 * t57 * t149 * t153 / f64x8::splat(216.0) + t52 * t141 * t67 / f64x8::splat(9.0) - t164 * t171 / f64x8::splat(432.0);
            let t179 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t128 * t71 - t137 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t174));
            let t180 = t77 * t121;
            let t182 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t180)));
            let t185 = ((t81).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t82 * t182));
            let t186 = t185 * t26;
            let t190 = t84 * t133;
            let t193 = t5 * t190 * t115 / f64x8::splat(8.0);
            let t195 = ((t76).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t186 * t115 - t193));
            let tvrho0 = t75 + t119 + t6 * (t179 + t195);
            acc_vrho_0 = tvrho0;
            let t199 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t122)));
            let t202 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t199));
            let t203 = t202 * t26;
            let t208 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t203 * t71 - t137));
            let t210 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t180)));
            let t213 = ((t81).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t82 * t210));
            let t214 = t213 * t26;
            let t218 = t86 * v_rho1;
            let t220 = f64x8::splat(1.0) / t88 / t218;
            let t221 = v_sigma2 * t220;
            let t225 = t103 * t86;
            let t227 = f64x8::splat(1.0) / t87 / t225;
            let t229 = t96 * t96;
            let t230 = f64x8::splat(1.0) / t229;
            let t231 = t230 * param_C;
            let t238 = t101 * v_sigma2;
            let t239 = t162 * t238;
            let t240 = t103 * t103;
            let t241 = t240 * v_rho1;
            let t242 = f64x8::splat(1.0) / t241;
            let t243 = t110 * t110;
            let t244 = f64x8::splat(1.0) / t243;
            let t246 = t242 * t244 * param_E;
            let t249 = -t34 * t221 * t97 / f64x8::splat(9.0) + t146 * t101 * t227 * t231 / f64x8::splat(216.0) + t52 * t221 * t111 / f64x8::splat(9.0) - t239 * t246 / f64x8::splat(432.0);
            let t254 = ((t76).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t214 * t115 - t193 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t85 * t249));
            let tvrho1 = t75 + t119 + t6 * (t208 + t254);
            acc_vrho_1 = tvrho1;
            let t257 = t33 * t39;
            let t269 = f64x8::splat(1.0) / t165;
            let t271 = t269 * t169 * param_E;
            let t274 = t29 * t257 * t47 / f64x8::splat(24.0) - t146 * v_sigma0 * t62 * t153 / f64x8::splat(576.0) - t51 * t257 * t67 / f64x8::splat(24.0) + t162 * t57 * t271 / f64x8::splat(1152.0);
            let t278 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t274));
            let tvsigma0 = t6 * t278;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t279 = t33 * t90;
            let t291 = f64x8::splat(1.0) / t240;
            let t293 = t291 * t244 * param_E;
            let t296 = t29 * t279 * t97 / f64x8::splat(24.0) - t146 * v_sigma2 * t106 * t231 / f64x8::splat(576.0) - t51 * t279 * t111 / f64x8::splat(24.0) + t162 * t101 * t293 / f64x8::splat(1152.0);
            let t300 = ((t76).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t85 * t296));
            let tvsigma2 = t6 * t300;
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
