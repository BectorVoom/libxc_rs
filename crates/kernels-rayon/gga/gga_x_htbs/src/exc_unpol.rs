//! GGA_X_HTBS exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_htbs.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_htbs_exc_unpol(
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = t20 * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t25 = t21 / t23;
            let t26 = ((v_sigma).sqrt());
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t26 * t27;
            let t30 = f64x8::splat(1.0) / t18 / v_rho;
            let t32 = t25 * t28 * t30;
            let t33 = t32 / f64x8::splat(12.0);
            let t34 = (t33).simd_le(f64x8::splat(0.6));
            let t35 = t23 * t23;
            let t36 = f64x8::splat(1.0) / t35;
            let t37 = t20 * t36;
            let t38 = t27 * t27;
            let t39 = v_sigma * t38;
            let t40 = v_rho * v_rho;
            let t41 = t18 * t18;
            let t43 = f64x8::splat(1.0) / t41 / t40;
            let t45 = t37 * t39 * t43;
            let t47 = t37 * v_sigma;
            let t48 = t38 * t43;
            let t50 = (simd::exp(-t45 / f64x8::splat(24.0)));
            let t51 = t48 * t50;
            let t55 = f64x8::splat(1.0) / t23 / t22;
            let t56 = t21 * t55;
            let t57 = v_sigma * v_sigma;
            let t58 = t57 * t27;
            let t59 = t40 * t40;
            let t60 = t59 * v_rho;
            let t62 = f64x8::splat(1.0) / t18 / t60;
            let t64 = t56 * t58 * t62;
            let t66 = f64x8::splat(1.0) + f64x8::splat(2.7560657413756314e-05) * t64;
            let t67 = (simd::ln(t66));
            let t68 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t45 + f64x8::splat(0.004002424276710846) * t47 * t51 + t67;
            let t71 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t68;
            let t72 = (f64x8::splat(2.6)).simd_le(t33);
            let t74 = (simd::exp(-f64x8::splat(0.011376190545424806) * t45));
            let t76 = f64x8::splat(1.804) - f64x8::splat(0.804) * t74;
            let t77 = f64x8::splat(0.190125) * t32;
            let t78 = f64x8::splat(0.195) * t45;
            let t79 = t26 * v_sigma;
            let t80 = f64x8::splat(1.0) / t59;
            let t82 = f64x8::splat(0.017625664237781676) * t79 * t80;
            let t83 = f64x8::splat(0.005208333333333333) * t64;
            let t86 = t20 / t35 / t22;
            let t87 = t26 * t57;
            let t88 = t87 * t38;
            let t89 = t59 * t40;
            let t91 = f64x8::splat(1.0) / t41 / t89;
            let t94 = f64x8::splat(0.0003255208333333333) * t86 * t88 * t91;
            let t95 = -f64x8::splat(0.40608) + t77 - t78 + t82 - t83 + t94;
            let t97 = f64x8::splat(1.40608) - t77 + t78 - t82 + t83 - t94;
            let t100 = ((t34).select(t71, (t72).select(t76, t97 * t71 + t95 * t76)));
            let t104 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t100));
            let tzk0 = f64x8::splat(2.0) * t104;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
