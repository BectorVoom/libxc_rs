//! GGA_X_SSB_SW fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ssb_sw.c`
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
pub fn gga_x_ssb_sw_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_A = f64x8::splat(param_A);
    let param_B = f64x8::splat(param_B);
    let param_C = f64x8::splat(param_C);
    let param_D = f64x8::splat(param_D);
    let param_E = f64x8::splat(param_E);
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
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = param_B * t20 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t18 * t18;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t39 = f64x8::splat(1.0) + param_C * t20 * t25 * t29 * t33 / f64x8::splat(24.0);
            let t40 = f64x8::splat(1.0) / t39;
            let t46 = param_D * t20 * t25;
            let t47 = t20 * t20;
            let t50 = f64x8::splat(1.0) / t23 / t22;
            let t52 = v_sigma * v_sigma;
            let t54 = t30 * t30;
            let t55 = t54 * v_rho;
            let t57 = f64x8::splat(1.0) / t18 / t55;
            let t61 = f64x8::splat(1.0) + param_E * t47 * t50 * t52 * t27 * t57 / f64x8::splat(288.0);
            let t62 = f64x8::splat(1.0) / t61;
            let t67 = param_A + t26 * t29 * t33 * t40 / f64x8::splat(24.0) - t46 * t29 * t33 * t62 / f64x8::splat(24.0);
            let t71 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t67));
            let tzk0 = f64x8::splat(2.0) * t71;
            acc_zk = tzk0;
            let t73 = t17 / t31;
            let t77 = t30 * v_rho;
            let t79 = f64x8::splat(1.0) / t31 / t77;
            let t84 = param_B * t47;
            let t86 = t84 * t50 * t52;
            let t87 = t54 * t30;
            let t89 = f64x8::splat(1.0) / t18 / t87;
            let t91 = t39 * t39;
            let t92 = f64x8::splat(1.0) / t91;
            let t93 = t92 * param_C;
            let t94 = t27 * t89 * t93;
            let t101 = t22 * t22;
            let t102 = f64x8::splat(1.0) / t101;
            let t103 = param_D * t102;
            let t104 = t52 * v_sigma;
            let t105 = t103 * t104;
            let t106 = t54 * t54;
            let t107 = t106 * v_rho;
            let t108 = f64x8::splat(1.0) / t107;
            let t109 = t61 * t61;
            let t110 = f64x8::splat(1.0) / t109;
            let t112 = t108 * t110 * param_E;
            let t115 = -t26 * t29 * t79 * t40 / f64x8::splat(9.0) + t86 * t94 / f64x8::splat(108.0) + t46 * t29 * t79 * t62 / f64x8::splat(9.0) - t105 * t112 / f64x8::splat(108.0);
            let t120 = ((t2).select(f64x8::splat(0.0), -t6 * t73 * t67 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t115));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t120 + f64x8::splat(2.0) * t71;
            acc_vrho = tvrho0;
            let t123 = t28 * t33;
            let t130 = t27 * t57 * t93;
            let t137 = f64x8::splat(1.0) / t106;
            let t139 = t137 * t110 * param_E;
            let t142 = t26 * t123 * t40 / f64x8::splat(24.0) - t84 * t50 * v_sigma * t130 / f64x8::splat(288.0) - t46 * t123 * t62 / f64x8::splat(24.0) + t103 * t52 * t139 / f64x8::splat(288.0);
            let t146 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t142));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t146;
            acc_vsigma = tvsigma0;
            let t151 = t17 / t31 / v_rho;
            let t159 = f64x8::splat(1.0) / t31 / t54;
            let t164 = t54 * t77;
            let t166 = f64x8::splat(1.0) / t18 / t164;
            let t168 = t27 * t166 * t93;
            let t171 = param_B * t102;
            let t172 = t171 * t104;
            let t173 = t106 * t30;
            let t174 = f64x8::splat(1.0) / t173;
            let t176 = f64x8::splat(1.0) / t91 / t39;
            let t178 = param_C * param_C;
            let t190 = t52 * t52;
            let t191 = t190 * v_sigma;
            let t194 = f64x8::splat(1.0) / t18 / t106 / t164;
            let t198 = f64x8::splat(1.0) / t109 / t61;
            let t199 = param_E * param_E;
            let t202 = t47 * t50 * t27;
            let t203 = t198 * t199 * t202;
            let t206 = f64x8::splat(11.0) / f64x8::splat(27.0) * t26 * t29 * t159 * t40 - t86 * t168 / f64x8::splat(12.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t172 * t174 * t176 * t178 - f64x8::splat(11.0) / f64x8::splat(27.0) * t46 * t29 * t159 * t62 + f64x8::splat(35.0) / f64x8::splat(324.0) * t105 * t174 * t110 * param_E - t103 * t191 * t194 * t203 / f64x8::splat(2916.0);
            let t211 = ((t2).select(f64x8::splat(0.0), t6 * t151 * t67 / f64x8::splat(12.0) - t6 * t73 * t115 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t206));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t211 + f64x8::splat(4.0) * t120;
            acc_v2rho2 = tv2rho20;
            let t217 = t28 * t79;
            let t222 = t84 * t50 * t27;
            let t224 = param_C * v_sigma;
            let t230 = t108 * t176 * t178;
            let t236 = t103 * t108;
            let t237 = t110 * param_E;
            let t238 = t237 * t52;
            let t241 = t106 * t87;
            let t243 = f64x8::splat(1.0) / t18 / t241;
            let t248 = -t26 * t217 * t40 / f64x8::splat(9.0) + t222 * t89 * t92 * t224 / f64x8::splat(36.0) - t171 * t52 * t230 / f64x8::splat(108.0) + t46 * t217 * t62 / f64x8::splat(9.0) - t236 * t238 / f64x8::splat(27.0) + t103 * t190 * t243 * t203 / f64x8::splat(7776.0);
            let t253 = ((t2).select(f64x8::splat(0.0), -t6 * t73 * t142 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t248));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t253 + f64x8::splat(2.0) * t146;
            acc_v2rhosigma = tv2rhosigma0;
            let t256 = t84 * t50;
            let t261 = t137 * t176 * t178;
            let t265 = t237 * v_sigma;
            let t268 = t106 * t55;
            let t270 = f64x8::splat(1.0) / t18 / t268;
            let t275 = -t256 * t130 / f64x8::splat(144.0) + t171 * v_sigma * t261 / f64x8::splat(288.0) + t103 * t137 * t265 / f64x8::splat(96.0) - t103 * t104 * t270 * t203 / f64x8::splat(20736.0);
            let t279 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t275));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t279;
            acc_v2sigma2 = tv2sigma20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
