//! LDA_C_VWN_3 vxc unpol kernel — explicit SIMD (exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_3.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py (exact math). Eight grid points per step; every lane runs maple2c's expression
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
pub fn lda_c_vwn_3_vxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
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
            let t33 = t11 + f64x8::splat(3.53021) * t12 + f64x8::splat(18.0578);
            let t34 = f64x8::splat(1.0) / t33;
            let t38 = (simd::ln(t4 * t9 * t34 / f64x8::splat(4.0)));
            let t40 = t12 + f64x8::splat(7.06042);
            let t43 = (simd::atan(f64x8::splat(4.730926909560113) / t40));
            let t45 = t26 + f64x8::splat(0.325);
            let t46 = t45 * t45;
            let t48 = (simd::ln(t46 * t34));
            let t50 = f64x8::splat(0.01554535) * t38 + f64x8::splat(0.05249139316978094) * t43 + f64x8::splat(0.0022478670955426118) * t48 - t20 - t25 - t31;
            let t52 = t11 + f64x8::splat(10.06155) * t12 + f64x8::splat(101.578);
            let t53 = f64x8::splat(1.0) / t52;
            let t57 = (simd::ln(t4 * t9 * t53 / f64x8::splat(4.0)));
            let t59 = t12 + f64x8::splat(20.1231);
            let t62 = (simd::atan(f64x8::splat(1.171685277708993) / t59));
            let t64 = t26 + f64x8::splat(0.743294);
            let t65 = t64 * t64;
            let t67 = (simd::ln(t65 * t53));
            let t70 = t11 + f64x8::splat(6.536) * t12 + f64x8::splat(42.7198);
            let t71 = f64x8::splat(1.0) / t70;
            let t75 = (simd::ln(t4 * t9 * t71 / f64x8::splat(4.0)));
            let t77 = t12 + f64x8::splat(13.072);
            let t80 = (simd::atan(f64x8::splat(0.0448998886412873) / t77));
            let t82 = t26 + f64x8::splat(0.409286);
            let t83 = t82 * t82;
            let t85 = (simd::ln(t83 * t71));
            let t87 = f64x8::splat(0.01554535) * t57 + f64x8::splat(0.6188180297906063) * t62 + f64x8::splat(0.002667310007273315) * t67 - f64x8::splat(0.0310907) * t75 - f64x8::splat(20.521972937837504) * t80 - f64x8::splat(0.004431373767749538) * t85;
            let t88 = f64x8::splat(1.0) / t87;
            let t90 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t91 = f64x8::splat(1.0) / t90;
            let t92 = t50 * t88 * t91;
            let t94 = t11 + f64x8::splat(0.534175) * t12 + f64x8::splat(11.4813);
            let t95 = f64x8::splat(1.0) / t94;
            let t99 = (simd::ln(t4 * t9 * t95 / f64x8::splat(4.0)));
            let t100 = t12 + f64x8::splat(1.06835);
            let t103 = (simd::atan(f64x8::splat(6.692072046645942) / t100));
            let t105 = t26 + f64x8::splat(0.228344);
            let t106 = t105 * t105;
            let t108 = (simd::ln(t106 * t95));
            let t110 = t99 + f64x8::splat(0.32323836906055065) * t103 + f64x8::splat(0.021608710360898266) * t108;
            let t112 = (simd::cbrt(zeta_threshold));
            let t114 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t112 * zeta_threshold, f64x8::splat(1.0)));
            let t116 = f64x8::splat(2.0) * t114 - f64x8::splat(2.0);
            let t118 = f64x8::splat(M_CBRT2);
            let t119 = t118 - f64x8::splat(1.0);
            let t121 = f64x8::splat(1.0) / t119 / f64x8::splat(2.0);
            let t122 = f64x8::splat(9.0) * t119;
            let t123 = t121 * t122;
            let t124 = t110 * t116 * t123;
            let t126 = t92 * t124 / f64x8::splat(24.0);
            let tzk0 = t20 + t25 + t31 - t126;
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
            let t150 = t1 * t1;
            let t152 = f64x8::splat(1.0) / t3;
            let t153 = (-t4 * t129 * t15 / f64x8::splat(12.0) - t133 * t136 * t145 / f64x8::splat(4.0)) * t150 * t152;
            let t154 = t5 * t7;
            let t155 = t154 * t14;
            let t156 = t153 * t155;
            let t157 = f64x8::splat(0.010363566666666667) * t156;
            let t158 = t21 * t21;
            let t159 = f64x8::splat(1.0) / t158;
            let t161 = t159 * t139 * t1;
            let t163 = f64x8::splat(37.8469910464) * t159 + f64x8::splat(1.0);
            let t164 = f64x8::splat(1.0) / t163;
            let t167 = t161 * t141 * t128 * t164;
            let t168 = f64x8::splat(0.03976574567502677) * t167;
            let t169 = t27 * t15;
            let t170 = t169 * t139;
            let t173 = t28 * t135;
            let t175 = -t170 * t137 / f64x8::splat(6.0) - t173 * t145;
            let t176 = f64x8::splat(1.0) / t28;
            let t177 = t175 * t176;
            let t178 = t177 * t14;
            let t179 = f64x8::splat(0.0009690227711544374) * t178;
            let t183 = t33 * t33;
            let t184 = f64x8::splat(1.0) / t183;
            let t185 = t8 * t184;
            let t187 = -t138 - f64x8::splat(0.5883683333333334) * t143;
            let t193 = (-t4 * t129 * t34 / f64x8::splat(12.0) - t133 * t185 * t187 / f64x8::splat(4.0)) * t150 * t152;
            let t194 = t154 * t33;
            let t197 = t40 * t40;
            let t198 = f64x8::splat(1.0) / t197;
            let t200 = t198 * t139 * t1;
            let t202 = f64x8::splat(22.3816694236) * t198 + f64x8::splat(1.0);
            let t203 = f64x8::splat(1.0) / t202;
            let t208 = t45 * t34;
            let t209 = t208 * t139;
            let t212 = t46 * t184;
            let t214 = -t209 * t137 / f64x8::splat(6.0) - t212 * t187;
            let t215 = f64x8::splat(1.0) / t46;
            let t216 = t214 * t215;
            let t219 = f64x8::splat(0.005181783333333334) * t193 * t194 + f64x8::splat(0.041388824077869424) * t200 * t141 * t128 * t203 + f64x8::splat(0.0022478670955426118) * t216 * t33 - t157 - t168 - t179;
            let t221 = t219 * t88 * t91;
            let t222 = t221 * t124;
            let t224 = t87 * t87;
            let t225 = f64x8::splat(1.0) / t224;
            let t226 = t50 * t225;
            let t227 = t91 * t110;
            let t228 = t226 * t227;
            let t229 = t116 * t121;
            let t233 = t52 * t52;
            let t234 = f64x8::splat(1.0) / t233;
            let t235 = t8 * t234;
            let t237 = -t138 - f64x8::splat(1.676925) * t143;
            let t243 = (-t4 * t129 * t53 / f64x8::splat(12.0) - t133 * t235 * t237 / f64x8::splat(4.0)) * t150 * t152;
            let t244 = t154 * t52;
            let t247 = t59 * t59;
            let t248 = f64x8::splat(1.0) / t247;
            let t250 = t248 * t139 * t1;
            let t252 = f64x8::splat(1.37284639) * t248 + f64x8::splat(1.0);
            let t253 = f64x8::splat(1.0) / t252;
            let t258 = t64 * t53;
            let t259 = t258 * t139;
            let t262 = t65 * t234;
            let t264 = -t259 * t137 / f64x8::splat(6.0) - t262 * t237;
            let t265 = f64x8::splat(1.0) / t65;
            let t266 = t264 * t265;
            let t272 = t70 * t70;
            let t273 = f64x8::splat(1.0) / t272;
            let t274 = t8 * t273;
            let t276 = -t138 - f64x8::splat(1.0893333333333333) * t143;
            let t282 = (-t4 * t129 * t71 / f64x8::splat(12.0) - t133 * t274 * t276 / f64x8::splat(4.0)) * t150 * t152;
            let t283 = t154 * t70;
            let t286 = t77 * t77;
            let t287 = f64x8::splat(1.0) / t286;
            let t289 = t287 * t139 * t1;
            let t291 = f64x8::splat(0.002016) * t287 + f64x8::splat(1.0);
            let t292 = f64x8::splat(1.0) / t291;
            let t297 = t82 * t71;
            let t298 = t297 * t139;
            let t301 = t83 * t273;
            let t303 = -t298 * t137 / f64x8::splat(6.0) - t301 * t276;
            let t304 = f64x8::splat(1.0) / t83;
            let t305 = t303 * t304;
            let t308 = f64x8::splat(0.005181783333333334) * t243 * t244 + f64x8::splat(0.12084332918108974) * t250 * t141 * t128 * t253 + f64x8::splat(0.002667310007273315) * t266 * t52 - f64x8::splat(0.010363566666666667) * t282 * t283 - f64x8::splat(0.15357238326806924) * t289 * t141 * t128 * t292 - f64x8::splat(0.004431373767749538) * t305 * t70;
            let t309 = t122 * t308;
            let t310 = t229 * t309;
            let t311 = t228 * t310;
            let t316 = t94 * t94;
            let t317 = f64x8::splat(1.0) / t316;
            let t318 = t8 * t317;
            let t320 = -t138 - f64x8::splat(0.08902916666666667) * t143;
            let t326 = (-t4 * t129 * t95 / f64x8::splat(12.0) - t133 * t318 * t320 / f64x8::splat(4.0)) * t150 * t152;
            let t327 = t154 * t94;
            let t330 = t100 * t100;
            let t331 = f64x8::splat(1.0) / t330;
            let t333 = t331 * t139 * t1;
            let t335 = f64x8::splat(44.7838282775) * t331 + f64x8::splat(1.0);
            let t336 = f64x8::splat(1.0) / t335;
            let t341 = t105 * t95;
            let t342 = t341 * t139;
            let t345 = t106 * t317;
            let t347 = -t342 * t137 / f64x8::splat(6.0) - t345 * t320;
            let t348 = f64x8::splat(1.0) / t106;
            let t349 = t347 * t348;
            let t352 = t326 * t327 / f64x8::splat(3.0) + f64x8::splat(0.36052240899892257) * t333 * t141 * t128 * t336 + f64x8::splat(0.021608710360898266) * t349 * t94;
            let t354 = t352 * t116 * t123;
            let t355 = t92 * t354;
            let tvrho0 = t20 + t25 + t31 - t126 + v_rho * (t157 + t168 + t179 - t222 / f64x8::splat(24.0) + t311 / f64x8::splat(24.0) - t355 / f64x8::splat(24.0));
            acc_vrho = tvrho0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
