//! MGGA_XC_CC06 vxc pol kernel — explicit SIMD (bit-exact).
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

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_cc06_vxc_pol(
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
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = v_rho0 * t8;
            let t11 = (f64x8::splat(2.0) * t9).simd_le(zeta_threshold);
            let t12 = (simd::cbrt(zeta_threshold));
            let t13 = t12 * zeta_threshold;
            let t14 = f64x8::splat(M_CBRT2);
            let t15 = t14 * v_rho0;
            let t16 = (simd::cbrt(t9));
            let t20 = ((t11).select(t13, f64x8::splat(2.0) * t15 * t8 * t16));
            let t21 = (simd::cbrt(t7));
            let t25 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t20 * t21));
            let t26 = (v_rho1).simd_le(dens_threshold);
            let t27 = v_rho1 * t8;
            let t29 = (f64x8::splat(2.0) * t27).simd_le(zeta_threshold);
            let t30 = t14 * v_rho1;
            let t31 = (simd::cbrt(t27));
            let t35 = ((t29).select(t13, f64x8::splat(2.0) * t30 * t8 * t31));
            let t39 = ((t26).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t35 * t21));
            let t40 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t41 = (simd::cbrt(t40));
            let t42 = t3 * t41;
            let t43 = f64x8::splat(M_CBRT4);
            let t44 = t43 * t43;
            let t47 = t42 * t44 / t21;
            let t49 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t47;
            let t50 = ((t47).sqrt());
            let t53 = ((t47) * (t47).sqrt());
            let t55 = t3 * t3;
            let t56 = t41 * t41;
            let t57 = t55 * t56;
            let t58 = t21 * t21;
            let t59 = f64x8::splat(1.0) / t58;
            let t61 = t57 * t43 * t59;
            let t63 = f64x8::splat(3.79785) * t50 + f64x8::splat(0.8969) * t47 + f64x8::splat(0.204775) * t53 + f64x8::splat(0.123235) * t61;
            let t66 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t63;
            let t67 = (simd::ln(t66));
            let t69 = f64x8::splat(0.062182) * t49 * t67;
            let t70 = v_rho0 - v_rho1;
            let t71 = t70 * t70;
            let t72 = t71 * t71;
            let t73 = t7 * t7;
            let t74 = t73 * t73;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t72 * t75;
            let t77 = t70 * t8;
            let t78 = f64x8::splat(1.0) + t77;
            let t79 = (t78).simd_le(zeta_threshold);
            let t80 = (simd::cbrt(t78));
            let t82 = ((t79).select(t13, t80 * t78));
            let t83 = f64x8::splat(1.0) - t77;
            let t84 = (t83).simd_le(zeta_threshold);
            let t85 = (simd::cbrt(t83));
            let t87 = ((t84).select(t13, t85 * t83));
            let t88 = t82 + t87 - f64x8::splat(2.0);
            let t91 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t14 - f64x8::splat(2.0));
            let t92 = t88 * t91;
            let t94 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t47;
            let t99 = f64x8::splat(7.05945) * t50 + f64x8::splat(1.549425) * t47 + f64x8::splat(0.420775) * t53 + f64x8::splat(0.1562925) * t61;
            let t102 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t99;
            let t103 = (simd::ln(t102));
            let t107 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t47;
            let t112 = f64x8::splat(5.1785) * t50 + f64x8::splat(0.905775) * t47 + f64x8::splat(0.1100325) * t53 + f64x8::splat(0.1241775) * t61;
            let t115 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t112;
            let t116 = (simd::ln(t115));
            let t117 = t107 * t116;
            let t119 = -f64x8::splat(0.03109) * t94 * t103 + t69 - f64x8::splat(0.019751789702565206) * t117;
            let t120 = t92 * t119;
            let t124 = t25 + t39 - t69 + t76 * t120 + f64x8::splat(0.019751789702565206) * t92 * t117;
            let t125 = t55 * t43;
            let t126 = (simd::cbrt(v_rho0));
            let t127 = t126 * t126;
            let t129 = f64x8::splat(1.0) / t127 / v_rho0;
            let t130 = v_lapl0 * t129;
            let t131 = t78 / f64x8::splat(2.0);
            let t132 = (simd::cbrt(t131));
            let t133 = t132 * t132;
            let t134 = t133 * t131;
            let t136 = (simd::cbrt(v_rho1));
            let t137 = t136 * t136;
            let t139 = f64x8::splat(1.0) / t137 / v_rho1;
            let t140 = v_lapl1 * t139;
            let t141 = t83 / f64x8::splat(2.0);
            let t142 = (simd::cbrt(t141));
            let t143 = t142 * t142;
            let t144 = t143 * t141;
            let t148 = t125 * t56 * (t130 * t134 + t140 * t144);
            let t150 = -f64x8::splat(0.0007) + f64x8::splat(0.002) * t148;
            let t152 = f64x8::splat(1.0) + f64x8::splat(0.0065) * t148;
            let t153 = f64x8::splat(1.0) / t152;
            let t155 = t150 * t153 + f64x8::splat(1.0);
            let tzk0 = t124 * t155;
            acc_zk = tzk0;
            let t156 = t14 * t8;
            let t159 = f64x8::splat(1.0) / t73;
            let t162 = f64x8::splat(2.0) * t15 * t159 * t16;
            let t163 = t16 * t16;
            let t164 = f64x8::splat(1.0) / t163;
            let t165 = t8 * t164;
            let t167 = -v_rho0 * t159 + t8;
            let t172 = ((t11).select(f64x8::splat(0.0), f64x8::splat(2.0) * t156 * t16 - t162 + f64x8::splat(2.0) / f64x8::splat(3.0) * t15 * t165 * t167));
            let t178 = t6 * t20 * t59 / f64x8::splat(8.0);
            let t180 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t172 * t21 - t178));
            let t183 = f64x8::splat(2.0) * t30 * t159 * t31;
            let t184 = v_rho1 * v_rho1;
            let t185 = t14 * t184;
            let t186 = t73 * t7;
            let t187 = f64x8::splat(1.0) / t186;
            let t188 = t31 * t31;
            let t189 = f64x8::splat(1.0) / t188;
            let t190 = t187 * t189;
            let t194 = ((t29).select(f64x8::splat(0.0), -t183 - f64x8::splat(2.0) / f64x8::splat(3.0) * t185 * t190));
            let t200 = t6 * t35 * t59 / f64x8::splat(8.0);
            let t202 = ((t26).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t194 * t21 - t200));
            let t204 = f64x8::splat(1.0) / t21 / t7;
            let t205 = t44 * t204;
            let t208 = f64x8::splat(0.0011073577833333333) * t42 * t205 * t67;
            let t209 = t63 * t63;
            let t210 = f64x8::splat(1.0) / t209;
            let t211 = t49 * t210;
            let t213 = f64x8::splat(1.0) / t50 * t3;
            let t214 = t41 * t44;
            let t215 = t214 * t204;
            let t216 = t213 * t215;
            let t218 = t42 * t205;
            let t220 = ((t47).sqrt());
            let t221 = t220 * t3;
            let t222 = t221 * t215;
            let t225 = f64x8::splat(1.0) / t58 / t7;
            let t227 = t57 * t43 * t225;
            let t229 = -f64x8::splat(0.632975) * t216 - f64x8::splat(0.29896666666666666) * t218 - f64x8::splat(0.1023875) * t222 - f64x8::splat(0.08215666666666667) * t227;
            let t230 = f64x8::splat(1.0) / t66;
            let t231 = t229 * t230;
            let t233 = f64x8::splat(1.0) * t211 * t231;
            let t234 = t71 * t70;
            let t235 = t234 * t75;
            let t237 = f64x8::splat(4.0) * t235 * t120;
            let t238 = t74 * t7;
            let t239 = f64x8::splat(1.0) / t238;
            let t240 = t72 * t239;
            let t242 = f64x8::splat(4.0) * t240 * t120;
            let t243 = t70 * t159;
            let t244 = t8 - t243;
            let t247 = ((t79).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t80 * t244));
            let t248 = -t244;
            let t251 = ((t84).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t85 * t248));
            let t253 = (t247 + t251) * t91;
            let t254 = t253 * t119;
            let t259 = t99 * t99;
            let t260 = f64x8::splat(1.0) / t259;
            let t261 = t94 * t260;
            let t266 = -f64x8::splat(1.176575) * t216 - f64x8::splat(0.516475) * t218 - f64x8::splat(0.2103875) * t222 - f64x8::splat(0.104195) * t227;
            let t267 = f64x8::splat(1.0) / t102;
            let t268 = t266 * t267;
            let t274 = t112 * t112;
            let t275 = f64x8::splat(1.0) / t274;
            let t276 = t107 * t275;
            let t281 = -f64x8::splat(0.8630833333333333) * t216 - f64x8::splat(0.301925) * t218 - f64x8::splat(0.05501625) * t222 - f64x8::splat(0.082785) * t227;
            let t282 = f64x8::splat(1.0) / t115;
            let t283 = t281 * t282;
            let t286 = f64x8::splat(0.0005323644333333333) * t42 * t205 * t103 + f64x8::splat(1.0) * t261 * t268 - t208 - t233 + f64x8::splat(0.0001831155503675316) * t42 * t205 * t116 + f64x8::splat(0.5848223397455204) * t276 * t283;
            let t287 = t92 * t286;
            let t288 = t76 * t287;
            let t291 = t92 * t3;
            let t293 = t214 * t204 * t116;
            let t295 = f64x8::splat(0.0001831155503675316) * t291 * t293;
            let t296 = t92 * t107;
            let t298 = t275 * t281 * t282;
            let t300 = f64x8::splat(0.5848223397455204) * t296 * t298;
            let t301 = t180 + t202 + t208 + t233 + t237 - t242 + t76 * t254 + t288 + f64x8::splat(0.019751789702565206) * t253 * t117 - t295 - t300;
            let t302 = t7 * t301;
            let t304 = t7 * t124;
            let t305 = v_rho0 * v_rho0;
            let t307 = f64x8::splat(1.0) / t127 / t305;
            let t308 = v_lapl0 * t307;
            let t310 = t244 / f64x8::splat(2.0);
            let t311 = t133 * t310;
            let t313 = -t310;
            let t314 = t143 * t313;
            let t317 = f64x8::splat(5.0) / f64x8::splat(3.0) * t130 * t311 - f64x8::splat(5.0) / f64x8::splat(3.0) * t308 * t134 + f64x8::splat(5.0) / f64x8::splat(3.0) * t140 * t314;
            let t318 = t56 * t317;
            let t322 = t152 * t152;
            let t323 = f64x8::splat(1.0) / t322;
            let t324 = t150 * t323;
            let t325 = t324 * t55;
            let t326 = t43 * t56;
            let t327 = t326 * t317;
            let t330 = f64x8::splat(0.002) * t125 * t318 * t153 - f64x8::splat(0.0065) * t325 * t327;
            let tvrho0 = t155 * t302 + t304 * t330 + tzk0;
            acc_vrho_0 = tvrho0;
            let t332 = t14 * t305;
            let t333 = t187 * t164;
            let t337 = ((t11).select(f64x8::splat(0.0), -t162 - f64x8::splat(2.0) / f64x8::splat(3.0) * t332 * t333));
            let t342 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t337 * t21 - t178));
            let t345 = t8 * t189;
            let t347 = -v_rho1 * t159 + t8;
            let t352 = ((t29).select(f64x8::splat(0.0), f64x8::splat(2.0) * t156 * t31 - t183 + f64x8::splat(2.0) / f64x8::splat(3.0) * t30 * t345 * t347));
            let t357 = ((t26).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t352 * t21 - t200));
            let t358 = -t8 - t243;
            let t361 = ((t79).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t80 * t358));
            let t362 = -t358;
            let t365 = ((t84).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t85 * t362));
            let t367 = (t361 + t365) * t91;
            let t368 = t367 * t119;
            let t372 = t342 + t357 + t208 + t233 - t237 - t242 + t76 * t368 + t288 + f64x8::splat(0.019751789702565206) * t367 * t117 - t295 - t300;
            let t373 = t7 * t372;
            let t375 = t358 / f64x8::splat(2.0);
            let t376 = t133 * t375;
            let t379 = f64x8::splat(1.0) / t137 / t184;
            let t380 = v_lapl1 * t379;
            let t382 = -t375;
            let t383 = t143 * t382;
            let t386 = f64x8::splat(5.0) / f64x8::splat(3.0) * t130 * t376 + f64x8::splat(5.0) / f64x8::splat(3.0) * t140 * t383 - f64x8::splat(5.0) / f64x8::splat(3.0) * t380 * t144;
            let t387 = t56 * t386;
            let t391 = t326 * t386;
            let t394 = f64x8::splat(0.002) * t125 * t387 * t153 - f64x8::splat(0.0065) * t325 * t391;
            let tvrho1 = t155 * t373 + t304 * t394 + tzk0;
            acc_vrho_1 = tvrho1;
            let tvsigma0 = f64x8::splat(0.0);
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = f64x8::splat(0.0);
            acc_vsigma_2 = tvsigma2;
            let t396 = t125 * t56;
            let t397 = t129 * t134;
            let t401 = t326 * t397;
            let t404 = f64x8::splat(0.002) * t396 * t397 * t153 - f64x8::splat(0.0065) * t325 * t401;
            let tvlapl0 = t304 * t404;
            acc_vlapl_0 = tvlapl0;
            let t405 = t139 * t144;
            let t412 = f64x8::splat(0.002) * t396 * t405 * t153 - f64x8::splat(0.0065) * t325 * t326 * t405;
            let tvlapl1 = t304 * t412;
            acc_vlapl_1 = tvlapl1;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau_0 = tvtau0;
            let tvtau1 = f64x8::splat(0.0);
            acc_vtau_1 = tvtau1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
