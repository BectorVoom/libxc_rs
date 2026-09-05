//! MGGA_XC_ZLP exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_zlp.c`
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

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_zlp_exc_pol(
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
            let t2 = f64x8::splat(M_CBRT3);
            let t4 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t11 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t12 = v_rho0 + v_rho1;
            let t13 = t12 * t12;
            let t14 = (simd::cbrt(t12));
            let t15 = t14 * t14;
            let t17 = f64x8::splat(1.0) / t15 / t13;
            let t19 = (simd::cbrt(v_rho0));
            let t20 = t19 * t19;
            let t22 = f64x8::splat(1.0) / t20 / v_rho0;
            let t23 = v_lapl0 * t22;
            let t24 = v_rho0 - v_rho1;
            let t25 = f64x8::splat(1.0) / t12;
            let t26 = t24 * t25;
            let t28 = f64x8::splat(1.0) / f64x8::splat(2.0) + t26 / f64x8::splat(2.0);
            let t29 = (simd::cbrt(t28));
            let t30 = t29 * t29;
            let t31 = t30 * t28;
            let t33 = (simd::cbrt(v_rho1));
            let t34 = t33 * t33;
            let t36 = f64x8::splat(1.0) / t34 / v_rho1;
            let t37 = v_lapl1 * t36;
            let t39 = f64x8::splat(1.0) / f64x8::splat(2.0) - t26 / f64x8::splat(2.0);
            let t40 = (simd::cbrt(t39));
            let t41 = t40 * t40;
            let t42 = t41 * t39;
            let t49 = f64x8::splat(0.207108) * t5 * t7 + f64x8::splat(0.005387725) * t5 * t7 * (t11 * t17 / f64x8::splat(8.0) - t23 * t31 / f64x8::splat(8.0) - t37 * t42 / f64x8::splat(8.0));
            let t52 = f64x8::splat(1.0) + f64x8::splat(488.4942506669168) / t14;
            let t53 = (simd::ln(t52));
            let t56 = f64x8::splat(1.0) - f64x8::splat(0.002047107) * t53 * t14;
            let t58 = t2 * t2;
            let t59 = t49 * t56 * t58;
            let t60 = f64x8::splat(1.0) / t4;
            let t61 = t60 * t6;
            let t62 = t61 * t14;
            let t63 = t59 * t62;
            let tzk0 = -t63 / f64x8::splat(3.0);
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
