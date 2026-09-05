//! LDA_C_PZ kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pz.c`
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
pub fn lda_c_pz_kxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    param_gamma_0: f64,
    param_beta1_0: f64,
    param_beta2_0: f64,
    param_a_0: f64,
    param_c_0: f64,
    param_d_0: f64,
    param_b_0: f64,
    param_gamma_1: f64,
    param_beta1_1: f64,
    param_beta2_1: f64,
    param_a_1: f64,
    param_c_1: f64,
    param_d_1: f64,
    param_b_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma_0 = f64x8::splat(param_gamma_0);
    let param_beta1_0 = f64x8::splat(param_beta1_0);
    let param_beta2_0 = f64x8::splat(param_beta2_0);
    let param_a_0 = f64x8::splat(param_a_0);
    let param_c_0 = f64x8::splat(param_c_0);
    let param_d_0 = f64x8::splat(param_d_0);
    let param_b_0 = f64x8::splat(param_b_0);
    let param_gamma_1 = f64x8::splat(param_gamma_1);
    let param_beta1_1 = f64x8::splat(param_beta1_1);
    let param_beta2_1 = f64x8::splat(param_beta2_1);
    let param_a_1 = f64x8::splat(param_a_1);
    let param_c_1 = f64x8::splat(param_c_1);
    let param_d_1 = f64x8::splat(param_d_1);
    let param_b_1 = f64x8::splat(param_b_1);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t6 * t8;
            let t10 = t1 * t3 * t9;
            let t11 = t10 / f64x8::splat(4.0);
            let t12 = (f64x8::splat(1.0)).simd_le(t11);
            let t13 = param_gamma_0;
            let t14 = param_beta1_0;
            let t15 = ((t10).sqrt());
            let t19 = param_beta2_0 * t1;
            let t20 = t3 * t6;
            let t21 = t20 * t8;
            let t24 = f64x8::splat(1.0) + t14 * t15 / f64x8::splat(2.0) + t19 * t21 / f64x8::splat(4.0);
            let t27 = param_a_0;
            let t28 = (simd::ln(t11));
            let t32 = param_c_0 * t1;
            let t33 = t32 * t3;
            let t34 = t9 * t28;
            let t38 = param_d_0 * t1;
            let t42 = ((t12).select(t13 / t24, t27 * t28 + param_b_0 + t33 * t34 / f64x8::splat(4.0) + t38 * t21 / f64x8::splat(4.0)));
            let t43 = param_gamma_1;
            let t44 = param_beta1_1;
            let t48 = param_beta2_1 * t1;
            let t51 = f64x8::splat(1.0) + t44 * t15 / f64x8::splat(2.0) + t48 * t21 / f64x8::splat(4.0);
            let t54 = param_a_1;
            let t58 = param_c_1 * t1;
            let t59 = t58 * t3;
            let t63 = param_d_1 * t1;
            let t67 = ((t12).select(t43 / t51, t54 * t28 + param_b_1 + t59 * t34 / f64x8::splat(4.0) + t63 * t21 / f64x8::splat(4.0)));
            let t70 = (simd::cbrt(zeta_threshold));
            let t72 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t70 * zeta_threshold, f64x8::splat(1.0)));
            let t74 = f64x8::splat(2.0) * t72 - f64x8::splat(2.0);
            let t76 = f64x8::splat(M_CBRT2);
            let t79 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t76 - f64x8::splat(2.0));
            let t80 = (t67 - t42) * t74 * t79;
            let tzk0 = t42 + t80;
            acc_zk = tzk0;
            let t81 = t24 * t24;
            let t83 = t13 / t81;
            let t84 = f64x8::splat(1.0) / t15;
            let t86 = t14 * t84 * t1;
            let t88 = f64x8::splat(1.0) / t7 / v_rho;
            let t89 = t20 * t88;
            let t93 = -t19 * t89 / f64x8::splat(12.0) - t86 * t89 / f64x8::splat(12.0);
            let t95 = f64x8::splat(1.0) / v_rho;
            let t99 = t6 * t88 * t28;
            let t107 = ((t12).select(-t83 * t93, -t27 * t95 / f64x8::splat(3.0) - t33 * t99 / f64x8::splat(12.0) - t32 * t89 / f64x8::splat(12.0) - t38 * t89 / f64x8::splat(12.0)));
            let t108 = t51 * t51;
            let t110 = t43 / t108;
            let t112 = t44 * t84 * t1;
            let t116 = -t112 * t89 / f64x8::splat(12.0) - t48 * t89 / f64x8::splat(12.0);
            let t127 = ((t12).select(-t110 * t116, -t54 * t95 / f64x8::splat(3.0) - t59 * t99 / f64x8::splat(12.0) - t58 * t89 / f64x8::splat(12.0) - t63 * t89 / f64x8::splat(12.0)));
            let t130 = (t127 - t107) * t74 * t79;
            let tvrho0 = t42 + t80 + v_rho * (t107 + t130);
            acc_vrho = tvrho0;
            let t137 = t13 / t81 / t24;
            let t138 = t93 * t93;
            let t142 = f64x8::splat(1.0) / t15 / t10;
            let t144 = t1 * t1;
            let t145 = t14 * t142 * t144;
            let t146 = t3 * t3;
            let t147 = t146 * t5;
            let t148 = v_rho * v_rho;
            let t149 = t7 * t7;
            let t152 = t147 / t149 / t148;
            let t156 = f64x8::splat(1.0) / t7 / t148;
            let t157 = t20 * t156;
            let t162 = -t145 * t152 / f64x8::splat(18.0) + t86 * t157 / f64x8::splat(9.0) + t19 * t157 / f64x8::splat(9.0);
            let t165 = f64x8::splat(1.0) / t148;
            let t169 = t6 * t156 * t28;
            let t177 = ((t12).select(f64x8::splat(2.0) * t137 * t138 - t83 * t162, t27 * t165 / f64x8::splat(3.0) + t33 * t169 / f64x8::splat(9.0) + f64x8::splat(5.0) / f64x8::splat(36.0) * t32 * t157 + t38 * t157 / f64x8::splat(9.0)));
            let t180 = t43 / t108 / t51;
            let t181 = t116 * t116;
            let t185 = t44 * t142 * t144;
            let t192 = -t185 * t152 / f64x8::splat(18.0) + t112 * t157 / f64x8::splat(9.0) + t48 * t157 / f64x8::splat(9.0);
            let t204 = ((t12).select(-t110 * t192 + f64x8::splat(2.0) * t180 * t181, t54 * t165 / f64x8::splat(3.0) + t59 * t169 / f64x8::splat(9.0) + f64x8::splat(5.0) / f64x8::splat(36.0) * t58 * t157 + t63 * t157 / f64x8::splat(9.0)));
            let t207 = (t204 - t177) * t74 * t79;
            let tv2rho20 = f64x8::splat(2.0) * t107 + f64x8::splat(2.0) * t130 + v_rho * (t177 + t207);
            acc_v2rho2 = tv2rho20;
            let t212 = t81 * t81;
            let t214 = t13 / t212;
            let t227 = f64x8::splat(1.0) / t15 / t144 / t146 / t5 * t149 / f64x8::splat(4.0);
            let t228 = t14 * t227;
            let t229 = t148 * t148;
            let t230 = f64x8::splat(1.0) / t229;
            let t231 = t2 * t230;
            let t234 = t148 * v_rho;
            let t237 = t147 / t149 / t234;
            let t241 = f64x8::splat(1.0) / t7 / t234;
            let t242 = t20 * t241;
            let t247 = -t228 * t231 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t145 * t237 - f64x8::splat(7.0) / f64x8::splat(27.0) * t86 * t242 - f64x8::splat(7.0) / f64x8::splat(27.0) * t19 * t242;
            let t250 = f64x8::splat(1.0) / t234;
            let t254 = t6 * t241 * t28;
            let t262 = ((t12).select(f64x8::splat(6.0) * t137 * t93 * t162 - f64x8::splat(6.0) * t214 * t138 * t93 - t83 * t247, -f64x8::splat(2.0) / f64x8::splat(3.0) * t27 * t250 - f64x8::splat(7.0) / f64x8::splat(27.0) * t33 * t254 - f64x8::splat(13.0) / f64x8::splat(36.0) * t32 * t242 - f64x8::splat(7.0) / f64x8::splat(27.0) * t38 * t242));
            let t263 = t108 * t108;
            let t265 = t43 / t263;
            let t272 = t44 * t227;
            let t281 = -t272 * t231 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t185 * t237 - f64x8::splat(7.0) / f64x8::splat(27.0) * t112 * t242 - f64x8::splat(7.0) / f64x8::splat(27.0) * t48 * t242;
            let t293 = ((t12).select(f64x8::splat(6.0) * t180 * t116 * t192 - f64x8::splat(6.0) * t265 * t181 * t116 - t110 * t281, -f64x8::splat(2.0) / f64x8::splat(3.0) * t54 * t250 - f64x8::splat(7.0) / f64x8::splat(27.0) * t59 * t254 - f64x8::splat(13.0) / f64x8::splat(36.0) * t58 * t242 - f64x8::splat(7.0) / f64x8::splat(27.0) * t63 * t242));
            let t296 = (t293 - t262) * t74 * t79;
            let tv3rho30 = f64x8::splat(3.0) * t177 + f64x8::splat(3.0) * t207 + v_rho * (t262 + t296);
            acc_v3rho3 = tv3rho30;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        ip += 8;
    }
}
