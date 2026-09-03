//! LDA_X_YUKAWA exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_yukawa.c`
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
pub fn lda_x_yukawa_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t3 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = t6 * t3 * t1;
            let t8 = f64x8::splat(M_CBRT2);
            let t9 = t8 * t8;
            let t10 = v_rho0 - v_rho1;
            let t11 = v_rho0 + v_rho1;
            let t12 = f64x8::splat(1.0) / t11;
            let t13 = t12 * t10;
            let t14 = f64x8::splat(1.0) + t13;
            let t15 = (t14).simd_le(zeta_threshold);
            let t16 = (simd::cbrt(zeta_threshold));
            let t17 = t16 * zeta_threshold;
            let t18 = (simd::cbrt(t14));
            let t20 = ((t15).select(t17, t18 * t14));
            let t21 = t20 * t9;
            let t22 = (simd::cbrt(t11));
            let t23 = (simd::cbrt(f64x8::splat(9.0)));
            let t24 = t23 * t23;
            let t25 = t3 * t3;
            let t26 = t25 * t24;
            let t27 = param_hyb_omega_0 * t26;
            let t28 = f64x8::splat(1.0) / t22;
            let t29 = t28 * t1;
            let t30 = ((t15).select(t16, t18));
            let t31 = f64x8::splat(1.0) / t30;
            let t34 = t31 * t29 * t27 / f64x8::splat(18.0);
            let t35 = (f64x8::splat(1.92)).simd_le(t34);
            let t36 = (f64x8::splat(1.92)).simd_lt(t34);
            let t37 = ((t36).select(t34, f64x8::splat(1.92)));
            let t38 = t37 * t37;
            let t41 = t38 * t38;
            let t42 = f64x8::splat(1.0) / t41;
            let t44 = t41 * t38;
            let t45 = f64x8::splat(1.0) / t44;
            let t47 = t41 * t41;
            let t48 = f64x8::splat(1.0) / t47;
            let t50 = t47 * t38;
            let t51 = f64x8::splat(1.0) / t50;
            let t53 = t47 * t41;
            let t54 = f64x8::splat(1.0) / t53;
            let t56 = t47 * t44;
            let t57 = f64x8::splat(1.0) / t56;
            let t59 = t47 * t47;
            let t60 = f64x8::splat(1.0) / t59;
            let t63 = f64x8::splat(1.0) / t59 / t38;
            let t66 = f64x8::splat(1.0) / t59 / t41;
            let t69 = f64x8::splat(1.0) / t59 / t44;
            let t72 = f64x8::splat(1.0) / t59 / t47;
            let t75 = f64x8::splat(1.0) / t59 / t50;
            let t78 = f64x8::splat(1.0) / t59 / t53;
            let t81 = f64x8::splat(1.0) / t59 / t56;
            let t83 = t59 * t59;
            let t84 = f64x8::splat(1.0) / t83;
            let t87 = f64x8::splat(1.0) / t83 / t38;
            let t90 = f64x8::splat(1.0) / t83 / t41;
            let t92 = f64x8::splat(1.0) / t38 / f64x8::splat(9.0) - t42 / f64x8::splat(30.0) + t45 / f64x8::splat(70.0) - t48 / f64x8::splat(135.0) + t51 / f64x8::splat(231.0) - t54 / f64x8::splat(364.0) + t57 / f64x8::splat(540.0) - t60 / f64x8::splat(765.0) + t63 / f64x8::splat(1045.0) - t66 / f64x8::splat(1386.0) + t69 / f64x8::splat(1794.0) - t72 / f64x8::splat(2275.0) + t75 / f64x8::splat(2835.0) - t78 / f64x8::splat(3480.0) + t81 / f64x8::splat(4216.0) - t84 / f64x8::splat(5049.0) + t87 / f64x8::splat(5985.0) - t90 / f64x8::splat(7030.0);
            let t93 = ((t36).select(f64x8::splat(1.92), t34));
            let t94 = (simd::atan2(f64x8::splat(1.0), t93));
            let t95 = t93 * t93;
            let t96 = t95 + f64x8::splat(3.0);
            let t97 = f64x8::splat(1.0) / t95;
            let t98 = f64x8::splat(1.0) + t97;
            let t99 = (simd::ln(t98));
            let t101 = -t99 * t96 + f64x8::splat(1.0);
            let t104 = t94 + t101 * t93 / f64x8::splat(4.0);
            let t108 = ((t35).select(t92, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t104 * t93));
            let t109 = t108 * t22;
            let t111 = t109 * t21 * t7;
            let t112 = f64x8::splat(1.0) - t13;
            let t113 = (t112).simd_le(zeta_threshold);
            let t114 = (simd::cbrt(t112));
            let t116 = ((t113).select(t17, t114 * t112));
            let t117 = t116 * t9;
            let t118 = ((t113).select(t16, t114));
            let t119 = f64x8::splat(1.0) / t118;
            let t122 = t119 * t29 * t27 / f64x8::splat(18.0);
            let t123 = (f64x8::splat(1.92)).simd_le(t122);
            let t124 = (f64x8::splat(1.92)).simd_lt(t122);
            let t125 = ((t124).select(t122, f64x8::splat(1.92)));
            let t126 = t125 * t125;
            let t129 = t126 * t126;
            let t130 = f64x8::splat(1.0) / t129;
            let t132 = t129 * t126;
            let t133 = f64x8::splat(1.0) / t132;
            let t135 = t129 * t129;
            let t136 = f64x8::splat(1.0) / t135;
            let t138 = t135 * t126;
            let t139 = f64x8::splat(1.0) / t138;
            let t141 = t135 * t129;
            let t142 = f64x8::splat(1.0) / t141;
            let t144 = t135 * t132;
            let t145 = f64x8::splat(1.0) / t144;
            let t147 = t135 * t135;
            let t148 = f64x8::splat(1.0) / t147;
            let t151 = f64x8::splat(1.0) / t147 / t126;
            let t154 = f64x8::splat(1.0) / t147 / t129;
            let t157 = f64x8::splat(1.0) / t147 / t132;
            let t160 = f64x8::splat(1.0) / t147 / t135;
            let t163 = f64x8::splat(1.0) / t147 / t138;
            let t166 = f64x8::splat(1.0) / t147 / t141;
            let t169 = f64x8::splat(1.0) / t147 / t144;
            let t171 = t147 * t147;
            let t172 = f64x8::splat(1.0) / t171;
            let t175 = f64x8::splat(1.0) / t171 / t126;
            let t178 = f64x8::splat(1.0) / t171 / t129;
            let t180 = f64x8::splat(1.0) / t126 / f64x8::splat(9.0) - t130 / f64x8::splat(30.0) + t133 / f64x8::splat(70.0) - t136 / f64x8::splat(135.0) + t139 / f64x8::splat(231.0) - t142 / f64x8::splat(364.0) + t145 / f64x8::splat(540.0) - t148 / f64x8::splat(765.0) + t151 / f64x8::splat(1045.0) - t154 / f64x8::splat(1386.0) + t157 / f64x8::splat(1794.0) - t160 / f64x8::splat(2275.0) + t163 / f64x8::splat(2835.0) - t166 / f64x8::splat(3480.0) + t169 / f64x8::splat(4216.0) - t172 / f64x8::splat(5049.0) + t175 / f64x8::splat(5985.0) - t178 / f64x8::splat(7030.0);
            let t181 = ((t124).select(f64x8::splat(1.92), t122));
            let t182 = (simd::atan2(f64x8::splat(1.0), t181));
            let t183 = t181 * t181;
            let t184 = t183 + f64x8::splat(3.0);
            let t185 = f64x8::splat(1.0) / t183;
            let t186 = f64x8::splat(1.0) + t185;
            let t187 = (simd::ln(t186));
            let t189 = -t187 * t184 + f64x8::splat(1.0);
            let t192 = t182 + t189 * t181 / f64x8::splat(4.0);
            let t196 = ((t123).select(t180, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t192 * t181));
            let t197 = t196 * t22;
            let t199 = t197 * t117 * t7;
            let tzk0 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t111 - f64x8::splat(3.0) / f64x8::splat(32.0) * t199;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
