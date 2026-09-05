//! GGA_X_WC fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_wc.c`
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
pub fn gga_x_wc_fxc_unpol(
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
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t20 * t24;
            let t26 = f64x8::splat(M_CBRT2);
            let t27 = t26 * t26;
            let t28 = v_sigma * t27;
            let t29 = v_rho * v_rho;
            let t30 = t18 * t18;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t34 = t25 * t28 * t32;
            let t36 = t25 * v_sigma;
            let t37 = t27 * t32;
            let t39 = (simd::exp(-t34 / f64x8::splat(24.0)));
            let t40 = t37 * t39;
            let t43 = t20 * t20;
            let t46 = t43 / t22 / t21;
            let t47 = v_sigma * v_sigma;
            let t49 = t29 * t29;
            let t50 = t49 * v_rho;
            let t52 = f64x8::splat(1.0) / t18 / t50;
            let t56 = f64x8::splat(1.0) + f64x8::splat(2.7560657413756314e-05) * t46 * t47 * t26 * t52;
            let t57 = (simd::ln(t56));
            let t58 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t34 + f64x8::splat(0.004002424276710846) * t36 * t40 + t57;
            let t61 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t58;
            let t65 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t17 * t18 * t61));
            let tzk0 = f64x8::splat(2.0) * t65;
            acc_zk = tzk0;
            let t66 = f64x8::splat(1.0) / t30;
            let t71 = t3 * t17;
            let t72 = t58 * t58;
            let t73 = f64x8::splat(1.0) / t72;
            let t74 = t18 * t73;
            let t75 = t29 * v_rho;
            let t77 = f64x8::splat(1.0) / t30 / t75;
            let t81 = t27 * t77;
            let t82 = t81 * t39;
            let t85 = t46 * t47;
            let t86 = t49 * t29;
            let t88 = f64x8::splat(1.0) / t18 / t86;
            let t89 = t26 * t88;
            let t90 = t89 * t39;
            let t93 = f64x8::splat(1.0) / t56;
            let t94 = t89 * t93;
            let t97 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t25 * t28 * t77 - f64x8::splat(0.010673131404562256) * t36 * t82 + f64x8::splat(0.0008894276170468547) * t85 * t90 - f64x8::splat(0.00014699017287336702) * t85 * t94;
            let t102 = ((t2).select(f64x8::splat(0.0), -t6 * t17 * t66 * t61 / f64x8::splat(8.0) - f64x8::splat(0.1655109536374632) * t71 * t74 * t97));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t102 + f64x8::splat(2.0) * t65;
            acc_vrho = tvrho0;
            let t109 = t46 * v_sigma;
            let t110 = t26 * t52;
            let t111 = t110 * t39;
            let t114 = t110 * t93;
            let t117 = f64x8::splat(5.0) / f64x8::splat(972.0) * t25 * t37 + f64x8::splat(0.004002424276710846) * t25 * t40 - f64x8::splat(0.0003335353563925705) * t109 * t111 + f64x8::splat(5.512131482751263e-05) * t109 * t114;
            let t121 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t71 * t74 * t117));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t121;
            acc_vsigma = tvsigma0;
            let t125 = f64x8::splat(1.0) / t30 / v_rho;
            let t130 = t66 * t73;
            let t135 = f64x8::splat(1.0) / t72 / t58;
            let t136 = t18 * t135;
            let t137 = t97 * t97;
            let t142 = f64x8::splat(1.0) / t30 / t49;
            let t146 = t27 * t142;
            let t147 = t146 * t39;
            let t150 = t49 * t75;
            let t152 = f64x8::splat(1.0) / t18 / t150;
            let t153 = t26 * t152;
            let t154 = t153 * t39;
            let t157 = t21 * t21;
            let t158 = f64x8::splat(1.0) / t157;
            let t159 = t47 * v_sigma;
            let t160 = t158 * t159;
            let t161 = t49 * t49;
            let t162 = t161 * t29;
            let t163 = f64x8::splat(1.0) / t162;
            let t167 = t153 * t93;
            let t172 = t20 / t23 / t157;
            let t173 = t47 * t47;
            let t174 = t172 * t173;
            let t175 = t161 * t49;
            let t177 = f64x8::splat(1.0) / t30 / t175;
            let t179 = t56 * t56;
            let t180 = f64x8::splat(1.0) / t179;
            let t181 = t27 * t177 * t180;
            let t184 = f64x8::splat(110.0) / f64x8::splat(2187.0) * t25 * t28 * t142 + f64x8::splat(0.039134815150061605) * t36 * t147 - f64x8::splat(0.008004848553421692) * t85 * t154 + f64x8::splat(0.0011859034893958063) * t160 * t163 * t39 + f64x8::splat(0.0009309377615313244) * t85 * t167 - f64x8::splat(1.2963666552805393e-07) * t174 * t181;
            let t189 = ((t2).select(f64x8::splat(0.0), t6 * t17 * t125 * t61 / f64x8::splat(12.0) - f64x8::splat(0.1103406357583088) * t71 * t130 * t97 + f64x8::splat(0.3310219072749264) * t71 * t136 * t137 - f64x8::splat(0.1655109536374632) * t71 * t74 * t184));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t189 + f64x8::splat(4.0) * t102;
            acc_v2rho2 = tv2rho20;
            let t195 = t71 * t18;
            let t196 = t135 * t117;
            let t197 = t196 * t97;
            let t204 = t46 * t26;
            let t209 = t158 * t47;
            let t210 = t161 * v_rho;
            let t211 = f64x8::splat(1.0) / t210;
            let t217 = t172 * t159;
            let t218 = t161 * t75;
            let t220 = f64x8::splat(1.0) / t30 / t218;
            let t225 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t25 * t81 - f64x8::splat(0.010673131404562256) * t25 * t82 + f64x8::splat(0.002668282851140564) * t204 * t88 * v_sigma * t39 - f64x8::splat(0.00044471380852342736) * t209 * t211 * t39 - f64x8::splat(0.00029398034574673403) * t109 * t94 + f64x8::splat(4.861374957302022e-08) * t217 * t27 * t220 * t180;
            let t230 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.0551703178791544) * t71 * t130 * t117 + f64x8::splat(0.3310219072749264) * t195 * t197 - f64x8::splat(0.1655109536374632) * t71 * t74 * t225));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t230 + f64x8::splat(2.0) * t121;
            acc_v2rhosigma = tv2rhosigma0;
            let t233 = t117 * t117;
            let t239 = t158 * v_sigma;
            let t240 = f64x8::splat(1.0) / t161;
            let t248 = f64x8::splat(1.0) / t30 / t162;
            let t250 = t27 * t248 * t180;
            let t253 = -f64x8::splat(0.000667070712785141) * t46 * t111 + f64x8::splat(0.00016676767819628525) * t239 * t240 * t39 + f64x8::splat(5.512131482751263e-05) * t46 * t114 - f64x8::splat(1.8230156089882582e-08) * t172 * t47 * t250;
            let t258 = ((t2).select(f64x8::splat(0.0), f64x8::splat(0.3310219072749264) * t71 * t136 * t233 - f64x8::splat(0.1655109536374632) * t71 * t74 * t253));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t258;
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
