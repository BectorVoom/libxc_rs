//! GGA_C_TCA exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_tca.c`
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
pub fn gga_c_tca_exc_pol(
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
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        {
            let t1 = v_rho0 - v_rho1;
            let t2 = v_rho0 + v_rho1;
            let t3 = f64x8::splat(1.0) / t2;
            let t4 = t1 * t3;
            let t5 = f64x8::splat(1.0) + t4;
            let t6 = (t5).simd_le(zeta_threshold);
            let t7 = (simd::cbrt(zeta_threshold));
            let t8 = t7 * t7;
            let t9 = (simd::cbrt(t5));
            let t10 = t9 * t9;
            let t11 = ((t6).select(t8, t10));
            let t12 = f64x8::splat(1.0) - t4;
            let t13 = (t12).simd_le(zeta_threshold);
            let t14 = (simd::cbrt(t12));
            let t15 = t14 * t14;
            let t16 = ((t13).select(t8, t15));
            let t18 = t11 / f64x8::splat(2.0) + t16 / f64x8::splat(2.0);
            let t19 = t18 * t18;
            let t20 = t19 * t18;
            let t21 = f64x8::splat(M_CBRT3);
            let t23 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t24 = t21 * t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t25 * t25;
            let t27 = (simd::cbrt(t2));
            let t32 = f64x8::splat(4.88827) + f64x8::splat(0.79425925) * t24 * t26 / t27;
            let t33 = (simd::atan(t32));
            let t35 = -f64x8::splat(0.655868) * t33 + f64x8::splat(0.897889);
            let t36 = t20 * t35;
            let t37 = t21 * t21;
            let t38 = t36 * t37;
            let t39 = f64x8::splat(1.0) / t23;
            let t40 = t39 * t25;
            let t41 = f64x8::splat(M_CBRT6);
            let t42 = t41 * t41;
            let t43 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t44 = (simd::cbrt(t43));
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t42 * t45;
            let t47 = f64x8::splat(M_CBRT2);
            let t49 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t50 = ((t49).sqrt());
            let t51 = t47 * t50;
            let t52 = t27 * t2;
            let t53 = f64x8::splat(1.0) / t52;
            let t55 = t46 * t51 * t53;
            let t56 = (simd::pow(t55, f64x8::splat(2.3)));
            let t58 = f64x8::splat(1.0) + f64x8::splat(0.004712150703442276) * t56;
            let t59 = f64x8::splat(1.0) / t58;
            let t62 = t38 * t40 * t27 * t59;
            let tzk0 = t62 / f64x8::splat(3.0);
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
