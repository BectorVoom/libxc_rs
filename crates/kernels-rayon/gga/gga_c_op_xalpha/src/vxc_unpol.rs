//! GGA_C_OP_XALPHA vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_xalpha.c`
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
pub fn gga_c_op_xalpha_vxc_unpol(
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        {
            let t1 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t4 = (t1) | ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold));
            let t5 = zeta_threshold - f64x8::splat(1.0);
            let t6 = -t5;
            let t7 = ((t1).select(t5, (t1).select(t6, f64x8::splat(0.0))));
            let t8 = t7 * t7;
            let t9 = f64x8::splat(1.0) - t8;
            let t10 = t9 * v_rho;
            let t11 = f64x8::splat(1.0) + t7;
            let t14 = (t11 * v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t15 = f64x8::splat(M_CBRT3);
            let t16 = t15 * t15;
            let t18 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t20 = t16 / t18;
            let t21 = f64x8::splat(M_CBRT4);
            let t22 = f64x8::splat(M_CBRT2);
            let t23 = t21 * t22;
            let t24 = (t11).simd_le(zeta_threshold);
            let t25 = f64x8::splat(1.0) - t7;
            let t26 = (t25).simd_le(zeta_threshold);
            let t27 = ((t24).select(t5, (t26).select(t6, t7)));
            let t28 = f64x8::splat(1.0) + t27;
            let t29 = t28 * v_rho;
            let t30 = (simd::cbrt(t29));
            let t35 = ((t14).select(f64x8::splat(0.0), t20 * t23 / t30 / f64x8::splat(9.0)));
            let t39 = (t25 * v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t40 = ((t26).select(t5, (t24).select(t6, -t7)));
            let t41 = f64x8::splat(1.0) + t40;
            let t42 = t41 * v_rho;
            let t43 = (simd::cbrt(t42));
            let t48 = ((t39).select(f64x8::splat(0.0), t20 * t23 / t43 / f64x8::splat(9.0)));
            let t49 = t35 + t48;
            let t50 = (t49).simd_eq(f64x8::splat(0.0));
            let t51 = ((t50).select(f64x8::splat(f64::EPSILON), t49));
            let t54 = f64x8::splat(3.90299956) / t51 + f64x8::splat(0.5764);
            let t55 = t51 * t51;
            let t56 = t55 * t55;
            let t57 = f64x8::splat(1.0) / t56;
            let t59 = t55 * t51;
            let t60 = f64x8::splat(1.0) / t59;
            let t62 = f64x8::splat(1.0) / t55;
            let t64 = f64x8::splat(43.31320905673766) * t57 + f64x8::splat(19.051463748196298) * t60 + f64x8::splat(2.094820520028) * t62;
            let t65 = f64x8::splat(1.0) / t64;
            let tzk0 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t10 * t54 * t65));
            acc_zk = tzk0;
            let t69 = t9 * t54;
            let t72 = t20 * t21;
            let t79 = ((t14).select(f64x8::splat(0.0), -t72 * t22 / t30 / t29 * t28 / f64x8::splat(27.0)));
            let t86 = ((t39).select(f64x8::splat(0.0), -t72 * t22 / t43 / t42 * t41 / f64x8::splat(27.0)));
            let t88 = ((t50).select(f64x8::splat(0.0), t79 + t86));
            let t93 = t64 * t64;
            let t94 = f64x8::splat(1.0) / t93;
            let t95 = t54 * t94;
            let t97 = f64x8::splat(1.0) / t56 / t51;
            let t98 = t97 * t88;
            let t100 = t57 * t88;
            let t104 = -f64x8::splat(173.25283622695065) * t98 - f64x8::splat(57.15439124458889) * t100 - f64x8::splat(4.189641040056) * t60 * t88;
            let t109 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t69 * t65 + f64x8::splat(0.97574989) * t10 * t62 * t88 * t65 + f64x8::splat(0.25) * t10 * t95 * t104));
            let tvrho0 = v_rho * t109 + tzk0;
            acc_vrho = tvrho0;
            let tvsigma0 = f64x8::splat(0.0);
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
