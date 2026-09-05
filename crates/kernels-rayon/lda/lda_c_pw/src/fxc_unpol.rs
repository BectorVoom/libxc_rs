//! LDA_C_PW fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pw.c`
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
pub fn lda_c_pw_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_a_0: f64,
    param_alpha1_0: f64,
    param_beta1_0: f64,
    param_beta2_0: f64,
    param_beta3_0: f64,
    param_pp_0: f64,
    param_beta4_0: f64,
    param_a_2: f64,
    param_alpha1_2: f64,
    param_beta1_2: f64,
    param_beta2_2: f64,
    param_beta3_2: f64,
    param_pp_2: f64,
    param_beta4_2: f64,
    param_fz20: f64,
    param_a_1: f64,
    param_alpha1_1: f64,
    param_beta1_1: f64,
    param_beta2_1: f64,
    param_beta3_1: f64,
    param_pp_1: f64,
    param_beta4_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a_0 = f64x8::splat(param_a_0);
    let param_alpha1_0 = f64x8::splat(param_alpha1_0);
    let param_beta1_0 = f64x8::splat(param_beta1_0);
    let param_beta2_0 = f64x8::splat(param_beta2_0);
    let param_beta3_0 = f64x8::splat(param_beta3_0);
    let param_pp_0 = f64x8::splat(param_pp_0);
    let param_beta4_0 = f64x8::splat(param_beta4_0);
    let param_a_2 = f64x8::splat(param_a_2);
    let param_alpha1_2 = f64x8::splat(param_alpha1_2);
    let param_beta1_2 = f64x8::splat(param_beta1_2);
    let param_beta2_2 = f64x8::splat(param_beta2_2);
    let param_beta3_2 = f64x8::splat(param_beta3_2);
    let param_pp_2 = f64x8::splat(param_pp_2);
    let param_beta4_2 = f64x8::splat(param_beta4_2);
    let param_fz20 = f64x8::splat(param_fz20);
    let param_a_1 = f64x8::splat(param_a_1);
    let param_alpha1_1 = f64x8::splat(param_alpha1_1);
    let param_beta1_1 = f64x8::splat(param_beta1_1);
    let param_beta2_1 = f64x8::splat(param_beta2_1);
    let param_beta3_1 = f64x8::splat(param_beta3_1);
    let param_pp_1 = f64x8::splat(param_pp_1);
    let param_beta4_1 = f64x8::splat(param_beta4_1);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        {
            let t1 = param_a_0;
            let t2 = param_alpha1_0;
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t2 * t3;
            let t5 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t6 = (simd::cbrt(t5));
            let t7 = f64x8::splat(M_CBRT4);
            let t8 = t7 * t7;
            let t9 = t6 * t8;
            let t10 = (simd::cbrt(v_rho));
            let t11 = f64x8::splat(1.0) / t10;
            let t12 = t9 * t11;
            let t15 = f64x8::splat(1.0) + t4 * t12 / f64x8::splat(4.0);
            let t17 = f64x8::splat(1.0) / t1;
            let t18 = param_beta1_0;
            let t19 = t3 * t6;
            let t21 = t19 * t8 * t11;
            let t22 = ((t21).sqrt());
            let t26 = param_beta2_0 * t3;
            let t29 = param_beta3_0;
            let t30 = ((t21) * (t21).sqrt());
            let t34 = t21 / f64x8::splat(4.0);
            let t36 = param_pp_0 + f64x8::splat(1.0);
            let t37 = (simd::pow(t34, t36));
            let t38 = param_beta4_0 * t37;
            let t39 = t18 * t22 / f64x8::splat(2.0) + t26 * t12 / f64x8::splat(4.0) + f64x8::splat(0.125) * t29 * t30 + t38;
            let t43 = f64x8::splat(1.0) + t17 / t39 / f64x8::splat(2.0);
            let t44 = (simd::ln(t43));
            let t45 = t1 * t15 * t44;
            let t47 = (simd::cbrt(zeta_threshold));
            let t49 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t47 * zeta_threshold, f64x8::splat(1.0)));
            let t52 = f64x8::splat(M_CBRT2);
            let t56 = (f64x8::splat(2.0) * t49 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t52 - f64x8::splat(2.0));
            let t57 = param_a_2;
            let t59 = param_alpha1_2;
            let t60 = t59 * t3;
            let t63 = f64x8::splat(1.0) + t60 * t12 / f64x8::splat(4.0);
            let t64 = f64x8::splat(1.0) / t57;
            let t65 = param_beta1_2;
            let t69 = param_beta2_2 * t3;
            let t72 = param_beta3_2;
            let t77 = param_pp_2 + f64x8::splat(1.0);
            let t78 = (simd::pow(t34, t77));
            let t79 = param_beta4_2 * t78;
            let t80 = t65 * t22 / f64x8::splat(2.0) + t69 * t12 / f64x8::splat(4.0) + f64x8::splat(0.125) * t72 * t30 + t79;
            let t84 = f64x8::splat(1.0) + t64 / t80 / f64x8::splat(2.0);
            let t85 = (simd::ln(t84));
            let t87 = f64x8::splat(1.0) / param_fz20;
            let t89 = t56 * t57 * t63 * t85 * t87;
            let tzk0 = -f64x8::splat(2.0) * t45 + f64x8::splat(2.0) * t89;
            acc_zk = tzk0;
            let t94 = t1 * t2 * t3;
            let t96 = f64x8::splat(1.0) / t10 / v_rho;
            let t99 = t94 * t9 * t96 * t44;
            let t101 = t39 * t39;
            let t102 = f64x8::splat(1.0) / t101;
            let t103 = t15 * t102;
            let t104 = f64x8::splat(1.0) / t22;
            let t106 = t18 * t104 * t3;
            let t107 = t9 * t96;
            let t112 = ((t21).sqrt());
            let t114 = t29 * t112 * t3;
            let t117 = f64x8::splat(1.0) / v_rho;
            let t121 = -t106 * t107 / f64x8::splat(12.0) - t26 * t107 / f64x8::splat(12.0) - f64x8::splat(0.0625) * t114 * t107 - t38 * t36 * t117 / f64x8::splat(3.0);
            let t122 = f64x8::splat(1.0) / t43;
            let t123 = t121 * t122;
            let t124 = t103 * t123;
            let t127 = t56 * t57 * t59 * t3;
            let t131 = t127 * t9 * t96 * t85 * t87;
            let t133 = t56 * t63;
            let t134 = t80 * t80;
            let t135 = f64x8::splat(1.0) / t134;
            let t137 = t65 * t104 * t3;
            let t143 = t72 * t112 * t3;
            let t149 = -t137 * t107 / f64x8::splat(12.0) - t69 * t107 / f64x8::splat(12.0) - f64x8::splat(0.0625) * t143 * t107 - t79 * t77 * t117 / f64x8::splat(3.0);
            let t151 = f64x8::splat(1.0) / t84;
            let t152 = t151 * t87;
            let t154 = t133 * t135 * t149 * t152;
            let tvrho0 = -f64x8::splat(2.0) * t45 + f64x8::splat(2.0) * t89 + v_rho * (t99 / f64x8::splat(6.0) + t124 - t131 / f64x8::splat(6.0) - t154);
            acc_vrho = tvrho0;
            let t161 = v_rho * v_rho;
            let t163 = f64x8::splat(1.0) / t10 / t161;
            let t166 = t94 * t9 * t163 * t44;
            let t168 = t4 * t9;
            let t169 = t96 * t102;
            let t171 = t168 * t169 * t123;
            let t173 = t101 * t39;
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t15 * t174;
            let t176 = t121 * t121;
            let t177 = t176 * t122;
            let t178 = t175 * t177;
            let t181 = f64x8::splat(1.0) / t22 / t21;
            let t183 = t3 * t3;
            let t184 = t18 * t181 * t183;
            let t185 = t6 * t6;
            let t186 = t185 * t7;
            let t187 = t10 * t10;
            let t190 = t186 / t187 / t161;
            let t193 = t9 * t163;
            let t198 = f64x8::splat(1.0)/((t21).sqrt());
            let t200 = t29 * t198 * t183;
            let t205 = t36 * t36;
            let t206 = f64x8::splat(1.0) / t161;
            let t213 = -t184 * t190 / f64x8::splat(18.0) + t106 * t193 / f64x8::splat(9.0) + t26 * t193 / f64x8::splat(9.0) + f64x8::splat(0.041666666666666664) * t200 * t190 + f64x8::splat(0.08333333333333333) * t114 * t193 + t38 * t205 * t206 / f64x8::splat(9.0) + t38 * t36 * t206 / f64x8::splat(3.0);
            let t214 = t213 * t122;
            let t215 = t103 * t214;
            let t216 = t101 * t101;
            let t217 = f64x8::splat(1.0) / t216;
            let t218 = t15 * t217;
            let t219 = t43 * t43;
            let t220 = f64x8::splat(1.0) / t219;
            let t222 = t176 * t220 * t17;
            let t223 = t218 * t222;
            let t228 = t127 * t9 * t163 * t85 * t87;
            let t231 = t56 * t60 * t6;
            let t232 = t8 * t96;
            let t233 = t232 * t135;
            let t234 = t149 * t151;
            let t235 = t234 * t87;
            let t237 = t231 * t233 * t235;
            let t239 = t134 * t80;
            let t240 = f64x8::splat(1.0) / t239;
            let t241 = t149 * t149;
            let t244 = t133 * t240 * t241 * t152;
            let t247 = t65 * t181 * t183;
            let t255 = t72 * t198 * t183;
            let t260 = t77 * t77;
            let t267 = -t247 * t190 / f64x8::splat(18.0) + t137 * t193 / f64x8::splat(9.0) + t69 * t193 / f64x8::splat(9.0) + f64x8::splat(0.041666666666666664) * t255 * t190 + f64x8::splat(0.08333333333333333) * t143 * t193 + t79 * t260 * t206 / f64x8::splat(9.0) + t79 * t77 * t206 / f64x8::splat(3.0);
            let t270 = t133 * t135 * t267 * t152;
            let t271 = t134 * t134;
            let t272 = f64x8::splat(1.0) / t271;
            let t274 = t56 * t63 * t272;
            let t275 = t84 * t84;
            let t276 = f64x8::splat(1.0) / t275;
            let t277 = t241 * t276;
            let t278 = t87 * t64;
            let t280 = t274 * t277 * t278;
            let tv2rho20 = t99 / f64x8::splat(3.0) + f64x8::splat(2.0) * t124 - t131 / f64x8::splat(3.0) - f64x8::splat(2.0) * t154 + v_rho * (-f64x8::splat(2.0) / f64x8::splat(9.0) * t166 - t171 / f64x8::splat(6.0) - f64x8::splat(2.0) * t178 + t215 + t223 / f64x8::splat(2.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t228 + t237 / f64x8::splat(6.0) + f64x8::splat(2.0) * t244 - t270 - t280 / f64x8::splat(2.0));
            acc_v2rho2 = tv2rho20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(v2rho2, ip, m, acc_v2rho2);
        ip += 8;
    }
}
