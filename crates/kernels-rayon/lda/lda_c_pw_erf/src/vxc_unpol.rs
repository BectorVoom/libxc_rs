//! LDA_C_PW_ERF vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pw_erf.c`
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
pub fn lda_c_pw_erf_vxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t10;
            let t13 = ((t10).sqrt());
            let t16 = ((t10) * (t10).sqrt());
            let t18 = t1 * t1;
            let t19 = t3 * t3;
            let t20 = t18 * t19;
            let t21 = t7 * t7;
            let t22 = f64x8::splat(1.0) / t21;
            let t23 = t5 * t22;
            let t24 = t20 * t23;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.062182) * t12 * t30;
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t50;
            let t54 = (simd::ln(t53));
            let t57 = f64x8::splat(0.019751789702565206) * t43 * t45 * t54;
            let t58 = (simd::ln(f64x8::splat(2.0)));
            let t59 = t58 - f64x8::splat(1.0);
            let t60 = f64x8::splat(2.0) * t59;
            let t62 = f64x8::splat(2.923025) * param_hyb_omega_0 * t13;
            let t64 = (simd::cbrt(f64x8::splat(9.0)));
            let t65 = t64 * t64;
            let t73 = param_hyb_omega_0 * param_hyb_omega_0;
            let t75 = (f64x8::splat(3.44851) - f64x8::splat(M_PI) * t5 * t65 * t3 / t59 / f64x8::splat(12.0)) * t73 * t1;
            let t76 = t3 * t6;
            let t77 = t76 * t8;
            let t80 = t73 * param_hyb_omega_0;
            let t81 = t13 * t10;
            let t84 = f64x8::splat(1.0) + t62 + t75 * t77 / f64x8::splat(4.0) + f64x8::splat(0.48968) * t80 * t81;
            let t85 = t73 * t1;
            let t88 = f64x8::splat(1.0) + t62 + f64x8::splat(0.8621275) * t85 * t77;
            let t89 = f64x8::splat(1.0) / t88;
            let t91 = (simd::ln(t84 * t89));
            let t93 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t94 = f64x8::splat(1.0) / t93;
            let t96 = f64x8::splat(1.0) / v_rho;
            let t100 = t3 * t2;
            let t101 = t1 * t100;
            let t103 = f64x8::splat(1.0) / t7 / v_rho;
            let t104 = t6 * t103;
            let t107 = f64x8::splat(1.0) + f64x8::splat(0.005175) * t10 + f64x8::splat(0.0204825) * t24 - f64x8::splat(0.0030486129349252553) * t96 + f64x8::splat(0.0003485625) * t101 * t104;
            let t110 = (simd::exp(-f64x8::splat(0.1881) * t10));
            let t111 = f64x8::splat(M_SQRT2);
            let t112 = t110 * t111;
            let t116 = t18 * t19 * t94;
            let t117 = t116 * t5;
            let t119 = f64x8::splat(1.0) / t21 / v_rho;
            let t121 = t4 * t9 * t39;
            let t124 = (f64x8::splat(1.0) - f64x8::splat(0.0056675) * t121) * t65;
            let t125 = f64x8::splat(1.0) / t100;
            let t126 = t124 * t125;
            let t127 = t1 * t21;
            let t129 = t39 * t39;
            let t133 = f64x8::splat(1.0) + f64x8::splat(0.107975) * t121 + f64x8::splat(0.01) * t20 * t23 * t129;
            let t134 = f64x8::splat(1.0) / t133;
            let t137 = t126 * t127 * t134 / f64x8::splat(15.0);
            let t139 = (simd::exp(-f64x8::splat(0.0775) * t10));
            let t142 = -f64x8::splat(1.2375) * t10 + t24 / f64x8::splat(4.0);
            let t143 = t139 * t142;
            let t144 = f64x8::splat(M_PI) * v_rho;
            let t147 = t137 + f64x8::splat(4.0) / f64x8::splat(3.0) * t143 * t144;
            let t154 = t107 * t110;
            let t156 = t154 / f64x8::splat(2.0) - f64x8::splat(1.0) / f64x8::splat(2.0);
            let t159 = t5 * t119;
            let t161 = (simd::exp(-f64x8::splat(0.13675) * t10));
            let t164 = -f64x8::splat(0.097) * t10 + f64x8::splat(0.169) * t24;
            let t166 = t161 * t164 * t1;
            let t168 = f64x8::splat(1.0) / t19 * t6;
            let t169 = t168 * t21;
            let t172 = t65 * t125;
            let t175 = t137 + t166 * t169 / f64x8::splat(3.0) - t172 * t127 / f64x8::splat(15.0);
            let t179 = -t32 + t57;
            let t184 = t73 * t73;
            let t186 = t116 * t159;
            let t187 = t184 * param_hyb_omega_0;
            let t188 = t111 * t187;
            let t189 = t154 * t188;
            let t195 = v_rho * v_rho;
            let t196 = f64x8::splat(1.0) / t195;
            let t200 = t184 * t73;
            let t203 = f64x8::splat(1.0) / t21 / t195;
            let t205 = t184 * t184;
            let t209 = t60 * t91 * t94 + (-f64x8::splat(0.031505407223141116) * t96 * t107 * t112 - f64x8::splat(0.005388405304614574) * t117 * t119 * t147 * t111) * t80 + (-f64x8::splat(0.0837628205355044) * t96 * t156 - f64x8::splat(0.011938374665504766) * t116 * t159 * t175 + f64x8::splat(0.42708890021612717) * t101 * t104 * t179) * t184 - f64x8::splat(0.01197423401025461) * t186 * t189 + (-f64x8::splat(0.031835665774679375) * t116 * t159 * t156 + f64x8::splat(0.05332506774217938) * t196 * t179) * t200 + f64x8::splat(0.020267214298646783) * t117 * t203 * t179 * t205;
            let t213 = f64x8::splat(1.0) + f64x8::splat(0.15403623315025) * t20 * t23 * t73;
            let t214 = t213 * t213;
            let t215 = t214 * t214;
            let t216 = f64x8::splat(1.0) / t215;
            let t217 = t209 * t216;
            let tzk0 = -t32 + t57 - t217;
            acc_zk = tzk0;
            let t219 = t4 * t104 * t30;
            let t220 = f64x8::splat(0.0011073577833333333) * t219;
            let t221 = t26 * t26;
            let t222 = f64x8::splat(1.0) / t221;
            let t223 = t12 * t222;
            let t224 = f64x8::splat(1.0) / t13;
            let t225 = t224 * t1;
            let t226 = t76 * t103;
            let t227 = t225 * t226;
            let t229 = t4 * t104;
            let t231 = ((t10).sqrt());
            let t232 = t231 * t1;
            let t233 = t232 * t226;
            let t235 = t20 * t159;
            let t237 = -f64x8::splat(0.632975) * t227 - f64x8::splat(0.29896666666666666) * t229 - f64x8::splat(0.1023875) * t233 - f64x8::splat(0.08215666666666667) * t235;
            let t238 = f64x8::splat(1.0) / t29;
            let t239 = t237 * t238;
            let t240 = t223 * t239;
            let t241 = f64x8::splat(1.0) * t240;
            let t242 = t43 * t1;
            let t245 = t242 * t76 * t103 * t54;
            let t246 = f64x8::splat(0.0001831155503675316) * t245;
            let t247 = t43 * t45;
            let t248 = t50 * t50;
            let t249 = f64x8::splat(1.0) / t248;
            let t254 = -f64x8::splat(0.8630833333333333) * t227 - f64x8::splat(0.301925) * t229 - f64x8::splat(0.05501625) * t233 - f64x8::splat(0.082785) * t235;
            let t256 = f64x8::splat(1.0) / t53;
            let t257 = t249 * t254 * t256;
            let t258 = t247 * t257;
            let t259 = f64x8::splat(0.5848223397455204) * t258;
            let t261 = param_hyb_omega_0 * t224 * t1;
            let t263 = f64x8::splat(0.48717083333333333) * t261 * t226;
            let t267 = t80 * t13 * t1;
            let t270 = -t263 - t75 * t226 / f64x8::splat(12.0) - f64x8::splat(0.24484) * t267 * t226;
            let t272 = t88 * t88;
            let t273 = f64x8::splat(1.0) / t272;
            let t274 = t84 * t273;
            let t277 = -t263 - f64x8::splat(0.28737583333333333) * t85 * t226;
            let t280 = t60 * (t270 * t89 - t274 * t277);
            let t281 = f64x8::splat(1.0) / t84;
            let t283 = t281 * t88 * t94;
            let t292 = f64x8::splat(1.0) / t7 / t195;
            let t293 = t6 * t292;
            let t296 = -f64x8::splat(0.001725) * t229 - f64x8::splat(0.013655) * t235 + f64x8::splat(0.0030486129349252553) * t196 - f64x8::splat(0.00046475) * t101 * t293;
            let t302 = t76 * t112;
            let t309 = t18 * t6;
            let t310 = t309 * t22;
            let t311 = t39 * t65;
            let t312 = t311 * t134;
            let t314 = f64x8::splat(0.0003956661414271145) * t310 * t312;
            let t315 = t1 * t8;
            let t318 = f64x8::splat(2.0) / f64x8::splat(45.0) * t126 * t315 * t134;
            let t319 = t133 * t133;
            let t320 = f64x8::splat(1.0) / t319;
            let t327 = -f64x8::splat(0.035991666666666665) * t4 * t104 * t39 - f64x8::splat(0.006666666666666667) * t20 * t159 * t129;
            let t328 = t320 * t327;
            let t331 = t126 * t127 * t328 / f64x8::splat(15.0);
            let t332 = t4 * t6;
            let t333 = t8 * t139;
            let t339 = f64x8::splat(0.4125) * t229 - t235 / f64x8::splat(6.0);
            let t340 = t139 * t339;
            let t345 = t314 + t318 - t331 + f64x8::splat(0.10821041362364843) * t332 * t333 * t142 + f64x8::splat(4.0) / f64x8::splat(3.0) * t340 * t144 + f64x8::splat(4.0) / f64x8::splat(3.0) * t143 * f64x8::splat(M_PI);
            let t354 = t296 * t110;
            let t357 = t107 * t1 * t3;
            let t358 = t104 * t110;
            let t361 = t354 / f64x8::splat(2.0) + f64x8::splat(0.03135) * t357 * t358;
            let t364 = t5 * t203;
            let t370 = t18 / t3 * t5;
            let t371 = t22 * t161;
            let t377 = f64x8::splat(0.03233333333333333) * t229 - f64x8::splat(0.11266666666666666) * t235;
            let t379 = t161 * t377 * t1;
            let t382 = t168 * t8;
            let t387 = t314 + t318 - t331 + f64x8::splat(0.06077777777777778) * t370 * t371 * t164 + t379 * t169 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t166 * t382 - f64x8::splat(2.0) / f64x8::splat(45.0) * t172 * t315;
            let t394 = t220 + t241 - t246 - t259;
            let t400 = t116 * t364;
            let t403 = t354 * t188;
            let t406 = t195 * v_rho;
            let t407 = f64x8::splat(1.0) / t406;
            let t408 = t407 * t107;
            let t409 = t112 * t187;
            let t425 = f64x8::splat(1.0) / t21 / t406;
            let t434 = t280 * t283 + (f64x8::splat(0.031505407223141116) * t196 * t107 * t112 - f64x8::splat(0.031505407223141116) * t96 * t296 * t112 - f64x8::splat(0.001975389032890948) * t292 * t107 * t1 * t302 + f64x8::splat(0.008980675507690957) * t117 * t203 * t147 * t111 - f64x8::splat(0.005388405304614574) * t117 * t119 * t345 * t111) * t80 + (f64x8::splat(0.0837628205355044) * t196 * t156 - f64x8::splat(0.0837628205355044) * t96 * t361 + f64x8::splat(0.019897291109174608) * t116 * t364 * t175 - f64x8::splat(0.011938374665504766) * t116 * t159 * t387 - f64x8::splat(0.5694518669548363) * t101 * t293 * t179 + f64x8::splat(0.42708890021612717) * t101 * t104 * t394) * t184 + f64x8::splat(0.019957056683757683) * t400 * t189 - f64x8::splat(0.01197423401025461) * t186 * t403 - f64x8::splat(0.0002905674151788692) * t408 * t409 + (f64x8::splat(0.053059442957798957) * t116 * t364 * t156 - f64x8::splat(0.031835665774679375) * t116 * t159 * t361 - f64x8::splat(0.10665013548435875) * t407 * t179 + f64x8::splat(0.05332506774217938) * t196 * t394) * t200 - f64x8::splat(0.054045904796391424) * t117 * t425 * t179 * t205 + f64x8::splat(0.020267214298646783) * t117 * t203 * t394 * t205;
            let t435 = t434 * t216;
            let t437 = f64x8::splat(1.0) / t215 / t213;
            let t439 = t209 * t437 * t18;
            let t440 = t19 * t5;
            let t442 = t440 * t119 * t73;
            let t443 = t439 * t442;
            let tvrho0 = -t32 + t57 - t217 + v_rho * (t220 + t241 - t246 - t259 - t435 - f64x8::splat(0.41076328840066667) * t443);
            acc_vrho = tvrho0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
