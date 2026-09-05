//! GGA_K_EXP4 lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_exp4.c`
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
pub fn gga_k_exp4_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
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
        let mut acc_v4rho4 = V_ZERO;
        let mut acc_v4rho3sigma = V_ZERO;
        let mut acc_v4rho2sigma2 = V_ZERO;
        let mut acc_v4rhosigma3 = V_ZERO;
        let mut acc_v4sigma4 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t3 * t3;
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 * t5 * f64x8::splat(M_PI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t15 = t14 * t14;
            let t17 = (simd::cbrt(t12));
            let t18 = t17 * t17;
            let t20 = (((t12).simd_le(zeta_threshold)).select(t15 * zeta_threshold, t18 * t12));
            let t21 = (simd::cbrt(v_rho));
            let t22 = t21 * t21;
            let t23 = t20 * t22;
            let t24 = f64x8::splat(M_CBRT6);
            let t25 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t26 = (simd::cbrt(t25));
            let t27 = t26 * t26;
            let t28 = f64x8::splat(1.0) / t27;
            let t29 = t24 * t28;
            let t30 = f64x8::splat(M_CBRT2);
            let t31 = t30 * t30;
            let t32 = v_sigma * t31;
            let t33 = v_rho * v_rho;
            let t35 = f64x8::splat(1.0) / t22 / t33;
            let t39 = (simd::exp(-f64x8::splat(8.325416666666667) * t29 * t32 * t35));
            let t41 = t24 * t24;
            let t43 = f64x8::splat(1.0) / t26 / t25;
            let t44 = t41 * t43;
            let t45 = v_sigma * v_sigma;
            let t47 = t33 * t33;
            let t48 = t47 * v_rho;
            let t50 = f64x8::splat(1.0) / t21 / t48;
            let t54 = (simd::exp(-f64x8::splat(0.015095833333333333) * t44 * t45 * t30 * t50));
            let t56 = f64x8::splat(2.0788) - f64x8::splat(0.8524) * t39 - f64x8::splat(1.2264) * t54;
            let t60 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t56));
            let tzk0 = f64x8::splat(2.0) * t60;
            acc_zk = tzk0;
            let t62 = t20 / t21;
            let t66 = t29 * v_sigma;
            let t67 = t33 * v_rho;
            let t71 = t31 / t22 / t67 * t39;
            let t74 = t44 * t45;
            let t75 = t47 * t33;
            let t77 = f64x8::splat(1.0) / t21 / t75;
            let t78 = t30 * t77;
            let t79 = t78 * t54;
            let t82 = -f64x8::splat(18.92422711111111) * t66 * t71 - f64x8::splat(0.09873882666666667) * t74 * t79;
            let t87 = ((t2).select(f64x8::splat(0.0), t7 * t62 * t56 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t82));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t87 + f64x8::splat(2.0) * t60;
            acc_vrho = tvrho0;
            let t94 = t44 * v_sigma;
            let t95 = t30 * t50;
            let t96 = t95 * t54;
            let t99 = f64x8::splat(7.096585166666666) * t29 * t31 * t35 * t39 + f64x8::splat(0.03702706) * t94 * t96;
            let t103 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t99));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t103;
            acc_vsigma = tvsigma0;
            let t108 = t20 / t21 / v_rho;
            let t118 = t31 / t22 / t47 * t39;
            let t121 = t47 * t67;
            let t123 = f64x8::splat(1.0) / t21 / t121;
            let t124 = t30 * t123;
            let t125 = t124 * t39;
            let t128 = t124 * t54;
            let t131 = t25 * t25;
            let t134 = t24 / t27 / t131;
            let t135 = t45 * t45;
            let t136 = t134 * t135;
            let t137 = t47 * t47;
            let t138 = t137 * t47;
            let t140 = f64x8::splat(1.0) / t22 / t138;
            let t142 = t31 * t140 * t54;
            let t145 = f64x8::splat(69.38883274074074) * t66 * t118 - f64x8::splat(840.277737571358) * t74 * t125 + f64x8::splat(0.6253459022222222) * t74 * t128 - f64x8::splat(0.047697435868444445) * t136 * t142;
            let t150 = ((t2).select(f64x8::splat(0.0), -t7 * t108 * t56 / f64x8::splat(30.0) + t7 * t62 * t82 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t145));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t150 + f64x8::splat(4.0) * t87;
            acc_v2rho2 = tv2rho20;
            let t158 = t44 * t30;
            let t165 = t45 * v_sigma;
            let t166 = t134 * t165;
            let t167 = t137 * t67;
            let t169 = f64x8::splat(1.0) / t22 / t167;
            let t174 = -f64x8::splat(18.92422711111111) * t29 * t71 + f64x8::splat(315.10415158925923) * t158 * t77 * v_sigma * t39 - f64x8::splat(0.19747765333333334) * t94 * t79 + f64x8::splat(0.017886538450666668) * t166 * t31 * t169 * t54;
            let t179 = ((t2).select(f64x8::splat(0.0), t7 * t62 * t99 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t174));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t179 + f64x8::splat(2.0) * t103;
            acc_v2rhosigma = tv2rhosigma0;
            let t188 = t137 * t33;
            let t190 = f64x8::splat(1.0) / t22 / t188;
            let t192 = t31 * t190 * t54;
            let t195 = -f64x8::splat(118.16405684597223) * t44 * t95 * t39 + f64x8::splat(0.03702706) * t44 * t96 - f64x8::splat(0.006707451919) * t134 * t45 * t192;
            let t199 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t195));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t199;
            acc_v2sigma2 = tv2sigma20;
            let t204 = t20 / t21 / t33;
            let t217 = t31 / t22 / t48 * t39;
            let t221 = f64x8::splat(1.0) / t21 / t137;
            let t222 = t30 * t221;
            let t226 = f64x8::splat(1.0) / t167;
            let t230 = t222 * t54;
            let t235 = f64x8::splat(1.0) / t22 / t137 / t48;
            let t237 = t31 * t235 * t54;
            let t240 = t135 * t45;
            let t241 = t137 * t137;
            let t243 = f64x8::splat(1.0) / t241 / t67;
            let t247 = -f64x8::splat(323.81455279012346) * t66 * t217 + f64x8::splat(9243.055113284938) * t74 * t222 * t39 - f64x8::splat(2298.155034770158) * t165 * t226 * t39 - f64x8::splat(4.58586994962963) * t74 * t230 + f64x8::splat(0.9062512815004444) * t136 * t237 - f64x8::splat(4.856608744702908e-06) * t240 * t243 * t54;
            let t252 = ((t2).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(45.0) * t7 * t204 * t56 - t7 * t108 * t82 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t7 * t62 * t145 + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t247));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t252 + f64x8::splat(6.0) * t150;
            acc_v3rho3 = tv3rho30;
            let t268 = f64x8::splat(1.0) / t188;
            let t276 = t135 * v_sigma;
            let t278 = f64x8::splat(1.0) / t241 / t33;
            let t282 = f64x8::splat(69.38883274074074) * t29 * t118 - f64x8::splat(2835.937364303333) * t158 * t123 * v_sigma * t39 + f64x8::splat(861.8081380388094) * t268 * t45 * t39 + f64x8::splat(1.2506918044444444) * t94 * t128 - f64x8::splat(0.30407115366133336) * t166 * t142 + f64x8::splat(1.8212282792635907e-06) * t276 * t278 * t54;
            let t287 = ((t2).select(f64x8::splat(0.0), -t7 * t108 * t99 / f64x8::splat(30.0) + t7 * t62 * t174 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t282));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t287 + f64x8::splat(4.0) * t179;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t296 = t137 * v_rho;
            let t297 = f64x8::splat(1.0) / t296;
            let t303 = t134 * t31;
            let t309 = f64x8::splat(1.0) / t241 / v_rho;
            let t313 = f64x8::splat(630.2083031785185) * t44 * t78 * t39 - f64x8::splat(323.1780517645535) * t297 * v_sigma * t39 - f64x8::splat(0.19747765333333334) * t44 * t79 + f64x8::splat(0.08943269225333333) * t303 * t169 * t45 * t54 - f64x8::splat(6.829606047238465e-07) * t135 * t309 * t54;
            let t318 = ((t2).select(f64x8::splat(0.0), t7 * t62 * t195 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t313));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t318 + f64x8::splat(2.0) * t199;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t328 = f64x8::splat(1.0) / t241;
            let t332 = f64x8::splat(121.19176941170757) / t137 * t39 - f64x8::splat(0.020122355757) * t303 * t190 * v_sigma * t54 + f64x8::splat(2.5611022677144246e-07) * t165 * t328 * t54;
            let t336 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t332));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t336;
            acc_v3sigma3 = tv3sigma30;
            let t362 = t30 / t21 / t296;
            let t372 = f64x8::splat(1.0) / t22 / t137 / t75;
            let t376 = t28 * t31 * t39;
            let t391 = t135 * t135;
            let t398 = t43 * t30 * t54;
            let t406 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(14.0) / f64x8::splat(135.0) * t7 * t20 / t21 / t67 * t56 + f64x8::splat(8.0) / f64x8::splat(45.0) * t7 * t204 * t82 - t7 * t108 * t145 / f64x8::splat(5.0) + f64x8::splat(2.0) / f64x8::splat(5.0) * t7 * t62 * t247 + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * (f64x8::splat(1834.9491324773662) * t66 * t31 / t22 / t75 * t39 - f64x8::splat(91403.54500915106) * t74 * t362 * t39 + f64x8::splat(50559.41076494349) * t165 / t138 * t39 - f64x8::splat(51021.5952774917) * t135 * t372 * t24 * t376 + f64x8::splat(38.21558291358025) * t74 * t362 * t54 - f64x8::splat(14.600715090840493) * t136 * t31 * t372 * t54 + f64x8::splat(0.00018455113229871054) * t240 / t241 / t47 * t54 - f64x8::splat(3.9101096626796974e-07) * t391 / t21 / t241 / t296 * t41 * t398)));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t406 + f64x8::splat(8.0) * t252;
            acc_v4rho4 = tv4rho40;
            let t452 = ((t2).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(45.0) * t7 * t204 * t99 - t7 * t108 * t174 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t7 * t62 * t282 + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * (-f64x8::splat(323.81455279012346) * t29 * t217 + f64x8::splat(23877.89237598609) * t158 * t221 * v_sigma * t39 - f64x8::splat(16374.354622737379) * t226 * t45 * t39 + f64x8::splat(19133.09822905939) * t235 * t165 * t24 * t376 - f64x8::splat(9.17173989925926) * t94 * t230 + f64x8::splat(4.455735467377186) * t166 * t237 - f64x8::splat(6.374298977422567e-05) * t276 * t243 * t54 + f64x8::splat(1.4662911235048865e-07) * t135 * t165 / t21 / t241 / t137 * t41 * t398)));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t452 + f64x8::splat(6.0) * t287;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t467 = t140 * t45;
            let t491 = ((t2).select(f64x8::splat(0.0), -t7 * t108 * t195 / f64x8::splat(30.0) + t7 * t62 * t313 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * (-f64x8::splat(3991.3192534639506) * t44 * t125 + f64x8::splat(4632.218741958601) * t268 * v_sigma * t39 - f64x8::splat(7174.911835897271) * t467 * t24 * t376 + f64x8::splat(1.2506918044444444) * t44 * t128 - f64x8::splat(1.1387762813591111) * t303 * t467 * t54 + f64x8::splat(2.0716471676623347e-05) * t278 * t135 * t54 - f64x8::splat(5.4985917131433246e-08) * t240 / t21 / t241 / t121 * t41 * t398)));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t491 + f64x8::splat(4.0) * t318;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t523 = ((t2).select(f64x8::splat(0.0), t7 * t62 * t332 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * (-f64x8::splat(969.5341552936605) * t297 * t39 + f64x8::splat(2690.5919384614767) * t169 * t24 * t28 * t32 * t39 + f64x8::splat(0.214638461408) * t303 * t169 * v_sigma * t54 - f64x8::splat(6.146645442514619e-06) * t309 * t165 * t54 + f64x8::splat(2.0619718924287465e-08) * t276 / t21 / t241 / t75 * t41 * t398)));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t523 + f64x8::splat(2.0) * t336;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t545 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * (-f64x8::splat(1008.9719769230537) * t190 * t24 * t376 - f64x8::splat(0.020122355757) * t134 * t192 + f64x8::splat(1.5366613606286548e-06) * t328 * t45 * t54 - f64x8::splat(7.7323945966078e-09) * t135 / t21 / t241 / t48 * t41 * t398)));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t545;
            acc_v4sigma4 = tv4sigma40;
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
        { let a: [f64; 8] = acc_v4rho4.into(); v4rho4[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho3sigma.into(); v4rho3sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho2sigma2.into(); v4rho2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rhosigma3.into(); v4rhosigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4sigma4.into(); v4sigma4[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
