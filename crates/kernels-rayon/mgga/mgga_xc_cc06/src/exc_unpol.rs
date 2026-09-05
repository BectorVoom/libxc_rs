//! MGGA_XC_CC06 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_cc06.c`
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
pub fn mgga_xc_cc06_exc_unpol(
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t9 = (simd::cbrt(zeta_threshold));
            let t11 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t9 * zeta_threshold, f64x8::splat(1.0)));
            let t12 = (simd::cbrt(v_rho));
            let t16 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t11 * t12));
            let t18 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t19 = (simd::cbrt(t18));
            let t20 = t4 * t19;
            let t21 = f64x8::splat(M_CBRT4);
            let t22 = t21 * t21;
            let t25 = t20 * t22 / t12;
            let t27 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t25;
            let t28 = ((t25).sqrt());
            let t31 = ((t25) * (t25).sqrt());
            let t33 = t4 * t4;
            let t34 = t19 * t19;
            let t35 = t33 * t34;
            let t36 = t12 * t12;
            let t37 = f64x8::splat(1.0) / t36;
            let t39 = t35 * t21 * t37;
            let t41 = f64x8::splat(3.79785) * t28 + f64x8::splat(0.8969) * t25 + f64x8::splat(0.204775) * t31 + f64x8::splat(0.123235) * t39;
            let t44 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t41;
            let t45 = (simd::ln(t44));
            let t50 = f64x8::splat(M_CBRT2);
            let t54 = (f64x8::splat(2.0) * t11 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t50 - f64x8::splat(2.0));
            let t56 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t25;
            let t61 = f64x8::splat(5.1785) * t28 + f64x8::splat(0.905775) * t25 + f64x8::splat(0.1100325) * t31 + f64x8::splat(0.1241775) * t39;
            let t64 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t61;
            let t65 = (simd::ln(t64));
            let t69 = f64x8::splat(2.0) * t16 - f64x8::splat(0.062182) * t27 * t45 + f64x8::splat(0.019751789702565206) * t54 * t56 * t65;
            let t70 = t33 * t21;
            let t71 = t34 * v_lapl;
            let t73 = f64x8::splat(1.0) / t36 / v_rho;
            let t75 = t70 * t71 * t73;
            let t77 = -f64x8::splat(0.0007) + f64x8::splat(0.002) * t75;
            let t79 = f64x8::splat(1.0) + f64x8::splat(0.0065) * t75;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = t77 * t80 + f64x8::splat(1.0);
            let tzk0 = t69 * t82;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
