//! LDA_X vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x.c`
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
pub fn lda_x_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alpha = f64x8::splat(param_alpha);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t8 = v_rho0 * t7;
            let t10 = (f64x8::splat(2.0) * t8).simd_le(zeta_threshold);
            let t11 = (simd::cbrt(zeta_threshold));
            let t12 = t11 * zeta_threshold;
            let t13 = f64x8::splat(M_CBRT2);
            let t14 = t13 * v_rho0;
            let t15 = (simd::cbrt(t8));
            let t19 = ((t10).select(t12, f64x8::splat(2.0) * t14 * t7 * t15));
            let t20 = (simd::cbrt(t6));
            let t24 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t19 * t20));
            let t25 = param_alpha * t24;
            let t26 = (v_rho1).simd_le(dens_threshold);
            let t27 = v_rho1 * t7;
            let t29 = (f64x8::splat(2.0) * t27).simd_le(zeta_threshold);
            let t30 = t13 * v_rho1;
            let t31 = (simd::cbrt(t27));
            let t35 = ((t29).select(t12, f64x8::splat(2.0) * t30 * t7 * t31));
            let t39 = ((t26).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t35 * t20));
            let t40 = param_alpha * t39;
            let tzk0 = t25 + t40;
            acc_zk = tzk0;
            let t41 = t13 * t7;
            let t44 = t6 * t6;
            let t45 = f64x8::splat(1.0) / t44;
            let t48 = f64x8::splat(2.0) * t14 * t45 * t15;
            let t49 = t15 * t15;
            let t50 = f64x8::splat(1.0) / t49;
            let t51 = t7 * t50;
            let t53 = -v_rho0 * t45 + t7;
            let t58 = ((t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t41 * t15 - t48 + f64x8::splat(2.0) / f64x8::splat(3.0) * t14 * t51 * t53));
            let t62 = t20 * t20;
            let t63 = f64x8::splat(1.0) / t62;
            let t66 = t5 * t19 * t63 / f64x8::splat(8.0);
            let t68 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t58 * t20 - t66));
            let t69 = param_alpha * t68;
            let t72 = f64x8::splat(2.0) * t30 * t45 * t31;
            let t73 = v_rho1 * v_rho1;
            let t74 = t13 * t73;
            let t75 = t44 * t6;
            let t76 = f64x8::splat(1.0) / t75;
            let t77 = t31 * t31;
            let t78 = f64x8::splat(1.0) / t77;
            let t79 = t76 * t78;
            let t83 = ((t29).select(f64x8::splat(0.0), -t72 - f64x8::splat(2.0) / f64x8::splat(3.0) * t74 * t79));
            let t89 = t5 * t35 * t63 / f64x8::splat(8.0);
            let t91 = ((t26).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t83 * t20 - t89));
            let t92 = param_alpha * t91;
            let tvrho0 = t25 + t40 + t6 * (t69 + t92);
            acc_vrho_0 = tvrho0;
            let t95 = v_rho0 * v_rho0;
            let t96 = t13 * t95;
            let t97 = t76 * t50;
            let t101 = ((t10).select(f64x8::splat(0.0), -t48 - f64x8::splat(2.0) / f64x8::splat(3.0) * t96 * t97));
            let t106 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t101 * t20 - t66));
            let t107 = param_alpha * t106;
            let t110 = t7 * t78;
            let t112 = -v_rho1 * t45 + t7;
            let t117 = ((t29).select(f64x8::splat(0.0), f64x8::splat(2.0) * t41 * t31 - t72 + f64x8::splat(2.0) / f64x8::splat(3.0) * t30 * t110 * t112));
            let t122 = ((t26).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t117 * t20 - t89));
            let t123 = param_alpha * t122;
            let tvrho1 = t25 + t40 + t6 * (t107 + t123);
            acc_vrho_1 = tvrho1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}
