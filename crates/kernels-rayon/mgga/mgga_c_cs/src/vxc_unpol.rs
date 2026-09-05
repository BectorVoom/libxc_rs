//! MGGA_C_CS vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cs.c`
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
pub fn mgga_c_cs_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t2 = (simd::cbrt(v_rho));
            let t3 = f64x8::splat(1.0) / t2;
            let t5 = f64x8::splat(1.0) + f64x8::splat(0.349) * t3;
            let t6 = f64x8::splat(1.0) / t5;
            let t8 = (simd::exp(-f64x8::splat(0.2533) * t3));
            let t10 = zeta_threshold * zeta_threshold;
            let t11 = (simd::cbrt(zeta_threshold));
            let t12 = t11 * t11;
            let t14 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t12 * t10, f64x8::splat(1.0)));
            let t15 = f64x8::splat(M_CBRT2);
            let t16 = t14 * t15;
            let t17 = t15 * t15;
            let t18 = v_tau * t17;
            let t19 = t2 * t2;
            let t21 = f64x8::splat(1.0) / t19 / v_rho;
            let t23 = v_lapl * t17;
            let t29 = v_rho * v_rho;
            let t31 = f64x8::splat(1.0) / t19 / t29;
            let t36 = t16 * (t18 * t21 - t23 * t21 / f64x8::splat(8.0)) / f64x8::splat(4.0) - v_sigma * t31 / f64x8::splat(8.0) + v_lapl * t21 / f64x8::splat(8.0);
            let t39 = f64x8::splat(1.0) + f64x8::splat(0.264) * t8 * t36;
            let tzk0 = -f64x8::splat(0.04918) * t6 * t39;
            acc_zk = tzk0;
            let t42 = t5 * t5;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t3 * t43;
            let t47 = v_rho * t6;
            let t49 = f64x8::splat(1.0) / t2 / v_rho;
            let t50 = t49 * t8;
            let t60 = t29 * v_rho;
            let t62 = f64x8::splat(1.0) / t19 / t60;
            let t67 = t16 * (-f64x8::splat(5.0) / f64x8::splat(3.0) * t18 * t31 + f64x8::splat(5.0) / f64x8::splat(24.0) * t23 * t31) / f64x8::splat(4.0) + v_sigma * t62 / f64x8::splat(3.0) - f64x8::splat(5.0) / f64x8::splat(24.0) * v_lapl * t31;
            let t70 = f64x8::splat(0.0222904) * t50 * t36 + f64x8::splat(0.264) * t8 * t67;
            let tvrho0 = tzk0 - f64x8::splat(0.005721273333333333) * t44 * t39 - f64x8::splat(0.04918) * t47 * t70;
            acc_vrho = tvrho0;
            let t73 = t21 * t6;
            let tvsigma0 = f64x8::splat(0.00162294) * t73 * t8;
            acc_vsigma = tvsigma0;
            let t78 = -t14 * t21 / f64x8::splat(16.0) + t21 / f64x8::splat(8.0);
            let t79 = t8 * t78;
            let tvlapl0 = -f64x8::splat(0.01298352) * t47 * t79;
            acc_vlapl = tvlapl0;
            let t84 = t8 * t14;
            let tvtau0 = -f64x8::splat(0.00649176) / t19 * t6 * t84;
            acc_vtau = tvtau0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        ip += 8;
    }
}
