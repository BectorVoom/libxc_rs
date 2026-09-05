//! GGA_C_REVTCA kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_revtca.c`
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
pub fn gga_c_revtca_kxc_unpol(
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
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
        {
            let t2 = (simd::cbrt(zeta_threshold));
            let t3 = t2 * t2;
            let t4 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t3, f64x8::splat(1.0)));
            let t5 = t4 * t4;
            let t6 = t5 * t4;
            let t7 = f64x8::splat(M_CBRT3);
            let t9 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t10 = t7 * t9;
            let t11 = f64x8::splat(M_CBRT4);
            let t12 = t11 * t11;
            let t13 = (simd::cbrt(v_rho));
            let t18 = f64x8::splat(4.88827) + f64x8::splat(0.79425925) * t10 * t12 / t13;
            let t19 = (simd::atan(t18));
            let t21 = -f64x8::splat(0.655868) * t19 + f64x8::splat(0.897889);
            let t22 = t6 * t21;
            let t23 = t7 * t7;
            let t24 = t22 * t23;
            let t25 = f64x8::splat(1.0) / t9;
            let t26 = t25 * t11;
            let t27 = f64x8::splat(M_CBRT6);
            let t28 = t27 * t27;
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = t28 * t31;
            let t33 = f64x8::splat(M_CBRT2);
            let t34 = ((v_sigma).sqrt());
            let t35 = t33 * t34;
            let t37 = f64x8::splat(1.0) / t13 / v_rho;
            let t39 = t32 * t35 * t37;
            let t40 = (simd::pow(t39, f64x8::splat(2.3)));
            let t42 = f64x8::splat(1.0) + f64x8::splat(0.004712150703442276) * t40;
            let t43 = f64x8::splat(1.0) / t42;
            let t46 = t24 * t26 * t13 * t43;
            let tzk0 = t46 / f64x8::splat(3.0);
            acc_zk = tzk0;
            let t48 = t18 * t18;
            let t49 = t48 + f64x8::splat(1.0);
            let t50 = f64x8::splat(1.0) / t49;
            let t51 = t6 * t50;
            let t55 = f64x8::splat(1.0) / v_rho * t6;
            let t57 = t23 * t25;
            let t58 = t57 * t11;
            let t60 = t42 * t42;
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = (simd::pow(t39, f64x8::splat(1.3)));
            let t63 = t61 * t62;
            let t64 = t63 * t28;
            let t65 = t31 * t33;
            let t66 = t65 * t34;
            let t67 = t64 * t66;
            let tvrho0 = f64x8::splat(4.0) / f64x8::splat(9.0) * t46 + f64x8::splat(0.6945723010386666) * t51 * t43 + f64x8::splat(0.004816865163518771) * t55 * t21 * t58 * t67;
            acc_vrho = tvrho0;
            let t70 = t22 * t58;
            let t71 = f64x8::splat(1.0) / t34;
            let t72 = t65 * t71;
            let tvsigma0 = -f64x8::splat(0.001806324436319539) * t70 * t64 * t72;
            acc_vsigma = tvsigma0;
            let t76 = t50 * t43;
            let t79 = t13 * t13;
            let t85 = v_rho * v_rho;
            let t86 = f64x8::splat(1.0) / t85;
            let t91 = t49 * t49;
            let t92 = f64x8::splat(1.0) / t91;
            let t93 = t6 * t92;
            let t94 = t43 * t18;
            let t102 = f64x8::splat(1.0) / t13 / t85;
            let t107 = t85 * v_rho;
            let t109 = f64x8::splat(1.0) / t13 / t107;
            let t110 = t109 * t6;
            let t112 = t110 * t21 * t58;
            let t114 = f64x8::splat(1.0) / t60 / t42;
            let t115 = (simd::pow(t39, f64x8::splat(2.6)));
            let t116 = t114 * t115;
            let t117 = t116 * t27;
            let t118 = t30 * t30;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = t33 * t33;
            let t121 = t119 * t120;
            let t122 = t121 * v_sigma;
            let t123 = t117 * t122;
            let t126 = (simd::pow(t39, f64x8::splat(0.3)));
            let t127 = t61 * t126;
            let t128 = t127 * t27;
            let t129 = t128 * t122;
            let tv2rho20 = f64x8::splat(0.9260964013848889) * t55 * t76 + f64x8::splat(4.0) / f64x8::splat(27.0) * t24 * t26 / t79 * t43 + f64x8::splat(0.0016056217211729237) * t24 * t26 * t86 * t67 + f64x8::splat(0.3677803165958304) * t93 * t94 * t10 * t12 * t37 + f64x8::splat(0.020073966722509357) * t51 * t63 * t32 * t35 * t102 + f64x8::splat(0.0008352788401267458) * t112 * t123 - f64x8::splat(0.05009539770059522) * t112 * t129;
            acc_v2rho2 = tv2rho20;
            let t133 = t50 * t61;
            let t134 = t6 * t37 * t133;
            let t135 = t62 * t28;
            let t136 = t135 * t72;
            let t139 = t121 * t102;
            let tv2rhosigma0 = -f64x8::splat(0.0037638687604705044) * t134 * t136 - f64x8::splat(0.0003132295650475297) * t70 * t117 * t139 + f64x8::splat(0.018785774137723206) * t70 * t128 * t139;
            acc_v2rhosigma = tv2rhosigma0;
            let t147 = t24 * t26 * t114;
            let t148 = t115 * t27;
            let t149 = t148 * t119;
            let t150 = f64x8::splat(1.0) / v_sigma;
            let t151 = t120 * t150;
            let t152 = t151 * t37;
            let t157 = t24 * t26 * t61;
            let t158 = t126 * t27;
            let t159 = t158 * t119;
            let t163 = t34 * v_sigma;
            let t164 = f64x8::splat(1.0) / t163;
            let t165 = t65 * t164;
            let tv2sigma20 = f64x8::splat(0.00011746108689282363) * t147 * t149 * t152 - f64x8::splat(0.007044665301646202) * t157 * t159 * t152 + f64x8::splat(0.0009031622181597695) * t70 * t64 * t165;
            acc_v2sigma2 = tv2sigma20;
            let t169 = t6 * t102;
            let t170 = t92 * t43;
            let t172 = t18 * t7;
            let t173 = t9 * t12;
            let t174 = t172 * t173;
            let t184 = f64x8::splat(1.0) / t91 / t49;
            let t185 = t6 * t184;
            let t188 = t9 * t9;
            let t189 = t23 * t188;
            let t191 = f64x8::splat(1.0) / t79 / t85;
            let t193 = t189 * t11 * t191;
            let t199 = t85 * t85;
            let t200 = t199 * v_rho;
            let t202 = f64x8::splat(1.0) / t79 / t200;
            let t203 = t202 * t6;
            let t204 = t21 * t23;
            let t205 = t204 * t25;
            let t206 = t203 * t205;
            let t207 = t11 * t114;
            let t208 = (simd::pow(t39, f64x8::splat(1.6)));
            let t209 = f64x8::splat(1.0) / t29;
            let t210 = t208 * t209;
            let t212 = t207 * t210 * t163;
            let t216 = (simd::pow(t39, -f64x8::splat(0.7)));
            let t217 = t216 * t209;
            let t219 = t11 * t61 * t217 * t163;
            let t225 = t110 * t133;
            let t226 = t135 * t66;
            let t231 = t120 * v_sigma;
            let t233 = f64x8::splat(1.0) / t79 / t199;
            let t235 = t27 * t119 * t231 * t233;
            let t241 = t203 * t204;
            let t242 = t60 * t60;
            let t243 = f64x8::splat(1.0) / t242;
            let t244 = (simd::pow(t39, f64x8::splat(3.9)));
            let t245 = t243 * t244;
            let t247 = t26 * t245 * t163;
            let t250 = t114 * t208;
            let t251 = t250 * t163;
            let t252 = t26 * t251;
            let t255 = t93 * t61;
            let t259 = f64x8::splat(1.0) / t79 / t107;
            let t262 = t32 * t35;
            let t266 = f64x8::splat(1.0) / t107;
            let t272 = f64x8::splat(1.0) / t13 / t199;
            let t274 = t24 * t26 * t272;
            let tv3rho30 = -f64x8::splat(1e-20) * t169 * t170 * t174 - f64x8::splat(8.0) / f64x8::splat(81.0) * t24 * t26 / t79 / v_rho * t43 + f64x8::splat(1.5579355649288897) * t185 * t43 * t48 * t193 - f64x8::splat(0.6173976009232592) * t6 * t86 * t76 - f64x8::splat(0.03474759974927263) * t206 * t212 + f64x8::splat(0.24045790896285704) * t206 * t219 - f64x8::splat(0.38948389123222243) * t93 * t43 * t193 - f64x8::splat(0.030110950083764035) * t225 * t226 + f64x8::splat(0.0052214539139616815) * t51 * t116 * t235 - f64x8::splat(0.313153880871146) * t51 * t127 * t235 + f64x8::splat(4.402708977978636e-05) * t241 * t247 - f64x8::splat(0.0017603339676632507) * t241 * t252 + f64x8::splat(0.01594393375354524) * t255 * t172 * t9 * t12 * t259 * t62 * t262 - f64x8::splat(0.0010704144807819492) * t24 * t26 * t266 * t67 - f64x8::splat(0.0025058365203802376) * t274 * t123 + f64x8::splat(0.15028619310178565) * t274 * t129;
            acc_v3rho3 = tv3rho30;
            let t279 = t169 * t133;
            let t282 = t6 * t191;
            let t284 = t282 * t92 * t64;
            let t285 = t72 * t174;
            let t288 = t6 * t259;
            let t289 = t50 * t114;
            let t290 = t288 * t289;
            let t291 = t148 * t121;
            let t294 = t288 * t133;
            let t295 = t158 * t121;
            let t298 = t22 * t57;
            let t299 = t11 * t243;
            let t306 = t209 * t233 * t34;
            let t310 = t121 * t109;
            let t319 = t61 * t216;
            let tv3rho2sigma0 = f64x8::splat(0.005018491680627339) * t279 * t136 - f64x8::splat(0.001992991719193155) * t284 * t285 - f64x8::splat(0.0013053634784904204) * t290 * t291 + f64x8::splat(0.0782884702177865) * t294 * t295 - f64x8::splat(1.6510158667419884e-05) * t298 * t299 * t244 * t233 * t34 + f64x8::splat(0.013030349905977234) * t70 * t250 * t306 + f64x8::splat(0.0007308689851109025) * t70 * t117 * t310 + f64x8::splat(0.000660125237873719) * t298 * t207 * t208 * t233 * t34 - f64x8::splat(0.0901717158610714) * t70 * t319 * t306 - f64x8::splat(0.043833472988020816) * t70 * t128 * t310;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t326 = t282 * t289;
            let t327 = t121 * t150;
            let t328 = t148 * t327;
            let t331 = t244 * t71;
            let t336 = t209 * t71;
            let t337 = t336 * t259;
            let t345 = t282 * t133;
            let t346 = t158 * t327;
            let t349 = t208 * t71;
            let t357 = t135 * t165;
            let tv3rhosigma20 = f64x8::splat(0.0002447556522169538) * t326 * t328 + f64x8::splat(6.191309500282457e-06) * t298 * t299 * t331 * t259 - f64x8::splat(0.004886381214741463) * t70 * t250 * t337 + f64x8::splat(1e-23) * t147 * t149 * t151 * t102 - f64x8::splat(0.014679088165834967) * t345 * t346 - f64x8::splat(0.00024754696420264467) * t298 * t207 * t349 * t259 + f64x8::splat(0.03381439344790177) * t70 * t319 * t337 + f64x8::splat(0.0018819343802352522) * t134 * t357;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t360 = t244 * t164;
            let t365 = t209 * t164;
            let t366 = t365 * t191;
            let t370 = v_sigma * v_sigma;
            let t371 = f64x8::splat(1.0) / t370;
            let t372 = t120 * t371;
            let t373 = t372 * t37;
            let t377 = t208 * t164;
            let t389 = f64x8::splat(1.0) / t34 / t370;
            let t390 = t65 * t389;
            let tv3sigma30 = -f64x8::splat(2.3217410626059214e-06) * t298 * t299 * t360 * t191 + f64x8::splat(0.0018323929555280486) * t70 * t250 * t366 - f64x8::splat(0.00017619163033923545) * t147 * t149 * t373 + f64x8::splat(9.283011157599174e-05) * t298 * t207 * t377 * t191 - f64x8::splat(0.012680397542963165) * t70 * t319 * t366 + f64x8::splat(0.010566997952469305) * t157 * t159 * t373 - f64x8::splat(0.0013547433272396543) * t70 * t64 * t390;
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
