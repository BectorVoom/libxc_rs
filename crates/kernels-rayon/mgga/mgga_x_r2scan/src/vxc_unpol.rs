//! MGGA_X_R2SCAN vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_r2scan.c`
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
pub fn mgga_x_r2scan_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_dp2: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_dp2 = f64x8::splat(param_dp2);
    let param_eta = f64x8::splat(param_eta);
    let param_k1 = f64x8::splat(param_k1);
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
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = t7 * t18;
            let t20 = (simd::cbrt(v_rho));
            let t22 = f64x8::splat(20.0) / f64x8::splat(27.0) + f64x8::splat(5.0) / f64x8::splat(3.0) * param_eta;
            let t23 = f64x8::splat(M_CBRT6);
            let t24 = t23 * t23;
            let t25 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t26 = (simd::cbrt(t25));
            let t27 = t26 * t25;
            let t28 = f64x8::splat(1.0) / t27;
            let t29 = t24 * t28;
            let t30 = v_sigma * v_sigma;
            let t32 = f64x8::splat(M_CBRT2);
            let t33 = v_rho * v_rho;
            let t34 = t33 * t33;
            let t35 = t34 * v_rho;
            let t37 = f64x8::splat(1.0) / t20 / t35;
            let t38 = t32 * t37;
            let t39 = param_dp2 * param_dp2;
            let t40 = t39 * t39;
            let t41 = f64x8::splat(1.0) / t40;
            let t45 = (simd::exp(-t29 * t30 * t38 * t41 / f64x8::splat(288.0)));
            let t49 = (-f64x8::splat(0.162742215233874) * t22 * t45 + f64x8::splat(10.0) / f64x8::splat(81.0)) * t23;
            let t50 = t26 * t26;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = t49 * t51;
            let t53 = t32 * t32;
            let t54 = v_sigma * t53;
            let t55 = t20 * t20;
            let t57 = f64x8::splat(1.0) / t55 / t33;
            let t58 = t54 * t57;
            let t61 = param_k1 + t52 * t58 / f64x8::splat(24.0);
            let t65 = param_k1 * (f64x8::splat(1.0) - param_k1 / t61);
            let t66 = v_tau * t53;
            let t67 = t55 * v_rho;
            let t68 = f64x8::splat(1.0) / t67;
            let t71 = t66 * t68 - t58 / f64x8::splat(8.0);
            let t75 = t53 * t57;
            let t78 = f64x8::splat(3.0) / f64x8::splat(10.0) * t24 * t50 + param_eta * v_sigma * t75 / f64x8::splat(8.0);
            let t79 = f64x8::splat(1.0) / t78;
            let t80 = t71 * t79;
            let t81 = (t80).simd_le(f64x8::splat(0.0));
            let t82 = (f64x8::splat(0.0)).simd_lt(t80);
            let t83 = ((t82).select(f64x8::splat(0.0), t80));
            let t84 = param_c1 * t83;
            let t85 = f64x8::splat(1.0) - t83;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = (simd::exp(-t84 * t86));
            let t89 = (t80).simd_le(f64x8::splat(2.5));
            let t90 = (f64x8::splat(2.5)).simd_lt(t80);
            let t91 = ((t90).select(f64x8::splat(2.5), t80));
            let t93 = t91 * t91;
            let t95 = t93 * t91;
            let t97 = t93 * t93;
            let t99 = t97 * t91;
            let t101 = t97 * t93;
            let t106 = ((t90).select(t80, f64x8::splat(2.5)));
            let t107 = f64x8::splat(1.0) - t106;
            let t110 = (simd::exp(param_c2 / t107));
            let t112 = ((t81).select(t88, (t89).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t91 - f64x8::splat(0.4445555) * t93 - f64x8::splat(0.663086601049) * t95 + f64x8::splat(1.45129704449) * t97 - f64x8::splat(0.887998041597) * t99 + f64x8::splat(0.234528941479) * t101 - f64x8::splat(0.023185843322) * t97 * t95, -param_d * t110)));
            let t113 = f64x8::splat(0.174) - t65;
            let t115 = t112 * t113 + t65 + f64x8::splat(1.0);
            let t117 = ((f64x8::splat(3.0)).sqrt());
            let t118 = f64x8::splat(1.0) / t26;
            let t119 = t24 * t118;
            let t120 = ((v_sigma).sqrt());
            let t121 = t120 * t32;
            let t123 = f64x8::splat(1.0) / t20 / v_rho;
            let t125 = t119 * t121 * t123;
            let t126 = ((t125).sqrt());
            let t130 = (simd::exp(-f64x8::splat(9.8958) * t117 / t126));
            let t131 = f64x8::splat(1.0) - t130;
            let t135 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t115 * t131));
            let tzk0 = f64x8::splat(2.0) * t135;
            acc_zk = tzk0;
            let t136 = f64x8::splat(1.0) / t55;
            let t141 = param_k1 * param_k1;
            let t142 = t61 * t61;
            let t143 = f64x8::splat(1.0) / t142;
            let t144 = t141 * t143;
            let t145 = t30 * v_sigma;
            let t146 = t22 * t145;
            let t147 = t34 * t34;
            let t148 = t147 * v_rho;
            let t149 = f64x8::splat(1.0) / t148;
            let t151 = t149 * t41 * t45;
            let t154 = t33 * v_rho;
            let t156 = f64x8::splat(1.0) / t55 / t154;
            let t157 = t54 * t156;
            let t160 = -f64x8::splat(1.5469524941471938e-05) * t146 * t151 - t52 * t157 / f64x8::splat(9.0);
            let t165 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t66 * t57 + t157 / f64x8::splat(3.0);
            let t167 = t78 * t78;
            let t168 = f64x8::splat(1.0) / t167;
            let t169 = t71 * t168;
            let t170 = t169 * param_eta;
            let t173 = t165 * t79 + t170 * t157 / f64x8::splat(3.0);
            let t174 = ((t82).select(f64x8::splat(0.0), t173));
            let t177 = t85 * t85;
            let t178 = f64x8::splat(1.0) / t177;
            let t179 = t178 * t174;
            let t181 = -param_c1 * t174 * t86 - t84 * t179;
            let t182 = t181 * t88;
            let t183 = ((t90).select(f64x8::splat(0.0), t173));
            let t185 = t91 * t183;
            let t187 = t93 * t183;
            let t189 = t95 * t183;
            let t191 = t97 * t183;
            let t193 = t99 * t183;
            let t198 = param_d * param_c2;
            let t199 = t107 * t107;
            let t200 = f64x8::splat(1.0) / t199;
            let t201 = ((t90).select(t173, f64x8::splat(0.0)));
            let t205 = ((t81).select(t182, (t89).select(-f64x8::splat(0.667) * t183 - f64x8::splat(0.889111) * t185 - f64x8::splat(1.989259803147) * t187 + f64x8::splat(5.80518817796) * t189 - f64x8::splat(4.439990207985) * t191 + f64x8::splat(1.407173648874) * t193 - f64x8::splat(0.162300903254) * t101 * t183, -t198 * t200 * t201 * t110)));
            let t207 = t112 * t141;
            let t208 = t143 * t160;
            let t210 = t205 * t113 + t144 * t160 - t207 * t208;
            let t215 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t216 = t215 * t215;
            let t217 = t216 * t216;
            let t219 = t217 * t215 * t18;
            let t220 = f64x8::splat(1.0) / t33;
            let t221 = t220 * t115;
            let t223 = f64x8::splat(1.0) / t126 / t125;
            let t225 = t219 * t221 * t223;
            let t226 = t121 * t130;
            let t227 = t119 * t226;
            let t231 = ((t3).select(f64x8::splat(0.0), -t19 * t136 * t115 * t131 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t210 * t131 - f64x8::splat(1.6891736332904388) * t225 * t227));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t231 + f64x8::splat(2.0) * t135;
            acc_vrho = tvrho0;
            let t234 = t22 * t30;
            let t235 = f64x8::splat(1.0) / t147;
            let t237 = t235 * t41 * t45;
            let t240 = t51 * t53;
            let t244 = f64x8::splat(5.801071853051976e-06) * t234 * t237 + t49 * t240 * t57 / f64x8::splat(24.0);
            let t246 = t75 * t79;
            let t247 = param_eta * t53;
            let t248 = t247 * t57;
            let t251 = -t169 * t248 / f64x8::splat(8.0) - t246 / f64x8::splat(8.0);
            let t252 = ((t82).select(f64x8::splat(0.0), t251));
            let t253 = param_c1 * t252;
            let t255 = t178 * t252;
            let t257 = -t253 * t86 - t84 * t255;
            let t258 = t257 * t88;
            let t259 = ((t90).select(f64x8::splat(0.0), t251));
            let t261 = t91 * t259;
            let t263 = t93 * t259;
            let t265 = t95 * t259;
            let t267 = t97 * t259;
            let t269 = t99 * t259;
            let t274 = ((t90).select(t251, f64x8::splat(0.0)));
            let t278 = ((t81).select(t258, (t89).select(-f64x8::splat(0.667) * t259 - f64x8::splat(0.889111) * t261 - f64x8::splat(1.989259803147) * t263 + f64x8::splat(5.80518817796) * t265 - f64x8::splat(4.439990207985) * t267 + f64x8::splat(1.407173648874) * t269 - f64x8::splat(0.162300903254) * t101 * t259, -t198 * t200 * t274 * t110)));
            let t280 = t143 * t244;
            let t282 = t278 * t113 + t144 * t244 - t207 * t280;
            let t287 = f64x8::splat(1.0) / v_rho;
            let t288 = t287 * t115;
            let t290 = t219 * t288 * t223;
            let t291 = f64x8::splat(1.0) / t120;
            let t293 = t291 * t32 * t130;
            let t294 = t119 * t293;
            let t298 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t282 * t131 + f64x8::splat(0.6334401124839145) * t290 * t294));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t298;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t301 = t53 * t68 * t79;
            let t302 = ((t82).select(f64x8::splat(0.0), t301));
            let t303 = param_c1 * t302;
            let t305 = t178 * t302;
            let t307 = -t303 * t86 - t84 * t305;
            let t308 = t307 * t88;
            let t309 = ((t90).select(f64x8::splat(0.0), t301));
            let t311 = t91 * t309;
            let t313 = t93 * t309;
            let t315 = t95 * t309;
            let t317 = t97 * t309;
            let t319 = t99 * t309;
            let t324 = ((t90).select(t301, f64x8::splat(0.0)));
            let t328 = ((t81).select(t308, (t89).select(-f64x8::splat(0.667) * t309 - f64x8::splat(0.889111) * t311 - f64x8::splat(1.989259803147) * t313 + f64x8::splat(5.80518817796) * t315 - f64x8::splat(4.439990207985) * t317 + f64x8::splat(1.407173648874) * t319 - f64x8::splat(0.162300903254) * t101 * t309, -t198 * t200 * t324 * t110)));
            let t329 = t20 * t328;
            let t330 = t113 * t131;
            let t334 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t329 * t330));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t334;
            acc_vtau = tvtau0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        ip += 8;
    }
}
