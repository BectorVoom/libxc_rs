//! HYB_MGGA_X_M05 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_m05.c`
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
pub fn hyb_mgga_x_m05_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_csi_HF: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_a_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_csi_HF = f64x8::splat(param_csi_HF);
    let param_a_1 = f64x8::splat(param_a_1);
    let param_a_2 = f64x8::splat(param_a_2);
    let param_a_3 = f64x8::splat(param_a_3);
    let param_a_4 = f64x8::splat(param_a_4);
    let param_a_5 = f64x8::splat(param_a_5);
    let param_a_6 = f64x8::splat(param_a_6);
    let param_a_7 = f64x8::splat(param_a_7);
    let param_a_8 = f64x8::splat(param_a_8);
    let param_a_9 = f64x8::splat(param_a_9);
    let param_a_10 = f64x8::splat(param_a_10);
    let param_a_11 = f64x8::splat(param_a_11);
    let param_a_0 = f64x8::splat(param_a_0);
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
            let t29 = t28 * param_csi_HF;
            let t30 = f64x8::splat(M_CBRT6);
            let t31 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t32 = (simd::cbrt(t31));
            let t33 = t32 * t32;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t30 * t34;
            let t36 = v_rho0 * v_rho0;
            let t37 = (simd::cbrt(v_rho0));
            let t38 = t37 * t37;
            let t40 = f64x8::splat(1.0) / t38 / t36;
            let t44 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t35 * v_sigma0 * t40;
            let t47 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t44;
            let t48 = param_a_0;
            let t49 = param_a_1;
            let t50 = t30 * t30;
            let t52 = f64x8::splat(3.0) / f64x8::splat(10.0) * t50 * t33;
            let t54 = f64x8::splat(1.0) / t38 / v_rho0;
            let t55 = v_tau0 * t54;
            let t56 = t52 - t55;
            let t57 = t49 * t56;
            let t58 = t52 + t55;
            let t59 = f64x8::splat(1.0) / t58;
            let t61 = param_a_2;
            let t62 = t56 * t56;
            let t63 = t61 * t62;
            let t64 = t58 * t58;
            let t65 = f64x8::splat(1.0) / t64;
            let t67 = param_a_3;
            let t68 = t62 * t56;
            let t69 = t67 * t68;
            let t70 = t64 * t58;
            let t71 = f64x8::splat(1.0) / t70;
            let t73 = param_a_4;
            let t74 = t62 * t62;
            let t75 = t73 * t74;
            let t76 = t64 * t64;
            let t77 = f64x8::splat(1.0) / t76;
            let t79 = param_a_5;
            let t80 = t74 * t56;
            let t81 = t79 * t80;
            let t82 = t76 * t58;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = param_a_6;
            let t86 = t74 * t62;
            let t87 = t85 * t86;
            let t88 = t76 * t64;
            let t89 = f64x8::splat(1.0) / t88;
            let t91 = param_a_7;
            let t92 = t74 * t68;
            let t93 = t91 * t92;
            let t94 = t76 * t70;
            let t95 = f64x8::splat(1.0) / t94;
            let t97 = param_a_8;
            let t98 = t74 * t74;
            let t99 = t97 * t98;
            let t100 = t76 * t76;
            let t101 = f64x8::splat(1.0) / t100;
            let t103 = param_a_9;
            let t104 = t98 * t56;
            let t105 = t103 * t104;
            let t107 = f64x8::splat(1.0) / t100 / t58;
            let t109 = param_a_10;
            let t110 = t98 * t62;
            let t111 = t109 * t110;
            let t113 = f64x8::splat(1.0) / t100 / t64;
            let t115 = param_a_11;
            let t117 = t115 * t98 * t68;
            let t119 = f64x8::splat(1.0) / t100 / t70;
            let t121 = t99 * t101 + t105 * t107 + t111 * t113 + t117 * t119 + t57 * t59 + t63 * t65 + t69 * t71 + t75 * t77 + t81 * t83 + t87 * t89 + t93 * t95 + t48;
            let t122 = t47 * t121;
            let t123 = t29 * t122;
            let t126 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t123));
            let t127 = (v_rho1).simd_le(dens_threshold);
            let t128 = -t17;
            let t130 = ((t15).select(t12, (t11).select(t16, t128 * t8)));
            let t131 = f64x8::splat(1.0) + t130;
            let t132 = (t131).simd_le(zeta_threshold);
            let t133 = (simd::cbrt(t131));
            let t135 = ((t132).select(t23, t133 * t131));
            let t136 = t6 * t135;
            let t137 = v_rho1 * v_rho1;
            let t138 = (simd::cbrt(v_rho1));
            let t139 = t138 * t138;
            let t141 = f64x8::splat(1.0) / t139 / t137;
            let t145 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t35 * v_sigma2 * t141;
            let t148 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t145;
            let t150 = f64x8::splat(1.0) / t139 / v_rho1;
            let t151 = v_tau1 * t150;
            let t152 = t52 - t151;
            let t153 = t49 * t152;
            let t154 = t52 + t151;
            let t155 = f64x8::splat(1.0) / t154;
            let t157 = t152 * t152;
            let t158 = t61 * t157;
            let t159 = t154 * t154;
            let t160 = f64x8::splat(1.0) / t159;
            let t162 = t157 * t152;
            let t163 = t67 * t162;
            let t164 = t159 * t154;
            let t165 = f64x8::splat(1.0) / t164;
            let t167 = t157 * t157;
            let t168 = t73 * t167;
            let t169 = t159 * t159;
            let t170 = f64x8::splat(1.0) / t169;
            let t172 = t167 * t152;
            let t173 = t79 * t172;
            let t174 = t169 * t154;
            let t175 = f64x8::splat(1.0) / t174;
            let t177 = t167 * t157;
            let t178 = t85 * t177;
            let t179 = t169 * t159;
            let t180 = f64x8::splat(1.0) / t179;
            let t182 = t167 * t162;
            let t183 = t91 * t182;
            let t184 = t169 * t164;
            let t185 = f64x8::splat(1.0) / t184;
            let t187 = t167 * t167;
            let t188 = t97 * t187;
            let t189 = t169 * t169;
            let t190 = f64x8::splat(1.0) / t189;
            let t192 = t187 * t152;
            let t193 = t103 * t192;
            let t195 = f64x8::splat(1.0) / t189 / t154;
            let t197 = t187 * t157;
            let t198 = t109 * t197;
            let t200 = f64x8::splat(1.0) / t189 / t159;
            let t203 = t115 * t187 * t162;
            let t205 = f64x8::splat(1.0) / t189 / t164;
            let t207 = t153 * t155 + t158 * t160 + t163 * t165 + t168 * t170 + t173 * t175 + t178 * t180 + t183 * t185 + t188 * t190 + t193 * t195 + t198 * t200 + t203 * t205 + t48;
            let t208 = t148 * t207;
            let t209 = t29 * t208;
            let t212 = ((t127).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t136 * t209));
            let tzk0 = t126 + t212;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
