//! LDA_C_RC04 kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_rc04.c`
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
pub fn lda_c_rc04_kxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        {
            let t2 = (simd::cbrt(zeta_threshold));
            let t3 = t2 * t2;
            let t4 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t3, f64x8::splat(1.0)));
            let t5 = t4 * t4;
            let t6 = t5 * t4;
            let t7 = f64x8::splat(M_CBRT3);
            let t9 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t10 = t7 * t9;
            let t11 = f64x8::splat(M_CBRT4);
            let t12 = t11 * t11;
            let t13 = (simd::cbrt(v_rho));
            let t18 = f64x8::splat(4.88827) + f64x8::splat(0.79425925) * t10 * t12 / t13;
            let t19 = (simd::atan(t18));
            let t23 = t7 * t7;
            let t24 = t6 * (-f64x8::splat(0.655868) * t19 + f64x8::splat(0.897889)) * t23;
            let t26 = f64x8::splat(1.0) / t9 * t11;
            let t28 = t24 * t26 * t13;
            let tzk0 = t28 / f64x8::splat(3.0);
            acc_zk = tzk0;
            let t30 = t18 * t18;
            let t31 = t30 + f64x8::splat(1.0);
            let t32 = f64x8::splat(1.0) / t31;
            let tvrho0 = f64x8::splat(4.0) / f64x8::splat(9.0) * t28 + f64x8::splat(0.6945723010386666) * t6 * t32;
            acc_vrho = tvrho0;
            let t39 = t13 * t13;
            let t44 = t31 * t31;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t6 * t45;
            let tv2rho20 = f64x8::splat(0.9260964013848889) * t6 / v_rho * t32 + f64x8::splat(4.0) / f64x8::splat(27.0) * t24 * t26 / t39 + f64x8::splat(0.3677803165958304) * t46 * t18 * t10 * t12 / t13 / v_rho;
            acc_v2rho2 = tv2rho20;
            let t54 = v_rho * v_rho;
            let t65 = t18 * t7 * t9 * t12;
            let t74 = f64x8::splat(1.0) / t44 / t31;
            let t75 = t6 * t74;
            let t77 = t9 * t9;
            let t78 = t23 * t77;
            let t80 = f64x8::splat(1.0) / t39 / t54;
            let t86 = t77 * t11;
            let tv3rho30 = -f64x8::splat(0.6173976009232592) * t6 / t54 * t32 - f64x8::splat(1e-20) * t6 / t13 / t54 * t45 * t65 - f64x8::splat(8.0) / f64x8::splat(81.0) * t24 * t26 / t39 / v_rho + f64x8::splat(1.5579355649288897) * t75 * t30 * t78 * t11 * t80 - f64x8::splat(0.38948389123222243) * t46 * t23 * t86 * t80;
            acc_v3rho3 = tv3rho30;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        ip += 8;
    }
}
