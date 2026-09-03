//! HYB_LDA_XC_BN05 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/hyb_lda_xc_bn05.c`
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
pub fn hyb_lda_xc_bn05_exc_pol(
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
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t3 * t1;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = t6 * t4;
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
            let t101 = -t96 * t99 + f64x8::splat(1.0);
            let t104 = t94 + t101 * t93 / f64x8::splat(4.0);
            let t108 = ((t35).select(t92, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t104 * t93));
            let t109 = t108 * t22;
            let t112 = f64x8::splat(3.0) / f64x8::splat(32.0) * t109 * t21 * t7;
            let t113 = f64x8::splat(1.0) - t13;
            let t114 = (t113).simd_le(zeta_threshold);
            let t115 = (simd::cbrt(t113));
            let t117 = ((t114).select(t17, t115 * t113));
            let t118 = t117 * t9;
            let t119 = ((t114).select(t16, t115));
            let t120 = f64x8::splat(1.0) / t119;
            let t123 = t120 * t29 * t27 / f64x8::splat(18.0);
            let t124 = (f64x8::splat(1.92)).simd_le(t123);
            let t125 = (f64x8::splat(1.92)).simd_lt(t123);
            let t126 = ((t125).select(t123, f64x8::splat(1.92)));
            let t127 = t126 * t126;
            let t130 = t127 * t127;
            let t131 = f64x8::splat(1.0) / t130;
            let t133 = t130 * t127;
            let t134 = f64x8::splat(1.0) / t133;
            let t136 = t130 * t130;
            let t137 = f64x8::splat(1.0) / t136;
            let t139 = t136 * t127;
            let t140 = f64x8::splat(1.0) / t139;
            let t142 = t136 * t130;
            let t143 = f64x8::splat(1.0) / t142;
            let t145 = t136 * t133;
            let t146 = f64x8::splat(1.0) / t145;
            let t148 = t136 * t136;
            let t149 = f64x8::splat(1.0) / t148;
            let t152 = f64x8::splat(1.0) / t148 / t127;
            let t155 = f64x8::splat(1.0) / t148 / t130;
            let t158 = f64x8::splat(1.0) / t148 / t133;
            let t161 = f64x8::splat(1.0) / t148 / t136;
            let t164 = f64x8::splat(1.0) / t148 / t139;
            let t167 = f64x8::splat(1.0) / t148 / t142;
            let t170 = f64x8::splat(1.0) / t148 / t145;
            let t172 = t148 * t148;
            let t173 = f64x8::splat(1.0) / t172;
            let t176 = f64x8::splat(1.0) / t172 / t127;
            let t179 = f64x8::splat(1.0) / t172 / t130;
            let t181 = f64x8::splat(1.0) / t127 / f64x8::splat(9.0) - t131 / f64x8::splat(30.0) + t134 / f64x8::splat(70.0) - t137 / f64x8::splat(135.0) + t140 / f64x8::splat(231.0) - t143 / f64x8::splat(364.0) + t146 / f64x8::splat(540.0) - t149 / f64x8::splat(765.0) + t152 / f64x8::splat(1045.0) - t155 / f64x8::splat(1386.0) + t158 / f64x8::splat(1794.0) - t161 / f64x8::splat(2275.0) + t164 / f64x8::splat(2835.0) - t167 / f64x8::splat(3480.0) + t170 / f64x8::splat(4216.0) - t173 / f64x8::splat(5049.0) + t176 / f64x8::splat(5985.0) - t179 / f64x8::splat(7030.0);
            let t182 = ((t125).select(f64x8::splat(1.92), t123));
            let t183 = (simd::atan2(f64x8::splat(1.0), t182));
            let t184 = t182 * t182;
            let t185 = t184 + f64x8::splat(3.0);
            let t186 = f64x8::splat(1.0) / t184;
            let t187 = f64x8::splat(1.0) + t186;
            let t188 = (simd::ln(t187));
            let t190 = -t185 * t188 + f64x8::splat(1.0);
            let t193 = t183 + t190 * t182 / f64x8::splat(4.0);
            let t197 = ((t124).select(t181, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t193 * t182));
            let t198 = t197 * t22;
            let t201 = f64x8::splat(3.0) / f64x8::splat(32.0) * t198 * t118 * t7;
            let t203 = t28 * t6 * t4;
            let t205 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t203;
            let t206 = ((t203).sqrt());
            let t209 = ((t203) * (t203).sqrt());
            let t211 = t1 * t1;
            let t212 = t25 * t211;
            let t213 = t22 * t22;
            let t214 = f64x8::splat(1.0) / t213;
            let t216 = t214 * t5 * t212;
            let t218 = f64x8::splat(3.79785) * t206 + f64x8::splat(0.8969) * t203 + f64x8::splat(0.204775) * t209 + f64x8::splat(0.123235) * t216;
            let t221 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t218;
            let t222 = (simd::ln(t221));
            let t224 = f64x8::splat(0.0621814) * t222 * t205;
            let t225 = t10 * t10;
            let t226 = t225 * t225;
            let t227 = t11 * t11;
            let t228 = t227 * t227;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t229 * t226;
            let t231 = t20 + t117 - f64x8::splat(2.0);
            let t234 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t8 - f64x8::splat(2.0));
            let t235 = t234 * t231;
            let t237 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t203;
            let t242 = f64x8::splat(7.05945) * t206 + f64x8::splat(1.549425) * t203 + f64x8::splat(0.420775) * t209 + f64x8::splat(0.1562925) * t216;
            let t245 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t242;
            let t246 = (simd::ln(t245));
            let t250 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t203;
            let t255 = f64x8::splat(5.1785) * t206 + f64x8::splat(0.905775) * t203 + f64x8::splat(0.1100325) * t209 + f64x8::splat(0.1241775) * t216;
            let t258 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t255;
            let t259 = (simd::ln(t258));
            let t260 = t259 * t250;
            let t262 = -f64x8::splat(0.0310907) * t246 * t237 + t224 - f64x8::splat(0.0197516734986138) * t260;
            let t263 = t262 * t235;
            let t267 = -t224 + t263 * t230 + f64x8::splat(0.0197516734986138) * t260 * t235;
            let t270 = f64x8::splat(3.2) - f64x8::splat(0.225) * t203 + t216 / f64x8::splat(4.0);
            let t271 = f64x8::splat(1.0) / t270;
            let t273 = f64x8::splat(3.4602) * t271 * t267;
            let tzk0 = -t112 - t201 + t273;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
