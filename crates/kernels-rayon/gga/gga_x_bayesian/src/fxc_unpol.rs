//! GGA_X_BAYESIAN fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_bayesian.c`
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
pub fn gga_x_bayesian_fxc_unpol(
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
            let t33 = t28 * t32;
            let t34 = t20 * t20;
            let t35 = f64x8::splat(1.0) / t22;
            let t36 = t34 * t35;
            let t37 = ((v_sigma).sqrt());
            let t44 = f64x8::splat(1.0) + t36 * t37 * t27 / t18 / v_rho / f64x8::splat(12.0);
            let t45 = t44 * t44;
            let t46 = f64x8::splat(1.0) / t45;
            let t47 = t33 * t46;
            let t50 = f64x8::splat(0.1926) + f64x8::splat(0.07900833333333333) * t26 * t47;
            let t51 = t46 * t50;
            let t55 = f64x8::splat(1.0008) + t26 * t33 * t51 / f64x8::splat(24.0);
            let t59 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t55));
            let tzk0 = f64x8::splat(2.0) * t59;
            acc_zk = tzk0;
            let t61 = t17 / t30;
            let t65 = t29 * v_rho;
            let t67 = f64x8::splat(1.0) / t30 / t65;
            let t68 = t28 * t67;
            let t72 = f64x8::splat(1.0) / t21;
            let t73 = t37 * v_sigma;
            let t74 = t72 * t73;
            let t75 = t29 * t29;
            let t76 = t75 * v_rho;
            let t77 = f64x8::splat(1.0) / t76;
            let t79 = f64x8::splat(1.0) / t45 / t44;
            let t80 = t77 * t79;
            let t84 = t68 * t46;
            let t89 = -f64x8::splat(0.2106888888888889) * t26 * t84 + f64x8::splat(0.2106888888888889) * t74 * t80;
            let t90 = t46 * t89;
            let t94 = -t26 * t68 * t51 / f64x8::splat(9.0) + t74 * t80 * t50 / f64x8::splat(9.0) + t26 * t33 * t90 / f64x8::splat(24.0);
            let t99 = ((t2).select(f64x8::splat(0.0), -t6 * t61 * t55 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t94));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t99 + f64x8::splat(2.0) * t59;
            acc_vrho = tvrho0;
            let t102 = t25 * t28;
            let t103 = t32 * t46;
            let t106 = t72 * t37;
            let t107 = f64x8::splat(1.0) / t75;
            let t108 = t107 * t79;
            let t115 = f64x8::splat(0.07900833333333333) * t25 * t47 - f64x8::splat(0.07900833333333333) * t106 * t108;
            let t116 = t46 * t115;
            let t120 = t102 * t103 * t50 / f64x8::splat(24.0) - t106 * t108 * t50 / f64x8::splat(24.0) + t26 * t33 * t116 / f64x8::splat(24.0);
            let t124 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t120));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t124;
            acc_vsigma = tvsigma0;
            let t129 = t17 / t30 / v_rho;
            let t137 = f64x8::splat(1.0) / t30 / t75;
            let t138 = t28 * t137;
            let t142 = t75 * t29;
            let t143 = f64x8::splat(1.0) / t142;
            let t144 = t143 * t79;
            let t151 = v_sigma * v_sigma;
            let t152 = t72 * t151;
            let t153 = t75 * t65;
            let t155 = f64x8::splat(1.0) / t18 / t153;
            let t156 = t45 * t45;
            let t157 = f64x8::splat(1.0) / t156;
            let t159 = t152 * t155 * t157;
            let t161 = t35 * t27;
            let t162 = t50 * t34 * t161;
            let t168 = t138 * t46;
            let t175 = t157 * t34 * t161;
            let t178 = f64x8::splat(0.772525925925926) * t26 * t168 - f64x8::splat(1.6152814814814815) * t74 * t144 + f64x8::splat(0.07022962962962963) * t152 * t155 * t175;
            let t179 = t46 * t178;
            let t183 = f64x8::splat(11.0) / f64x8::splat(27.0) * t26 * t138 * t51 - f64x8::splat(23.0) / f64x8::splat(27.0) * t74 * t144 * t50 - f64x8::splat(2.0) / f64x8::splat(9.0) * t26 * t68 * t90 + t159 * t162 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t74 * t80 * t89 + t26 * t33 * t179 / f64x8::splat(24.0);
            let t188 = ((t2).select(f64x8::splat(0.0), t6 * t129 * t55 / f64x8::splat(12.0) - t6 * t61 * t94 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t183));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t188 + f64x8::splat(4.0) * t99;
            acc_v2rho2 = tv2rho20;
            let t194 = t67 * t46;
            let t198 = t72 * t77;
            let t199 = t79 * t50;
            let t200 = t199 * t37;
            let t206 = t72 * v_sigma;
            let t208 = f64x8::splat(1.0) / t18 / t142;
            let t210 = t206 * t208 * t157;
            let t224 = t79 * t37;
            let t230 = -f64x8::splat(0.2106888888888889) * t25 * t84 + f64x8::splat(0.5267222222222222) * t198 * t224 - f64x8::splat(0.026336111111111112) * t206 * t208 * t175;
            let t231 = t46 * t230;
            let t235 = -t102 * t194 * t50 / f64x8::splat(9.0) + f64x8::splat(5.0) / f64x8::splat(18.0) * t198 * t200 + t102 * t103 * t89 / f64x8::splat(24.0) - t210 * t162 / f64x8::splat(72.0) - t106 * t108 * t89 / f64x8::splat(24.0) - t26 * t68 * t116 / f64x8::splat(9.0) + t74 * t80 * t115 / f64x8::splat(9.0) + t26 * t33 * t231 / f64x8::splat(24.0);
            let t240 = ((t2).select(f64x8::splat(0.0), -t6 * t61 * t120 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t235));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t240 + f64x8::splat(2.0) * t124;
            acc_v2rhosigma = tv2rhosigma0;
            let t243 = t72 * t107;
            let t244 = f64x8::splat(1.0) / t37;
            let t245 = t199 * t244;
            let t253 = t72 / t18 / t76;
            let t254 = t253 * t157;
            let t260 = t79 * t244;
            let t263 = t36 * t27;
            let t266 = -f64x8::splat(0.1185125) * t243 * t260 + f64x8::splat(0.009876041666666667) * t254 * t263;
            let t267 = t46 * t266;
            let t271 = -t243 * t245 / f64x8::splat(16.0) + t102 * t103 * t115 / f64x8::splat(12.0) + t254 * t162 / f64x8::splat(192.0) - t106 * t108 * t115 / f64x8::splat(12.0) + t26 * t33 * t267 / f64x8::splat(24.0);
            let t275 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t271));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t275;
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
