//! GGA_X_ITYH_PBE exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh_pbe.c`
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
pub fn gga_x_ityh_pbe_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_mu: f64,
    param_kappa: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_mu = f64x8::splat(param_mu);
    let param_kappa = f64x8::splat(param_kappa);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = t6 * t17;
            let t19 = (simd::cbrt(v_rho));
            let t20 = t3 * t3;
            let t21 = f64x8::splat(M_PI) * t20;
            let t22 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t24 * t25;
            let t27 = f64x8::splat(M_CBRT6);
            let t28 = param_mu * t27;
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t32 = f64x8::splat(1.0) / t31;
            let t34 = f64x8::splat(M_CBRT2);
            let t35 = t34 * t34;
            let t36 = v_sigma * t35;
            let t37 = v_rho * v_rho;
            let t38 = t19 * t19;
            let t40 = f64x8::splat(1.0) / t38 / t37;
            let t44 = param_kappa + t28 * t32 * t36 * t40 / f64x8::splat(24.0);
            let t49 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t44);
            let t52 = t21 * t26 / t49;
            let t53 = ((t52).sqrt());
            let t55 = param_hyb_omega_0 / t53;
            let t56 = t11 * v_rho;
            let t57 = (simd::cbrt(t56));
            let t58 = f64x8::splat(1.0) / t57;
            let t61 = t55 * t34 * t58 / f64x8::splat(2.0);
            let t62 = (f64x8::splat(1.35)).simd_le(t61);
            let t63 = (f64x8::splat(1.35)).simd_lt(t61);
            let t64 = ((t63).select(t61, f64x8::splat(1.35)));
            let t65 = t64 * t64;
            let t68 = t65 * t65;
            let t69 = f64x8::splat(1.0) / t68;
            let t71 = t68 * t65;
            let t72 = f64x8::splat(1.0) / t71;
            let t74 = t68 * t68;
            let t75 = f64x8::splat(1.0) / t74;
            let t78 = f64x8::splat(1.0) / t74 / t65;
            let t81 = f64x8::splat(1.0) / t74 / t68;
            let t84 = f64x8::splat(1.0) / t74 / t71;
            let t86 = t74 * t74;
            let t87 = f64x8::splat(1.0) / t86;
            let t90 = ((t63).select(f64x8::splat(1.35), t61));
            let t91 = ((f64x8::splat(M_PI)).sqrt());
            let t92 = f64x8::splat(1.0) / t90;
            let t94 = (simd::erf(t92 / f64x8::splat(2.0)));
            let t96 = t90 * t90;
            let t97 = f64x8::splat(1.0) / t96;
            let t99 = (simd::exp(-t97 / f64x8::splat(4.0)));
            let t100 = t99 - f64x8::splat(1.0);
            let t103 = t99 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t96 * t100;
            let t106 = f64x8::splat(2.0) * t90 * t103 + t91 * t94;
            let t110 = ((t62).select(f64x8::splat(1.0) / t65 / f64x8::splat(36.0) - t69 / f64x8::splat(960.0) + t72 / f64x8::splat(26880.0) - t75 / f64x8::splat(829440.0) + t78 / f64x8::splat(28385280.0) - t81 / f64x8::splat(1073479680.0) + t84 / f64x8::splat(44590694400.0) - t87 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t106));
            let t115 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t110 * t49));
            let tzk0 = f64x8::splat(2.0) * t115;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
