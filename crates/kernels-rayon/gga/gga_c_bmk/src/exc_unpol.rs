//! GGA_C_BMK exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_bmk.c`
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
pub fn gga_c_bmk_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_ss_0: f64,
    param_c_ab_1: f64,
    param_c_ab_2: f64,
    param_c_ab_3: f64,
    param_c_ab_4: f64,
    param_c_ab_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c_ss_1 = f64x8::splat(param_c_ss_1);
    let param_c_ss_2 = f64x8::splat(param_c_ss_2);
    let param_c_ss_3 = f64x8::splat(param_c_ss_3);
    let param_c_ss_4 = f64x8::splat(param_c_ss_4);
    let param_c_ss_0 = f64x8::splat(param_c_ss_0);
    let param_c_ab_1 = f64x8::splat(param_c_ab_1);
    let param_c_ab_2 = f64x8::splat(param_c_ab_2);
    let param_c_ab_3 = f64x8::splat(param_c_ab_3);
    let param_c_ab_4 = f64x8::splat(param_c_ab_4);
    let param_c_ab_0 = f64x8::splat(param_c_ab_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t4 = ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold)) | (t3);
            let t5 = ((t3).select(zeta_threshold, f64x8::splat(1.0)));
            let t6 = f64x8::splat(M_CBRT3);
            let t7 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t8 = (simd::cbrt(t7));
            let t9 = t6 * t8;
            let t10 = f64x8::splat(M_CBRT4);
            let t11 = t10 * t10;
            let t12 = t9 * t11;
            let t13 = (simd::cbrt(v_rho));
            let t14 = f64x8::splat(1.0) / t13;
            let t15 = f64x8::splat(M_CBRT2);
            let t17 = (simd::cbrt(zeta_threshold));
            let t19 = ((t3).select(f64x8::splat(1.0) / t17, f64x8::splat(1.0)));
            let t21 = t12 * t14 * t15 * t19;
            let t23 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t21;
            let t24 = ((t21).sqrt());
            let t27 = ((t21) * (t21).sqrt());
            let t29 = t6 * t6;
            let t30 = t8 * t8;
            let t31 = t29 * t30;
            let t32 = t31 * t10;
            let t33 = t13 * t13;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t15 * t15;
            let t37 = t19 * t19;
            let t39 = t32 * t34 * t35 * t37;
            let t41 = f64x8::splat(3.79785) * t24 + f64x8::splat(0.8969) * t21 + f64x8::splat(0.204775) * t27 + f64x8::splat(0.123235) * t39;
            let t44 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t41;
            let t45 = (simd::ln(t44));
            let t47 = f64x8::splat(0.062182) * t23 * t45;
            let t49 = t17 * zeta_threshold;
            let t51 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t49, f64x8::splat(2.0) * t15));
            let t53 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t49, f64x8::splat(0.0)));
            let t57 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t15 - f64x8::splat(2.0));
            let t58 = (t51 + t53 - f64x8::splat(2.0)) * t57;
            let t60 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t21;
            let t65 = f64x8::splat(7.05945) * t24 + f64x8::splat(1.549425) * t21 + f64x8::splat(0.420775) * t27 + f64x8::splat(0.1562925) * t39;
            let t68 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t65;
            let t69 = (simd::ln(t68));
            let t73 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t21;
            let t78 = f64x8::splat(5.1785) * t24 + f64x8::splat(0.905775) * t21 + f64x8::splat(0.1100325) * t27 + f64x8::splat(0.1241775) * t39;
            let t81 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t78;
            let t82 = (simd::ln(t81));
            let t83 = t73 * t82;
            let t92 = ((t4).select(f64x8::splat(0.0), t5 * (-t47 + t58 * (-f64x8::splat(0.03109) * t60 * t69 + t47 - f64x8::splat(0.019751789702565206) * t83) + f64x8::splat(0.019751789702565206) * t58 * t83) / f64x8::splat(2.0)));
            let t94 = param_c_ss_1;
            let t95 = t94 * v_sigma;
            let t96 = v_rho * v_rho;
            let t98 = f64x8::splat(1.0) / t33 / t96;
            let t99 = t35 * t98;
            let t101 = v_sigma * t35 * t98;
            let t103 = f64x8::splat(1.0) + f64x8::splat(0.2) * t101;
            let t104 = f64x8::splat(1.0) / t103;
            let t108 = param_c_ss_2;
            let t109 = v_sigma * v_sigma;
            let t110 = t108 * t109;
            let t111 = t96 * t96;
            let t112 = t111 * v_rho;
            let t114 = f64x8::splat(1.0) / t13 / t112;
            let t115 = t15 * t114;
            let t116 = t103 * t103;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t115 * t117;
            let t121 = param_c_ss_3;
            let t122 = t109 * v_sigma;
            let t123 = t121 * t122;
            let t124 = t111 * t111;
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t116 * t103;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t125 * t127;
            let t131 = param_c_ss_4;
            let t132 = t109 * t109;
            let t133 = t131 * t132;
            let t134 = t124 * t96;
            let t136 = f64x8::splat(1.0) / t33 / t134;
            let t137 = t35 * t136;
            let t138 = t116 * t116;
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t137 * t139;
            let t143 = param_c_ss_0 + f64x8::splat(0.2) * t95 * t99 * t104 + f64x8::splat(0.08) * t110 * t118 + f64x8::splat(0.032) * t123 * t128 + f64x8::splat(0.0064) * t133 * t140;
            let t145 = f64x8::splat(2.0) * t92 * t143;
            let t147 = t9 * t11 * t14;
            let t149 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t147;
            let t150 = ((t147).sqrt());
            let t153 = ((t147) * (t147).sqrt());
            let t156 = t31 * t10 * t34;
            let t158 = f64x8::splat(3.79785) * t150 + f64x8::splat(0.8969) * t147 + f64x8::splat(0.204775) * t153 + f64x8::splat(0.123235) * t156;
            let t161 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t158;
            let t162 = (simd::ln(t161));
            let t165 = ((t3).select(t49, f64x8::splat(1.0)));
            let t168 = (f64x8::splat(2.0) * t165 - f64x8::splat(2.0)) * t57;
            let t170 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t147;
            let t175 = f64x8::splat(5.1785) * t150 + f64x8::splat(0.905775) * t147 + f64x8::splat(0.1100325) * t153 + f64x8::splat(0.1241775) * t156;
            let t178 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t175;
            let t179 = (simd::ln(t178));
            let t184 = -f64x8::splat(0.062182) * t149 * t162 + f64x8::splat(0.019751789702565206) * t168 * t170 * t179 - f64x8::splat(2.0) * t92;
            let t186 = param_c_ab_1;
            let t187 = t186 * v_sigma;
            let t189 = f64x8::splat(1.0) + f64x8::splat(0.006) * t101;
            let t190 = f64x8::splat(1.0) / t189;
            let t194 = param_c_ab_2;
            let t195 = t194 * t109;
            let t196 = t189 * t189;
            let t197 = f64x8::splat(1.0) / t196;
            let t198 = t115 * t197;
            let t201 = param_c_ab_3;
            let t202 = t201 * t122;
            let t203 = t196 * t189;
            let t204 = f64x8::splat(1.0) / t203;
            let t205 = t125 * t204;
            let t208 = param_c_ab_4;
            let t209 = t208 * t132;
            let t210 = t196 * t196;
            let t211 = f64x8::splat(1.0) / t210;
            let t212 = t137 * t211;
            let t215 = param_c_ab_0 + f64x8::splat(0.006) * t187 * t99 * t190 + f64x8::splat(7.2e-05) * t195 * t198 + f64x8::splat(8.64e-07) * t202 * t205 + f64x8::splat(5.184e-09) * t209 * t212;
            let t216 = t184 * t215;
            let tzk0 = t145 + t216;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
