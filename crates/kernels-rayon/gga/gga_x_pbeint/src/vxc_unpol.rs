//! GGA_X_PBEINT vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbeint.c`
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
pub fn gga_x_pbeint_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_muPBE: f64,
    param_muGE: f64,
    param_alpha: f64,
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_muPBE = f64x8::splat(param_muPBE);
    let param_muGE = f64x8::splat(param_muGE);
    let param_alpha = f64x8::splat(param_alpha);
    let param_kappa = f64x8::splat(param_kappa);
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
            let t19 = t17 * t18;
            let t20 = param_muPBE - param_muGE;
            let t21 = t20 * param_alpha;
            let t22 = f64x8::splat(M_CBRT6);
            let t23 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t24 = (simd::cbrt(t23));
            let t25 = t24 * t24;
            let t26 = f64x8::splat(1.0) / t25;
            let t27 = t22 * t26;
            let t28 = t21 * t27;
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t29 * t29;
            let t31 = v_sigma * t30;
            let t32 = v_rho * v_rho;
            let t33 = t18 * t18;
            let t35 = f64x8::splat(1.0) / t33 / t32;
            let t38 = t31 * t35;
            let t41 = f64x8::splat(1.0) + param_alpha * t22 * t26 * t38 / f64x8::splat(24.0);
            let t42 = f64x8::splat(1.0) / t41;
            let t43 = t35 * t42;
            let t48 = (param_muGE + t28 * t31 * t43 / f64x8::splat(24.0)) * t22;
            let t49 = t48 * t26;
            let t52 = param_kappa + t49 * t38 / f64x8::splat(24.0);
            let t57 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t52);
            let t61 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t57));
            let tzk0 = f64x8::splat(2.0) * t61;
            acc_zk = tzk0;
            let t62 = f64x8::splat(1.0) / t33;
            let t63 = t17 * t62;
            let t67 = t6 * t17;
            let t68 = param_kappa * param_kappa;
            let t69 = t18 * t68;
            let t70 = t52 * t52;
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t32 * v_rho;
            let t74 = f64x8::splat(1.0) / t33 / t72;
            let t75 = t74 * t42;
            let t79 = param_alpha * param_alpha;
            let t80 = t20 * t79;
            let t81 = t22 * t22;
            let t83 = f64x8::splat(1.0) / t24 / t23;
            let t84 = t81 * t83;
            let t85 = t80 * t84;
            let t86 = v_sigma * v_sigma;
            let t87 = t86 * t29;
            let t88 = t32 * t32;
            let t89 = t88 * t32;
            let t91 = f64x8::splat(1.0) / t18 / t89;
            let t92 = t41 * t41;
            let t93 = f64x8::splat(1.0) / t92;
            let t94 = t91 * t93;
            let t99 = (-t28 * t31 * t75 / f64x8::splat(9.0) + t85 * t87 * t94 / f64x8::splat(108.0)) * t22;
            let t100 = t99 * t26;
            let t103 = t31 * t74;
            let t106 = t100 * t38 / f64x8::splat(24.0) - t49 * t103 / f64x8::splat(9.0);
            let t107 = t71 * t106;
            let t112 = ((t2).select(f64x8::splat(0.0), -t6 * t63 * t57 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t107));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t112 + f64x8::splat(2.0) * t61;
            acc_vrho = tvrho0;
            let t115 = t21 * t22;
            let t116 = t26 * t30;
            let t121 = t88 * v_rho;
            let t124 = f64x8::splat(1.0) / t18 / t121 * t93;
            let t129 = (t115 * t116 * t43 / f64x8::splat(24.0) - t85 * v_sigma * t29 * t124 / f64x8::splat(288.0)) * t22;
            let t130 = t129 * t26;
            let t132 = t116 * t35;
            let t135 = t130 * t38 / f64x8::splat(24.0) + t48 * t132 / f64x8::splat(24.0);
            let t136 = t71 * t135;
            let t140 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t136));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t140;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
