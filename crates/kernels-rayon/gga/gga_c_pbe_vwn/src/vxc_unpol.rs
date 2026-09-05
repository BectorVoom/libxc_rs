//! GGA_C_PBE_VWN vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbe_vwn.c`
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
pub fn gga_c_pbe_vwn_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_gamma: f64,
    param_BB: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma = f64x8::splat(param_gamma);
    let param_BB = f64x8::splat(param_BB);
    let param_beta = f64x8::splat(param_beta);
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
            let t68 = t54 * t54;
            let t69 = ((t53).select(t68, f64x8::splat(1.0)));
            let t70 = t69 * t69;
            let t71 = t70 * t69;
            let t72 = param_gamma * t71;
            let t73 = v_rho * v_rho;
            let t75 = f64x8::splat(1.0) / t7 / t73;
            let t78 = f64x8::splat(1.0) / t70;
            let t79 = t1 * t1;
            let t81 = f64x8::splat(1.0) / t3;
            let t82 = t81 * t5;
            let t83 = t78 * t79 * t82;
            let t86 = param_BB * param_beta;
            let t87 = f64x8::splat(1.0) / param_gamma;
            let t90 = f64x8::splat(1.0) / t71;
            let t92 = (simd::exp(-(t20 + t25 + t31 - t67) * t87 * t90));
            let t93 = t92 - f64x8::splat(1.0);
            let t94 = f64x8::splat(1.0) / t93;
            let t95 = t87 * t94;
            let t96 = v_sigma * v_sigma;
            let t98 = t86 * t95 * t96;
            let t99 = t73 * t73;
            let t100 = t7 * t7;
            let t102 = f64x8::splat(1.0) / t100 / t99;
            let t103 = t59 * t59;
            let t104 = t102 * t103;
            let t105 = t70 * t70;
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t104 * t106;
            let t108 = t3 * t3;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t1 * t109;
            let t111 = t110 * t6;
            let t112 = t107 * t111;
            let t115 = v_sigma * t75 * t59 * t83 / f64x8::splat(96.0) + t98 * t112 / f64x8::splat(3072.0);
            let t116 = param_beta * t115;
            let t117 = param_beta * t87;
            let t120 = t117 * t94 * t115 + f64x8::splat(1.0);
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t87 * t121;
            let t124 = t116 * t122 + f64x8::splat(1.0);
            let t125 = (simd::ln(t124));
            let t126 = t72 * t125;
            let tzk0 = t20 + t25 + t31 - t67 + t126;
            acc_zk = tzk0;
            let t128 = f64x8::splat(1.0) / t7 / v_rho;
            let t129 = t6 * t128;
            let t133 = t4 * t6;
            let t134 = t14 * t14;
            let t135 = f64x8::splat(1.0) / t134;
            let t136 = t8 * t135;
            let t137 = t4 * t129;
            let t138 = t137 / f64x8::splat(12.0);
            let t139 = f64x8::splat(1.0) / t12;
            let t140 = t139 * t1;
            let t141 = t3 * t6;
            let t143 = t140 * t141 * t128;
            let t145 = -t138 - f64x8::splat(0.31062) * t143;
            let t151 = (-t4 * t129 * t15 / f64x8::splat(12.0) - t133 * t136 * t145 / f64x8::splat(4.0)) * t79 * t81;
            let t152 = t5 * t7;
            let t153 = t152 * t14;
            let t154 = t151 * t153;
            let t155 = f64x8::splat(0.010363566666666667) * t154;
            let t156 = t21 * t21;
            let t157 = f64x8::splat(1.0) / t156;
            let t159 = t157 * t139 * t1;
            let t161 = f64x8::splat(37.8469910464) * t157 + f64x8::splat(1.0);
            let t162 = f64x8::splat(1.0) / t161;
            let t165 = t159 * t141 * t128 * t162;
            let t166 = f64x8::splat(0.03976574567502677) * t165;
            let t167 = t27 * t15;
            let t168 = t167 * t139;
            let t171 = t28 * t135;
            let t173 = -t168 * t137 / f64x8::splat(6.0) - t171 * t145;
            let t174 = f64x8::splat(1.0) / t28;
            let t175 = t173 * t174;
            let t176 = t175 * t14;
            let t177 = f64x8::splat(0.0009690227711544374) * t176;
            let t181 = t35 * t35;
            let t182 = f64x8::splat(1.0) / t181;
            let t183 = t8 * t182;
            let t185 = -t138 - f64x8::splat(0.09425583333333333) * t143;
            let t191 = (-t4 * t129 * t36 / f64x8::splat(12.0) - t133 * t183 * t185 / f64x8::splat(4.0)) * t79 * t81;
            let t192 = t152 * t35;
            let t195 = t41 * t41;
            let t196 = f64x8::splat(1.0) / t195;
            let t198 = t196 * t139 * t1;
            let t200 = f64x8::splat(50.7386806551) * t196 + f64x8::splat(1.0);
            let t201 = f64x8::splat(1.0) / t200;
            let t206 = t46 * t36;
            let t207 = t206 * t139;
            let t210 = t47 * t182;
            let t212 = -t207 * t137 / f64x8::splat(6.0) - t210 * t185;
            let t213 = f64x8::splat(1.0) / t47;
            let t214 = t212 * t213;
            let t219 = t33 * (t191 * t192 / f64x8::splat(3.0) + f64x8::splat(0.37717812030896175) * t198 * t141 * t128 * t201 + f64x8::splat(0.00041403379428206277) * t214 * t35) * t65;
            let t220 = t219 / f64x8::splat(24.0);
            let t221 = t73 * v_rho;
            let t223 = f64x8::splat(1.0) / t7 / t221;
            let t228 = param_gamma * param_gamma;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t86 * t229;
            let t231 = t93 * t93;
            let t232 = f64x8::splat(1.0) / t231;
            let t233 = t232 * t96;
            let t234 = t233 * t102;
            let t235 = t230 * t234;
            let t237 = f64x8::splat(1.0) / t105 / t71;
            let t238 = t103 * t237;
            let t239 = t238 * t1;
            let t240 = t109 * t6;
            let t241 = t155 + t166 + t177 - t220;
            let t242 = t241 * t92;
            let t243 = t240 * t242;
            let t244 = t239 * t243;
            let t247 = t99 * v_rho;
            let t249 = f64x8::splat(1.0) / t100 / t247;
            let t250 = t249 * t103;
            let t251 = t250 * t106;
            let t252 = t251 * t111;
            let t255 = -f64x8::splat(7.0) / f64x8::splat(288.0) * v_sigma * t223 * t59 * t83 + t235 * t244 / f64x8::splat(3072.0) - f64x8::splat(7.0) / f64x8::splat(4608.0) * t98 * t252;
            let t256 = param_beta * t255;
            let t258 = t120 * t120;
            let t259 = f64x8::splat(1.0) / t258;
            let t260 = t87 * t259;
            let t262 = param_beta * t229 * t232;
            let t264 = t90 * t92;
            let t269 = t262 * t115 * t241 * t264 + t117 * t94 * t255;
            let t270 = t260 * t269;
            let t272 = -t116 * t270 + t256 * t122;
            let t273 = f64x8::splat(1.0) / t124;
            let t275 = t72 * t272 * t273;
            let tvrho0 = t20 + t25 + t31 - t67 + t126 + v_rho * (t155 + t166 + t177 - t220 + t275);
            acc_vrho = tvrho0;
            let t278 = v_rho * param_gamma;
            let t282 = t79 * t81 * t5;
            let t286 = t86 * t95 * v_sigma;
            let t289 = t75 * t59 * t78 * t282 / f64x8::splat(96.0) + t286 * t112 / f64x8::splat(1536.0);
            let t290 = param_beta * t289;
            let t292 = param_beta * param_beta;
            let t293 = t292 * t115;
            let t294 = t293 * t229;
            let t295 = t259 * t94;
            let t296 = t295 * t289;
            let t298 = t290 * t122 - t294 * t296;
            let tvsigma0 = t278 * t71 * t298 * t273;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
