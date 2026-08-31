//! MGGA_K_PC07 vxc unpol kernel — explicit SIMD (exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_pc07.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py (exact math). Eight grid points per step; every lane runs maple2c's expression
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
pub fn mgga_k_pc07_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_b = f64x8::splat(param_b);
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
            let t5 = t4 * t4;
            let t6 = f64x8::splat(M_CBRTPI);
            let t8 = t5 * t6 * f64x8::splat(M_PI);
            let t9 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t10 = zeta_threshold - f64x8::splat(1.0);
            let t12 = ((t9).select(t10, (t9).select(-t10, f64x8::splat(0.0))));
            let t13 = f64x8::splat(1.0) + t12;
            let t15 = (simd::cbrt(zeta_threshold));
            let t16 = t15 * t15;
            let t18 = (simd::cbrt(t13));
            let t19 = t18 * t18;
            let t21 = (((t13).simd_le(zeta_threshold)).select(t16 * zeta_threshold, t19 * t13));
            let t22 = (simd::cbrt(v_rho));
            let t23 = t22 * t22;
            let t24 = t21 * t23;
            let t25 = f64x8::splat(M_CBRT6);
            let t26 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t27 = (simd::cbrt(t26));
            let t28 = t27 * t27;
            let t29 = f64x8::splat(1.0) / t28;
            let t30 = t25 * t29;
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t31 * t31;
            let t33 = v_sigma * t32;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t23 / t34;
            let t38 = t30 * t33 * t36;
            let t39 = f64x8::splat(5.0) / f64x8::splat(72.0) * t38;
            let t41 = v_lapl * t32;
            let t43 = f64x8::splat(1.0) / t23 / v_rho;
            let t47 = t25 * t25;
            let t49 = f64x8::splat(1.0) / t27 / t26;
            let t50 = t47 * t49;
            let t51 = v_lapl * v_lapl;
            let t52 = t51 * t31;
            let t53 = t34 * v_rho;
            let t55 = f64x8::splat(1.0) / t22 / t53;
            let t58 = t50 * t52 * t55 / f64x8::splat(2916.0);
            let t59 = t50 * v_sigma;
            let t60 = t34 * t34;
            let t62 = f64x8::splat(1.0) / t22 / t60;
            let t63 = t31 * t62;
            let t64 = t63 * v_lapl;
            let t66 = t59 * t64 / f64x8::splat(2592.0);
            let t67 = v_sigma * v_sigma;
            let t68 = t67 * t31;
            let t69 = t60 * v_rho;
            let t71 = f64x8::splat(1.0) / t22 / t69;
            let t74 = t50 * t68 * t71 / f64x8::splat(8748.0);
            let t75 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(648.0) * t38 + f64x8::splat(5.0) / f64x8::splat(54.0) * t30 * t41 * t43 + t58 - t66 + t74;
            let t76 = t58 - t66 + t74;
            let t77 = t76 * t76;
            let t78 = f64x8::splat(1.0) + t39;
            let t79 = t78 * t78;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = t77 * t80 + f64x8::splat(1.0);
            let t83 = ((t82).sqrt());
            let t84 = f64x8::splat(1.0) / t83;
            let t86 = t75 * t84 - t39;
            let t87 = param_a / f64x8::splat(40.0);
            let t88 = (t86).simd_le(t87);
            let t89 = f64x8::splat(39.0) / f64x8::splat(40.0) * param_a;
            let t90 = (t89).simd_le(t86);
            let t91 = param_a * param_b;
            let t92 = (t86).simd_lt(t87);
            let t93 = ((t92).select(t87, t86));
            let t94 = (t93).simd_lt(t89);
            let t95 = ((t94).select(t93, t89));
            let t96 = f64x8::splat(1.0) / t95;
            let t98 = (simd::exp(-t91 * t96));
            let t99 = param_a - t95;
            let t102 = (simd::exp(-param_a / t99));
            let t103 = f64x8::splat(1.0) + t102;
            let t104 = (simd::pow(t103, param_b));
            let t105 = t98 * t104;
            let t107 = (simd::exp(-param_a * t96));
            let t108 = t107 + t102;
            let t109 = (simd::pow(t108, param_b));
            let t110 = f64x8::splat(1.0) / t109;
            let t111 = t105 * t110;
            let t112 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(1.0), t111)));
            let t114 = t86 * t112 + t39;
            let t118 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t114));
            let tzk0 = f64x8::splat(2.0) * t118;
            acc_zk = tzk0;
            let t120 = t21 / t22;
            let t125 = f64x8::splat(1.0) / t23 / t53;
            let t126 = t33 * t125;
            let t127 = t30 * t126;
            let t128 = f64x8::splat(5.0) / f64x8::splat(27.0) * t127;
            let t135 = f64x8::splat(5.0) / f64x8::splat(4374.0) * t50 * t52 * t62;
            let t136 = t31 * t71;
            let t137 = t136 * v_lapl;
            let t139 = f64x8::splat(13.0) / f64x8::splat(7776.0) * t59 * t137;
            let t140 = t60 * t34;
            let t142 = f64x8::splat(1.0) / t22 / t140;
            let t145 = f64x8::splat(4.0) / f64x8::splat(6561.0) * t50 * t68 * t142;
            let t146 = -f64x8::splat(5.0) / f64x8::splat(243.0) * t127 - f64x8::splat(25.0) / f64x8::splat(162.0) * t30 * t41 * t36 - t135 + t139 - t145;
            let t149 = f64x8::splat(1.0) / t83 / t82;
            let t150 = t75 * t149;
            let t151 = t76 * t80;
            let t152 = -t135 + t139 - t145;
            let t155 = t79 * t78;
            let t156 = f64x8::splat(1.0) / t155;
            let t158 = t77 * t156 * t25;
            let t159 = t29 * v_sigma;
            let t160 = t32 * t125;
            let t161 = t159 * t160;
            let t164 = f64x8::splat(2.0) * t151 * t152 + f64x8::splat(10.0) / f64x8::splat(27.0) * t158 * t161;
            let t167 = t146 * t84 - t150 * t164 / f64x8::splat(2.0) + t128;
            let t169 = t95 * t95;
            let t170 = f64x8::splat(1.0) / t169;
            let t171 = t91 * t170;
            let t172 = ((t92).select(f64x8::splat(0.0), t167));
            let t173 = ((t94).select(t172, f64x8::splat(0.0)));
            let t174 = t173 * t98;
            let t175 = t104 * t110;
            let t176 = t174 * t175;
            let t178 = t105 * t91;
            let t179 = t99 * t99;
            let t180 = f64x8::splat(1.0) / t179;
            let t181 = t180 * t173;
            let t182 = f64x8::splat(1.0) / t103;
            let t184 = t102 * t182 * t110;
            let t187 = param_a * t170;
            let t188 = t173 * t107;
            let t190 = param_a * t180;
            let t191 = t173 * t102;
            let t193 = t187 * t188 - t190 * t191;
            let t195 = f64x8::splat(1.0) / t108;
            let t199 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(0.0), -t111 * param_b * t193 * t195 - t178 * t181 * t184 + t171 * t176)));
            let t201 = t167 * t112 + t86 * t199 - t128;
            let t206 = ((t3).select(f64x8::splat(0.0), t8 * t120 * t114 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t201));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t206 + f64x8::splat(2.0) * t118;
            acc_vrho = tvrho0;
            let t209 = t32 * t36;
            let t210 = t30 * t209;
            let t211 = f64x8::splat(5.0) / f64x8::splat(72.0) * t210;
            let t213 = t50 * t64;
            let t214 = t213 / f64x8::splat(2592.0);
            let t215 = v_sigma * t31;
            let t217 = t50 * t215 * t71;
            let t218 = t217 / f64x8::splat(4374.0);
            let t219 = f64x8::splat(5.0) / f64x8::splat(648.0) * t210 - t214 + t218;
            let t221 = -t214 + t218;
            let t224 = t29 * t32;
            let t225 = t224 * t36;
            let t228 = f64x8::splat(2.0) * t151 * t221 - f64x8::splat(5.0) / f64x8::splat(36.0) * t158 * t225;
            let t231 = t219 * t84 - t150 * t228 / f64x8::splat(2.0) - t211;
            let t233 = ((t92).select(f64x8::splat(0.0), t231));
            let t234 = ((t94).select(t233, f64x8::splat(0.0)));
            let t235 = t234 * t98;
            let t236 = t235 * t175;
            let t238 = t180 * t234;
            let t241 = t234 * t107;
            let t243 = t234 * t102;
            let t245 = t187 * t241 - t190 * t243;
            let t246 = param_b * t245;
            let t250 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(0.0), -t111 * t246 * t195 - t178 * t238 * t184 + t171 * t236)));
            let t252 = t231 * t112 + t86 * t250 + t211;
            let t256 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t252));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t256;
            acc_vsigma = tvsigma0;
            let t264 = t50 * v_lapl * t31 * t55 / f64x8::splat(1458.0);
            let t267 = t50 * t215 * t62 / f64x8::splat(2592.0);
            let t268 = f64x8::splat(5.0) / f64x8::splat(54.0) * t30 * t32 * t43 + t264 - t267;
            let t270 = t264 - t267;
            let t271 = t151 * t270;
            let t273 = -t150 * t271 + t268 * t84;
            let t275 = ((t92).select(f64x8::splat(0.0), t273));
            let t276 = ((t94).select(t275, f64x8::splat(0.0)));
            let t277 = t276 * t98;
            let t278 = t277 * t175;
            let t280 = t180 * t276;
            let t283 = t276 * t107;
            let t285 = t276 * t102;
            let t287 = t187 * t283 - t190 * t285;
            let t288 = param_b * t287;
            let t292 = ((t88).select(f64x8::splat(0.0), (t90).select(f64x8::splat(0.0), -t111 * t288 * t195 - t178 * t280 * t184 + t171 * t278)));
            let t294 = t273 * t112 + t86 * t292;
            let t298 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t294));
            let tvlapl0 = f64x8::splat(2.0) * v_rho * t298;
            acc_vlapl = tvlapl0;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau = tvtau0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
