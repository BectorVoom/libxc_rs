//! GGA_C_P86VWN vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_p86vwn.c`
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
pub fn gga_c_p86vwn_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_aa: f64,
    param_bb: f64,
    param_ftilde: f64,
    param_malpha: f64,
    param_mbeta: f64,
    param_mgamma: f64,
    param_mdelta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_aa = f64x8::splat(param_aa);
    let param_bb = f64x8::splat(param_bb);
    let param_ftilde = f64x8::splat(param_ftilde);
    let param_malpha = f64x8::splat(param_malpha);
    let param_mbeta = f64x8::splat(param_mbeta);
    let param_mgamma = f64x8::splat(param_mgamma);
    let param_mdelta = f64x8::splat(param_mdelta);
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
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t6 * t8;
            let t10 = t4 * t9;
            let t11 = t10 / f64x8::splat(4.0);
            let t12 = ((t10).sqrt());
            let t14 = t11 + f64x8::splat(1.86372) * t12 + f64x8::splat(12.9352);
            let t15 = f64x8::splat(1.0) / t14;
            let t19 = (simd::ln(t4 * t9 * t15 / f64x8::splat(4.0)));
            let t20 = f64x8::splat(0.0310907) * t19;
            let t21 = t12 + f64x8::splat(3.72744);
            let t24 = (simd::atan(f64x8::splat(6.15199081975908) / t21));
            let t25 = f64x8::splat(0.038783294878113016) * t24;
            let t26 = t12 / f64x8::splat(2.0);
            let t27 = t26 + f64x8::splat(0.10498);
            let t28 = t27 * t27;
            let t30 = (simd::ln(t28 * t15));
            let t31 = f64x8::splat(0.0009690227711544374) * t30;
            let t32 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t33 = f64x8::splat(1.0) / t32;
            let t35 = t11 + f64x8::splat(0.565535) * t12 + f64x8::splat(13.0045);
            let t36 = f64x8::splat(1.0) / t35;
            let t40 = (simd::ln(t4 * t9 * t36 / f64x8::splat(4.0)));
            let t41 = t12 + f64x8::splat(1.13107);
            let t44 = (simd::atan(f64x8::splat(7.123108917818118) / t41));
            let t46 = t26 + f64x8::splat(0.0047584);
            let t47 = t46 * t46;
            let t49 = (simd::ln(t47 * t36));
            let t53 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t54 = (simd::cbrt(zeta_threshold));
            let t56 = ((t53).select(t54 * zeta_threshold, f64x8::splat(1.0)));
            let t59 = f64x8::splat(M_CBRT2);
            let t60 = t59 - f64x8::splat(1.0);
            let t65 = f64x8::splat(9.0) * t56 - f64x8::splat(9.0);
            let t67 = t33 * (t40 + f64x8::splat(0.31770800474394145) * t44 + f64x8::splat(0.00041403379428206277) * t49) * t65 / f64x8::splat(24.0);
            let t68 = v_rho * v_rho;
            let t70 = f64x8::splat(1.0) / t7 / t68;
            let t71 = v_sigma * t70;
            let t72 = param_aa + param_bb;
            let t73 = param_ftilde * t72;
            let t74 = param_malpha * t1;
            let t75 = t3 * t6;
            let t76 = t75 * t8;
            let t79 = t1 * t1;
            let t80 = param_mbeta * t79;
            let t81 = t3 * t3;
            let t82 = t81 * t5;
            let t83 = t7 * t7;
            let t84 = f64x8::splat(1.0) / t83;
            let t85 = t82 * t84;
            let t88 = param_bb + t74 * t76 / f64x8::splat(4.0) + t80 * t85 / f64x8::splat(4.0);
            let t89 = param_mgamma * t1;
            let t92 = param_mdelta * t79;
            let t95 = f64x8::splat(1.0) / v_rho;
            let t98 = f64x8::splat(1.0) + t89 * t76 / f64x8::splat(4.0) + t92 * t85 / f64x8::splat(4.0) + f64x8::splat(2387.32414637843) * param_mbeta * t95;
            let t99 = f64x8::splat(1.0) / t98;
            let t101 = t88 * t99 + param_aa;
            let t102 = f64x8::splat(1.0) / t101;
            let t103 = ((v_sigma).sqrt());
            let t104 = t102 * t103;
            let t105 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t107 = f64x8::splat(1.0) / t105 / v_rho;
            let t110 = (simd::exp(-t73 * t104 * t107));
            let t112 = t54 * t54;
            let t114 = ((t53).select(t112 * zeta_threshold, f64x8::splat(1.0)));
            let t115 = ((t114).sqrt());
            let t116 = f64x8::splat(1.0) / t115;
            let t117 = t110 * t101 * t116;
            let t118 = t71 * t117;
            let tzk0 = t20 + t25 + t31 - t67 + t118;
            acc_zk = tzk0;
            let t120 = f64x8::splat(1.0) / t7 / v_rho;
            let t121 = t6 * t120;
            let t125 = t4 * t6;
            let t126 = t14 * t14;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t8 * t127;
            let t129 = t4 * t121;
            let t130 = t129 / f64x8::splat(12.0);
            let t131 = f64x8::splat(1.0) / t12;
            let t132 = t131 * t1;
            let t133 = t75 * t120;
            let t134 = t132 * t133;
            let t136 = -t130 - f64x8::splat(0.31062) * t134;
            let t142 = f64x8::splat(1.0) / t3;
            let t143 = (-t4 * t121 * t15 / f64x8::splat(12.0) - t125 * t128 * t136 / f64x8::splat(4.0)) * t79 * t142;
            let t144 = t5 * t7;
            let t145 = t144 * t14;
            let t146 = t143 * t145;
            let t148 = t21 * t21;
            let t149 = f64x8::splat(1.0) / t148;
            let t151 = t149 * t131 * t1;
            let t153 = f64x8::splat(37.8469910464) * t149 + f64x8::splat(1.0);
            let t154 = f64x8::splat(1.0) / t153;
            let t157 = t151 * t75 * t120 * t154;
            let t159 = t27 * t15;
            let t160 = t159 * t131;
            let t163 = t28 * t127;
            let t165 = -t160 * t129 / f64x8::splat(6.0) - t163 * t136;
            let t166 = f64x8::splat(1.0) / t28;
            let t167 = t165 * t166;
            let t168 = t167 * t14;
            let t173 = t35 * t35;
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t8 * t174;
            let t177 = -t130 - f64x8::splat(0.09425583333333333) * t134;
            let t183 = (-t4 * t121 * t36 / f64x8::splat(12.0) - t125 * t175 * t177 / f64x8::splat(4.0)) * t79 * t142;
            let t184 = t144 * t35;
            let t187 = t41 * t41;
            let t188 = f64x8::splat(1.0) / t187;
            let t190 = t188 * t131 * t1;
            let t192 = f64x8::splat(50.7386806551) * t188 + f64x8::splat(1.0);
            let t193 = f64x8::splat(1.0) / t192;
            let t198 = t46 * t36;
            let t199 = t198 * t131;
            let t202 = t47 * t174;
            let t204 = -t199 * t129 / f64x8::splat(6.0) - t202 * t177;
            let t205 = f64x8::splat(1.0) / t47;
            let t206 = t204 * t205;
            let t211 = t33 * (t183 * t184 / f64x8::splat(3.0) + f64x8::splat(0.37717812030896175) * t190 * t75 * t120 * t193 + f64x8::splat(0.00041403379428206277) * t206 * t35) * t65;
            let t213 = t68 * v_rho;
            let t215 = f64x8::splat(1.0) / t7 / t213;
            let t216 = v_sigma * t215;
            let t217 = t216 * t117;
            let t219 = t101 * t101;
            let t220 = f64x8::splat(1.0) / t219;
            let t221 = t73 * t220;
            let t222 = t103 * t107;
            let t226 = f64x8::splat(1.0) / t83 / v_rho;
            let t227 = t82 * t226;
            let t230 = -t74 * t133 / f64x8::splat(12.0) - t80 * t227 / f64x8::splat(6.0);
            let t232 = t98 * t98;
            let t233 = f64x8::splat(1.0) / t232;
            let t234 = t88 * t233;
            let t242 = -t89 * t133 / f64x8::splat(12.0) - t92 * t227 / f64x8::splat(6.0) - f64x8::splat(2387.32414637843) * param_mbeta / t68;
            let t244 = t230 * t99 - t234 * t242;
            let t248 = f64x8::splat(1.0) / t105 / t68;
            let t252 = t221 * t222 * t244 + f64x8::splat(7.0) / f64x8::splat(6.0) * t73 * t104 * t248;
            let t253 = t71 * t252;
            let t254 = t253 * t117;
            let t256 = t110 * t244 * t116;
            let t257 = t71 * t256;
            let tvrho0 = t20 + t25 + t31 - t67 + t118 + v_rho * (f64x8::splat(0.010363566666666667) * t146 + f64x8::splat(0.03976574567502677) * t157 + f64x8::splat(0.0009690227711544374) * t168 - t211 / f64x8::splat(24.0) - f64x8::splat(7.0) / f64x8::splat(3.0) * t217 + t254 + t257);
            acc_vrho = tvrho0;
            let t260 = t70 * t110;
            let t261 = t101 * t116;
            let t262 = t260 * t261;
            let t263 = ((v_rho).sqrt());
            let t265 = f64x8::splat(1.0) / t263 / t213;
            let t266 = t103 * t265;
            let t267 = t266 * param_ftilde;
            let t269 = t72 * t110 * t116;
            let t271 = t267 * t269 / f64x8::splat(2.0);
            let tvsigma0 = v_rho * (t262 - t271);
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
