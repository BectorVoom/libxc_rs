//! GGA_X_RGE2 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_rge2.c`
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
pub fn gga_x_rge2_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
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
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
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
            let t18 = (simd::cbrt(v_rho));
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t25 = t20 / t23;
            let t26 = f64x8::splat(M_CBRT2);
            let t27 = t26 * t26;
            let t28 = v_sigma * t27;
            let t29 = v_rho * v_rho;
            let t30 = t18 * t18;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t36 = t20 * t20;
            let t38 = f64x8::splat(1.0) / t22 / t21;
            let t39 = t36 * t38;
            let t40 = v_sigma * v_sigma;
            let t41 = t40 * t26;
            let t42 = t29 * t29;
            let t43 = t42 * v_rho;
            let t45 = f64x8::splat(1.0) / t18 / t43;
            let t49 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t25 * t28 * t32 + f64x8::splat(6.582356890714508e-05) * t39 * t41 * t45;
            let t52 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t49;
            let t56 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t17 * t18 * t52));
            let tzk0 = f64x8::splat(2.0) * t56;
            acc_zk = tzk0;
            let t57 = f64x8::splat(1.0) / t30;
            let t62 = t3 * t17;
            let t63 = t49 * t49;
            let t64 = f64x8::splat(1.0) / t63;
            let t65 = t18 * t64;
            let t66 = t29 * v_rho;
            let t68 = f64x8::splat(1.0) / t30 / t66;
            let t72 = t42 * t29;
            let t74 = f64x8::splat(1.0) / t18 / t72;
            let t78 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t25 * t28 * t68 - f64x8::splat(0.00035105903417144045) * t39 * t41 * t74;
            let t83 = ((t2).select(f64x8::splat(0.0), -t6 * t17 * t57 * t52 / f64x8::splat(8.0) - f64x8::splat(0.1655109536374632) * t62 * t65 * t78));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t83 + f64x8::splat(2.0) * t56;
            acc_vrho = tvrho0;
            let t89 = v_sigma * t26;
            let t93 = f64x8::splat(5.0) / f64x8::splat(972.0) * t25 * t27 * t32 + f64x8::splat(0.00013164713781429015) * t39 * t89 * t45;
            let t97 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t62 * t65 * t93));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t97;
            acc_vsigma = tvsigma0;
            let t101 = f64x8::splat(1.0) / t30 / v_rho;
            let t106 = t57 * t64;
            let t111 = f64x8::splat(1.0) / t63 / t49;
            let t112 = t18 * t111;
            let t113 = t78 * t78;
            let t118 = f64x8::splat(1.0) / t30 / t42;
            let t122 = t42 * t66;
            let t124 = f64x8::splat(1.0) / t18 / t122;
            let t128 = f64x8::splat(110.0) / f64x8::splat(2187.0) * t25 * t28 * t118 + f64x8::splat(0.0022233738830857892) * t39 * t41 * t124;
            let t133 = ((t2).select(f64x8::splat(0.0), t6 * t17 * t101 * t52 / f64x8::splat(12.0) - f64x8::splat(0.1103406357583088) * t62 * t106 * t78 + f64x8::splat(0.3310219072749264) * t62 * t112 * t113 - f64x8::splat(0.1655109536374632) * t62 * t65 * t128));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t133 + f64x8::splat(4.0) * t83;
            acc_v2rho2 = tv2rho20;
            let t139 = t62 * t18;
            let t140 = t111 * t93;
            let t141 = t140 * t78;
            let t150 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t25 * t27 * t68 - f64x8::splat(0.0007021180683428809) * t39 * t89 * t74;
            let t155 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.0551703178791544) * t62 * t106 * t93 + f64x8::splat(0.3310219072749264) * t139 * t141 - f64x8::splat(0.1655109536374632) * t62 * t65 * t150));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t155 + f64x8::splat(2.0) * t97;
            acc_v2rhosigma = tv2rhosigma0;
            let t158 = t93 * t93;
            let t162 = f64x8::splat(1.0) / t43;
            let t165 = t38 * t26;
            let t166 = t64 * t36 * t165;
            let t170 = ((t2).select(f64x8::splat(0.0), f64x8::splat(0.3310219072749264) * t62 * t112 * t158 - f64x8::splat(2.1789043323285708e-05) * t62 * t162 * t166));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t170;
            acc_v2sigma2 = tv2sigma20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        ip += 8;
    }
}
