//! GGA_X_EV93 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ev93.c`
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
pub fn gga_x_ev93_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_b1: f64,
    param_b2: f64,
    param_b3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a1 = f64x8::splat(param_a1);
    let param_a2 = f64x8::splat(param_a2);
    let param_a3 = f64x8::splat(param_a3);
    let param_b1 = f64x8::splat(param_b1);
    let param_b2 = f64x8::splat(param_b2);
    let param_b3 = f64x8::splat(param_b3);
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
            let t18 = t6 * t17;
            let t19 = (simd::cbrt(v_rho));
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = param_a1 * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t21 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t19 * t19;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = t29 * t33;
            let t37 = t20 * t20;
            let t38 = param_a2 * t37;
            let t40 = f64x8::splat(1.0) / t23 / t22;
            let t41 = t38 * t40;
            let t42 = v_sigma * v_sigma;
            let t43 = t42 * t27;
            let t44 = t30 * t30;
            let t45 = t44 * v_rho;
            let t47 = f64x8::splat(1.0) / t19 / t45;
            let t48 = t43 * t47;
            let t51 = t22 * t22;
            let t52 = f64x8::splat(1.0) / t51;
            let t53 = param_a3 * t52;
            let t54 = t42 * v_sigma;
            let t55 = t44 * t44;
            let t56 = f64x8::splat(1.0) / t55;
            let t57 = t54 * t56;
            let t60 = f64x8::splat(1.0) + t26 * t34 / f64x8::splat(24.0) + t41 * t48 / f64x8::splat(288.0) + t53 * t57 / f64x8::splat(576.0);
            let t61 = t19 * t60;
            let t62 = param_b1 * t20;
            let t63 = t62 * t25;
            let t66 = param_b2 * t37;
            let t67 = t66 * t40;
            let t70 = param_b3 * t52;
            let t73 = f64x8::splat(1.0) + t63 * t34 / f64x8::splat(24.0) + t67 * t48 / f64x8::splat(288.0) + t70 * t57 / f64x8::splat(576.0);
            let t74 = f64x8::splat(1.0) / t73;
            let t78 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t61 * t74));
            let tzk0 = f64x8::splat(2.0) * t78;
            acc_zk = tzk0;
            let t79 = f64x8::splat(1.0) / t31;
            let t80 = t79 * t60;
            let t84 = t30 * v_rho;
            let t86 = f64x8::splat(1.0) / t31 / t84;
            let t87 = t29 * t86;
            let t90 = t44 * t30;
            let t92 = f64x8::splat(1.0) / t19 / t90;
            let t93 = t43 * t92;
            let t96 = t55 * v_rho;
            let t97 = f64x8::splat(1.0) / t96;
            let t98 = t54 * t97;
            let t101 = -t26 * t87 / f64x8::splat(9.0) - t41 * t93 / f64x8::splat(54.0) - t53 * t98 / f64x8::splat(72.0);
            let t102 = t19 * t101;
            let t106 = t73 * t73;
            let t107 = f64x8::splat(1.0) / t106;
            let t114 = -t63 * t87 / f64x8::splat(9.0) - t67 * t93 / f64x8::splat(54.0) - t70 * t98 / f64x8::splat(72.0);
            let t115 = t107 * t114;
            let t120 = ((t2).select(f64x8::splat(0.0), -t18 * t80 * t74 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t102 * t74 + f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t61 * t115));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t120 + f64x8::splat(2.0) * t78;
            acc_vrho = tvrho0;
            let t123 = t25 * t28;
            let t124 = t123 * t33;
            let t127 = v_sigma * t27;
            let t128 = t127 * t47;
            let t131 = t42 * t56;
            let t134 = t21 * t124 / f64x8::splat(24.0) + t41 * t128 / f64x8::splat(144.0) + t53 * t131 / f64x8::splat(192.0);
            let t135 = t19 * t134;
            let t144 = t62 * t124 / f64x8::splat(24.0) + t67 * t128 / f64x8::splat(144.0) + t70 * t131 / f64x8::splat(192.0);
            let t145 = t107 * t144;
            let t150 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t135 * t74 + f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t61 * t145));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t150;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
