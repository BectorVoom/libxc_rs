//! GGA_X_RPBE kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_rpbe.c`
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
pub fn gga_x_rpbe_kxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    param_rpbe_mu: f64,
    param_rpbe_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_rpbe_mu = f64x8::splat(param_rpbe_mu);
    let param_rpbe_kappa = f64x8::splat(param_rpbe_kappa);
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
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
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
            let t21 = param_rpbe_mu * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t18 * t18;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = f64x8::splat(1.0) / param_rpbe_kappa;
            let t39 = (simd::exp(-t21 * t25 * t29 * t33 * t34 / f64x8::splat(24.0)));
            let t42 = f64x8::splat(1.0) + param_rpbe_kappa * (f64x8::splat(1.0) - t39);
            let t46 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t17 * t18 * t42));
            let tzk0 = f64x8::splat(2.0) * t46;
            acc_zk = tzk0;
            let t52 = t30 * v_rho;
            let t55 = t17 / t18 / t52;
            let t59 = t29 * t39;
            let t60 = t20 * t25 * t59;
            let t64 = ((t2).select(f64x8::splat(0.0), -t6 * t17 / t31 * t42 / f64x8::splat(8.0) + t6 * t55 * param_rpbe_mu * t60 / f64x8::splat(24.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t64 + f64x8::splat(2.0) * t46;
            acc_vrho = tvrho0;
            let t72 = t25 * t28 * t39;
            let t73 = t21 * t72;
            let t76 = ((t2).select(f64x8::splat(0.0), -t6 * t17 / t18 / t30 * t73 / f64x8::splat(64.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t76;
            acc_vsigma = tvsigma0;
            let t85 = t30 * t30;
            let t88 = t17 / t18 / t85;
            let t93 = t85 * t52;
            let t96 = param_rpbe_mu * param_rpbe_mu;
            let t98 = t6 * t17 / t93 * t96;
            let t99 = t20 * t20;
            let t102 = t99 / t23 / t22;
            let t103 = v_sigma * v_sigma;
            let t106 = t27 * t34 * t39;
            let t107 = t102 * t103 * t106;
            let t111 = ((t2).select(f64x8::splat(0.0), t6 * t17 / t31 / v_rho * t42 / f64x8::splat(12.0) - t6 * t88 * param_rpbe_mu * t60 / f64x8::splat(8.0) + t98 * t107 / f64x8::splat(108.0)));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t111 + f64x8::splat(4.0) * t64;
            acc_v2rho2 = tv2rho20;
            let t117 = t85 * t30;
            let t121 = t6 * t17 / t117 * t96;
            let t125 = t102 * t27 * v_sigma * t34 * t39;
            let t129 = ((t2).select(f64x8::splat(0.0), f64x8::splat(7.0) / f64x8::splat(192.0) * t6 * t55 * t73 - t121 * t125 / f64x8::splat(288.0)));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t129 + f64x8::splat(2.0) * t76;
            acc_v2rhosigma = tv2rhosigma0;
            let t132 = t85 * v_rho;
            let t137 = t102 * t106;
            let t140 = ((t2).select(f64x8::splat(0.0), t6 * t17 / t132 * t96 * t137 / f64x8::splat(768.0)));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t140;
            acc_v2sigma2 = tv2sigma20;
            let t149 = t17 / t18 / t132;
            let t154 = t85 * t85;
            let t158 = t6 * t17 / t154 * t96;
            let t161 = t22 * t22;
            let t164 = t3 / t4 / t161;
            let t165 = t154 * t30;
            let t169 = t164 * t17 / t31 / t165;
            let t170 = t96 * param_rpbe_mu;
            let t171 = t103 * v_sigma;
            let t173 = param_rpbe_kappa * param_rpbe_kappa;
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t174 * t39;
            let t176 = t170 * t171 * t175;
            let t180 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t17 * t33 * t42 + f64x8::splat(115.0) / f64x8::splat(216.0) * t6 * t149 * param_rpbe_mu * t60 - f64x8::splat(5.0) / f64x8::splat(54.0) * t158 * t107 + t169 * t176 / f64x8::splat(81.0)));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t180 + f64x8::splat(6.0) * t111;
            acc_v3rho3 = tv3rho30;
            let t189 = t154 * v_rho;
            let t193 = t164 * t17 / t31 / t189;
            let t195 = t170 * t103 * t175;
            let t199 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(35.0) / f64x8::splat(288.0) * t6 * t88 * t73 + f64x8::splat(25.0) / f64x8::splat(864.0) * t98 * t125 - t193 * t195 / f64x8::splat(216.0)));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t199 + f64x8::splat(4.0) * t129;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t205 = f64x8::splat(1.0) / t31 / t154;
            let t210 = t170 * t174 * v_sigma * t39;
            let t214 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(768.0) * t121 * t137 + t164 * t17 * t205 * t210 / f64x8::splat(576.0)));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t214 + f64x8::splat(2.0) * t140;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t217 = t164 * t17;
            let t224 = ((t2).select(f64x8::splat(0.0), -t217 / t31 / t93 * t170 * t175 / f64x8::splat(1536.0)));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t224;
            acc_v3sigma3 = tv3sigma30;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v3rho2sigma, ip, m, acc_v3rho2sigma);
        store_add(v3rhosigma2, ip, m, acc_v3rhosigma2);
        store_add(v3sigma3, ip, m, acc_v3sigma3);
        ip += 8;
    }
}
