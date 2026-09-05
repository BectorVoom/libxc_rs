//! LDA_C_ML1 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_ml1.c`
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
pub fn lda_c_ml1_vxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_fc: f64,
    param_q: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_fc = f64x8::splat(param_fc);
    let param_q = f64x8::splat(param_q);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        {
            let t1 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t2 = (simd::cbrt(v_rho));
            let t4 = zeta_threshold - f64x8::splat(1.0);
            let t6 = ((t1).select(t4, (t1).select(-t4, f64x8::splat(0.0))));
            let t7 = f64x8::splat(1.0) + t6;
            let t8 = (simd::pow(t7, param_q));
            let t9 = f64x8::splat(1.0) - t6;
            let t10 = (simd::pow(t9, param_q));
            let t11 = t8 + t10;
            let t12 = t6 * t6;
            let t13 = f64x8::splat(1.0) - t12;
            let t14 = (simd::cbrt(t13));
            let t16 = (simd::cbrt(t7));
            let t17 = (simd::cbrt(t9));
            let t18 = t16 + t17;
            let t20 = t11 * t14 / t18;
            let t23 = f64x8::splat(1.0) + f64x8::splat(10.874334072525) * t2 * param_fc * t20;
            let t26 = f64x8::splat(1.0) / t2;
            let t27 = f64x8::splat(1.0) / param_fc;
            let t32 = f64x8::splat(1.0) / t11 / t14 * t18;
            let t33 = t26 * t27 * t32;
            let t35 = f64x8::splat(1.0) + f64x8::splat(0.09195962397381102) * t33;
            let t36 = (simd::ln(t35));
            let t42 = t2 * t2;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = param_fc * param_fc;
            let t45 = f64x8::splat(1.0) / t44;
            let t47 = t11 * t11;
            let t48 = f64x8::splat(1.0) / t47;
            let t49 = t14 * t14;
            let t50 = f64x8::splat(1.0) / t49;
            let t52 = t18 * t18;
            let t53 = t48 * t50 * t52;
            let t57 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.69079225) / t23 + f64x8::splat(0.07036135105016941) * t36 * t26 * t27 * t32 + f64x8::splat(0.0635250071315033) * t33 - f64x8::splat(0.012312144854458484) * t43 * t45 * t53));
            let tzk0 = v_rho * t57;
            acc_zk = tzk0;
            let t59 = v_rho * v_rho;
            let t60 = t23 * t23;
            let t61 = f64x8::splat(1.0) / t60;
            let t67 = f64x8::splat(1.0) / t42 / v_rho;
            let t68 = t67 * t45;
            let t72 = t50 * t52 / t35;
            let t76 = f64x8::splat(1.0) / t2 / v_rho;
            let t87 = ((t1).select(f64x8::splat(0.0), f64x8::splat(2.5039685670704026) * t61 * t43 * param_fc * t20 - f64x8::splat(0.002156801128287631) * t68 * t48 * t72 - f64x8::splat(0.023453783683389805) * t36 * t76 * t27 * t32 - f64x8::splat(0.021175002377167768) * t76 * t27 * t32 + f64x8::splat(0.008208096569638989) * t68 * t53));
            let tvrho0 = t59 * t87 + f64x8::splat(2.0) * tzk0;
            acc_vrho = tvrho0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        ip += 8;
    }
}
