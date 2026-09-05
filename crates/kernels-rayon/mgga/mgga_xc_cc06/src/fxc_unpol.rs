//! MGGA_XC_CC06 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_cc06.c`
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
pub fn mgga_xc_cc06_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
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
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t9 = (simd::cbrt(zeta_threshold));
            let t11 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t9 * zeta_threshold, f64x8::splat(1.0)));
            let t12 = (simd::cbrt(v_rho));
            let t16 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t11 * t12));
            let t18 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t19 = (simd::cbrt(t18));
            let t20 = t4 * t19;
            let t21 = f64x8::splat(M_CBRT4);
            let t22 = t21 * t21;
            let t25 = t20 * t22 / t12;
            let t27 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t25;
            let t28 = ((t25).sqrt());
            let t31 = ((t25) * (t25).sqrt());
            let t33 = t4 * t4;
            let t34 = t19 * t19;
            let t35 = t33 * t34;
            let t36 = t12 * t12;
            let t37 = f64x8::splat(1.0) / t36;
            let t39 = t35 * t21 * t37;
            let t41 = f64x8::splat(3.79785) * t28 + f64x8::splat(0.8969) * t25 + f64x8::splat(0.204775) * t31 + f64x8::splat(0.123235) * t39;
            let t44 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t41;
            let t45 = (simd::ln(t44));
            let t50 = f64x8::splat(M_CBRT2);
            let t54 = (f64x8::splat(2.0) * t11 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t50 - f64x8::splat(2.0));
            let t56 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t25;
            let t61 = f64x8::splat(5.1785) * t28 + f64x8::splat(0.905775) * t25 + f64x8::splat(0.1100325) * t31 + f64x8::splat(0.1241775) * t39;
            let t64 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t61;
            let t65 = (simd::ln(t64));
            let t69 = f64x8::splat(2.0) * t16 - f64x8::splat(0.062182) * t27 * t45 + f64x8::splat(0.019751789702565206) * t54 * t56 * t65;
            let t70 = t33 * t21;
            let t71 = t34 * v_lapl;
            let t73 = f64x8::splat(1.0) / t36 / v_rho;
            let t75 = t70 * t71 * t73;
            let t77 = -f64x8::splat(0.0007) + f64x8::splat(0.002) * t75;
            let t79 = f64x8::splat(1.0) + f64x8::splat(0.0065) * t75;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = t77 * t80 + f64x8::splat(1.0);
            let tzk0 = t69 * t82;
            acc_zk = tzk0;
            let t86 = ((t3).select(f64x8::splat(0.0), -t7 * t11 * t37 / f64x8::splat(8.0)));
            let t89 = f64x8::splat(1.0) / t12 / v_rho;
            let t90 = t22 * t89;
            let t94 = t41 * t41;
            let t95 = f64x8::splat(1.0) / t94;
            let t96 = t27 * t95;
            let t98 = f64x8::splat(1.0) / t28 * t4;
            let t99 = t19 * t22;
            let t100 = t99 * t89;
            let t101 = t98 * t100;
            let t103 = t20 * t90;
            let t105 = ((t25).sqrt());
            let t106 = t105 * t4;
            let t107 = t106 * t100;
            let t109 = t21 * t73;
            let t110 = t35 * t109;
            let t112 = -f64x8::splat(0.632975) * t101 - f64x8::splat(0.29896666666666666) * t103 - f64x8::splat(0.1023875) * t107 - f64x8::splat(0.08215666666666667) * t110;
            let t113 = f64x8::splat(1.0) / t44;
            let t114 = t112 * t113;
            let t117 = t54 * t4;
            let t122 = t54 * t56;
            let t123 = t61 * t61;
            let t124 = f64x8::splat(1.0) / t123;
            let t129 = -f64x8::splat(0.8630833333333333) * t101 - f64x8::splat(0.301925) * t103 - f64x8::splat(0.05501625) * t107 - f64x8::splat(0.082785) * t110;
            let t131 = f64x8::splat(1.0) / t64;
            let t132 = t124 * t129 * t131;
            let t135 = f64x8::splat(2.0) * t86 + f64x8::splat(0.0011073577833333333) * t20 * t90 * t45 + f64x8::splat(1.0) * t96 * t114 - f64x8::splat(0.0001831155503675316) * t117 * t99 * t89 * t65 - f64x8::splat(0.5848223397455204) * t122 * t132;
            let t136 = v_rho * t135;
            let t138 = v_rho * t69;
            let t139 = t70 * t34;
            let t140 = v_rho * v_rho;
            let t142 = f64x8::splat(1.0) / t36 / t140;
            let t143 = v_lapl * t142;
            let t147 = t79 * t79;
            let t148 = f64x8::splat(1.0) / t147;
            let t150 = t77 * t148 * t33;
            let t151 = t21 * t34;
            let t155 = -f64x8::splat(0.0033333333333333335) * t139 * t143 * t80 + f64x8::splat(0.010833333333333334) * t150 * t151 * t143;
            let tvrho0 = t136 * t82 + t138 * t155 + tzk0;
            acc_vrho = tvrho0;
            let tvsigma0 = f64x8::splat(0.0);
            acc_vsigma = tvsigma0;
            let t163 = f64x8::splat(0.002) * t35 * t109 * t80 - f64x8::splat(0.0065) * t150 * t151 * t73;
            let tvlapl0 = t138 * t163;
            acc_vlapl = tvlapl0;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau = tvtau0;
            let t171 = ((t3).select(f64x8::splat(0.0), t7 * t11 * t73 / f64x8::splat(12.0)));
            let t174 = f64x8::splat(1.0) / t12 / t140;
            let t175 = t22 * t174;
            let t179 = t20 * t22;
            let t180 = t89 * t95;
            let t184 = t94 * t41;
            let t185 = f64x8::splat(1.0) / t184;
            let t186 = t27 * t185;
            let t187 = t112 * t112;
            let t188 = t187 * t113;
            let t193 = f64x8::splat(1.0) / t28 / t25 * t33;
            let t194 = t151 * t142;
            let t195 = t193 * t194;
            let t197 = t99 * t174;
            let t198 = t98 * t197;
            let t200 = t20 * t175;
            let t202 = f64x8::splat(1.0)/((t25).sqrt());
            let t203 = t202 * t33;
            let t204 = t203 * t194;
            let t206 = t106 * t197;
            let t208 = t21 * t142;
            let t209 = t35 * t208;
            let t211 = -f64x8::splat(0.4219833333333333) * t195 + f64x8::splat(0.8439666666666666) * t198 + f64x8::splat(0.3986222222222222) * t200 + f64x8::splat(0.06825833333333334) * t204 + f64x8::splat(0.13651666666666668) * t206 + f64x8::splat(0.1369277777777778) * t209;
            let t212 = t211 * t113;
            let t215 = t94 * t94;
            let t216 = f64x8::splat(1.0) / t215;
            let t217 = t27 * t216;
            let t218 = t44 * t44;
            let t219 = f64x8::splat(1.0) / t218;
            let t220 = t187 * t219;
            let t227 = t54 * t20;
            let t231 = t123 * t61;
            let t232 = f64x8::splat(1.0) / t231;
            let t233 = t129 * t129;
            let t235 = t232 * t233 * t131;
            let t244 = -f64x8::splat(0.5753888888888888) * t195 + f64x8::splat(1.1507777777777777) * t198 + f64x8::splat(0.4025666666666667) * t200 + f64x8::splat(0.0366775) * t204 + f64x8::splat(0.073355) * t206 + f64x8::splat(0.137975) * t209;
            let t246 = t124 * t244 * t131;
            let t249 = t123 * t123;
            let t250 = f64x8::splat(1.0) / t249;
            let t251 = t250 * t233;
            let t252 = t64 * t64;
            let t253 = f64x8::splat(1.0) / t252;
            let t254 = t251 * t253;
            let t257 = f64x8::splat(2.0) * t171 - f64x8::splat(0.0014764770444444443) * t20 * t175 * t45 - f64x8::splat(0.035616666666666665) * t179 * t180 * t114 - f64x8::splat(2.0) * t186 * t188 + f64x8::splat(1.0) * t96 * t212 + f64x8::splat(16.081824322151103) * t217 * t220 + f64x8::splat(0.0002441540671567088) * t117 * t99 * t174 * t65 + f64x8::splat(0.010843580882781523) * t227 * t90 * t132 + f64x8::splat(1.169644679491041) * t122 * t235 - f64x8::splat(0.5848223397455204) * t122 * t246 - f64x8::splat(17.315755899375862) * t122 * t254;
            let t258 = v_rho * t257;
            let t262 = t140 * v_rho;
            let t264 = f64x8::splat(1.0) / t36 / t262;
            let t265 = v_lapl * t264;
            let t270 = t19 * t18;
            let t271 = t4 * t22 * t270;
            let t272 = v_lapl * v_lapl;
            let t273 = t140 * t140;
            let t274 = t273 * v_rho;
            let t276 = f64x8::splat(1.0) / t12 / t274;
            let t277 = t272 * t276;
            let t282 = f64x8::splat(1.0) / t147 / t79;
            let t284 = t77 * t282 * t4;
            let t285 = t22 * t270;
            let t292 = f64x8::splat(0.008888888888888889) * t139 * t265 * t80 - f64x8::splat(0.00021666666666666666) * t271 * t277 * t148 + f64x8::splat(0.0007041666666666666) * t284 * t285 * t277 - f64x8::splat(0.028888888888888888) * t150 * t151 * t265;
            let tv2rho20 = f64x8::splat(2.0) * t135 * t82 + f64x8::splat(2.0) * t136 * t155 + t138 * t292 + f64x8::splat(2.0) * t69 * t155 + t258 * t82;
            acc_v2rho2 = tv2rho20;
            let tv2rhosigma0 = f64x8::splat(0.0);
            acc_v2rhosigma = tv2rhosigma0;
            let t300 = f64x8::splat(1.0) / t12 / t273;
            let t311 = -f64x8::splat(0.0033333333333333335) * t35 * t208 * t80 + f64x8::splat(0.00013) * t271 * t300 * t148 * v_lapl - f64x8::splat(0.0004225) * t284 * t285 * t300 * v_lapl + f64x8::splat(0.010833333333333334) * t150 * t194;
            let tv2rholapl0 = t136 * t163 + t138 * t311 + t69 * t163;
            acc_v2rholapl = tv2rholapl0;
            let tv2rhotau0 = f64x8::splat(0.0);
            acc_v2rhotau = tv2rhotau0;
            let tv2sigma20 = f64x8::splat(0.0);
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let tv2sigmatau0 = f64x8::splat(0.0);
            acc_v2sigmatau = tv2sigmatau0;
            let t313 = t4 * t270;
            let t315 = f64x8::splat(1.0) / t12 / t262;
            let t316 = t22 * t315;
            let t323 = -f64x8::splat(7.8e-05) * t313 * t316 * t148 + f64x8::splat(0.0002535) * t284 * t285 * t315;
            let tv2lapl20 = t138 * t323;
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let tv2tau20 = f64x8::splat(0.0);
            acc_v2tau2 = tv2tau20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rholapl.into(); v2rholapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhotau.into(); v2rhotau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigmalapl.into(); v2sigmalapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigmatau.into(); v2sigmatau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2lapl2.into(); v2lapl2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2lapltau.into(); v2lapltau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2tau2.into(); v2tau2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
