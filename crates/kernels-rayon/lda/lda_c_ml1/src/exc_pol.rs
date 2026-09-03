//! LDA_C_ML1 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_ml1.c`
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
pub fn lda_c_ml1_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    param_fc: f64,
    param_q: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_fc = f64x8::splat(param_fc);
    let param_q = f64x8::splat(param_q);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t1 = v_rho0 + v_rho1;
            let t2 = v_rho0 - v_rho1;
            let t3 = f64x8::splat(1.0) / t1;
            let t4 = t2 * t3;
            let t5 = ((t4).abs());
            let t7 = (f64x8::splat(1.0) - t5).simd_le(zeta_threshold);
            let t8 = t2 * t2;
            let t9 = t1 * t1;
            let t10 = f64x8::splat(1.0) / t9;
            let t12 = -t8 * t10 + f64x8::splat(1.0);
            let t13 = (simd::cbrt(t1));
            let t14 = t13 * param_fc;
            let t16 = (f64x8::splat(1.0) + t4).simd_le(zeta_threshold);
            let t17 = zeta_threshold - f64x8::splat(1.0);
            let t19 = (f64x8::splat(1.0) - t4).simd_le(zeta_threshold);
            let t21 = ((t16).select(t17, (t19).select(-t17, t4)));
            let t22 = f64x8::splat(1.0) + t21;
            let t23 = (simd::pow(t22, param_q));
            let t24 = f64x8::splat(1.0) - t21;
            let t25 = (simd::pow(t24, param_q));
            let t26 = t23 + t25;
            let t27 = t21 * t21;
            let t28 = f64x8::splat(1.0) - t27;
            let t29 = (simd::cbrt(t28));
            let t30 = t26 * t29;
            let t31 = (simd::cbrt(t22));
            let t32 = (simd::cbrt(t24));
            let t33 = t31 + t32;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t30 * t34;
            let t38 = f64x8::splat(1.0) + f64x8::splat(10.874334072525) * t14 * t35;
            let t41 = f64x8::splat(1.0) / t13;
            let t42 = f64x8::splat(1.0) / param_fc;
            let t43 = t41 * t42;
            let t44 = f64x8::splat(1.0) / t26;
            let t45 = f64x8::splat(1.0) / t29;
            let t46 = t44 * t45;
            let t47 = t46 * t33;
            let t48 = t43 * t47;
            let t50 = f64x8::splat(1.0) + f64x8::splat(0.09195962397381102) * t48;
            let t51 = (simd::ln(t50));
            let t52 = t51 * t41;
            let t53 = t52 * t42;
            let t57 = t13 * t13;
            let t58 = f64x8::splat(1.0) / t57;
            let t59 = param_fc * param_fc;
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t58 * t60;
            let t62 = t26 * t26;
            let t63 = f64x8::splat(1.0) / t62;
            let t64 = t29 * t29;
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t63 * t65;
            let t67 = t33 * t33;
            let t68 = t66 * t67;
            let t71 = -f64x8::splat(2.763169) / t38 + f64x8::splat(0.28144540420067765) * t53 * t47 + f64x8::splat(0.2541000285260132) * t48 - f64x8::splat(0.049248579417833935) * t61 * t68;
            let t74 = ((t7).select(f64x8::splat(0.0), t12 * t71 / f64x8::splat(4.0)));
            let tzk0 = t1 * t74;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
