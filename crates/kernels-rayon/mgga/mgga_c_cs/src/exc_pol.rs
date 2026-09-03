//! MGGA_C_CS exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cs.c`
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
pub fn mgga_c_cs_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t2 = v_rho0 - v_rho1;
            let t3 = t2 * t2;
            let t4 = v_rho0 + v_rho1;
            let t5 = t4 * t4;
            let t6 = f64x8::splat(1.0) / t5;
            let t8 = -t3 * t6 + f64x8::splat(1.0);
            let t9 = (simd::cbrt(t4));
            let t10 = f64x8::splat(1.0) / t9;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.349) * t10;
            let t13 = f64x8::splat(1.0) / t12;
            let t14 = t8 * t13;
            let t16 = (simd::exp(-f64x8::splat(0.2533) * t10));
            let t17 = f64x8::splat(1.0) / t4;
            let t18 = t2 * t17;
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = zeta_threshold * zeta_threshold;
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * t22;
            let t24 = t23 * t21;
            let t25 = t19 * t19;
            let t26 = (simd::cbrt(t19));
            let t27 = t26 * t26;
            let t29 = ((t20).select(t24, t27 * t25));
            let t30 = f64x8::splat(M_CBRT2);
            let t31 = t29 * t30;
            let t32 = (simd::cbrt(v_rho0));
            let t33 = t32 * t32;
            let t35 = f64x8::splat(1.0) / t33 / v_rho0;
            let t37 = v_lapl0 * t35;
            let t39 = v_tau0 * t35 - t37 / f64x8::splat(8.0);
            let t41 = f64x8::splat(1.0) - t18;
            let t42 = (t41).simd_le(zeta_threshold);
            let t43 = t41 * t41;
            let t44 = (simd::cbrt(t41));
            let t45 = t44 * t44;
            let t47 = ((t42).select(t24, t45 * t43));
            let t48 = t47 * t30;
            let t49 = (simd::cbrt(v_rho1));
            let t50 = t49 * t49;
            let t52 = f64x8::splat(1.0) / t50 / v_rho1;
            let t54 = v_lapl1 * t52;
            let t56 = v_tau1 * t52 - t54 / f64x8::splat(8.0);
            let t59 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t60 = t9 * t9;
            let t62 = f64x8::splat(1.0) / t60 / t5;
            let t64 = t19 / f64x8::splat(2.0);
            let t65 = (simd::cbrt(t64));
            let t66 = t65 * t65;
            let t67 = t66 * t64;
            let t69 = t41 / f64x8::splat(2.0);
            let t70 = (simd::cbrt(t69));
            let t71 = t70 * t70;
            let t72 = t71 * t69;
            let t75 = t31 * t39 / f64x8::splat(8.0) + t37 * t67 / f64x8::splat(8.0) + t48 * t56 / f64x8::splat(8.0) + t54 * t72 / f64x8::splat(8.0) - t59 * t62 / f64x8::splat(8.0);
            let t78 = f64x8::splat(1.0) + f64x8::splat(0.264) * t16 * t75;
            let tzk0 = -f64x8::splat(0.04918) * t14 * t78;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
