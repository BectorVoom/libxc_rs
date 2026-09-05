//! GGA_X_LAG lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lag.c`
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
pub fn gga_x_lag_lxc_unpol(
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
            let t4 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t5 = zeta_threshold - f64x8::splat(1.0);
            let t7 = ((t4).select(t5, (t4).select(-t5, f64x8::splat(0.0))));
            let t8 = f64x8::splat(1.0) + t7;
            let t10 = (simd::cbrt(zeta_threshold));
            let t12 = (simd::cbrt(t8));
            let t14 = (((t8).simd_le(zeta_threshold)).select(t10 * zeta_threshold, t12 * t8));
            let t15 = t3 * t14;
            let t16 = (simd::cbrt(v_rho));
            let t17 = f64x8::splat(M_CBRT6);
            let t18 = t17 * t17;
            let t19 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t20 = (simd::cbrt(t19));
            let t21 = f64x8::splat(1.0) / t20;
            let t22 = t18 * t21;
            let t23 = ((v_sigma).sqrt());
            let t24 = f64x8::splat(M_CBRT2);
            let t29 = t22 * t23 * t24 / t16 / v_rho;
            let t30 = (simd::pow(t29, f64x8::splat(2.626712)));
            let t33 = f64x8::splat(1.0) + f64x8::splat(0.00013471619689594795) * t30;
            let t34 = (simd::pow(t33, -f64x8::splat(0.657946)));
            let t38 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(1.540002877192757e-05) * t15 * t16 * t30 * t34));
            let tzk0 = f64x8::splat(2.0) * t38;
            acc_zk = tzk0;
            let t39 = t16 * t16;
            let t45 = v_rho * v_rho;
            let t46 = f64x8::splat(1.0) / t45;
            let t47 = (simd::pow(t29, f64x8::splat(1.626712)));
            let t49 = t15 * t46 * t47;
            let t50 = t34 * t18;
            let t52 = t21 * t23 * t24;
            let t53 = t50 * t52;
            let t56 = (simd::pow(t29, f64x8::splat(4.253424)));
            let t58 = t15 * t46 * t56;
            let t59 = (simd::pow(t33, -f64x8::splat(1.657946)));
            let t60 = t59 * t18;
            let t61 = t60 * t52;
            let t65 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.133342923975857e-06) * t15 / t39 * t30 * t34 + f64x8::splat(5.393525383408988e-05) * t49 * t53 - f64x8::splat(4.780604235623332e-09) * t58 * t61));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t65 + f64x8::splat(2.0) * t38;
            acc_vrho = tvrho0;
            let t68 = f64x8::splat(1.0) / v_rho;
            let t70 = t15 * t68 * t47;
            let t71 = f64x8::splat(1.0) / t23;
            let t73 = t21 * t71 * t24;
            let t74 = t50 * t73;
            let t78 = t15 * t68 * t56;
            let t79 = t60 * t73;
            let t83 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(2.0225720187783704e-05) * t70 * t74 + f64x8::splat(1.7927265883587494e-09) * t78 * t79));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t83;
            acc_vsigma = tvsigma0;
            let t92 = t45 * v_rho;
            let t93 = f64x8::splat(1.0) / t92;
            let t95 = t15 * t93 * t47;
            let t99 = t15 * t93 * t56;
            let t102 = t45 * t45;
            let t104 = f64x8::splat(1.0) / t16 / t102;
            let t105 = (simd::pow(t29, f64x8::splat(0.626712)));
            let t107 = t15 * t104 * t105;
            let t108 = t34 * t17;
            let t109 = t20 * t20;
            let t110 = f64x8::splat(1.0) / t109;
            let t112 = t24 * t24;
            let t113 = t110 * v_sigma * t112;
            let t114 = t108 * t113;
            let t117 = (simd::pow(t29, f64x8::splat(3.253424)));
            let t119 = t15 * t104 * t117;
            let t120 = t59 * t17;
            let t121 = t120 * t113;
            let t124 = (simd::pow(t29, f64x8::splat(5.880136)));
            let t126 = t15 * t104 * t124;
            let t127 = (simd::pow(t33, -f64x8::splat(2.657946)));
            let t128 = t127 * t17;
            let t129 = t128 * t113;
            let t133 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.4222286159839043e-06) * t15 / t39 / v_rho * t30 * t34 - f64x8::splat(8.989208972348313e-05) * t95 * t53 + f64x8::splat(7.967673726038885e-09) * t99 * t61 - f64x8::splat(0.0007018969970796801) * t107 * t114 + f64x8::splat(2.631296584261165e-07) * t119 * t121 - f64x8::splat(2.2437549929142988e-11) * t126 * t129));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t133 + f64x8::splat(4.0) * t65;
            acc_v2rho2 = tv2rho20;
            let t139 = f64x8::splat(1.0) / t16 / t92;
            let t141 = t15 * t139 * t105;
            let t142 = t110 * t112;
            let t143 = t108 * t142;
            let t147 = t15 * t139 * t117;
            let t148 = t120 * t142;
            let t154 = t15 * t139 * t124;
            let t155 = t128 * t142;
            let t159 = ((t2).select(f64x8::splat(0.0), f64x8::splat(2.0225720187783704e-05) * t49 * t74 + f64x8::splat(0.00026321137390488005) * t141 * t143 - f64x8::splat(9.86736219097937e-08) * t147 * t148 - f64x8::splat(1.7927265883587494e-09) * t58 * t79 + f64x8::splat(8.414081223428621e-12) * t154 * t155));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t159 + f64x8::splat(2.0) * t83;
            acc_v2rhosigma = tv2rhosigma0;
            let t163 = f64x8::splat(1.0) / t16 / t45;
            let t165 = t15 * t163 * t105;
            let t166 = f64x8::splat(1.0) / v_sigma;
            let t168 = t110 * t166 * t112;
            let t169 = t108 * t168;
            let t173 = t15 * t163 * t117;
            let t174 = t120 * t168;
            let t177 = t23 * v_sigma;
            let t178 = f64x8::splat(1.0) / t177;
            let t180 = t21 * t178 * t24;
            let t181 = t50 * t180;
            let t185 = t15 * t163 * t124;
            let t186 = t128 * t168;
            let t189 = t60 * t180;
            let t193 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(9.870426521433003e-05) * t165 * t169 + f64x8::splat(3.700260821617263e-08) * t173 * t174 + f64x8::splat(1.0112860093891852e-05) * t70 * t181 - f64x8::splat(3.1552804587857326e-12) * t185 * t186 - f64x8::splat(8.963632941793747e-10) * t78 * t189));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t193;
            acc_v2sigma2 = tv2sigma20;
            let t202 = f64x8::splat(1.0) / t102;
            let t204 = t15 * t202 * t47;
            let t208 = t15 * t202 * t56;
            let t211 = t102 * v_rho;
            let t213 = f64x8::splat(1.0) / t16 / t211;
            let t215 = t15 * t213 * t105;
            let t219 = t15 * t213 * t117;
            let t223 = t15 * t213 * t124;
            let t226 = t102 * t45;
            let t229 = t15 / t39 / t226;
            let t230 = (simd::pow(t29, -f64x8::splat(0.373288)));
            let t231 = t230 * t34;
            let t232 = f64x8::splat(1.0) / t19;
            let t233 = t232 * t177;
            let t234 = t231 * t233;
            let t237 = (simd::pow(t29, f64x8::splat(2.253424)));
            let t238 = t237 * t59;
            let t239 = t238 * t177;
            let t242 = t238 * t233;
            let t245 = (simd::pow(t29, f64x8::splat(4.880136)));
            let t246 = t245 * t127;
            let t247 = t246 * t177;
            let t250 = t246 * t233;
            let t253 = (simd::pow(t29, f64x8::splat(7.506848)));
            let t254 = (simd::pow(t33, -f64x8::splat(3.657946)));
            let t255 = t253 * t254;
            let t256 = t255 * t177;
            let t259 = -f64x8::splat(5.703714359973174e-06) * t15 / t39 / t45 * t30 * t34 + f64x8::splat(0.00025769065720731833) * t204 * t53 - f64x8::splat(2.2840664681311472e-08) * t208 * t61 + f64x8::splat(0.004211381982478081) * t215 * t114 - f64x8::splat(1.5787779505566992e-06) * t219 * t121 + f64x8::splat(1.3462529957485794e-10) * t223 * t129 + f64x8::splat(0.007038196333340808) * t229 * t234 - f64x8::splat(2.64920853001845e-07) * t229 * t239 - f64x8::splat(1.3697157533365275e-05) * t229 * t242 + f64x8::splat(2.502607216998568e-10) * t229 * t247 + f64x8::splat(2.1109735214424183e-09) * t229 * t250 - f64x8::splat(3.42116558884763e-14) * t229 * t256;
            let t260 = ((t2).select(f64x8::splat(0.0), t259));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t260 + f64x8::splat(6.0) * t133;
            acc_v3rho3 = tv3rho30;
            let t272 = t15 / t39 / t211;
            let t273 = t232 * t23;
            let t274 = t231 * t273;
            let t277 = t238 * t23;
            let t280 = t238 * t273;
            let t283 = t246 * t23;
            let t290 = t246 * t273;
            let t293 = t255 * t23;
            let t296 = -f64x8::splat(4.045144037556741e-05) * t95 * t74 - f64x8::splat(0.0011405826202544802) * t107 * t143 + f64x8::splat(4.2758569494243933e-07) * t119 * t148 - f64x8::splat(0.002639323625002803) * t272 * t274 + f64x8::splat(9.934531987569186e-08) * t272 * t277 + f64x8::splat(5.1364340750119784e-06) * t272 * t280 - f64x8::splat(9.38477706374463e-11) * t272 * t283 + f64x8::splat(3.5854531767174987e-09) * t99 * t79 - f64x8::splat(3.6461018634857355e-11) * t126 * t155 - f64x8::splat(7.916150705409068e-10) * t272 * t290 + f64x8::splat(1.2829370958178615e-14) * t272 * t293;
            let t297 = ((t2).select(f64x8::splat(0.0), t296));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t297 + f64x8::splat(4.0) * t159;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t304 = t15 / t39 / t102;
            let t305 = t232 * t71;
            let t306 = t231 * t305;
            let t309 = t238 * t71;
            let t314 = t238 * t305;
            let t317 = t246 * t71;
            let t324 = t246 * t305;
            let t327 = t255 * t71;
            let t332 = f64x8::splat(9.870426521433003e-05) * t141 * t169 + f64x8::splat(0.0009897463593760512) * t304 * t306 - f64x8::splat(3.725449495338445e-08) * t304 * t309 - f64x8::splat(3.700260821617263e-08) * t147 * t174 - f64x8::splat(1.926162778129492e-06) * t304 * t314 + f64x8::splat(3.519291398904236e-11) * t304 * t317 - f64x8::splat(1.0112860093891852e-05) * t49 * t181 + f64x8::splat(3.1552804587857326e-12) * t154 * t186 + f64x8::splat(2.9685565145284003e-10) * t304 * t324 - f64x8::splat(4.811014109316981e-15) * t304 * t327 + f64x8::splat(8.963632941793747e-10) * t58 * t189;
            let t333 = ((t2).select(f64x8::splat(0.0), t332));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t333 + f64x8::splat(2.0) * t193;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t337 = f64x8::splat(1.0) / t39 / t92;
            let t338 = t15 * t337;
            let t339 = t232 * t178;
            let t340 = t231 * t339;
            let t343 = t238 * t178;
            let t346 = v_sigma * v_sigma;
            let t347 = f64x8::splat(1.0) / t346;
            let t349 = t110 * t347 * t112;
            let t350 = t108 * t349;
            let t353 = t238 * t339;
            let t356 = t246 * t178;
            let t359 = t120 * t349;
            let t363 = f64x8::splat(1.0) / t23 / t346;
            let t365 = t21 * t363 * t24;
            let t366 = t50 * t365;
            let t369 = t246 * t339;
            let t372 = t255 * t178;
            let t375 = t128 * t349;
            let t378 = t60 * t365;
            let t381 = -f64x8::splat(0.0003711548847660192) * t338 * t340 + f64x8::splat(1.3970435607519168e-08) * t338 * t343 + f64x8::splat(0.00014805639782149502) * t165 * t350 + f64x8::splat(7.223110417985595e-07) * t338 * t353 - f64x8::splat(1.3197342745890886e-11) * t338 * t356 - f64x8::splat(5.550391232425895e-08) * t173 * t359 - f64x8::splat(1.5169290140837779e-05) * t70 * t366 - f64x8::splat(1.1132086929481502e-10) * t338 * t369 + f64x8::splat(1.8041302909938677e-15) * t338 * t372 + f64x8::splat(4.732920688178599e-12) * t185 * t375 + f64x8::splat(1.344544941269062e-09) * t78 * t378;
            let t382 = ((t2).select(f64x8::splat(0.0), t381));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t382;
            acc_v3sigma3 = tv3sigma30;
            let t385 = t102 * t102;
            let t387 = f64x8::splat(1.0) / t385 / v_rho;
            let t388 = (simd::pow(t29, f64x8::splat(9.13356)));
            let t391 = (simd::pow(t33, -f64x8::splat(4.657946)));
            let t393 = t22 * t24;
            let t397 = (simd::pow(t29, -f64x8::splat(1.373288)));
            let t402 = t232 * t346 * t393;
            let t405 = (simd::pow(t29, f64x8::splat(1.253424)));
            let t406 = t387 * t405;
            let t411 = (simd::pow(t29, f64x8::splat(3.880136)));
            let t412 = t387 * t411;
            let t417 = t102 * t92;
            let t420 = t15 / t39 / t417;
            let t438 = f64x8::splat(1.0) / t16 / t226;
            let t453 = (simd::pow(t29, f64x8::splat(6.506848)));
            let t460 = f64x8::splat(1.0) / t211;
            let t477 = -f64x8::splat(5.904490032667016e-17) * t15 * t387 * t388 * t391 * t346 * t393 + f64x8::splat(0.0035030323105068313) * t15 * t387 * t397 * t34 * t402 + f64x8::splat(4.115400468995482e-05) * t15 * t406 * t59 * t402 - f64x8::splat(1.3735783836050556e-08) * t15 * t412 * t127 * t402 + f64x8::splat(0.00017349732875596017) * t420 * t242 - f64x8::splat(2.673899793827063e-08) * t420 * t250 - f64x8::splat(0.08915048688898357) * t420 * t234 + f64x8::splat(3.3556641380233697e-06) * t420 * t239 - f64x8::splat(3.16996914153152e-09) * t420 * t247 + f64x8::splat(4.333476412540332e-13) * t420 * t256 + f64x8::splat(1.5209904959928464e-05) * t15 * t337 * t30 * t34 - f64x8::splat(8.252032251718143e-10) * t15 * t438 * t124 * t129 + f64x8::splat(1.017344114508429e-06) * t15 * t406 * t59 * t346 * t393 - f64x8::splat(2.921246469276406e-09) * t15 * t412 * t127 * t346 * t393 + f64x8::splat(9.244956470539412e-13) * t15 * t387 * t453 * t254 * t346 * t393 - f64x8::splat(0.0010107866088907215) * t15 * t460 * t47 * t53 + f64x8::splat(8.959206456390392e-08) * t15 * t460 * t56 * t61 - f64x8::splat(0.025814211781486015) * t15 * t438 * t105 * t114 + f64x8::splat(9.677324104338286e-06) * t15 * t438 * t117 * t121;
            let t478 = ((t2).select(f64x8::splat(0.0), t477));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t478 + f64x8::splat(8.0) * t260;
            acc_v4rho4 = tv4rho40;
            let t482 = f64x8::splat(1.0) / t385;
            let t495 = t482 * t405;
            let t501 = t482 * t411;
            let t516 = t232 * v_sigma * t393;
            let t544 = -f64x8::splat(3.4668586764522794e-13) * t15 * t482 * t453 * t254 * v_sigma * t393 + f64x8::splat(2.214183762250131e-17) * t15 * t482 * t388 * t391 * v_sigma * t393 - f64x8::splat(3.8150404294066087e-07) * t15 * t495 * t59 * v_sigma * t393 + f64x8::splat(1.0954674259786522e-09) * t15 * t501 * t127 * v_sigma * t393 - f64x8::splat(1.2829370958178614e-13) * t229 * t293 - f64x8::splat(9.934531987569185e-07) * t229 * t277 + f64x8::splat(9.38477706374463e-10) * t229 * t283 + f64x8::splat(5.150918938518958e-09) * t15 * t501 * t127 * t516 - f64x8::splat(0.0013136371164400617) * t15 * t482 * t397 * t34 * t516 - f64x8::splat(1.5432751758733057e-05) * t15 * t495 * t59 * t516 - f64x8::splat(1.0756359530152496e-08) * t208 * t79 + f64x8::splat(0.00012135432112670223) * t204 * t74 + f64x8::splat(7.916150705409068e-09) * t229 * t290 + f64x8::splat(0.02639323625002803) * t229 * t274 - f64x8::splat(5.136434075011978e-05) * t229 * t280 + f64x8::splat(1.7482590986457245e-10) * t223 * t155 + f64x8::splat(0.005468947435579174) * t215 * t143 - f64x8::splat(2.0502185885701577e-06) * t219 * t148;
            let t545 = ((t2).select(f64x8::splat(0.0), t544));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t545 + f64x8::splat(6.0) * t297;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t561 = f64x8::splat(1.0) / t417;
            let t563 = t15 * t561 * t411;
            let t575 = t15 * t561 * t405;
            let t593 = t21 * t24;
            let t610 = -f64x8::splat(1.682182024899427e-09) * t272 * t324 + f64x8::splat(1.0914922409400454e-05) * t272 * t314 - f64x8::splat(0.005608562703130956) * t272 * t306 + f64x8::splat(2.7262413286129556e-14) * t272 * t327 - f64x8::splat(1.994265126045734e-10) * t272 * t317 + f64x8::splat(2.111088047358452e-07) * t272 * t309 - f64x8::splat(1.9315946019446096e-09) * t563 * t127 * t232 * t393 + f64x8::splat(0.0004926139186650232) * t15 * t561 * t397 * t34 * t232 * t393 + f64x8::splat(5.787281909524896e-06) * t575 * t59 * t232 * t393 - f64x8::splat(6.310560917571465e-12) * t126 * t186 - f64x8::splat(1.7927265883587494e-09) * t99 * t189 - f64x8::splat(0.00019740853042866005) * t107 * t169 + f64x8::splat(7.400521643234527e-08) * t119 * t174 + f64x8::splat(2.0225720187783704e-05) * t95 * t181 - f64x8::splat(8.303189108437991e-18) * t15 * t561 * t388 * t391 * t18 * t593 + f64x8::splat(1.4306401610274782e-07) * t575 * t60 * t593 - f64x8::splat(4.108002847419946e-10) * t563 * t127 * t18 * t593 + f64x8::splat(1.300072003669605e-13) * t15 * t561 * t453 * t254 * t18 * t593;
            let t611 = ((t2).select(f64x8::splat(0.0), t610));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t611 + f64x8::splat(4.0) * t333;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t624 = f64x8::splat(1.0) / t226;
            let t637 = t624 * t405;
            let t643 = t624 * t411;
            let t652 = t232 * t166 * t393;
            let t676 = -f64x8::splat(4.732920688178599e-12) * t154 * t375 - f64x8::splat(1.344544941269062e-09) * t58 * t378 + f64x8::splat(6.013767636646226e-16) * t304 * t372 - f64x8::splat(4.399114248630295e-12) * t304 * t356 + f64x8::splat(4.656811869173056e-09) * t304 * t343 - f64x8::splat(4.875270013761018e-14) * t15 * t624 * t453 * t254 * t166 * t393 + f64x8::splat(3.1136959156642468e-18) * t15 * t624 * t388 * t391 * t166 * t393 - f64x8::splat(5.364900603853044e-08) * t15 * t637 * t59 * t166 * t393 + f64x8::splat(1.54050106778248e-10) * t15 * t643 * t127 * t166 * t393 - f64x8::splat(2.170230716071836e-06) * t15 * t637 * t59 * t652 + f64x8::splat(5.550391232425895e-08) * t147 * t359 + f64x8::splat(1.5169290140837779e-05) * t49 * t366 + f64x8::splat(7.243479757292285e-10) * t15 * t643 * t127 * t652 - f64x8::splat(0.00018473021949938368) * t15 * t624 * t397 * t34 * t652 - f64x8::splat(0.00014805639782149502) * t141 * t350 + f64x8::splat(2.407703472661865e-07) * t304 * t353 - f64x8::splat(3.7106956431605003e-11) * t304 * t369 - f64x8::splat(0.0001237182949220064) * t304 * t340;
            let t677 = ((t2).select(f64x8::splat(0.0), t676));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t677 + f64x8::splat(2.0) * t382;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t689 = t460 * t405;
            let t695 = t460 * t411;
            let t713 = t346 * v_sigma;
            let t717 = t21 / t23 / t713 * t24;
            let t724 = t232 * t347 * t393;
            let t729 = t110 / t713 * t112;
            let t751 = t232 * t363;
            let t761 = -f64x8::splat(5.412390872981603e-15) * t338 * t255 * t363 - f64x8::splat(4.19113068225575e-08) * t338 * t238 * t363 + f64x8::splat(3.959202823767266e-11) * t338 * t246 * t363 + f64x8::splat(2.0118377264448914e-08) * t15 * t689 * t59 * t347 * t393 - f64x8::splat(5.7768790041842993e-11) * t15 * t695 * t127 * t347 * t393 + f64x8::splat(1.8282262551603817e-14) * t15 * t460 * t453 * t254 * t347 * t393 - f64x8::splat(1.1676359683740925e-18) * t15 * t460 * t388 * t391 * t347 * t393 + f64x8::splat(3.792322535209445e-05) * t70 * t50 * t717 - f64x8::splat(2.716304908984607e-10) * t15 * t695 * t127 * t724 - f64x8::splat(1.1832301720446498e-11) * t185 * t128 * t729 - f64x8::splat(3.361362353172655e-09) * t78 * t60 * t717 + f64x8::splat(6.927383231226888e-05) * t15 * t460 * t397 * t34 * t724 - f64x8::splat(0.0003701409945537376) * t165 * t108 * t729 + f64x8::splat(8.138365185269385e-07) * t15 * t689 * t59 * t724 + f64x8::splat(1.3875978081064738e-07) * t173 * t120 * t729 + f64x8::splat(3.3396260788444505e-10) * t338 * t246 * t751 + f64x8::splat(0.0011134646542980576) * t338 * t231 * t751 - f64x8::splat(2.1669331253956785e-06) * t338 * t238 * t751;
            let t762 = ((t2).select(f64x8::splat(0.0), t761));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t762;
            acc_v4sigma4 = tv4sigma40;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v3rho2sigma, ip, m, acc_v3rho2sigma);
        store_add(v3rhosigma2, ip, m, acc_v3rhosigma2);
        store_add(v3sigma3, ip, m, acc_v3sigma3);
        store_add(v4rho4, ip, m, acc_v4rho4);
        store_add(v4rho3sigma, ip, m, acc_v4rho3sigma);
        store_add(v4rho2sigma2, ip, m, acc_v4rho2sigma2);
        store_add(v4rhosigma3, ip, m, acc_v4rhosigma3);
        store_add(v4sigma4, ip, m, acc_v4sigma4);
        ip += 8;
    }
}
