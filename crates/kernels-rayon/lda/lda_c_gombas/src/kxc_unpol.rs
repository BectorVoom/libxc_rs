//! LDA_C_GOMBAS kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_gombas.c`
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
pub fn lda_c_gombas_kxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        {
            let t1 = (simd::cbrt(v_rho));
            let t2 = f64x8::splat(1.0) / t1;
            let t4 = f64x8::splat(1.0) + f64x8::splat(0.0562) * t2;
            let t6 = f64x8::splat(0.0357) / t4;
            let t7 = t2 + f64x8::splat(2.39);
            let t9 = (simd::ln(t7 * t1));
            let t10 = f64x8::splat(0.0311) * t9;
            let tzk0 = -t6 - t10;
            acc_zk = tzk0;
            let t11 = t4 * t4;
            let t12 = f64x8::splat(1.0) / t11;
            let t14 = f64x8::splat(1.0) / t1 / v_rho;
            let t15 = t12 * t14;
            let t18 = t1 * t1;
            let t22 = -f64x8::splat(1.0) / v_rho / f64x8::splat(3.0) + t7 / t18 / f64x8::splat(3.0);
            let t23 = f64x8::splat(1.0) / t7;
            let t24 = t22 * t23;
            let t25 = t24 * t2;
            let tvrho0 = -t6 - t10 + v_rho * (-f64x8::splat(0.00066878) * t15 - f64x8::splat(0.0311) * t25);
            acc_vrho = tvrho0;
            let t32 = f64x8::splat(1.0) / t11 / t4;
            let t33 = v_rho * v_rho;
            let t35 = f64x8::splat(1.0) / t18 / t33;
            let t36 = t32 * t35;
            let t39 = f64x8::splat(1.0) / t1 / t33;
            let t40 = t12 * t39;
            let t44 = f64x8::splat(1.0) / t18 / v_rho;
            let t47 = f64x8::splat(2.0) / f64x8::splat(9.0) / t33 - f64x8::splat(2.0) / f64x8::splat(9.0) * t7 * t44;
            let t48 = t47 * t23;
            let t49 = t48 * t2;
            let t51 = t7 * t7;
            let t52 = f64x8::splat(1.0) / t51;
            let t53 = t22 * t52;
            let t54 = t53 * t44;
            let t56 = t24 * t14;
            let tv2rho20 = -f64x8::splat(0.00133756) * t15 - f64x8::splat(0.0622) * t25 + v_rho * (-f64x8::splat(2.5056957333333333e-05) * t36 + f64x8::splat(0.0008917066666666667) * t40 - f64x8::splat(0.0311) * t49 - f64x8::splat(0.010366666666666666) * t54 + f64x8::splat(0.010366666666666666) * t56);
            acc_v2rho2 = tv2rho20;
            let t65 = t11 * t11;
            let t66 = f64x8::splat(1.0) / t65;
            let t67 = t33 * t33;
            let t68 = f64x8::splat(1.0) / t67;
            let t69 = t66 * t68;
            let t71 = t33 * v_rho;
            let t73 = f64x8::splat(1.0) / t18 / t71;
            let t74 = t32 * t73;
            let t77 = f64x8::splat(1.0) / t1 / t71;
            let t78 = t12 * t77;
            let t80 = f64x8::splat(1.0) / t71;
            let t83 = f64x8::splat(10.0) / f64x8::splat(27.0) * t7 * t35 - f64x8::splat(10.0) / f64x8::splat(27.0) * t80;
            let t84 = t83 * t23;
            let t85 = t84 * t2;
            let t87 = t47 * t52;
            let t88 = t87 * t44;
            let t90 = t48 * t14;
            let t93 = f64x8::splat(1.0) / t51 / t7;
            let t94 = t22 * t93;
            let t95 = t94 * t80;
            let t97 = t53 * t35;
            let t99 = t24 * t39;
            let tv3rho30 = -f64x8::splat(7.5170872e-05) * t36 + f64x8::splat(0.00267512) * t40 - f64x8::splat(0.0933) * t49 - f64x8::splat(0.0311) * t54 + f64x8::splat(0.0311) * t56 + v_rho * (-f64x8::splat(1.4082010021333333e-06) * t69 + f64x8::splat(0.00010022782933333333) * t74 - f64x8::splat(0.0020806488888888888) * t78 - f64x8::splat(0.0311) * t85 - f64x8::splat(0.020733333333333333) * t88 + f64x8::splat(0.020733333333333333) * t90 - f64x8::splat(0.006911111111111111) * t95 + f64x8::splat(0.020733333333333333) * t97 - f64x8::splat(0.013822222222222222) * t99);
            acc_v3rho3 = tv3rho30;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        ip += 8;
    }
}
