//! HYB_MGGA_X_PJS18 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_pjs18.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn hyb_mgga_x_pjs18_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t13 = (t12).simd_le(zeta_threshold);
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = ((t13).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = (simd::cbrt(f64x8::splat(9.0)));
            let t22 = t21 * t21;
            let t24 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t25 = t24 * t24;
            let t26 = t22 * t25;
            let t27 = t26 * param_hyb_omega_0;
            let t28 = f64x8::splat(1.0) / t19;
            let t30 = f64x8::splat(M_CBRT6);
            let t31 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t32 = (simd::cbrt(t31));
            let t33 = t32 * t32;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t30 * t34;
            let t36 = f64x8::splat(M_CBRT2);
            let t37 = t36 * t36;
            let t38 = v_sigma * t37;
            let t39 = v_rho * v_rho;
            let t40 = t19 * t19;
            let t42 = f64x8::splat(1.0) / t40 / t39;
            let t43 = t38 * t42;
            let t46 = t30 * t30;
            let t48 = f64x8::splat(1.0) / t32 / t31;
            let t49 = t46 * t48;
            let t50 = v_sigma * v_sigma;
            let t51 = t50 * t36;
            let t52 = t39 * t39;
            let t53 = t52 * v_rho;
            let t55 = f64x8::splat(1.0) / t19 / t53;
            let t59 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t35 * t43 + f64x8::splat(0.00537989809245259) * t49 * t51 * t55;
            let t60 = (simd::pow(t59, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t62 = ((t13).select(t14, t16));
            let t63 = f64x8::splat(1.0) / t62;
            let t64 = f64x8::splat(1.0) / t60 * t63;
            let t67 = t27 * t4 * t28 * t64 / f64x8::splat(18.0);
            let t68 = (t67).simd_lt(f64x8::splat(1e-10));
            let t69 = ((t68).select(f64x8::splat(1e-10), t67));
            let t70 = (f64x8::splat(1.35)).simd_le(t69);
            let t71 = (f64x8::splat(1.35)).simd_lt(t69);
            let t72 = ((t71).select(t69, f64x8::splat(1.35)));
            let t73 = t72 * t72;
            let t76 = t73 * t73;
            let t77 = f64x8::splat(1.0) / t76;
            let t79 = t76 * t73;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = t76 * t76;
            let t83 = f64x8::splat(1.0) / t82;
            let t86 = f64x8::splat(1.0) / t82 / t73;
            let t89 = f64x8::splat(1.0) / t82 / t76;
            let t92 = f64x8::splat(1.0) / t82 / t79;
            let t94 = t82 * t82;
            let t95 = f64x8::splat(1.0) / t94;
            let t98 = ((t71).select(f64x8::splat(1.35), t69));
            let t99 = ((f64x8::splat(M_PI)).sqrt());
            let t100 = f64x8::splat(1.0) / t98;
            let t102 = (simd::erf(t100 / f64x8::splat(2.0)));
            let t104 = t98 * t98;
            let t105 = f64x8::splat(1.0) / t104;
            let t107 = (simd::exp(-t105 / f64x8::splat(4.0)));
            let t108 = t107 - f64x8::splat(1.0);
            let t111 = t107 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t104 * t108;
            let t114 = t102 * t99 + f64x8::splat(2.0) * t111 * t98;
            let t118 = ((t70).select(f64x8::splat(1.0) / t73 / f64x8::splat(36.0) - t77 / f64x8::splat(960.0) + t80 / f64x8::splat(26880.0) - t83 / f64x8::splat(829440.0) + t86 / f64x8::splat(28385280.0) - t89 / f64x8::splat(1073479680.0) + t92 / f64x8::splat(44590694400.0) - t95 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t98 * t114));
            let t119 = (simd::pow(t59, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t120 = f64x8::splat(1.0) / t119;
            let t122 = (f64x8::splat(0.27)).simd_le(t69);
            let t123 = (f64x8::splat(0.27)).simd_lt(t69);
            let t124 = ((t123).select(t69, f64x8::splat(0.27)));
            let t125 = t124 * t124;
            let t126 = t125 * t125;
            let t127 = t126 * t126;
            let t128 = t127 * t126;
            let t129 = t127 * t127;
            let t130 = t129 * t129;
            let t132 = f64x8::splat(1.0) / t130 / t128;
            let t134 = t126 * t125;
            let t135 = t127 * t134;
            let t137 = f64x8::splat(1.0) / t130 / t135;
            let t141 = f64x8::splat(1.0) / t134;
            let t143 = f64x8::splat(1.0) / t127;
            let t145 = t127 * t125;
            let t146 = f64x8::splat(1.0) / t145;
            let t148 = f64x8::splat(1.0) / t128;
            let t150 = f64x8::splat(1.0) / t135;
            let t152 = f64x8::splat(1.0) / t129;
            let t154 = t129 * t125;
            let t155 = f64x8::splat(1.0) / t154;
            let t158 = f64x8::splat(1.0) / t129 / t126;
            let t160 = t132 / f64x8::splat(3.3929038000650147e+37) - t137 / f64x8::splat(3.511556992918352e+39) + f64x8::splat(3.0) / f64x8::splat(2240.0) / t126 - t141 / f64x8::splat(11520.0) + f64x8::splat(3.0) / f64x8::splat(788480.0) * t143 - t146 / f64x8::splat(7454720.0) + t148 / f64x8::splat(247726080.0) - t150 / f64x8::splat(9358540800.0) + t152 / f64x8::splat(394474291200.0) - t155 / f64x8::splat(18311911833600.0) + t158 / f64x8::splat(927028425523200.0);
            let t162 = f64x8::splat(1.0) / t129 / t134;
            let t165 = f64x8::splat(1.0) / t129 / t127;
            let t168 = f64x8::splat(1.0) / t129 / t145;
            let t171 = f64x8::splat(1.0) / t129 / t128;
            let t174 = f64x8::splat(1.0) / t129 / t135;
            let t176 = f64x8::splat(1.0) / t130;
            let t179 = f64x8::splat(1.0) / t130 / t125;
            let t182 = f64x8::splat(1.0) / t130 / t126;
            let t185 = f64x8::splat(1.0) / t130 / t134;
            let t188 = f64x8::splat(1.0) / t130 / t127;
            let t191 = f64x8::splat(1.0) / t130 / t145;
            let t193 = -t162 / f64x8::splat(5.0785035485184e+16) + t165 / f64x8::splat(2.991700272218112e+18) - t168 / f64x8::splat(1.88514051721003e+20) + t171 / f64x8::splat(1.2648942844388573e+22) - t174 / f64x8::splat(9.002316741416457e+23) + t176 / f64x8::splat(6.772652029299977e+25) - t179 / f64x8::splat(5.36974553751641e+27) + t182 / f64x8::splat(4.474731034888079e+29) - t185 / f64x8::splat(3.909716563474291e+31) + t188 / f64x8::splat(3.5738523369945735e+33) - t191 / f64x8::splat(3.410951160703658e+35);
            let t195 = ((t123).select(f64x8::splat(0.27), t69));
            let t196 = t195 * t195;
            let t198 = t196 * t196;
            let t199 = f64x8::splat(64.0) * t198;
            let t200 = f64x8::splat(20.0) * t196 - t199;
            let t203 = (simd::exp(-f64x8::splat(1.0) / t196 / f64x8::splat(4.0)));
            let t207 = f64x8::splat(1.0) / t195;
            let t209 = (simd::erf(t207 / f64x8::splat(2.0)));
            let t212 = f64x8::splat(10.0) * t195 * t209 * t99 + t200 * t203 - f64x8::splat(36.0) * t196 + t199 - f64x8::splat(3.0);
            let t216 = ((t122).select(t160 + t193, f64x8::splat(24.0) * t196 * t212 + f64x8::splat(1.0)));
            let t217 = v_tau * t37;
            let t219 = f64x8::splat(1.0) / t40 / v_rho;
            let t225 = -f64x8::splat(0.14554132) * t217 * t219 + f64x8::splat(0.043662396) * t46 * t33 + f64x8::splat(0.04229627833333333) * t43;
            let t226 = t216 * t225;
            let t227 = t119 * t119;
            let t228 = f64x8::splat(1.0) / t227;
            let t229 = t35 * t228;
            let t232 = (f64x8::splat(0.32)).simd_le(t69);
            let t233 = (f64x8::splat(0.32)).simd_lt(t69);
            let t234 = ((t233).select(t69, f64x8::splat(0.32)));
            let t235 = t234 * t234;
            let t236 = t235 * t235;
            let t239 = t236 * t235;
            let t240 = f64x8::splat(1.0) / t239;
            let t242 = t236 * t236;
            let t243 = f64x8::splat(1.0) / t242;
            let t245 = t242 * t235;
            let t246 = f64x8::splat(1.0) / t245;
            let t248 = t242 * t236;
            let t249 = f64x8::splat(1.0) / t248;
            let t251 = t242 * t239;
            let t252 = f64x8::splat(1.0) / t251;
            let t254 = t242 * t242;
            let t255 = f64x8::splat(1.0) / t254;
            let t258 = f64x8::splat(1.0) / t254 / t235;
            let t261 = f64x8::splat(1.0) / t254 / t236;
            let t264 = f64x8::splat(1.0) / t254 / t239;
            let t267 = f64x8::splat(1.0) / t254 / t242;
            let t270 = f64x8::splat(1.0) / t254 / t245;
            let t273 = f64x8::splat(1.0) / t254 / t248;
            let t276 = f64x8::splat(1.0) / t254 / t251;
            let t278 = t254 * t254;
            let t279 = f64x8::splat(1.0) / t278;
            let t282 = f64x8::splat(1.0) / t278 / t235;
            let t285 = f64x8::splat(1.0) / t278 / t236;
            let t288 = f64x8::splat(1.0) / t278 / t239;
            let t290 = f64x8::splat(3.0) / f64x8::splat(7840.0) / t236 - t240 / f64x8::splat(56448.0) + f64x8::splat(5.0) / f64x8::splat(8515584.0) * t243 - t246 / f64x8::splat(61501440.0) + t249 / f64x8::splat(2530344960.0) - t252 / f64x8::splat(115811942400.0) + t255 / f64x8::splat(5811921223680.0) - t258 / f64x8::splat(316612955602944.0) + t261 / f64x8::splat(1.85827061661696e+16) - t264 / f64x8::splat(1.168055816159232e+18) + t267 / f64x8::splat(7.824446865801216e+19) - t270 / f64x8::splat(5.562511054710453e+21) + t273 / f64x8::splat(4.181740504354862e+23) - t276 / f64x8::splat(3.3139778504339334e+25) + t279 / f64x8::splat(2.7608516801793436e+27) - t282 / f64x8::splat(2.4119107039344544e+29) + t285 / f64x8::splat(2.2046293272414373e+31) - t288 / f64x8::splat(2.1042094544618633e+33);
            let t291 = ((t233).select(f64x8::splat(0.32), t69));
            let t293 = t291 * t291;
            let t294 = t293 * t291;
            let t296 = t293 * t293;
            let t297 = t296 * t291;
            let t299 = t296 * t294;
            let t301 = t296 * t296;
            let t302 = t301 * t291;
            let t304 = -f64x8::splat(8.0) * t291 + f64x8::splat(256.0) * t294 - f64x8::splat(576.0) * t297 + f64x8::splat(3840.0) * t299 - f64x8::splat(122880.0) * t302;
            let t305 = f64x8::splat(1.0) / t293;
            let t307 = (simd::exp(-t305 / f64x8::splat(4.0)));
            let t311 = t296 * t293;
            let t313 = -f64x8::splat(35.0) + f64x8::splat(224.0) * t293 - f64x8::splat(1440.0) * t296 + f64x8::splat(5120.0) * t311;
            let t317 = -f64x8::splat(2.0) + f64x8::splat(60.0) * t293;
            let t319 = f64x8::splat(1.0) / t291;
            let t321 = (simd::erf(t319 / f64x8::splat(2.0)));
            let t324 = f64x8::splat(2.0) * t317 * t321 * t99 + f64x8::splat(24.0) * t294 * t313 + t304 * t307;
            let t328 = ((t232).select(t290, f64x8::splat(1.0) + f64x8::splat(8.0) / f64x8::splat(7.0) * t291 * t324));
            let t329 = t328 * t30;
            let t330 = t329 * t34;
            let t332 = t38 * t42 * t228;
            let t335 = t118 * t120 + f64x8::splat(35.0) / f64x8::splat(81.0) * t226 * t229 + f64x8::splat(0.026329605555555555) * t330 * t332;
            let t339 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t335));
            let tzk0 = f64x8::splat(2.0) * t339;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
