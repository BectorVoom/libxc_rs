//! GGA_C_CHACHIYO exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_chachiyo.c`
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
pub fn gga_c_chachiyo_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    param_h: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_af = f64x8::splat(param_af);
    let param_ap = f64x8::splat(param_ap);
    let param_bf = f64x8::splat(param_bf);
    let param_bp = f64x8::splat(param_bp);
    let param_cf = f64x8::splat(param_cf);
    let param_cp = f64x8::splat(param_cp);
    let param_h = f64x8::splat(param_h);
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
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = t1 * t1;
            let t3 = param_bp * t2;
            let t5 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t7 = f64x8::splat(M_CBRT4);
            let t8 = f64x8::splat(1.0) / t5 * t7;
            let t9 = v_rho0 + v_rho1;
            let t10 = (simd::cbrt(t9));
            let t11 = t8 * t10;
            let t14 = param_cp * t1;
            let t15 = t5 * t5;
            let t17 = t7 * t7;
            let t18 = f64x8::splat(1.0) / t15 * t17;
            let t19 = t10 * t10;
            let t20 = t18 * t19;
            let t23 = f64x8::splat(1.0) + t3 * t11 / f64x8::splat(3.0) + t14 * t20 / f64x8::splat(3.0);
            let t24 = (simd::ln(t23));
            let t25 = param_ap * t24;
            let t26 = param_bf * t2;
            let t29 = param_cf * t1;
            let t32 = f64x8::splat(1.0) + t26 * t11 / f64x8::splat(3.0) + t29 * t20 / f64x8::splat(3.0);
            let t33 = (simd::ln(t32));
            let t35 = param_af * t33 - t25;
            let t36 = v_rho0 - v_rho1;
            let t37 = f64x8::splat(1.0) / t9;
            let t38 = t36 * t37;
            let t39 = f64x8::splat(1.0) + t38;
            let t40 = (t39).simd_le(zeta_threshold);
            let t41 = (simd::cbrt(zeta_threshold));
            let t42 = t41 * t41;
            let t43 = (simd::cbrt(t39));
            let t44 = t43 * t43;
            let t45 = ((t40).select(t42, t44));
            let t46 = f64x8::splat(1.0) - t38;
            let t47 = (t46).simd_le(zeta_threshold);
            let t48 = (simd::cbrt(t46));
            let t49 = t48 * t48;
            let t50 = ((t47).select(t42, t49));
            let t52 = t45 / f64x8::splat(2.0) + t50 / f64x8::splat(2.0);
            let t53 = t52 * t52;
            let t56 = -f64x8::splat(2.0) * t53 * t52 + f64x8::splat(2.0);
            let t58 = t35 * t56 + t25;
            let t59 = f64x8::splat(M_CBRTPI);
            let t60 = t2 * t59;
            let t61 = t9 * t9;
            let t63 = f64x8::splat(1.0) / t10 / t61;
            let t65 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t69 = f64x8::splat(1.0) + t60 * t63 * t65 / f64x8::splat(48.0);
            let t70 = f64x8::splat(1.0) / t58;
            let t71 = param_h * t70;
            let t72 = (simd::pow(t69, t71));
            let tzk0 = t58 * t72;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
