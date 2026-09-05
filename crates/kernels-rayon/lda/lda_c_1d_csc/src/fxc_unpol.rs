//! LDA_C_1D_CSC fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_1d_csc.c`
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
pub fn lda_c_1d_csc_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_para_4: f64,
    param_para_7: f64,
    param_para_9: f64,
    param_para_8: f64,
    param_para_1: f64,
    param_para_5: f64,
    param_para_2: f64,
    param_para_6: f64,
    param_para_3: f64,
    param_para_0: f64,
    param_ferro_4: f64,
    param_ferro_7: f64,
    param_ferro_9: f64,
    param_ferro_8: f64,
    param_ferro_1: f64,
    param_ferro_5: f64,
    param_ferro_2: f64,
    param_ferro_6: f64,
    param_ferro_3: f64,
    param_ferro_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_para_4 = f64x8::splat(param_para_4);
    let param_para_7 = f64x8::splat(param_para_7);
    let param_para_9 = f64x8::splat(param_para_9);
    let param_para_8 = f64x8::splat(param_para_8);
    let param_para_1 = f64x8::splat(param_para_1);
    let param_para_5 = f64x8::splat(param_para_5);
    let param_para_2 = f64x8::splat(param_para_2);
    let param_para_6 = f64x8::splat(param_para_6);
    let param_para_3 = f64x8::splat(param_para_3);
    let param_para_0 = f64x8::splat(param_para_0);
    let param_ferro_4 = f64x8::splat(param_ferro_4);
    let param_ferro_7 = f64x8::splat(param_ferro_7);
    let param_ferro_9 = f64x8::splat(param_ferro_9);
    let param_ferro_8 = f64x8::splat(param_ferro_8);
    let param_ferro_1 = f64x8::splat(param_ferro_1);
    let param_ferro_5 = f64x8::splat(param_ferro_5);
    let param_ferro_2 = f64x8::splat(param_ferro_2);
    let param_ferro_6 = f64x8::splat(param_ferro_6);
    let param_ferro_3 = f64x8::splat(param_ferro_3);
    let param_ferro_0 = f64x8::splat(param_ferro_0);
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
            let t1 = f64x8::splat(1.0) / v_rho;
            let t2 = t1 / f64x8::splat(2.0);
            let t3 = param_para_4;
            let t4 = v_rho * v_rho;
            let t5 = f64x8::splat(1.0) / t4;
            let t8 = t2 + t3 * t5 / f64x8::splat(4.0);
            let t9 = param_para_7;
            let t13 = param_para_9;
            let t14 = (simd::pow(t2, t13));
            let t15 = param_para_8 * t14;
            let t16 = f64x8::splat(1.0) + t9 * t1 / f64x8::splat(2.0) + t15;
            let t17 = (simd::ln(t16));
            let t18 = t8 * t17;
            let t21 = param_para_1;
            let t24 = param_para_5;
            let t25 = (simd::pow(t2, t24));
            let t26 = param_para_2 * t25;
            let t29 = param_para_6;
            let t30 = (simd::pow(t2, t29));
            let t31 = param_para_3 * t30;
            let t33 = t21 * t1 + f64x8::splat(2.0) * t26 + f64x8::splat(2.0) * t31 + f64x8::splat(2.0) * param_para_0;
            let t34 = f64x8::splat(1.0) / t33;
            let tzk0 = -t18 * t34;
            acc_zk = tzk0;
            let t37 = f64x8::splat(1.0) / t4 / v_rho;
            let t40 = -t3 * t37 / f64x8::splat(2.0) - t5 / f64x8::splat(2.0);
            let t41 = v_rho * t40;
            let t42 = t17 * t34;
            let t44 = v_rho * t8;
            let t49 = -t9 * t5 / f64x8::splat(2.0) - t15 * t13 * t1;
            let t50 = f64x8::splat(1.0) / t16;
            let t52 = t49 * t50 * t34;
            let t54 = t33 * t33;
            let t55 = f64x8::splat(1.0) / t54;
            let t56 = t17 * t55;
            let t64 = -f64x8::splat(2.0) * t26 * t24 * t1 - f64x8::splat(2.0) * t31 * t29 * t1 - t21 * t5;
            let t65 = t56 * t64;
            let tvrho0 = -t41 * t42 - t44 * t52 + t44 * t65 + tzk0;
            acc_vrho = tvrho0;
            let t67 = t40 * t17;
            let t70 = t8 * t49;
            let t71 = t50 * t34;
            let t74 = t55 * t64;
            let t77 = t4 * t4;
            let t78 = f64x8::splat(1.0) / t77;
            let t81 = t37 + f64x8::splat(3.0) / f64x8::splat(2.0) * t3 * t78;
            let t82 = v_rho * t81;
            let t89 = t13 * t13;
            let t94 = t15 * t13 * t5 + t15 * t89 * t5 + t9 * t37;
            let t96 = t94 * t50 * t34;
            let t98 = t49 * t49;
            let t99 = t16 * t16;
            let t100 = f64x8::splat(1.0) / t99;
            let t102 = t98 * t100 * t34;
            let t104 = t44 * t49;
            let t105 = t50 * t55;
            let t106 = t105 * t64;
            let t110 = f64x8::splat(1.0) / t54 / t33;
            let t111 = t17 * t110;
            let t112 = t64 * t64;
            let t113 = t111 * t112;
            let t117 = t24 * t24;
            let t122 = t29 * t29;
            let t128 = f64x8::splat(2.0) * t26 * t117 * t5 + f64x8::splat(2.0) * t31 * t122 * t5 + f64x8::splat(2.0) * t26 * t24 * t5 + f64x8::splat(2.0) * t31 * t29 * t5 + f64x8::splat(2.0) * t21 * t37;
            let t129 = t56 * t128;
            let tv2rho20 = t44 * t102 + f64x8::splat(2.0) * t104 * t106 - f64x8::splat(2.0) * t44 * t113 + t44 * t129 + f64x8::splat(2.0) * t18 * t74 - f64x8::splat(2.0) * t67 * t34 - f64x8::splat(2.0) * t41 * t52 + f64x8::splat(2.0) * t41 * t65 - t82 * t42 - t44 * t96 - f64x8::splat(2.0) * t70 * t71;
            acc_v2rho2 = tv2rho20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(v2rho2, ip, m, acc_v2rho2);
        ip += 8;
    }
}
