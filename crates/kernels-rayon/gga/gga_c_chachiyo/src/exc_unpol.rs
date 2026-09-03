//! GGA_C_CHACHIYO exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_chachiyo.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_chachiyo_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    param_h: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_af = f64x8::splat(param_af);
    let param_ap = f64x8::splat(param_ap);
    let param_bf = f64x8::splat(param_bf);
    let param_bp = f64x8::splat(param_bp);
    let param_cf = f64x8::splat(param_cf);
    let param_cp = f64x8::splat(param_cp);
    let param_h = f64x8::splat(param_h);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = t1 * t1;
            let t3 = param_bp * t2;
            let t5 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t7 = f64x8::splat(M_CBRT4);
            let t8 = f64x8::splat(1.0) / t5 * t7;
            let t9 = (simd::cbrt(v_rho));
            let t10 = t8 * t9;
            let t13 = param_cp * t1;
            let t14 = t5 * t5;
            let t16 = t7 * t7;
            let t17 = f64x8::splat(1.0) / t14 * t16;
            let t18 = t9 * t9;
            let t19 = t17 * t18;
            let t22 = f64x8::splat(1.0) + t3 * t10 / f64x8::splat(3.0) + t13 * t19 / f64x8::splat(3.0);
            let t23 = (simd::ln(t22));
            let t24 = param_ap * t23;
            let t25 = param_bf * t2;
            let t28 = param_cf * t1;
            let t31 = f64x8::splat(1.0) + t25 * t10 / f64x8::splat(3.0) + t28 * t19 / f64x8::splat(3.0);
            let t32 = (simd::ln(t31));
            let t36 = (simd::cbrt(zeta_threshold));
            let t37 = t36 * t36;
            let t38 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t37, f64x8::splat(1.0)));
            let t39 = t38 * t38;
            let t42 = -f64x8::splat(2.0) * t39 * t38 + f64x8::splat(2.0);
            let t44 = t24 + (param_af * t32 - t24) * t42;
            let t45 = f64x8::splat(M_CBRTPI);
            let t46 = t2 * t45;
            let t47 = v_rho * v_rho;
            let t49 = f64x8::splat(1.0) / t9 / t47;
            let t53 = f64x8::splat(1.0) + t46 * t49 * v_sigma / f64x8::splat(48.0);
            let t54 = f64x8::splat(1.0) / t44;
            let t55 = param_h * t54;
            let t56 = (simd::pow(t53, t55));
            let tzk0 = t44 * t56;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
