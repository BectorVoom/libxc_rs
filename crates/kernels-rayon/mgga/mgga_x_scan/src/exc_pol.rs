//! MGGA_X_SCAN exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_scan.c`
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
pub fn mgga_x_scan_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
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
            let t77 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau0 * t71 - t40 / f64x8::splat(8.0)) * t29 * t33;
            let t78 = f64x8::splat(1.0) - t77;
            let t80 = t78 * t78;
            let t82 = (simd::exp(-t80 / f64x8::splat(2.0)));
            let t85 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t59 + t66 * t78 * t82 / f64x8::splat(100.0);
            let t86 = t85 * t85;
            let t87 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t41 + t50 * t56 * t62 / f64x8::splat(576.0) + t86;
            let t92 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t87);
            let t93 = (t77).simd_le(f64x8::splat(1.0));
            let t94 = (simd::ln(f64x8::splat(f64::EPSILON)));
            let t97 = t94 / (-t94 + param_c1);
            let t98 = (-t97).simd_lt(t77);
            let t99 = (t77).simd_lt(-t97);
            let t100 = ((t99).select(t77, -t97));
            let t101 = param_c1 * t100;
            let t102 = f64x8::splat(1.0) - t100;
            let t103 = f64x8::splat(1.0) / t102;
            let t105 = (simd::exp(-t101 * t103));
            let t106 = ((t98).select(f64x8::splat(0.0), t105));
            let t107 = ((param_d).abs());
            let t110 = (simd::ln(f64x8::splat(f64::EPSILON) / t107));
            let t113 = (-t110 + param_c2) / t110;
            let t114 = (t77).simd_lt(-t113);
            let t115 = ((t114).select(-t113, t77));
            let t116 = f64x8::splat(1.0) - t115;
            let t119 = (simd::exp(param_c2 / t116));
            let t121 = ((t114).select(f64x8::splat(0.0), -param_d * t119));
            let t122 = ((t93).select(t106, t121));
            let t123 = f64x8::splat(1.0) - t122;
            let t126 = t92 * t123 + f64x8::splat(1.174) * t122;
            let t127 = t28 * t126;
            let t128 = ((f64x8::splat(3.0)).sqrt());
            let t129 = f64x8::splat(1.0) / t31;
            let t130 = t46 * t129;
            let t131 = ((v_sigma0).sqrt());
            let t132 = t36 * v_rho0;
            let t133 = f64x8::splat(1.0) / t132;
            let t135 = t130 * t131 * t133;
            let t136 = ((t135).sqrt());
            let t140 = (simd::exp(-f64x8::splat(9.8958) * t128 / t136));
            let t141 = f64x8::splat(1.0) - t140;
            let t142 = t127 * t141;
            let t145 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t142));
            let t146 = (v_rho1).simd_le(dens_threshold);
            let t147 = -t17;
            let t149 = ((t15).select(t12, (t11).select(t16, t147 * t8)));
            let t150 = f64x8::splat(1.0) + t149;
            let t151 = (t150).simd_le(zeta_threshold);
            let t152 = (simd::cbrt(t150));
            let t154 = ((t151).select(t23, t152 * t150));
            let t155 = t6 * t154;
            let t156 = v_rho1 * v_rho1;
            let t157 = (simd::cbrt(v_rho1));
            let t158 = t157 * t157;
            let t159 = t158 * t156;
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = v_sigma2 * t160;
            let t162 = t34 * t161;
            let t164 = v_sigma2 * v_sigma2;
            let t165 = t156 * t156;
            let t166 = t165 * v_rho1;
            let t168 = f64x8::splat(1.0) / t157 / t166;
            let t169 = t164 * t168;
            let t170 = t33 * v_sigma2;
            let t171 = t170 * t160;
            let t174 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t57 * t171));
            let t180 = t158 * v_rho1;
            let t181 = f64x8::splat(1.0) / t180;
            let t187 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau1 * t181 - t161 / f64x8::splat(8.0)) * t29 * t33;
            let t188 = f64x8::splat(1.0) - t187;
            let t190 = t188 * t188;
            let t192 = (simd::exp(-t190 / f64x8::splat(2.0)));
            let t195 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t171 + t66 * t188 * t192 / f64x8::splat(100.0);
            let t196 = t195 * t195;
            let t197 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t162 + t50 * t169 * t174 / f64x8::splat(576.0) + t196;
            let t202 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t197);
            let t203 = (t187).simd_le(f64x8::splat(1.0));
            let t204 = (-t97).simd_lt(t187);
            let t205 = (t187).simd_lt(-t97);
            let t206 = ((t205).select(t187, -t97));
            let t207 = param_c1 * t206;
            let t208 = f64x8::splat(1.0) - t206;
            let t209 = f64x8::splat(1.0) / t208;
            let t211 = (simd::exp(-t207 * t209));
            let t212 = ((t204).select(f64x8::splat(0.0), t211));
            let t213 = (t187).simd_lt(-t113);
            let t214 = ((t213).select(-t113, t187));
            let t215 = f64x8::splat(1.0) - t214;
            let t218 = (simd::exp(param_c2 / t215));
            let t220 = ((t213).select(f64x8::splat(0.0), -param_d * t218));
            let t221 = ((t203).select(t212, t220));
            let t222 = f64x8::splat(1.0) - t221;
            let t225 = t202 * t222 + f64x8::splat(1.174) * t221;
            let t226 = t28 * t225;
            let t227 = ((v_sigma2).sqrt());
            let t228 = t157 * v_rho1;
            let t229 = f64x8::splat(1.0) / t228;
            let t231 = t130 * t227 * t229;
            let t232 = ((t231).sqrt());
            let t236 = (simd::exp(-f64x8::splat(9.8958) * t128 / t232));
            let t237 = f64x8::splat(1.0) - t236;
            let t238 = t226 * t237;
            let t241 = ((t146).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t155 * t238));
            let tzk0 = t145 + t241;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
