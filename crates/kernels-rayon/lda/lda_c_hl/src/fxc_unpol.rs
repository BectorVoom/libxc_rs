//! LDA_C_HL fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_hl.c`
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
pub fn lda_c_hl_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_hl_c_0: f64,
    param_hl_r_0: f64,
    param_hl_c_1: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hl_c_0 = f64x8::splat(param_hl_c_0);
    let param_hl_r_0 = f64x8::splat(param_hl_r_0);
    let param_hl_c_1 = f64x8::splat(param_hl_c_1);
    let param_hl_r_1 = f64x8::splat(param_hl_r_1);
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
            let t1 = param_hl_c_0;
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = t2 / v_rho;
            let t5 = param_hl_r_0;
            let t6 = t5 * t5;
            let t7 = t6 * t5;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = f64x8::splat(1.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t4 * t8;
            let t12 = f64x8::splat(M_CBRT3);
            let t13 = t12 * t12;
            let t14 = (simd::cbrt(t2));
            let t15 = f64x8::splat(1.0) / t14;
            let t16 = t13 * t15;
            let t17 = f64x8::splat(M_CBRT4);
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t23 = f64x8::splat(1.0) + t16 * t19 * t5 / f64x8::splat(3.0);
            let t24 = (simd::ln(t23));
            let t26 = t14 * t14;
            let t27 = t13 * t26;
            let t28 = t18 * t18;
            let t30 = t17 / t28;
            let t31 = f64x8::splat(1.0) / t6;
            let t35 = t12 * t14;
            let t36 = t17 * t17;
            let t38 = t36 / t18;
            let t39 = f64x8::splat(1.0) / t5;
            let t44 = t1 * (t11 * t24 - t27 * t30 * t31 / f64x8::splat(4.0) + t35 * t38 * t39 / f64x8::splat(8.0) - f64x8::splat(1.0) / f64x8::splat(3.0));
            let t46 = (simd::cbrt(zeta_threshold));
            let t48 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t46 * zeta_threshold, f64x8::splat(1.0)));
            let t51 = f64x8::splat(M_CBRT2);
            let t55 = (f64x8::splat(2.0) * t48 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t51 - f64x8::splat(2.0));
            let t56 = param_hl_c_1;
            let t57 = param_hl_r_1;
            let t58 = t57 * t57;
            let t59 = t58 * t57;
            let t60 = f64x8::splat(1.0) / t59;
            let t63 = f64x8::splat(1.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t4 * t60;
            let t67 = f64x8::splat(1.0) + t16 * t19 * t57 / f64x8::splat(3.0);
            let t68 = (simd::ln(t67));
            let t70 = f64x8::splat(1.0) / t58;
            let t74 = f64x8::splat(1.0) / t57;
            let t81 = t55 * (-t56 * (t63 * t68 - t27 * t30 * t70 / f64x8::splat(4.0) + t35 * t38 * t74 / f64x8::splat(8.0) - f64x8::splat(1.0) / f64x8::splat(3.0)) + t44);
            let tzk0 = -t44 + t81;
            acc_zk = tzk0;
            let t82 = v_rho * v_rho;
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t2 * t83;
            let t85 = t8 * t24;
            let t89 = t11 * t13 * t15;
            let t90 = f64x8::splat(1.0) / t23;
            let t91 = t5 * t90;
            let t97 = t17 / t28 / v_rho;
            let t103 = t36 / t18 / v_rho;
            let t108 = t1 * (-f64x8::splat(3.0) / f64x8::splat(4.0) * t84 * t85 + t89 * t30 * t91 / f64x8::splat(9.0) + t27 * t97 * t31 / f64x8::splat(6.0) - t35 * t103 * t39 / f64x8::splat(24.0));
            let t109 = t60 * t68;
            let t113 = t63 * t13 * t15;
            let t114 = f64x8::splat(1.0) / t67;
            let t115 = t57 * t114;
            let t128 = t55 * (-t56 * (-f64x8::splat(3.0) / f64x8::splat(4.0) * t84 * t109 + t113 * t30 * t115 / f64x8::splat(9.0) + t27 * t97 * t70 / f64x8::splat(6.0) - t35 * t103 * t74 / f64x8::splat(24.0)) + t108);
            let tvrho0 = -t44 + t81 + v_rho * (-t108 + t128);
            acc_vrho = tvrho0;
            let t133 = t82 * v_rho;
            let t134 = f64x8::splat(1.0) / t133;
            let t135 = t2 * t134;
            let t139 = f64x8::splat(1.0) / t28 / t82;
            let t140 = t2 * t139;
            let t143 = t16 * t17 * t90;
            let t150 = f64x8::splat(1.0) / t26;
            let t151 = t11 * t12 * t150;
            let t152 = t23 * t23;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t6 * t153;
            let t158 = t17 * t139;
            let t164 = t36 / t18 / t82;
            let t169 = t1 * (f64x8::splat(3.0) / f64x8::splat(2.0) * t135 * t85 - t140 * t31 * t143 / f64x8::splat(6.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t89 * t97 * t91 - t151 * t103 * t154 / f64x8::splat(27.0) - f64x8::splat(5.0) / f64x8::splat(18.0) * t27 * t158 * t31 + t35 * t164 * t39 / f64x8::splat(18.0));
            let t174 = t16 * t17 * t114;
            let t181 = t63 * t12 * t150;
            let t182 = t67 * t67;
            let t183 = f64x8::splat(1.0) / t182;
            let t184 = t58 * t183;
            let t197 = t55 * (-t56 * (f64x8::splat(3.0) / f64x8::splat(2.0) * t135 * t109 - t140 * t70 * t174 / f64x8::splat(6.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t113 * t97 * t115 - t181 * t103 * t184 / f64x8::splat(27.0) - f64x8::splat(5.0) / f64x8::splat(18.0) * t27 * t158 * t70 + t35 * t164 * t74 / f64x8::splat(18.0)) + t169);
            let tv2rho20 = -f64x8::splat(2.0) * t108 + f64x8::splat(2.0) * t128 + v_rho * (-t169 + t197);
            acc_v2rho2 = tv2rho20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(v2rho2, ip, m, acc_v2rho2);
        ip += 8;
    }
}
