//! GGA_X_BEEFVDW exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_beefvdw.c`
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
pub fn gga_x_beefvdw_exc_unpol(
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
            let t11 = t10 + f64x8::splat(1.0);
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t20 * t24;
            let t26 = t25 * v_sigma;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_rho * v_rho;
            let t30 = t18 * t18;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t38 = f64x8::splat(4.0) + t25 * v_sigma * t28 * t32 / f64x8::splat(24.0);
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t28 * t32 * t39;
            let t41 = t26 * t40;
            let t43 = t41 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t44 = t43 * t43;
            let t45 = t44 * t44;
            let t46 = t45 * t45;
            let t47 = t46 * t45;
            let t48 = t46 * t46;
            let t49 = t48 * t47;
            let t51 = t45 * t43;
            let t52 = t46 * t51;
            let t55 = t44 * t43;
            let t56 = t46 * t55;
            let t57 = t48 * t56;
            let t59 = t46 * t44;
            let t60 = t48 * t59;
            let t62 = t46 * t43;
            let t63 = t48 * t62;
            let t65 = t45 * t55;
            let t66 = t48 * t65;
            let t68 = t48 * t46;
            let t70 = t45 * t44;
            let t71 = t48 * t70;
            let t78 = t48 * t44;
            let t81 = -f64x8::splat(5427.777462637186) * t49 + f64x8::splat(4135.586188014654) * t48 * t52 - f64x8::splat(29150.193011493262) * t57 + f64x8::splat(40074.93585443239) * t60 + f64x8::splat(90365.6111085228) * t63 - f64x8::splat(161142.1539984628) * t66 - f64x8::splat(132044.6618218215) * t68 + f64x8::splat(255894.79526235335) * t71 - f64x8::splat(0.6945973517763898) * t45 + f64x8::splat(0.527556201155898) * t55 - f64x8::splat(0.38916037779196816) * t44 + f64x8::splat(86.00573049927964) * t65 + f64x8::splat(30.54203495931585) * t70 + f64x8::splat(279670.48856303055) * t78 + f64x8::splat(0.037534251004296526) * t41;
            let t88 = t46 * t70;
            let t91 = t48 * t45;
            let t93 = t48 * t51;
            let t95 = t48 * t55;
            let t97 = t48 * t43;
            let t99 = t46 * t65;
            let t102 = f64x8::splat(1.1313514630621233) - f64x8::splat(7.2975787893717134) * t51 + f64x8::splat(3783.53964072524) * t59 - f64x8::splat(617.547861045286) * t62 - f64x8::splat(442.33229018433804) * t46 - f64x8::splat(20148.24517562505) * t47 + f64x8::splat(2274.8997850816486) * t56 + f64x8::splat(70504.54186903402) * t88 - f64x8::splat(2810.240180568463) * t52 - f64x8::splat(323524.0313604933) * t91 + f64x8::splat(180782.00670879145) * t93 - f64x8::splat(129814.81812794984) * t95 + f64x8::splat(56174.00797937267) * t97 - f64x8::splat(10276.426607863825) * t99 - f64x8::splat(168370.8413901412) * t48;
            let t103 = t81 + t102;
            let t107 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t103));
            let tzk0 = f64x8::splat(2.0) * t107;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
