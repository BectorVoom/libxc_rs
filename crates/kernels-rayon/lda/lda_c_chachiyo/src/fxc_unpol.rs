//! LDA_C_CHACHIYO fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_chachiyo.c`
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
pub fn lda_c_chachiyo_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
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
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
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
            let t38 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t36 * zeta_threshold, f64x8::splat(1.0)));
            let t40 = f64x8::splat(2.0) * t38 - f64x8::splat(2.0);
            let t42 = f64x8::splat(M_CBRT2);
            let t45 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t42 - f64x8::splat(2.0));
            let t46 = (param_af * t32 - t24) * t40 * t45;
            let tzk0 = t24 + t46;
            acc_zk = tzk0;
            let t48 = t8 / t18;
            let t52 = t17 / t9;
            let t55 = t3 * t48 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t13 * t52;
            let t57 = f64x8::splat(1.0) / t22;
            let t58 = param_ap * t55 * t57;
            let t63 = t25 * t48 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t28 * t52;
            let t65 = f64x8::splat(1.0) / t31;
            let t69 = (param_af * t63 * t65 - t58) * t40 * t45;
            let tvrho0 = t24 + t46 + v_rho * (t58 + t69);
            acc_vrho = tvrho0;
            let t76 = t8 / t18 / v_rho;
            let t80 = t17 / t9 / v_rho;
            let t83 = -f64x8::splat(2.0) / f64x8::splat(27.0) * t13 * t80 - f64x8::splat(2.0) / f64x8::splat(27.0) * t3 * t76;
            let t84 = param_ap * t83;
            let t85 = t84 * t57;
            let t86 = t55 * t55;
            let t88 = t22 * t22;
            let t89 = f64x8::splat(1.0) / t88;
            let t90 = param_ap * t86 * t89;
            let t94 = -f64x8::splat(2.0) / f64x8::splat(27.0) * t25 * t76 - f64x8::splat(2.0) / f64x8::splat(27.0) * t28 * t80;
            let t95 = param_af * t94;
            let t97 = t63 * t63;
            let t99 = t31 * t31;
            let t100 = f64x8::splat(1.0) / t99;
            let t104 = (-param_af * t97 * t100 + t95 * t65 - t85 + t90) * t40 * t45;
            let tv2rho20 = f64x8::splat(2.0) * t58 + f64x8::splat(2.0) * t69 + v_rho * (t85 - t90 + t104);
            acc_v2rho2 = tv2rho20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(v2rho2, ip, m, acc_v2rho2);
        ip += 8;
    }
}
