//! GGA_C_WI vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_wi.c`
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
pub fn gga_c_wi_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_k: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_d = f64x8::splat(param_d);
    let param_k = f64x8::splat(param_k);
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
            let t1 = param_b * v_sigma;
            let t2 = v_rho * v_rho;
            let t3 = (simd::cbrt(v_rho));
            let t4 = t3 * t3;
            let t6 = f64x8::splat(1.0) / t4 / t2;
            let t7 = param_k * v_sigma;
            let t9 = (simd::exp(-t7 * t6));
            let t12 = t1 * t6 * t9 + param_a;
            let t13 = f64x8::splat(M_CBRT3);
            let t15 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t16 = t13 * t15;
            let t17 = f64x8::splat(M_CBRT4);
            let t18 = t17 * t17;
            let t22 = t13 * t13;
            let t23 = f64x8::splat(M_CBRTPI);
            let t25 = ((v_sigma).sqrt());
            let t26 = t25 * v_sigma;
            let t27 = t2 * t2;
            let t28 = f64x8::splat(1.0) / t27;
            let t31 = f64x8::splat(1.0) / t3 / v_rho;
            let t32 = t25 * t31;
            let t33 = ((t32).sqrt());
            let t38 = f64x8::splat(1.0) + param_d * t17 * t22 * t23 * t33 * t26 * t28 / f64x8::splat(3.0);
            let t42 = param_c + t16 * t18 / t3 * t38 / f64x8::splat(4.0);
            let t43 = f64x8::splat(1.0) / t42;
            let tzk0 = t12 * t43;
            acc_zk = tzk0;
            let t44 = t2 * v_rho;
            let t46 = f64x8::splat(1.0) / t4 / t44;
            let t49 = v_sigma * v_sigma;
            let t50 = param_b * t49;
            let t51 = t27 * t2;
            let t53 = f64x8::splat(1.0) / t3 / t51;
            let t58 = f64x8::splat(8.0) / f64x8::splat(3.0) * t50 * t53 * param_k * t9 - f64x8::splat(8.0) / f64x8::splat(3.0) * t1 * t46 * t9;
            let t59 = v_rho * t58;
            let t61 = v_rho * t12;
            let t62 = t42 * t42;
            let t63 = f64x8::splat(1.0) / t62;
            let t71 = t33 * v_sigma * t6;
            let t72 = t23 * t71;
            let t73 = t72 * t25;
            let t76 = -t16 * t18 * t31 * t38 / f64x8::splat(12.0) - f64x8::splat(14.0) / f64x8::splat(3.0) * t15 * t6 * param_d * t73;
            let t77 = t63 * t76;
            let tvrho0 = t59 * t43 - t61 * t77 + tzk0;
            acc_vrho = tvrho0;
            let t81 = t27 * v_rho;
            let t83 = f64x8::splat(1.0) / t3 / t81;
            let t87 = -t1 * t83 * param_k * t9 + param_b * t6 * t9;
            let t88 = v_rho * t87;
            let t90 = f64x8::splat(1.0) / t4;
            let t91 = t90 * t12;
            let t92 = t63 * t15;
            let t93 = t91 * t92;
            let t94 = param_d * t23;
            let t95 = f64x8::splat(1.0) / t25;
            let t96 = t71 * t95;
            let t97 = t94 * t96;
            let tvsigma0 = t88 * t43 - f64x8::splat(7.0) / f64x8::splat(4.0) * t93 * t97;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
