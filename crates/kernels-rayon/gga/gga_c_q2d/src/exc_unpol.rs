//! GGA_C_Q2D exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_q2d.c`
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

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_q2d_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t1 = v_sigma * v_sigma;
            let t2 = v_rho * v_rho;
            let t3 = t2 * t2;
            let t4 = (simd::cbrt(v_rho));
            let t5 = t4 * t4;
            let t7 = f64x8::splat(1.0) / t5 / t3;
            let t8 = t1 * t7;
            let t9 = f64x8::splat(M_CBRT2);
            let t10 = t9 * t9;
            let t11 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t12 = (simd::cbrt(zeta_threshold));
            let t13 = t12 * t12;
            let t14 = ((t11).select(t13, f64x8::splat(1.0)));
            let t15 = t14 * t14;
            let t16 = t15 * t15;
            let t17 = f64x8::splat(1.0) / t16;
            let t18 = t10 * t17;
            let t20 = f64x8::splat(M_CBRT3);
            let t21 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t20 * t24;
            let t26 = f64x8::splat(M_CBRT4);
            let t27 = t26 * t26;
            let t29 = f64x8::splat(1.0) / t4 / t2;
            let t32 = f64x8::splat(1.0) / t15;
            let t33 = t20 * t20;
            let t35 = f64x8::splat(1.0) / t22;
            let t37 = t32 * t33 * t35 * t26;
            let t39 = v_sigma * t29 * t9 * t37 / f64x8::splat(96.0);
            let t40 = f64x8::splat(1.0) + t39;
            let t41 = t27 * t40;
            let t42 = t1 * v_sigma;
            let t43 = t2 * v_rho;
            let t44 = t3 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t47 = t16 * t15;
            let t48 = f64x8::splat(1.0) / t47;
            let t49 = t48 * f64x8::splat(M_PI);
            let t52 = f64x8::splat(1000000.0) + t42 * t45 * t49 / f64x8::splat(12288.0);
            let t53 = f64x8::splat(1.0) / t52;
            let t55 = t25 * t41 * t53;
            let t58 = f64x8::splat(1.0) - t8 * t18 * t55 / f64x8::splat(3072.0);
            let t59 = t20 * t22;
            let t60 = f64x8::splat(1.0) / t4;
            let t62 = t59 * t27 * t60;
            let t64 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t62;
            let t65 = ((t62).sqrt());
            let t68 = ((t62) * (t62).sqrt());
            let t70 = t33 * t23;
            let t71 = f64x8::splat(1.0) / t5;
            let t73 = t70 * t26 * t71;
            let t75 = f64x8::splat(3.79785) * t65 + f64x8::splat(0.8969) * t62 + f64x8::splat(0.204775) * t68 + f64x8::splat(0.123235) * t73;
            let t78 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t75;
            let t79 = (simd::ln(t78));
            let t81 = f64x8::splat(0.0621814) * t64 * t79;
            let t83 = ((t11).select(t12 * zeta_threshold, f64x8::splat(1.0)));
            let t89 = (f64x8::splat(2.0) * t83 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t9 - f64x8::splat(2.0));
            let t91 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t62;
            let t96 = f64x8::splat(5.1785) * t65 + f64x8::splat(0.905775) * t62 + f64x8::splat(0.1100325) * t68 + f64x8::splat(0.1241775) * t73;
            let t99 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t96;
            let t100 = (simd::ln(t99));
            let t103 = f64x8::splat(0.0197516734986138) * t89 * t91 * t100;
            let t104 = (simd::ln(f64x8::splat(2.0)));
            let t105 = f64x8::splat(1.0) - t104;
            let t106 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t107 = f64x8::splat(1.0) / t106;
            let t108 = t105 * t107;
            let t109 = t15 * t14;
            let t110 = f64x8::splat(1.0) / t105;
            let t113 = f64x8::splat(1.0) / t109;
            let t114 = t106 * t113;
            let t116 = (simd::exp(-(-t81 + t103) * t110 * t114));
            let t117 = t116 - f64x8::splat(1.0);
            let t118 = f64x8::splat(1.0) / t117;
            let t119 = t110 * t118;
            let t121 = t25 * t27;
            let t122 = t18 * t121;
            let t125 = t39 + f64x8::splat(0.0002143700905903487) * t119 * t8 * t122;
            let t126 = t125 * t110;
            let t129 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t119 * t125;
            let t130 = f64x8::splat(1.0) / t129;
            let t133 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t126 * t130;
            let t134 = (simd::ln(t133));
            let t137 = t108 * t109 * t134 + t103 - t81;
            let t138 = t58 * t137;
            let t139 = t18 * t20;
            let t140 = t8 * t139;
            let t141 = t24 * t27;
            let t142 = t40 * t53;
            let t143 = ((f64x8::splat(3.0)).sqrt());
            let t145 = f64x8::splat(M_CBRT6);
            let t146 = t145 * t145;
            let t147 = (simd::cbrt(t106));
            let t148 = f64x8::splat(1.0) / t147;
            let t149 = t146 * t148;
            let t150 = ((v_sigma).sqrt());
            let t152 = f64x8::splat(1.0) / t4 / v_rho;
            let t154 = t149 * t150 * t152;
            let t155 = ((t154).sqrt());
            let t156 = t143 * t60 * t155;
            let t158 = f64x8::splat(1.0) / t2;
            let t159 = t158 * t146;
            let t160 = t148 * t150;
            let t161 = t159 * t160;
            let t163 = f64x8::splat(1.0) / v_rho;
            let t164 = t143 * t163;
            let t165 = t155 * t154;
            let t166 = t164 * t165;
            let t168 = f64x8::splat(0.0245130624) * t156 + f64x8::splat(0.0138498611712) * t161 + f64x8::splat(0.0002310999830832) * t166;
            let t170 = ((t156) * (t156).sqrt());
            let t174 = f64x8::splat(0.2846248) * t156 - f64x8::splat(0.0031313960595450714) * t170 + f64x8::splat(0.08226186096) * t161 + f64x8::splat(0.00120051939264) * t166;
            let t176 = f64x8::splat(1.0) + f64x8::splat(1.0) / t174;
            let t177 = (simd::ln(t176));
            let t180 = (simd::exp(-f64x8::splat(0.3801624) * t156));
            let t182 = f64x8::splat(M_SQRT2);
            let t183 = (t180 - f64x8::splat(1.0)) * t182;
            let t184 = t183 * t143;
            let t185 = f64x8::splat(1.0) / t155;
            let t187 = ((zeta_threshold).sqrt());
            let t189 = ((t11).select(t187 * zeta_threshold, f64x8::splat(1.0)));
            let t190 = t189 - f64x8::splat(1.0);
            let t194 = -f64x8::splat(0.1925) + t168 * t177 - f64x8::splat(0.4981375370638352) * t184 * t4 * t185 * t190;
            let t196 = t141 * t142 * t194;
            let t198 = t140 * t196 / f64x8::splat(3072.0);
            let tzk0 = t138 + t198;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
