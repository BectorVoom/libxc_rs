//! GGA_X_PBE kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbe.c`
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
pub fn gga_x_pbe_kxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_kappa = f64x8::splat(param_kappa);
    let param_mu = f64x8::splat(param_mu);
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
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
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
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t30 = v_rho * v_rho;
            let t31 = t18 * t18;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t37 = param_kappa + param_mu * t20 * t25 * v_sigma * t28 * t33 / f64x8::splat(24.0);
            let t42 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t37);
            let t46 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t17 * t18 * t42));
            let tzk0 = f64x8::splat(2.0) * t46;
            acc_zk = tzk0;
            let t52 = t30 * v_rho;
            let t56 = param_kappa * param_kappa;
            let t58 = t6 * t17 / t18 / t52 * t56;
            let t59 = t37 * t37;
            let t61 = f64x8::splat(1.0) / t59 * param_mu;
            let t64 = t25 * v_sigma * t28;
            let t65 = t61 * t20 * t64;
            let t69 = ((t2).select(f64x8::splat(0.0), -t6 * t17 / t31 * t42 / f64x8::splat(8.0) + t58 * t65 / f64x8::splat(24.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t69 + f64x8::splat(2.0) * t46;
            acc_vrho = tvrho0;
            let t78 = t20 * t25 * t28;
            let t79 = t61 * t78;
            let t82 = ((t2).select(f64x8::splat(0.0), -t6 * t17 / t18 / t30 * t56 * t79 / f64x8::splat(64.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t82;
            acc_vsigma = tvsigma0;
            let t91 = t30 * t30;
            let t96 = t6 * t17 / t18 / t91 * t56;
            let t99 = t91 * t52;
            let t103 = t6 * t17 / t99 * t56;
            let t106 = param_mu * param_mu;
            let t107 = f64x8::splat(1.0) / t59 / t37 * t106;
            let t108 = t20 * t20;
            let t109 = t107 * t108;
            let t111 = f64x8::splat(1.0) / t23 / t22;
            let t112 = v_sigma * v_sigma;
            let t115 = t109 * t111 * t112 * t27;
            let t119 = ((t2).select(f64x8::splat(0.0), t6 * t17 / t31 / v_rho * t42 / f64x8::splat(12.0) - t96 * t65 / f64x8::splat(8.0) + t103 * t115 / f64x8::splat(54.0)));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t119 + f64x8::splat(4.0) * t69;
            acc_v2rho2 = tv2rho20;
            let t124 = t91 * t30;
            let t128 = t6 * t17 / t124 * t56;
            let t131 = t109 * t111 * t27 * v_sigma;
            let t135 = ((t2).select(f64x8::splat(0.0), f64x8::splat(7.0) / f64x8::splat(192.0) * t58 * t79 - t128 * t131 / f64x8::splat(144.0)));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t135 + f64x8::splat(2.0) * t82;
            acc_v2rhosigma = tv2rhosigma0;
            let t138 = t91 * v_rho;
            let t145 = t107 * t108 * t111 * t27;
            let t148 = ((t2).select(f64x8::splat(0.0), t6 * t17 / t138 * t56 * t145 / f64x8::splat(384.0)));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t148;
            acc_v2sigma2 = tv2sigma20;
            let t159 = t6 * t17 / t18 / t138 * t56;
            let t162 = t91 * t91;
            let t166 = t6 * t17 / t162 * t56;
            let t169 = t22 * t22;
            let t172 = t3 / t4 / t169;
            let t173 = t162 * t30;
            let t177 = t172 * t17 / t31 / t173;
            let t178 = t59 * t59;
            let t179 = f64x8::splat(1.0) / t178;
            let t180 = t56 * t179;
            let t181 = t106 * param_mu;
            let t182 = t112 * v_sigma;
            let t184 = t180 * t181 * t182;
            let t188 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t17 * t33 * t42 + f64x8::splat(115.0) / f64x8::splat(216.0) * t159 * t65 - f64x8::splat(5.0) / f64x8::splat(27.0) * t166 * t115 + f64x8::splat(2.0) / f64x8::splat(27.0) * t177 * t184));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t188 + f64x8::splat(6.0) * t119;
            acc_v3rho3 = tv3rho30;
            let t196 = t162 * v_rho;
            let t200 = t172 * t17 / t31 / t196;
            let t202 = t180 * t181 * t112;
            let t206 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(35.0) / f64x8::splat(288.0) * t96 * t79 + f64x8::splat(25.0) / f64x8::splat(432.0) * t103 * t131 - t200 * t202 / f64x8::splat(36.0)));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t206 + f64x8::splat(4.0) * t135;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t212 = f64x8::splat(1.0) / t31 / t162;
            let t216 = t180 * t181 * v_sigma;
            let t220 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(384.0) * t128 * t145 + t172 * t17 * t212 * t216 / f64x8::splat(96.0)));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t220 + f64x8::splat(2.0) * t148;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t223 = t172 * t17;
            let t227 = t179 * t181;
            let t231 = ((t2).select(f64x8::splat(0.0), -t223 / t31 / t99 * t56 * t227 / f64x8::splat(256.0)));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t231;
            acc_v3sigma3 = tv3sigma30;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho2sigma.into(); v3rho2sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rhosigma2.into(); v3rhosigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3sigma3.into(); v3sigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
