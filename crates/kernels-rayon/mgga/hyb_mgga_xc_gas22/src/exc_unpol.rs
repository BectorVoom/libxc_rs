//! HYB_MGGA_XC_GAS22 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_xc_gas22.c`
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
pub fn hyb_mgga_xc_gas22_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c_x_1: f64,
    param_c_x_2: f64,
    param_c_x_0: f64,
    param_c_ss_0: f64,
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_os_1: f64,
    param_c_os_2: f64,
    param_c_os_3: f64,
    param_c_os_4: f64,
    param_c_os_0: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c_x_1 = f64x8::splat(param_c_x_1);
    let param_c_x_2 = f64x8::splat(param_c_x_2);
    let param_c_x_0 = f64x8::splat(param_c_x_0);
    let param_c_ss_0 = f64x8::splat(param_c_ss_0);
    let param_c_ss_1 = f64x8::splat(param_c_ss_1);
    let param_c_ss_2 = f64x8::splat(param_c_ss_2);
    let param_c_ss_3 = f64x8::splat(param_c_ss_3);
    let param_c_ss_4 = f64x8::splat(param_c_ss_4);
    let param_c_os_1 = f64x8::splat(param_c_os_1);
    let param_c_os_2 = f64x8::splat(param_c_os_2);
    let param_c_os_3 = f64x8::splat(param_c_os_3);
    let param_c_os_4 = f64x8::splat(param_c_os_4);
    let param_c_os_0 = f64x8::splat(param_c_os_0);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
            let t6 = f64x8::splat(M_CBRT3);
            let t7 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t8 = (simd::cbrt(t7));
            let t9 = t6 * t8;
            let t10 = f64x8::splat(M_CBRT4);
            let t11 = t10 * t10;
            let t12 = f64x8::splat(M_CBRT2);
            let t14 = t9 * t11 * t12;
            let t15 = (f64x8::splat(2.0)).simd_le(zeta_threshold);
            let t16 = (simd::cbrt(zeta_threshold));
            let t17 = t16 * zeta_threshold;
            let t19 = ((t15).select(t17, f64x8::splat(2.0) * t12));
            let t20 = (simd::cbrt(v_rho));
            let t21 = t19 * t20;
            let t22 = (simd::cbrt(f64x8::splat(9.0)));
            let t23 = t22 * t22;
            let t24 = t8 * t8;
            let t26 = t23 * t24 * param_hyb_omega_0;
            let t27 = f64x8::splat(1.0) / t20;
            let t29 = ((t15).select(t16, t12));
            let t31 = t12 / t29;
            let t34 = t26 * t6 * t27 * t31 / f64x8::splat(18.0);
            let t35 = (f64x8::splat(1.35)).simd_le(t34);
            let t36 = (f64x8::splat(1.35)).simd_lt(t34);
            let t37 = ((t36).select(t34, f64x8::splat(1.35)));
            let t38 = t37 * t37;
            let t41 = t38 * t38;
            let t42 = f64x8::splat(1.0) / t41;
            let t44 = t41 * t38;
            let t45 = f64x8::splat(1.0) / t44;
            let t47 = t41 * t41;
            let t48 = f64x8::splat(1.0) / t47;
            let t51 = f64x8::splat(1.0) / t47 / t38;
            let t54 = f64x8::splat(1.0) / t47 / t41;
            let t57 = f64x8::splat(1.0) / t47 / t44;
            let t59 = t47 * t47;
            let t60 = f64x8::splat(1.0) / t59;
            let t63 = ((t36).select(f64x8::splat(1.35), t34));
            let t64 = ((f64x8::splat(M_PI)).sqrt());
            let t65 = f64x8::splat(1.0) / t63;
            let t67 = (simd::erf(t65 / f64x8::splat(2.0)));
            let t69 = t63 * t63;
            let t70 = f64x8::splat(1.0) / t69;
            let t72 = (simd::exp(-t70 / f64x8::splat(4.0)));
            let t73 = t72 - f64x8::splat(1.0);
            let t76 = t72 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t69 * t73;
            let t79 = f64x8::splat(2.0) * t63 * t76 + t64 * t67;
            let t83 = ((t35).select(f64x8::splat(1.0) / t38 / f64x8::splat(36.0) - t42 / f64x8::splat(960.0) + t45 / f64x8::splat(26880.0) - t48 / f64x8::splat(829440.0) + t51 / f64x8::splat(28385280.0) - t54 / f64x8::splat(1073479680.0) + t57 / f64x8::splat(44590694400.0) - t60 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t63 * t79));
            let t85 = param_c_x_1;
            let t86 = t85 * v_sigma;
            let t87 = t12 * t12;
            let t88 = v_rho * v_rho;
            let t89 = t20 * t20;
            let t91 = f64x8::splat(1.0) / t89 / t88;
            let t92 = t87 * t91;
            let t93 = v_sigma * t87;
            let t94 = t93 * t91;
            let t96 = f64x8::splat(1.0) + f64x8::splat(0.003840616724010807) * t94;
            let t97 = f64x8::splat(1.0) / t96;
            let t101 = param_c_x_2;
            let t102 = f64x8::splat(M_CBRT6);
            let t103 = t102 * t102;
            let t104 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t105 = (simd::cbrt(t104));
            let t106 = t105 * t105;
            let t107 = t103 * t106;
            let t108 = f64x8::splat(3.0) / f64x8::splat(10.0) * t107;
            let t109 = v_tau * t87;
            let t111 = f64x8::splat(1.0) / t89 / v_rho;
            let t112 = t109 * t111;
            let t113 = t108 - t112;
            let t114 = t101 * t113;
            let t115 = t108 + t112;
            let t116 = f64x8::splat(1.0) / t115;
            let t118 = param_c_x_0 + f64x8::splat(0.003840616724010807) * t86 * t92 * t97 + t114 * t116;
            let t119 = t83 * t118;
            let t123 = ((t5).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(64.0) * t14 * t21 * t119));
            let t124 = f64x8::splat(2.0) * t123;
            let t125 = ((t4).select(zeta_threshold, f64x8::splat(1.0)));
            let t126 = t9 * t11;
            let t129 = ((t4).select(f64x8::splat(1.0) / t16, f64x8::splat(1.0)));
            let t131 = t126 * t27 * t12 * t129;
            let t133 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t131;
            let t134 = ((t131).sqrt());
            let t137 = ((t131) * (t131).sqrt());
            let t139 = t6 * t6;
            let t140 = t139 * t24;
            let t141 = t140 * t10;
            let t142 = f64x8::splat(1.0) / t89;
            let t144 = t129 * t129;
            let t146 = t141 * t142 * t87 * t144;
            let t148 = f64x8::splat(3.79785) * t134 + f64x8::splat(0.8969) * t131 + f64x8::splat(0.204775) * t137 + f64x8::splat(0.123235) * t146;
            let t151 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t148;
            let t152 = (simd::ln(t151));
            let t154 = f64x8::splat(0.0621814) * t133 * t152;
            let t156 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t17, f64x8::splat(0.0)));
            let t160 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t12 - f64x8::splat(2.0));
            let t161 = (t19 + t156 - f64x8::splat(2.0)) * t160;
            let t163 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t131;
            let t168 = f64x8::splat(7.05945) * t134 + f64x8::splat(1.549425) * t131 + f64x8::splat(0.420775) * t137 + f64x8::splat(0.1562925) * t146;
            let t171 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t168;
            let t172 = (simd::ln(t171));
            let t176 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t131;
            let t181 = f64x8::splat(5.1785) * t134 + f64x8::splat(0.905775) * t131 + f64x8::splat(0.1100325) * t137 + f64x8::splat(0.1241775) * t146;
            let t184 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t181;
            let t185 = (simd::ln(t184));
            let t186 = t176 * t185;
            let t195 = ((t5).select(f64x8::splat(0.0), t125 * (-t154 + t161 * (-f64x8::splat(0.0310907) * t163 * t172 + t154 - f64x8::splat(0.0197516734986138) * t186) + f64x8::splat(0.0197516734986138) * t161 * t186) / f64x8::splat(2.0)));
            let t196 = param_c_ss_0;
            let t197 = t196 * v_sigma;
            let t199 = f64x8::splat(1.0) + f64x8::splat(0.46914023462026644) * t94;
            let t200 = f64x8::splat(1.0) / t199;
            let t204 = param_c_ss_1;
            let t205 = t204 * t113;
            let t207 = param_c_ss_2;
            let t208 = t113 * t113;
            let t209 = t207 * t208;
            let t210 = t115 * t115;
            let t211 = f64x8::splat(1.0) / t210;
            let t213 = param_c_ss_3;
            let t214 = v_sigma * v_sigma;
            let t215 = t214 * t214;
            let t216 = t215 * t214;
            let t217 = t213 * t216;
            let t218 = t88 * t88;
            let t219 = t218 * t218;
            let t220 = t219 * t219;
            let t221 = f64x8::splat(1.0) / t220;
            let t222 = t199 * t199;
            let t223 = t222 * t222;
            let t225 = f64x8::splat(1.0) / t223 / t222;
            let t226 = t221 * t225;
            let t229 = param_c_ss_4;
            let t230 = t208 * t208;
            let t231 = t229 * t230;
            let t232 = t210 * t210;
            let t233 = f64x8::splat(1.0) / t232;
            let t234 = t231 * t233;
            let t239 = f64x8::splat(0.46914023462026644) * t197 * t92 * t200 + t205 * t116 + t209 * t211 + f64x8::splat(0.17058312527037534) * t217 * t226 + f64x8::splat(0.17058312527037534) * t234 * t216 * t221 * t225;
            let t241 = f64x8::splat(2.0) * t195 * t239;
            let t243 = t9 * t11 * t27;
            let t245 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t243;
            let t246 = ((t243).sqrt());
            let t249 = ((t243) * (t243).sqrt());
            let t252 = t140 * t10 * t142;
            let t254 = f64x8::splat(3.79785) * t246 + f64x8::splat(0.8969) * t243 + f64x8::splat(0.204775) * t249 + f64x8::splat(0.123235) * t252;
            let t257 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t254;
            let t258 = (simd::ln(t257));
            let t261 = ((t4).select(t17, f64x8::splat(1.0)));
            let t264 = (f64x8::splat(2.0) * t261 - f64x8::splat(2.0)) * t160;
            let t266 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t243;
            let t271 = f64x8::splat(5.1785) * t246 + f64x8::splat(0.905775) * t243 + f64x8::splat(0.1100325) * t249 + f64x8::splat(0.1241775) * t252;
            let t274 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t271;
            let t275 = (simd::ln(t274));
            let t280 = -f64x8::splat(0.0621814) * t245 * t258 + f64x8::splat(0.0197516734986138) * t264 * t266 * t275 - f64x8::splat(2.0) * t195;
            let t282 = param_c_os_1;
            let t284 = f64x8::splat(3.0) / f64x8::splat(5.0) * t107 * t112;
            let t285 = v_tau * v_tau;
            let t286 = t285 * t12;
            let t287 = t88 * v_rho;
            let t289 = f64x8::splat(1.0) / t20 / t287;
            let t291 = f64x8::splat(4.0) * t286 * t289;
            let t292 = t284 - t291;
            let t293 = t292 * t292;
            let t294 = t282 * t293;
            let t295 = t284 + t291;
            let t296 = t295 * t295;
            let t297 = f64x8::splat(1.0) / t296;
            let t299 = param_c_os_2;
            let t300 = t293 * t293;
            let t301 = t300 * t293;
            let t302 = t299 * t301;
            let t303 = t296 * t296;
            let t305 = f64x8::splat(1.0) / t303 / t296;
            let t307 = param_c_os_3;
            let t308 = t307 * t301;
            let t309 = (simd::cbrt(t94));
            let t310 = t305 * t309;
            let t312 = param_c_os_4;
            let t313 = t312 * t293;
            let t314 = t297 * t309;
            let t316 = t294 * t297 + t302 * t305 + t308 * t310 + t313 * t314 + param_c_os_0;
            let t317 = t280 * t316;
            let tzk0 = t124 + t241 + t317;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
