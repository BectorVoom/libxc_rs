//! GGA_X_PBE_ERF_GWS exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbe_erf_gws.c`
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
pub fn gga_x_pbe_erf_gws_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_ax: f64,
    param_b_PBE: f64,
    param_kappa: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_ax = f64x8::splat(param_ax);
    let param_b_PBE = f64x8::splat(param_b_PBE);
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
            let t3 = param_hyb_omega_0 * param_hyb_omega_0;
            let t4 = param_ax * t3;
            let t5 = f64x8::splat(M_CBRT3);
            let t7 = f64x8::splat(M_CBRTPI);
            let t8 = t7 * f64x8::splat(M_PI);
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t11 = (simd::cbrt(zeta_threshold));
            let t12 = f64x8::splat(M_CBRT2);
            let t13 = ((t10).select(t11, t12));
            let t14 = t13 * t13;
            let t15 = f64x8::splat(1.0) / t14;
            let t16 = t9 * t15;
            let t17 = t12 * t12;
            let t18 = (simd::cbrt(v_rho));
            let t19 = t18 * t18;
            let t20 = f64x8::splat(1.0) / t19;
            let t25 = (simd::exp(-t4 * t5 * t16 * t17 * t20 / f64x8::splat(12.0)));
            let t26 = param_b_PBE * t25;
            let t27 = t26 * v_sigma;
            let t28 = param_kappa + f64x8::splat(1.0);
            let t29 = t5 * t28;
            let t30 = t5 * t5;
            let t31 = t12 * t30;
            let t32 = t7 * t7;
            let t34 = t31 / t32;
            let t35 = f64x8::splat(1.0) / t18;
            let t37 = f64x8::splat(1.0) / t13;
            let t40 = t34 * param_hyb_omega_0 * t35 * t37 / f64x8::splat(6.0);
            let t41 = (t40).simd_lt(f64x8::splat(0.05));
            let t42 = t14 * t14;
            let t43 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t44 = t32 * t43;
            let t45 = t42 * t44;
            let t46 = t18 * v_rho;
            let t47 = t45 * t46;
            let t49 = t14 * t8;
            let t50 = t49 * t17;
            let t52 = t5 * t19 * t3;
            let t53 = t50 * t52;
            let t55 = f64x8::splat(7.0) * t47 - f64x8::splat(6.0) * t53;
            let t56 = t14 * t13;
            let t57 = f64x8::splat(1.0) / param_hyb_omega_0;
            let t64 = (simd::erf(t57 * t5 * t32 * t13 * t17 * t18 / f64x8::splat(2.0)));
            let t66 = ((f64x8::splat(M_PI)).sqrt());
            let t67 = t66 * t43;
            let t68 = t56 * t64 * t67;
            let t69 = v_rho * param_hyb_omega_0;
            let t75 = t3 * t3;
            let t76 = t75 * t30;
            let t78 = f64x8::splat(12.0) * t76 * t12;
            let t79 = -f64x8::splat(36.0) * t68 * t31 * t69 + f64x8::splat(81.0) * t47 + f64x8::splat(54.0) * t53 - t78;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = (f64x8::splat(10000000000.0)).simd_lt(t40);
            let t83 = t43 * t43;
            let t84 = v_rho * v_rho;
            let t86 = t42 * t14;
            let t90 = t44 * t17 * t5;
            let t96 = t8 * t12 * t30;
            let t102 = t75 * t3;
            let t103 = f64x8::splat(1.0) / t102;
            let t107 = f64x8::splat(1.0) / t3;
            let t108 = t107 * t30;
            let t113 = t108 * t8 * t14 * t12 * t19 / f64x8::splat(2.0);
            let t114 = (simd::exp(t113));
            let t115 = t114 * t8;
            let t118 = t5 * t3;
            let t119 = t14 * t17 * t118;
            let t123 = t114 * t12;
            let t127 = (f64x8::splat(7.0) * t115 * t19 * t119 - f64x8::splat(12.0) * t123 * t76 + f64x8::splat(6.0) * t47 + f64x8::splat(11.0) * t53 + t78) * t8;
            let t128 = t19 * t14;
            let t129 = t127 * t128;
            let t130 = t17 * t30;
            let t131 = t42 * t114;
            let t132 = t44 * t12;
            let t136 = t56 * t114;
            let t143 = t14 * t114 * t8;
            let t148 = t114 * t17;
            let t153 = f64x8::splat(12.0) * t136 * t64 * t67 * t130 * t69 - f64x8::splat(27.0) * t131 * t132 * t46 - f64x8::splat(4.0) * t130 * t75 - f64x8::splat(36.0) * t143 * t52 + f64x8::splat(4.0) * t148 * t76 + f64x8::splat(24.0) * t49 * t52;
            let t156 = t130 * t107 / t153;
            let t159 = ((t41).select(t55 * t80, (t82).select((f64x8::splat(5600.0) * t96 * t19 * t75 * t14 - f64x8::splat(140.0) * t90 * t46 * t3 * t42 - f64x8::splat(1863.0) * t83 * t84 * t86) * t103 / f64x8::splat(201600.0), -t129 * t156 / f64x8::splat(18.0))));
            let t163 = t19 * t84;
            let t165 = param_kappa * t163 * t8;
            let t166 = f64x8::splat(27.0) / f64x8::splat(28.0) * t27 * t29 * t159 + t165;
            let t167 = t166 * t46;
            let t170 = ((t10).select(t11 * zeta_threshold, f64x8::splat(2.0) * t12));
            let t171 = t170 * t17;
            let t172 = t167 * t171;
            let t173 = (f64x8::splat(1.35)).simd_le(t40);
            let t174 = (f64x8::splat(1.35)).simd_lt(t40);
            let t175 = ((t174).select(t40, f64x8::splat(1.35)));
            let t176 = t175 * t175;
            let t177 = t176 * t176;
            let t178 = t177 * t176;
            let t179 = t177 * t177;
            let t182 = t179 * t177;
            let t184 = t179 * t176;
            let t190 = f64x8::splat(24088884019200.0) * t179 * t178 + f64x8::splat(19448.0) * t176 - f64x8::splat(807840.0) * t177 + f64x8::splat(30551040.0) * t178 - f64x8::splat(1045524480.0) * t179 - f64x8::splat(903333150720.0) * t182 + f64x8::splat(32261898240.0) * t184 - f64x8::splat(429.0);
            let t191 = t179 * t179;
            let t192 = f64x8::splat(1.0) / t191;
            let t195 = ((t174).select(f64x8::splat(1.35), t40));
            let t196 = t195 * t195;
            let t197 = t196 * t196;
            let t200 = f64x8::splat(32.0) * t197 - f64x8::splat(16.0) * t196;
            let t203 = (simd::exp(-f64x8::splat(1.0) / t196 / f64x8::splat(4.0)));
            let t207 = f64x8::splat(1.0) / t195;
            let t209 = (simd::erf(t207 / f64x8::splat(2.0)));
            let t210 = t66 * t209;
            let t215 = ((t173).select(t190 * t192 / f64x8::splat(867199824691200.0), t200 * t203 / f64x8::splat(3.0) - f64x8::splat(32.0) / f64x8::splat(3.0) * t197 - f64x8::splat(8.0) / f64x8::splat(3.0) * t210 * t195 + f64x8::splat(8.0) * t196 + f64x8::splat(1.0)));
            let t216 = f64x8::splat(1.0) / t7;
            let t217 = t215 * t216;
            let t218 = param_b_PBE * t159;
            let t220 = t25 * v_sigma * t5;
            let t224 = f64x8::splat(864.0) * t218 * t220 + f64x8::splat(896.0) * t165;
            let t225 = f64x8::splat(1.0) / t224;
            let t226 = t5 * t225;
            let t227 = t217 * t226;
            let t230 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(84.0) * t172 * t227));
            let t231 = f64x8::splat(1.0) / v_rho;
            let tzk0 = f64x8::splat(2.0) * t230 * t231;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
