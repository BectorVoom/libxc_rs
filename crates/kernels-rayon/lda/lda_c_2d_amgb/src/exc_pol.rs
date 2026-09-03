//! LDA_C_2D_AMGB exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_2d_amgb.c`
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
pub fn lda_c_2d_amgb_exc_pol(
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
            let t1 = v_rho0 + v_rho1;
            let t2 = ((t1).sqrt());
            let t3 = f64x8::splat(1.0) / t2;
            let t5 = f64x8::splat(1.0) / t1;
            let t8 = f64x8::splat(1.0) / t2 / t1;
            let t10 = f64x8::splat(0.04869723403850762) * t3 + f64x8::splat(0.018219548589342285) * t5 + f64x8::splat(0.000603947002028882) * t8;
            let t12 = ((f64x8::splat(M_PI)).sqrt());
            let t13 = f64x8::splat(1.0) / t12;
            let t14 = t13 * t3;
            let t15 = ((t14) * (t14).sqrt());
            let t19 = f64x8::splat(0.5654308006315614) * t3 - f64x8::splat(0.02069) * t15 + f64x8::splat(0.10821581200590331) * t5 + f64x8::splat(0.00313738702352666) * t8;
            let t21 = f64x8::splat(1.0) + f64x8::splat(1.0) / t19;
            let t22 = (simd::ln(t21));
            let t23 = t10 * t22;
            let t27 = -f64x8::splat(0.01914859446561085) * t3 - f64x8::splat(0.0024406887987971425) * t5 - f64x8::splat(1.643337945467037e-05) * t8;
            let t31 = f64x8::splat(0.2331795548802877) * t3 + f64x8::splat(0.021277965468762) * t5 + f64x8::splat(0.0001400599965454174) * t8;
            let t33 = f64x8::splat(1.0) + f64x8::splat(1.0) / t31;
            let t34 = (simd::ln(t33));
            let t36 = f64x8::splat(0.117331) + t27 * t34;
            let t37 = v_rho0 - v_rho1;
            let t38 = t37 * t37;
            let t39 = t36 * t38;
            let t40 = t1 * t1;
            let t41 = f64x8::splat(1.0) / t40;
            let t42 = t39 * t41;
            let t46 = -f64x8::splat(0.020927484222536923) * t3 + f64x8::splat(0.005208122695761946) * t5 - f64x8::splat(0.0048916627893863685) * t8;
            let t49 = f64x8::splat(0.8035757880366529) * t3 + f64x8::splat(0.2088776021566591) * t8;
            let t51 = f64x8::splat(1.0) + f64x8::splat(1.0) / t49;
            let t52 = (simd::ln(t51));
            let t54 = f64x8::splat(0.0234188) + t46 * t52;
            let t55 = t38 * t38;
            let t56 = t54 * t55;
            let t57 = t40 * t40;
            let t58 = f64x8::splat(1.0) / t57;
            let t59 = t56 * t58;
            let t61 = (simd::exp(-f64x8::splat(0.7552241765370266) * t3));
            let t63 = f64x8::splat(M_SQRT2);
            let t64 = (t61 - f64x8::splat(1.0)) * t63;
            let t65 = t13 * t2;
            let t66 = t37 * t5;
            let t67 = f64x8::splat(1.0) + t66;
            let t68 = (t67).simd_le(zeta_threshold);
            let t69 = ((zeta_threshold).sqrt());
            let t70 = t69 * zeta_threshold;
            let t71 = ((t67).sqrt());
            let t72 = t71 * t67;
            let t73 = ((t68).select(t70, t72));
            let t75 = f64x8::splat(1.0) - t66;
            let t76 = (t75).simd_le(zeta_threshold);
            let t77 = ((t75).sqrt());
            let t78 = t77 * t75;
            let t79 = ((t76).select(t70, t78));
            let t85 = t73 / f64x8::splat(2.0) + t79 / f64x8::splat(2.0) - f64x8::splat(1.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t38 * t41 - f64x8::splat(3.0) / f64x8::splat(128.0) * t55 * t58;
            let t88 = f64x8::splat(4.0) / f64x8::splat(3.0) * t64 * t65 * t85;
            let tzk0 = -f64x8::splat(0.1925) + t23 + t42 + t59 - t88;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
