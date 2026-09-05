//! MGGA_X_TM vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tm.c`
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
pub fn mgga_x_tm_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(1.0) / v_rho;
            let t22 = v_sigma * t21;
            let t23 = f64x8::splat(1.0) / v_tau;
            let t25 = t22 * t23 / f64x8::splat(8.0);
            let t26 = (t25).simd_lt(f64x8::splat(1.0));
            let t27 = ((t26).select(t25, f64x8::splat(1.0)));
            let t28 = t27 * t27;
            let t29 = t28 * t27;
            let t31 = t28 + f64x8::splat(3.0) * t29;
            let t32 = f64x8::splat(1.0) + t29;
            let t33 = t32 * t32;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t31 * t34;
            let t36 = f64x8::splat(M_CBRT6);
            let t37 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t38 = (simd::cbrt(t37));
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t36 * t40;
            let t42 = f64x8::splat(M_CBRT2);
            let t43 = t42 * t42;
            let t44 = v_sigma * t43;
            let t45 = v_rho * v_rho;
            let t46 = t19 * t19;
            let t48 = f64x8::splat(1.0) / t46 / t45;
            let t49 = t44 * t48;
            let t50 = t41 * t49;
            let t52 = t36 * t36;
            let t54 = f64x8::splat(1.0) / t38 / t37;
            let t55 = t52 * t54;
            let t56 = v_sigma * v_sigma;
            let t57 = t56 * t42;
            let t58 = t45 * t45;
            let t59 = t58 * v_rho;
            let t61 = f64x8::splat(1.0) / t19 / t59;
            let t65 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t50 + f64x8::splat(0.00537989809245259) * t55 * t57 * t61;
            let t66 = (simd::pow(t65, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t69 = v_tau * t43;
            let t71 = f64x8::splat(1.0) / t46 / v_rho;
            let t72 = t69 * t71;
            let t81 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t50 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.14554132) * t72 + f64x8::splat(0.256337604) * t52 * t39 + f64x8::splat(0.011867481666666667) * t49) * t36 * t40;
            let t82 = t66 * t66;
            let t83 = f64x8::splat(1.0) / t82;
            let t86 = f64x8::splat(1.0) / t66 + f64x8::splat(7.0) / f64x8::splat(9.0) * t81 * t83;
            let t88 = f64x8::splat(1.0) - t35;
            let t91 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t50) * t36;
            let t92 = t91 * t40;
            let t101 = (t72 - t49 / f64x8::splat(8.0)) * t36 * t40 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) + t50 / f64x8::splat(36.0);
            let t102 = t101 * t101;
            let t104 = t101 * t27;
            let t105 = f64x8::splat(1.0) - t27;
            let t108 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t92 * t49 + f64x8::splat(292.0) / f64x8::splat(405.0) * t102 - f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t105;
            let t109 = (simd::pow(t108, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t111 = t88 * t109 + t35 * t86;
            let t115 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t111));
            let tzk0 = f64x8::splat(2.0) * t115;
            acc_zk = tzk0;
            let t117 = t18 / t46;
            let t121 = f64x8::splat(1.0) / t45;
            let t122 = v_sigma * t121;
            let t125 = ((t26).select(-t122 * t23 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t126 = t27 * t125;
            let t128 = t28 * t125;
            let t130 = f64x8::splat(2.0) * t126 + f64x8::splat(9.0) * t128;
            let t131 = t130 * t34;
            let t134 = f64x8::splat(1.0) / t33 / t32;
            let t135 = t31 * t134;
            let t136 = t86 * t28;
            let t137 = t136 * t125;
            let t141 = f64x8::splat(1.0) / t66 / t65;
            let t142 = t45 * v_rho;
            let t144 = f64x8::splat(1.0) / t46 / t142;
            let t145 = t44 * t144;
            let t146 = t41 * t145;
            let t148 = t58 * t45;
            let t150 = f64x8::splat(1.0) / t19 / t148;
            let t152 = t55 * t57 * t150;
            let t154 = -f64x8::splat(0.40121303703703703) * t146 - f64x8::splat(0.028692789826413812) * t152;
            let t158 = t69 * t48;
            let t165 = -f64x8::splat(0.17051554074074074) * t146 - f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(0.24256886666666666) * t158 - f64x8::splat(0.031646617777777775) * t145) * t36 * t40;
            let t169 = f64x8::splat(1.0) / t82 / t65;
            let t170 = t81 * t169;
            let t173 = -t141 * t154 / f64x8::splat(5.0) + f64x8::splat(7.0) / f64x8::splat(9.0) * t165 * t83 - f64x8::splat(14.0) / f64x8::splat(45.0) * t170 * t154;
            let t177 = f64x8::splat(6.0) * t135 * t128 - t131;
            let t179 = t109 * t109;
            let t180 = t179 * t179;
            let t181 = t180 * t180;
            let t182 = t181 * t109;
            let t183 = f64x8::splat(1.0) / t182;
            let t184 = t88 * t183;
            let t195 = (-f64x8::splat(5.0) / f64x8::splat(3.0) * t158 + t145 / f64x8::splat(3.0)) * t36 * t40 / f64x8::splat(4.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t146;
            let t198 = t195 * t27;
            let t201 = t101 * t125;
            let t206 = -f64x8::splat(125.0) / f64x8::splat(19683.0) * t152 - f64x8::splat(10.0) / f64x8::splat(9.0) * t92 * t145 + f64x8::splat(584.0) / f64x8::splat(405.0) * t101 * t195 - f64x8::splat(146.0) / f64x8::splat(135.0) * t198 * t105 - f64x8::splat(146.0) / f64x8::splat(135.0) * t201 * t105 + f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t125;
            let t209 = t131 * t86 - f64x8::splat(6.0) * t135 * t137 + t35 * t173 + t177 * t109 + t184 * t206 / f64x8::splat(10.0);
            let t214 = ((t3).select(f64x8::splat(0.0), -t7 * t117 * t111 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t209));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t214 + f64x8::splat(2.0) * t115;
            acc_vrho = tvrho0;
            let t219 = ((t26).select(t21 * t23 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t220 = t27 * t219;
            let t222 = t28 * t219;
            let t224 = f64x8::splat(2.0) * t220 + f64x8::splat(9.0) * t222;
            let t225 = t224 * t34;
            let t227 = t136 * t219;
            let t230 = t43 * t48;
            let t233 = v_sigma * t42;
            let t235 = t55 * t233 * t61;
            let t237 = f64x8::splat(0.1504548888888889) * t41 * t230 + f64x8::splat(0.01075979618490518) * t235;
            let t245 = -t141 * t237 / f64x8::splat(5.0) + f64x8::splat(0.04460577520576132) * t41 * t230 * t83 - f64x8::splat(14.0) / f64x8::splat(45.0) * t170 * t237;
            let t249 = f64x8::splat(6.0) * t135 * t222 - t225;
            let t252 = t40 * t43;
            let t253 = t252 * t48;
            let t256 = t101 * t36;
            let t257 = t256 * t253;
            let t259 = t41 * t43;
            let t260 = t48 * t27;
            let t262 = t259 * t260 * t105;
            let t264 = t101 * t219;
            let t269 = f64x8::splat(125.0) / f64x8::splat(52488.0) * t235 + f64x8::splat(5.0) / f64x8::splat(12.0) * t91 * t253 - f64x8::splat(73.0) / f64x8::splat(14580.0) * t257 + f64x8::splat(73.0) / f64x8::splat(19440.0) * t262 - f64x8::splat(146.0) / f64x8::splat(135.0) * t264 * t105 + f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t219;
            let t272 = t225 * t86 - f64x8::splat(6.0) * t135 * t227 + t35 * t245 + t249 * t109 + t184 * t269 / f64x8::splat(10.0);
            let t276 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t272));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t276;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t278 = v_tau * v_tau;
            let t279 = f64x8::splat(1.0) / t278;
            let t282 = ((t26).select(-t22 * t279 / f64x8::splat(8.0), f64x8::splat(0.0)));
            let t283 = t27 * t282;
            let t285 = t28 * t282;
            let t287 = f64x8::splat(2.0) * t283 + f64x8::splat(9.0) * t285;
            let t288 = t287 * t34;
            let t290 = t136 * t282;
            let t293 = t35 * t43;
            let t294 = t71 * t36;
            let t295 = t40 * t83;
            let t296 = t294 * t295;
            let t301 = f64x8::splat(6.0) * t135 * t285 - t288;
            let t304 = t294 * t40;
            let t307 = t43 * t71;
            let t308 = t307 * t36;
            let t309 = t40 * t27;
            let t313 = t101 * t282;
            let t318 = f64x8::splat(146.0) / f64x8::splat(405.0) * t101 * t43 * t304 - f64x8::splat(73.0) / f64x8::splat(270.0) * t308 * t309 * t105 - f64x8::splat(146.0) / f64x8::splat(135.0) * t313 * t105 + f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t282;
            let t321 = t288 * t86 - f64x8::splat(6.0) * t135 * t290 - f64x8::splat(0.06288822469135802) * t293 * t296 + t301 * t109 + t184 * t318 / f64x8::splat(10.0);
            let t325 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t321));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t325;
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
