//! MGGA_C_M05 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_m05.c`
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
pub fn mgga_c_m05_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_css_1: f64,
    param_gamma_ss: f64,
    param_css_2: f64,
    param_css_3: f64,
    param_css_4: f64,
    param_css_0: f64,
    param_Fermi_D_cnst: f64,
    param_cab_1: f64,
    param_gamma_ab: f64,
    param_cab_2: f64,
    param_cab_3: f64,
    param_cab_4: f64,
    param_cab_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_css_1 = f64x8::splat(param_css_1);
    let param_gamma_ss = f64x8::splat(param_gamma_ss);
    let param_css_2 = f64x8::splat(param_css_2);
    let param_css_3 = f64x8::splat(param_css_3);
    let param_css_4 = f64x8::splat(param_css_4);
    let param_css_0 = f64x8::splat(param_css_0);
    let param_Fermi_D_cnst = f64x8::splat(param_Fermi_D_cnst);
    let param_cab_1 = f64x8::splat(param_cab_1);
    let param_gamma_ab = f64x8::splat(param_gamma_ab);
    let param_cab_2 = f64x8::splat(param_cab_2);
    let param_cab_3 = f64x8::splat(param_cab_3);
    let param_cab_4 = f64x8::splat(param_cab_4);
    let param_cab_0 = f64x8::splat(param_cab_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t4 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t5 = ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold)) | (t4);
            let t6 = ((t4).select(zeta_threshold, f64x8::splat(1.0)));
            let t7 = f64x8::splat(M_CBRT3);
            let t8 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t9 = (simd::cbrt(t8));
            let t10 = t7 * t9;
            let t11 = f64x8::splat(M_CBRT4);
            let t12 = t11 * t11;
            let t13 = t10 * t12;
            let t14 = (simd::cbrt(v_rho));
            let t15 = f64x8::splat(1.0) / t14;
            let t16 = f64x8::splat(M_CBRT2);
            let t18 = (simd::cbrt(zeta_threshold));
            let t20 = ((t4).select(f64x8::splat(1.0) / t18, f64x8::splat(1.0)));
            let t22 = t13 * t15 * t16 * t20;
            let t24 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t22;
            let t25 = ((t22).sqrt());
            let t28 = ((t22) * (t22).sqrt());
            let t30 = t7 * t7;
            let t31 = t9 * t9;
            let t32 = t30 * t31;
            let t33 = t32 * t11;
            let t34 = t14 * t14;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = t16 * t16;
            let t38 = t20 * t20;
            let t40 = t33 * t35 * t36 * t38;
            let t42 = f64x8::splat(3.79785) * t25 + f64x8::splat(0.8969) * t22 + f64x8::splat(0.204775) * t28 + f64x8::splat(0.123235) * t40;
            let t45 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t42;
            let t46 = (simd::ln(t45));
            let t48 = f64x8::splat(0.0621814) * t24 * t46;
            let t50 = t18 * zeta_threshold;
            let t52 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t50, f64x8::splat(2.0) * t16));
            let t54 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t50, f64x8::splat(0.0)));
            let t58 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t16 - f64x8::splat(2.0));
            let t59 = (t52 + t54 - f64x8::splat(2.0)) * t58;
            let t61 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t22;
            let t66 = f64x8::splat(7.05945) * t25 + f64x8::splat(1.549425) * t22 + f64x8::splat(0.420775) * t28 + f64x8::splat(0.1562925) * t40;
            let t69 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t66;
            let t70 = (simd::ln(t69));
            let t74 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t22;
            let t79 = f64x8::splat(5.1785) * t25 + f64x8::splat(0.905775) * t22 + f64x8::splat(0.1100325) * t28 + f64x8::splat(0.1241775) * t40;
            let t82 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t79;
            let t83 = (simd::ln(t82));
            let t84 = t74 * t83;
            let t93 = ((t5).select(f64x8::splat(0.0), t6 * (-t48 + t59 * (-f64x8::splat(0.0310907) * t61 * t70 + t48 - f64x8::splat(0.0197516734986138) * t84) + f64x8::splat(0.0197516734986138) * t59 * t84) / f64x8::splat(2.0)));
            let t95 = param_css_1;
            let t96 = t95 * param_gamma_ss;
            let t97 = t96 * v_sigma;
            let t98 = v_rho * v_rho;
            let t100 = f64x8::splat(1.0) / t34 / t98;
            let t101 = t36 * t100;
            let t104 = param_gamma_ss * v_sigma * t101 + f64x8::splat(1.0);
            let t105 = f64x8::splat(1.0) / t104;
            let t106 = t101 * t105;
            let t108 = param_css_2;
            let t109 = param_gamma_ss * param_gamma_ss;
            let t110 = t108 * t109;
            let t111 = v_sigma * v_sigma;
            let t112 = t110 * t111;
            let t113 = t98 * t98;
            let t114 = t113 * v_rho;
            let t116 = f64x8::splat(1.0) / t14 / t114;
            let t117 = t16 * t116;
            let t118 = t104 * t104;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = t117 * t119;
            let t123 = param_css_3;
            let t124 = t109 * param_gamma_ss;
            let t125 = t123 * t124;
            let t126 = t111 * v_sigma;
            let t127 = t113 * t113;
            let t128 = f64x8::splat(1.0) / t127;
            let t129 = t126 * t128;
            let t130 = t118 * t104;
            let t131 = f64x8::splat(1.0) / t130;
            let t135 = param_css_4;
            let t136 = t109 * t109;
            let t137 = t135 * t136;
            let t138 = t111 * t111;
            let t139 = t137 * t138;
            let t140 = t127 * t98;
            let t142 = f64x8::splat(1.0) / t34 / t140;
            let t143 = t36 * t142;
            let t144 = t118 * t118;
            let t145 = f64x8::splat(1.0) / t144;
            let t146 = t143 * t145;
            let t149 = f64x8::splat(4.0) * t125 * t129 * t131 + t97 * t106 + f64x8::splat(2.0) * t112 * t120 + f64x8::splat(4.0) * t139 * t146 + param_css_0;
            let t150 = t93 * t149;
            let t151 = f64x8::splat(1.0) / v_rho;
            let t153 = f64x8::splat(1.0) / v_tau;
            let t156 = f64x8::splat(1.0) - v_sigma * t151 * t153 / f64x8::splat(8.0);
            let t157 = v_tau * v_tau;
            let t159 = t98 * v_rho;
            let t161 = f64x8::splat(1.0) / t14 / t159;
            let t162 = param_Fermi_D_cnst * param_Fermi_D_cnst;
            let t163 = f64x8::splat(1.0) / t162;
            let t167 = (simd::exp(-f64x8::splat(8.0) * t157 * t16 * t161 * t163));
            let t168 = f64x8::splat(1.0) - t167;
            let t169 = t156 * t168;
            let t171 = f64x8::splat(2.0) * t150 * t169;
            let t173 = t10 * t12 * t15;
            let t175 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t173;
            let t176 = ((t173).sqrt());
            let t179 = ((t173) * (t173).sqrt());
            let t182 = t32 * t11 * t35;
            let t184 = f64x8::splat(3.79785) * t176 + f64x8::splat(0.8969) * t173 + f64x8::splat(0.204775) * t179 + f64x8::splat(0.123235) * t182;
            let t187 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t184;
            let t188 = (simd::ln(t187));
            let t191 = ((t4).select(t50, f64x8::splat(1.0)));
            let t194 = (f64x8::splat(2.0) * t191 - f64x8::splat(2.0)) * t58;
            let t196 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t173;
            let t201 = f64x8::splat(5.1785) * t176 + f64x8::splat(0.905775) * t173 + f64x8::splat(0.1100325) * t179 + f64x8::splat(0.1241775) * t182;
            let t204 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t201;
            let t205 = (simd::ln(t204));
            let t210 = -f64x8::splat(0.0621814) * t175 * t188 + f64x8::splat(0.0197516734986138) * t194 * t196 * t205 - f64x8::splat(2.0) * t93;
            let t212 = param_cab_1;
            let t213 = t212 * param_gamma_ab;
            let t214 = t213 * v_sigma;
            let t218 = f64x8::splat(2.0) * param_gamma_ab * v_sigma * t101 + f64x8::splat(1.0);
            let t219 = f64x8::splat(1.0) / t218;
            let t220 = t101 * t219;
            let t223 = param_cab_2;
            let t224 = param_gamma_ab * param_gamma_ab;
            let t225 = t223 * t224;
            let t226 = t225 * t111;
            let t227 = t218 * t218;
            let t228 = f64x8::splat(1.0) / t227;
            let t229 = t117 * t228;
            let t232 = param_cab_3;
            let t233 = t224 * param_gamma_ab;
            let t234 = t232 * t233;
            let t235 = t227 * t218;
            let t236 = f64x8::splat(1.0) / t235;
            let t240 = param_cab_4;
            let t241 = t224 * t224;
            let t242 = t240 * t241;
            let t243 = t242 * t138;
            let t244 = t227 * t227;
            let t245 = f64x8::splat(1.0) / t244;
            let t246 = t143 * t245;
            let t249 = f64x8::splat(32.0) * t234 * t129 * t236 + f64x8::splat(2.0) * t214 * t220 + f64x8::splat(8.0) * t226 * t229 + f64x8::splat(64.0) * t243 * t246 + param_cab_0;
            let t250 = t210 * t249;
            let tzk0 = t171 + t250;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
