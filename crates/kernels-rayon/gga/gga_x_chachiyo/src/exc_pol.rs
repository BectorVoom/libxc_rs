//! GGA_X_CHACHIYO exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_chachiyo.c`
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
pub fn gga_x_chachiyo_exc_pol(
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
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = t18 + f64x8::splat(1.0);
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = t5 * t25;
            let t27 = (simd::cbrt(t6));
            let t28 = t3 * t3;
            let t29 = t2 * t28;
            let t30 = f64x8::splat(M_CBRT2);
            let t31 = t30 * v_sigma0;
            let t32 = v_rho0 * v_rho0;
            let t33 = (simd::cbrt(v_rho0));
            let t34 = t33 * t33;
            let t36 = f64x8::splat(1.0) / t34 / t32;
            let t40 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t41 = t2 * t2;
            let t42 = t41 * t3;
            let t43 = t30 * t30;
            let t44 = ((v_sigma0).sqrt());
            let t47 = f64x8::splat(1.0) / t33 / v_rho0;
            let t49 = t42 * t43 * t44 * t47;
            let t51 = t49 / f64x8::splat(27.0) + f64x8::splat(1.0);
            let t52 = (simd::ln(t51));
            let t54 = f64x8::splat(2.0) / f64x8::splat(81.0) * t29 * t31 * t36 + t40 * t52;
            let t57 = t49 / f64x8::splat(9.0) + t40;
            let t58 = f64x8::splat(1.0) / t57;
            let t59 = f64x8::splat(1.0) / t52;
            let t60 = t58 * t59;
            let t61 = t27 * t54 * t60;
            let t64 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t61));
            let t65 = (v_rho1).simd_le(dens_threshold);
            let t66 = -t16;
            let t68 = ((t14).select(t11, (t10).select(t15, t66 * t7)));
            let t69 = t68 + f64x8::splat(1.0);
            let t70 = (t69).simd_le(zeta_threshold);
            let t71 = (simd::cbrt(t69));
            let t73 = ((t70).select(t22, t71 * t69));
            let t74 = t5 * t73;
            let t75 = t30 * v_sigma2;
            let t76 = v_rho1 * v_rho1;
            let t77 = (simd::cbrt(v_rho1));
            let t78 = t77 * t77;
            let t80 = f64x8::splat(1.0) / t78 / t76;
            let t84 = ((v_sigma2).sqrt());
            let t87 = f64x8::splat(1.0) / t77 / v_rho1;
            let t89 = t42 * t43 * t84 * t87;
            let t91 = t89 / f64x8::splat(27.0) + f64x8::splat(1.0);
            let t92 = (simd::ln(t91));
            let t94 = f64x8::splat(2.0) / f64x8::splat(81.0) * t29 * t75 * t80 + t40 * t92;
            let t97 = t89 / f64x8::splat(9.0) + t40;
            let t98 = f64x8::splat(1.0) / t97;
            let t99 = f64x8::splat(1.0) / t92;
            let t100 = t98 * t99;
            let t101 = t27 * t94 * t100;
            let t104 = ((t65).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t74 * t101));
            let tzk0 = t64 + t104;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
