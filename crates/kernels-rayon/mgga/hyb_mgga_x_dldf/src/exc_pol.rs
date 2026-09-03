//! HYB_MGGA_X_DLDF exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_dldf.c`
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
pub fn hyb_mgga_x_dldf_exc_pol(
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
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = v_rho0 + v_rho1;
            let t5 = f64x8::splat(1.0) / t4;
            let t8 = (f64x8::splat(2.0) * v_rho0 * t5).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t12 = (f64x8::splat(2.0) * v_rho1 * t5).simd_le(zeta_threshold);
            let t13 = -t9;
            let t14 = v_rho0 - v_rho1;
            let t16 = ((t8).select(t9, (t12).select(t13, t14 * t5)));
            let t17 = f64x8::splat(1.0) + t16;
            let t18 = (t17).simd_le(zeta_threshold);
            let t19 = (simd::cbrt(zeta_threshold));
            let t20 = t19 * zeta_threshold;
            let t21 = (simd::cbrt(t17));
            let t23 = ((t18).select(t20, t21 * t17));
            let t24 = t3 * t23;
            let t25 = (simd::cbrt(t4));
            let t26 = f64x8::splat(M_CBRT6);
            let t27 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t28 = (simd::cbrt(t27));
            let t29 = t28 * t28;
            let t30 = f64x8::splat(1.0) / t29;
            let t31 = t26 * t30;
            let t32 = v_rho0 * v_rho0;
            let t33 = (simd::cbrt(v_rho0));
            let t34 = t33 * t33;
            let t36 = f64x8::splat(1.0) / t34 / t32;
            let t40 = f64x8::splat(4.8827323) + f64x8::splat(0.0146297) * t31 * v_sigma0 * t36;
            let t43 = f64x8::splat(5.8827323) - f64x8::splat(23.84107471346329) / t40;
            let t44 = t25 * t43;
            let t45 = t26 * t26;
            let t47 = f64x8::splat(3.0) / f64x8::splat(10.0) * t45 * t29;
            let t49 = f64x8::splat(1.0) / t34 / v_rho0;
            let t50 = v_tau0 * t49;
            let t51 = t47 - t50;
            let t52 = t47 + t50;
            let t53 = f64x8::splat(1.0) / t52;
            let t56 = t51 * t51;
            let t57 = t52 * t52;
            let t58 = f64x8::splat(1.0) / t57;
            let t61 = t56 * t51;
            let t62 = t57 * t52;
            let t63 = f64x8::splat(1.0) / t62;
            let t66 = t56 * t56;
            let t67 = t57 * t57;
            let t68 = f64x8::splat(1.0) / t67;
            let t71 = f64x8::splat(1.0) - f64x8::splat(0.1637571) * t51 * t53 - f64x8::splat(0.1880028) * t56 * t58 - f64x8::splat(0.4490609) * t61 * t63 - f64x8::splat(0.0082359) * t66 * t68;
            let t72 = t44 * t71;
            let t75 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.09872727257880975) * t24 * t72));
            let t76 = (v_rho1).simd_le(dens_threshold);
            let t77 = -t14;
            let t79 = ((t12).select(t9, (t8).select(t13, t77 * t5)));
            let t80 = f64x8::splat(1.0) + t79;
            let t81 = (t80).simd_le(zeta_threshold);
            let t82 = (simd::cbrt(t80));
            let t84 = ((t81).select(t20, t82 * t80));
            let t85 = t3 * t84;
            let t86 = v_rho1 * v_rho1;
            let t87 = (simd::cbrt(v_rho1));
            let t88 = t87 * t87;
            let t90 = f64x8::splat(1.0) / t88 / t86;
            let t94 = f64x8::splat(4.8827323) + f64x8::splat(0.0146297) * t31 * v_sigma2 * t90;
            let t97 = f64x8::splat(5.8827323) - f64x8::splat(23.84107471346329) / t94;
            let t98 = t25 * t97;
            let t100 = f64x8::splat(1.0) / t88 / v_rho1;
            let t101 = v_tau1 * t100;
            let t102 = t47 - t101;
            let t103 = t47 + t101;
            let t104 = f64x8::splat(1.0) / t103;
            let t107 = t102 * t102;
            let t108 = t103 * t103;
            let t109 = f64x8::splat(1.0) / t108;
            let t112 = t107 * t102;
            let t113 = t108 * t103;
            let t114 = f64x8::splat(1.0) / t113;
            let t117 = t107 * t107;
            let t118 = t108 * t108;
            let t119 = f64x8::splat(1.0) / t118;
            let t122 = f64x8::splat(1.0) - f64x8::splat(0.1637571) * t102 * t104 - f64x8::splat(0.1880028) * t107 * t109 - f64x8::splat(0.4490609) * t112 * t114 - f64x8::splat(0.0082359) * t117 * t119;
            let t123 = t98 * t122;
            let t126 = ((t76).select(f64x8::splat(0.0), -f64x8::splat(0.09872727257880975) * t85 * t123));
            let tzk0 = t75 + t126;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
