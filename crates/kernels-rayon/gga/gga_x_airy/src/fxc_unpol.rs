//! GGA_X_AIRY fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_airy.c`
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
pub fn gga_x_airy_fxc_unpol(
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
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = t20 * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t21 * t24;
            let t26 = ((v_sigma).sqrt());
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t26 * t27;
            let t30 = f64x8::splat(1.0) / t18 / v_rho;
            let t32 = t25 * t28 * t30;
            let t33 = (simd::pow(t32, f64x8::splat(2.626712)));
            let t35 = f64x8::splat(1.0) + f64x8::splat(0.00013471619689594795) * t33;
            let t36 = (simd::pow(t35, -f64x8::splat(0.657946)));
            let t39 = (simd::pow(t32, f64x8::splat(3.217063)));
            let t41 = (simd::pow(t32, f64x8::splat(3.223476)));
            let t43 = f64x8::splat(1.0) - f64x8::splat(0.04521241301076986) * t39 + f64x8::splat(0.04540222195662038) * t41;
            let t44 = (simd::pow(t32, f64x8::splat(3.473804)));
            let t46 = f64x8::splat(1.0) + f64x8::splat(0.0004770218022490335) * t44;
            let t47 = f64x8::splat(1.0) / t46;
            let t49 = f64x8::splat(6.014601922021111e-05) * t33 * t36 + t43 * t47;
            let t53 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t49));
            let tzk0 = f64x8::splat(2.0) * t53;
            acc_zk = tzk0;
            let t54 = t18 * t18;
            let t56 = t17 / t54;
            let t60 = (simd::pow(t32, f64x8::splat(1.626712)));
            let t62 = t60 * t36 * t21;
            let t63 = t24 * t26;
            let t64 = v_rho * v_rho;
            let t66 = f64x8::splat(1.0) / t18 / t64;
            let t67 = t27 * t66;
            let t68 = t63 * t67;
            let t71 = (simd::pow(t32, f64x8::splat(4.253424)));
            let t72 = (simd::pow(t35, -f64x8::splat(1.657946)));
            let t74 = t71 * t72 * t21;
            let t77 = (simd::pow(t32, f64x8::splat(2.217063)));
            let t79 = t77 * t21 * t24;
            let t80 = t28 * t66;
            let t83 = (simd::pow(t32, f64x8::splat(2.223476)));
            let t85 = t83 * t21 * t24;
            let t88 = f64x8::splat(0.19393490805022173) * t79 * t80 - f64x8::splat(0.19513729709845176) * t85 * t80;
            let t90 = t46 * t46;
            let t91 = f64x8::splat(1.0) / t90;
            let t92 = t43 * t91;
            let t93 = (simd::pow(t32, f64x8::splat(2.473804)));
            let t94 = t93 * t21;
            let t95 = t92 * t94;
            let t98 = -f64x8::splat(0.00021064836058394556) * t62 * t68 + f64x8::splat(1.8671024483029836e-08) * t74 * t68 + t88 * t47 + f64x8::splat(0.0022094403263198687) * t95 * t68;
            let t103 = ((t2).select(f64x8::splat(0.0), -t6 * t56 * t49 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t98));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t103 + f64x8::splat(2.0) * t53;
            acc_vrho = tvrho0;
            let t106 = f64x8::splat(1.0) / t26;
            let t107 = t24 * t106;
            let t108 = t27 * t30;
            let t109 = t107 * t108;
            let t114 = t106 * t27;
            let t115 = t114 * t30;
            let t120 = -f64x8::splat(0.07272559051883315) * t79 * t115 + f64x8::splat(0.07317648641191941) * t85 * t115;
            let t124 = f64x8::splat(7.899313521897959e-05) * t62 * t109 - f64x8::splat(7.001634181136188e-09) * t74 * t109 + t120 * t47 - f64x8::splat(0.0008285401223699508) * t95 * t109;
            let t128 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t124));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t128;
            acc_vsigma = tvsigma0;
            let t133 = t17 / t54 / v_rho;
            let t140 = (simd::pow(t32, f64x8::splat(0.626712)));
            let t142 = t140 * t36 * t20;
            let t143 = t23 * t23;
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t144 * v_sigma;
            let t146 = t27 * t27;
            let t147 = t64 * t64;
            let t149 = f64x8::splat(1.0) / t54 / t147;
            let t150 = t146 * t149;
            let t151 = t145 * t150;
            let t154 = (simd::pow(t32, f64x8::splat(3.253424)));
            let t156 = t154 * t72 * t20;
            let t159 = t64 * v_rho;
            let t161 = f64x8::splat(1.0) / t18 / t159;
            let t162 = t27 * t161;
            let t163 = t63 * t162;
            let t166 = (simd::pow(t32, f64x8::splat(5.880136)));
            let t167 = (simd::pow(t35, -f64x8::splat(2.657946)));
            let t169 = t166 * t167 * t20;
            let t174 = (simd::pow(t32, f64x8::splat(1.217063)));
            let t175 = t174 * t20;
            let t176 = t175 * t144;
            let t177 = v_sigma * t146;
            let t178 = t177 * t149;
            let t181 = t28 * t161;
            let t184 = (simd::pow(t32, f64x8::splat(1.223476)));
            let t185 = t184 * t20;
            let t186 = t185 * t144;
            let t191 = -f64x8::splat(3.4397272723723904) * t176 * t178 - f64x8::splat(0.45251478545051743) * t79 * t181 + f64x8::splat(3.471064774426217) * t186 * t178 + f64x8::splat(0.45532035989638747) * t85 * t181;
            let t193 = t88 * t91;
            let t194 = t193 * t94;
            let t198 = f64x8::splat(1.0) / t90 / t46;
            let t199 = t43 * t198;
            let t200 = (simd::pow(t32, f64x8::splat(4.947608)));
            let t201 = t200 * t20;
            let t202 = t199 * t201;
            let t205 = (simd::pow(t32, f64x8::splat(1.473804)));
            let t206 = t205 * t20;
            let t207 = t92 * t206;
            let t212 = f64x8::splat(0.00274131372753785) * t142 * t151 - f64x8::splat(1.0276735016205997e-06) * t156 * t151 + f64x8::splat(0.0004915128413625396) * t62 * t163 + f64x8::splat(8.763160960794521e-11) * t169 * t151 - f64x8::splat(4.356572379373628e-08) * t74 * t163 + t191 * t47 + f64x8::splat(0.004418880652639737) * t194 * t68 + f64x8::splat(5.8579518666821375e-05) * t202 * t151 - f64x8::splat(0.04372577853609117) * t207 * t151 - f64x8::splat(0.005155360761413027) * t95 * t163;
            let t217 = ((t2).select(f64x8::splat(0.0), t6 * t133 * t49 / f64x8::splat(12.0) - t6 * t56 * t98 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t212));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t217 + f64x8::splat(4.0) * t103;
            acc_v2rho2 = tv2rho20;
            let t223 = t144 * t146;
            let t225 = f64x8::splat(1.0) / t54 / t159;
            let t226 = t223 * t225;
            let t231 = t107 * t67;
            let t240 = t114 * t66;
            let t247 = f64x8::splat(1.2898977271396463) * t175 * t226 + f64x8::splat(0.09696745402511087) * t79 * t240 - f64x8::splat(1.3016492904098316) * t185 * t226 - f64x8::splat(0.09756864854922588) * t85 * t240;
            let t249 = t120 * t91;
            let t250 = t249 * t94;
            let t255 = t199 * t200;
            let t256 = t20 * t144;
            let t257 = t146 * t225;
            let t258 = t256 * t257;
            let t261 = t92 * t205;
            let t266 = -f64x8::splat(0.0010279926478266937) * t142 * t226 + f64x8::splat(3.853775631077249e-07) * t156 * t226 - f64x8::splat(0.00010532418029197278) * t62 * t231 - f64x8::splat(3.2861853602979454e-11) * t169 * t226 + f64x8::splat(9.335512241514918e-09) * t74 * t231 + t247 * t47 + f64x8::splat(0.0022094403263198687) * t250 * t68 - f64x8::splat(0.0008285401223699508) * t194 * t109 - f64x8::splat(2.1967319500058017e-05) * t255 * t258 + f64x8::splat(0.01639716695103419) * t261 * t258 + f64x8::splat(0.0011047201631599344) * t95 * t231;
            let t271 = ((t2).select(f64x8::splat(0.0), -t6 * t56 * t124 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t266));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t271 + f64x8::splat(2.0) * t128;
            acc_v2rhosigma = tv2rhosigma0;
            let t274 = f64x8::splat(1.0) / v_sigma;
            let t275 = t144 * t274;
            let t277 = f64x8::splat(1.0) / t54 / t64;
            let t278 = t146 * t277;
            let t279 = t275 * t278;
            let t284 = t26 * v_sigma;
            let t285 = f64x8::splat(1.0) / t284;
            let t286 = t24 * t285;
            let t287 = t286 * t108;
            let t294 = t274 * t146;
            let t295 = t294 * t277;
            let t298 = t285 * t27;
            let t299 = t298 * t30;
            let t306 = -f64x8::splat(0.48371164767736735) * t176 * t295 + f64x8::splat(0.036362795259416575) * t79 * t299 + f64x8::splat(0.4881184839036868) * t186 * t295 - f64x8::splat(0.03658824320595971) * t85 * t299;
            let t316 = f64x8::splat(0.00038549724293501016) * t142 * t279 - f64x8::splat(1.4451658616539682e-07) * t156 * t279 - f64x8::splat(3.9496567609489795e-05) * t62 * t287 + f64x8::splat(1.2323195101117295e-11) * t169 * t279 + f64x8::splat(3.500817090568094e-09) * t74 * t287 + t306 * t47 - f64x8::splat(0.0016570802447399015) * t250 * t109 + f64x8::splat(8.237744812521756e-06) * t202 * t279 - f64x8::splat(0.006148937606637821) * t207 * t279 + f64x8::splat(0.0004142700611849754) * t95 * t287;
            let t320 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t316));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t320;
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
