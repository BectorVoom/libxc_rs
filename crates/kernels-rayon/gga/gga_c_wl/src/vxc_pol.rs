//! GGA_C_WL vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_wl.c`
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
pub fn gga_c_wl_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = v_rho0 - v_rho1;
            let t2 = t1 * t1;
            let t3 = v_rho0 + v_rho1;
            let t4 = t3 * t3;
            let t5 = f64x8::splat(1.0) / t4;
            let t7 = -t2 * t5 + f64x8::splat(1.0);
            let t8 = ((t7).sqrt());
            let t10 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t11 = ((t10).sqrt());
            let t12 = (simd::cbrt(t3));
            let t14 = f64x8::splat(1.0) / t12 / t3;
            let t17 = -f64x8::splat(0.7486) + f64x8::splat(0.06001) * t11 * t14;
            let t18 = t8 * t17;
            let t19 = ((v_sigma0).sqrt());
            let t20 = (simd::cbrt(v_rho0));
            let t22 = f64x8::splat(1.0) / t20 / v_rho0;
            let t25 = ((v_sigma2).sqrt());
            let t26 = (simd::cbrt(v_rho1));
            let t28 = f64x8::splat(1.0) / t26 / v_rho1;
            let t31 = f64x8::splat(M_CBRT3);
            let t33 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t34 = t31 * t33;
            let t35 = f64x8::splat(M_CBRT4);
            let t36 = t35 * t35;
            let t37 = f64x8::splat(1.0) / t12;
            let t41 = f64x8::splat(3.60073) + f64x8::splat(0.9) * t19 * t22 + f64x8::splat(0.9) * t25 * t28 + t34 * t36 * t37 / f64x8::splat(4.0);
            let t42 = f64x8::splat(1.0) / t41;
            let tzk0 = t18 * t42;
            acc_zk = tzk0;
            let t43 = f64x8::splat(1.0) / t8;
            let t44 = t3 * t43;
            let t45 = t17 * t42;
            let t46 = t1 * t5;
            let t47 = t4 * t3;
            let t48 = f64x8::splat(1.0) / t47;
            let t49 = t2 * t48;
            let t51 = -f64x8::splat(2.0) * t46 + f64x8::splat(2.0) * t49;
            let t55 = t14 * t8;
            let t56 = t11 * t42;
            let t58 = f64x8::splat(0.08001333333333334) * t55 * t56;
            let t59 = t3 * t8;
            let t60 = t41 * t41;
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t17 * t61;
            let t63 = v_rho0 * v_rho0;
            let t65 = f64x8::splat(1.0) / t20 / t63;
            let t70 = t34 * t36 * t14 / f64x8::splat(12.0);
            let t71 = -f64x8::splat(1.2) * t19 * t65 - t70;
            let tvrho0 = tzk0 + t44 * t45 * t51 / f64x8::splat(2.0) - t58 - t59 * t62 * t71;
            acc_vrho_0 = tvrho0;
            let t75 = f64x8::splat(2.0) * t46 + f64x8::splat(2.0) * t49;
            let t79 = v_rho1 * v_rho1;
            let t81 = f64x8::splat(1.0) / t26 / t79;
            let t84 = -f64x8::splat(1.2) * t25 * t81 - t70;
            let tvrho1 = tzk0 + t44 * t45 * t75 / f64x8::splat(2.0) - t58 - t59 * t62 * t84;
            acc_vrho_1 = tvrho1;
            let t87 = t37 * t8;
            let t88 = f64x8::splat(1.0) / t11;
            let t89 = t88 * t42;
            let t90 = t87 * t89;
            let t91 = f64x8::splat(0.030005) * t90;
            let t92 = t59 * t17;
            let t93 = f64x8::splat(1.0) / t19;
            let t94 = t61 * t93;
            let t95 = t94 * t22;
            let tvsigma0 = t91 - f64x8::splat(0.45) * t92 * t95;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.06001) * t90;
            acc_vsigma_1 = tvsigma1;
            let t98 = f64x8::splat(1.0) / t25;
            let t99 = t61 * t98;
            let t100 = t99 * t28;
            let tvsigma2 = t91 - f64x8::splat(0.45) * t92 * t100;
            acc_vsigma_2 = tvsigma2;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
