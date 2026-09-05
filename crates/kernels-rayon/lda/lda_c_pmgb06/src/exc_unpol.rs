//! LDA_C_PMGB06 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pmgb06.c`
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
pub fn lda_c_pmgb06_exc_unpol(
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
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t1 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t2 = (simd::cbrt(zeta_threshold));
            let t3 = t2 * t2;
            let t4 = ((t1).select(t3, f64x8::splat(1.0)));
            let t5 = t4 * t4;
            let t6 = t5 * t4;
            let t7 = (simd::ln(f64x8::splat(2.0)));
            let t8 = t7 - f64x8::splat(1.0);
            let t10 = f64x8::splat(2.0) * t6 * t8;
            let t11 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t12 = f64x8::splat(1.0) / t11;
            let t13 = f64x8::splat(M_CBRT3);
            let t14 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t15 = (simd::cbrt(t14));
            let t16 = t13 * t15;
            let t17 = f64x8::splat(M_CBRT4);
            let t18 = t17 * t17;
            let t19 = (simd::cbrt(v_rho));
            let t20 = f64x8::splat(1.0) / t19;
            let t21 = t18 * t20;
            let t22 = t16 * t21;
            let t23 = ((t22).sqrt());
            let t25 = f64x8::splat(1.0) / t4;
            let t27 = f64x8::splat(2.923025) * param_hyb_omega_0 * t23 * t25;
            let t29 = (simd::cbrt(f64x8::splat(9.0)));
            let t30 = t29 * t29;
            let t38 = param_hyb_omega_0 * param_hyb_omega_0;
            let t40 = (f64x8::splat(3.44851) - f64x8::splat(M_PI) * t17 * t30 * t15 / t8 / f64x8::splat(12.0)) * t38 * t13;
            let t41 = t15 * t18;
            let t42 = f64x8::splat(1.0) / t5;
            let t47 = t38 * param_hyb_omega_0;
            let t48 = t23 * t22;
            let t50 = f64x8::splat(1.0) / t6;
            let t53 = f64x8::splat(1.0) + t27 + t40 * t41 * t20 * t42 / f64x8::splat(4.0) + f64x8::splat(0.48968) * t47 * t48 * t50;
            let t55 = t38 * t13 * t15;
            let t59 = f64x8::splat(1.0) + t27 + f64x8::splat(0.8621275) * t55 * t21 * t42;
            let t60 = f64x8::splat(1.0) / t59;
            let t62 = (simd::ln(t53 * t60));
            let t65 = f64x8::splat(1.0) / v_rho;
            let t74 = (f64x8::splat(2.0) / f64x8::splat(45.0) * t17 * t30 * t15 * (t11 + f64x8::splat(6.0) * t7 - f64x8::splat(3.0)) * t14 - f64x8::splat(0.7524)) * t13;
            let t78 = t13 * t13;
            let t79 = t15 * t15;
            let t80 = t78 * t79;
            let t81 = t19 * t19;
            let t82 = f64x8::splat(1.0) / t81;
            let t83 = t17 * t82;
            let t84 = t80 * t83;
            let t87 = t15 * t14;
            let t88 = t13 * t87;
            let t90 = f64x8::splat(1.0) / t19 / v_rho;
            let t91 = t18 * t90;
            let t94 = f64x8::splat(1.0) - t74 * t41 * t20 / f64x8::splat(4.0) + f64x8::splat(0.0204825) * t84 - f64x8::splat(0.0030486129349252553) * t65 + f64x8::splat(0.0003485625) * t88 * t91;
            let t97 = (simd::exp(-f64x8::splat(0.1881) * t22));
            let t98 = f64x8::splat(M_SQRT2);
            let t99 = t97 * t98;
            let t103 = t78 * t79 * t12;
            let t104 = t103 * t17;
            let t106 = f64x8::splat(1.0) / t81 / v_rho;
            let t107 = zeta_threshold * zeta_threshold;
            let t108 = ((t1).select(t107, f64x8::splat(1.0)));
            let t109 = t108 * t30;
            let t110 = f64x8::splat(1.0) / t87;
            let t111 = t109 * t110;
            let t113 = f64x8::splat(M_CBRT2);
            let t115 = t16 * t21 * t113;
            let t117 = f64x8::splat(1.0) - f64x8::splat(0.0056675) * t115;
            let t119 = t113 * t113;
            let t123 = f64x8::splat(1.0) + f64x8::splat(0.107975) * t115 + f64x8::splat(0.01) * t80 * t83 * t119;
            let t124 = f64x8::splat(1.0) / t123;
            let t125 = t117 * t124;
            let t128 = t111 * t13 * t81 * t125 / f64x8::splat(15.0);
            let t131 = -f64x8::splat(1.2375) * t22 + t84 / f64x8::splat(4.0);
            let t133 = (simd::exp(-f64x8::splat(0.0775) * t22));
            let t134 = t131 * t133;
            let t135 = f64x8::splat(M_PI) * v_rho;
            let t138 = t128 + f64x8::splat(4.0) / f64x8::splat(3.0) * t134 * t135;
            let t145 = t94 * t97;
            let t147 = t145 / f64x8::splat(2.0) - f64x8::splat(1.0) / f64x8::splat(2.0);
            let t150 = t17 * t106;
            let t153 = -f64x8::splat(0.097) * t22 + f64x8::splat(0.169) * t84;
            let t155 = (simd::exp(-f64x8::splat(0.13675) * t22));
            let t157 = t153 * t155 * t13;
            let t159 = f64x8::splat(1.0) / t79 * t18;
            let t160 = t159 * t81;
            let t164 = ((t1).select(t3 * t107, f64x8::splat(1.0)));
            let t165 = t164 * t30;
            let t166 = t110 * t13;
            let t170 = t128 + t157 * t160 / f64x8::splat(3.0) - t165 * t166 * t81 / f64x8::splat(15.0);
            let t175 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t22;
            let t178 = ((t22) * (t22).sqrt());
            let t181 = f64x8::splat(3.79785) * t23 + f64x8::splat(0.8969) * t22 + f64x8::splat(0.204775) * t178 + f64x8::splat(0.123235) * t84;
            let t184 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t181;
            let t185 = (simd::ln(t184));
            let t189 = ((t1).select(t2 * zeta_threshold, f64x8::splat(1.0)));
            let t195 = (f64x8::splat(2.0) * t189 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t113 - f64x8::splat(2.0));
            let t197 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t22;
            let t202 = f64x8::splat(5.1785) * t23 + f64x8::splat(0.905775) * t22 + f64x8::splat(0.1100325) * t178 + f64x8::splat(0.1241775) * t84;
            let t205 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t202;
            let t206 = (simd::ln(t205));
            let t210 = -f64x8::splat(0.0621814) * t175 * t185 + f64x8::splat(0.0197516734986138) * t195 * t197 * t206;
            let t215 = t38 * t38;
            let t217 = t103 * t150;
            let t218 = t215 * param_hyb_omega_0;
            let t219 = t98 * t218;
            let t220 = t145 * t219;
            let t226 = v_rho * v_rho;
            let t227 = f64x8::splat(1.0) / t226;
            let t231 = t215 * t38;
            let t234 = f64x8::splat(1.0) / t81 / t226;
            let t236 = t215 * t215;
            let t240 = t10 * t12 * t62 + (-f64x8::splat(0.031505407223141116) * t65 * t94 * t99 - f64x8::splat(0.005388405304614574) * t104 * t106 * t138 * t98) * t47 + (-f64x8::splat(0.0837628205355044) * t65 * t147 - f64x8::splat(0.011938374665504766) * t103 * t150 * t170 + f64x8::splat(0.42708890021612717) * t88 * t91 * t210) * t215 - f64x8::splat(0.01197423401025461) * t217 * t220 + (-f64x8::splat(0.031835665774679375) * t103 * t150 * t147 + f64x8::splat(0.05332506774217938) * t227 * t210) * t231 + f64x8::splat(0.020267214298646783) * t104 * t234 * t210 * t236;
            let t244 = f64x8::splat(1.0) + f64x8::splat(0.15403623315025) * t80 * t83 * t38;
            let t245 = t244 * t244;
            let t246 = t245 * t245;
            let t247 = f64x8::splat(1.0) / t246;
            let tzk0 = t240 * t247;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
