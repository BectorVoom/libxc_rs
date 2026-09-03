//! LDA_XC_TETER93 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_teter93.c`
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
pub fn lda_xc_teter93_exc_pol(
    rho: &[f64],
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
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t1 = v_rho0 - v_rho1;
            let t2 = v_rho0 + v_rho1;
            let t3 = f64x8::splat(1.0) / t2;
            let t4 = t1 * t3;
            let t5 = f64x8::splat(1.0) + t4;
            let t6 = (t5).simd_le(zeta_threshold);
            let t7 = (simd::cbrt(zeta_threshold));
            let t8 = t7 * zeta_threshold;
            let t9 = (simd::cbrt(t5));
            let t11 = ((t6).select(t8, t9 * t5));
            let t12 = f64x8::splat(1.0) - t4;
            let t13 = (t12).simd_le(zeta_threshold);
            let t14 = (simd::cbrt(t12));
            let t16 = ((t13).select(t8, t14 * t12));
            let t18 = f64x8::splat(M_CBRT2);
            let t21 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t18 - f64x8::splat(2.0));
            let t22 = (t11 + t16 - f64x8::splat(2.0)) * t21;
            let t26 = f64x8::splat(M_CBRT3);
            let t27 = (f64x8::splat(2.217058676663745) + f64x8::splat(0.6157402568883344) * t22) * t26;
            let t28 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t29 = (simd::cbrt(t28));
            let t30 = f64x8::splat(M_CBRT4);
            let t31 = t30 * t30;
            let t32 = t29 * t31;
            let t33 = (simd::cbrt(t2));
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t32 * t34;
            let t40 = t26 * t26;
            let t41 = (f64x8::splat(0.7405551735357053) + f64x8::splat(0.1574201515892867) * t22) * t40;
            let t42 = t29 * t29;
            let t43 = t42 * t30;
            let t44 = t33 * t33;
            let t46 = t43 / t44;
            let t51 = (f64x8::splat(0.01968227878617998) + f64x8::splat(0.003532336663397157) * t22) * t28;
            let t54 = f64x8::splat(0.4581652932831429) + f64x8::splat(0.119086804055547) * t22 + t27 * t35 / f64x8::splat(4.0) + t41 * t46 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t51 * t3;
            let t55 = t26 * t29;
            let t61 = (f64x8::splat(4.504130959426697) + f64x8::splat(0.2673612973836267) * t22) * t40;
            let t66 = (f64x8::splat(1.110667363742916) + f64x8::splat(0.2052004607777787) * t22) * t28;
            let t71 = (f64x8::splat(0.02359291751427506) + f64x8::splat(0.004200005045691381) * t22) * t26;
            let t73 = t29 * t28 * t31;
            let t75 = f64x8::splat(1.0) / t33 / t2;
            let t76 = t73 * t75;
            let t79 = f64x8::splat(0.25) * t55 * t31 * t34 + t61 * t46 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t66 * t3 + f64x8::splat(3.0) / f64x8::splat(16.0) * t71 * t76;
            let t80 = f64x8::splat(1.0) / t79;
            let tzk0 = -t54 * t80;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
