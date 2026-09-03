//! GGA_C_BMK vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_bmk.c`
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
pub fn gga_c_bmk_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_ss_0: f64,
    param_c_ab_1: f64,
    param_c_ab_2: f64,
    param_c_ab_3: f64,
    param_c_ab_4: f64,
    param_c_ab_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c_ss_1 = f64x8::splat(param_c_ss_1);
    let param_c_ss_2 = f64x8::splat(param_c_ss_2);
    let param_c_ss_3 = f64x8::splat(param_c_ss_3);
    let param_c_ss_4 = f64x8::splat(param_c_ss_4);
    let param_c_ss_0 = f64x8::splat(param_c_ss_0);
    let param_c_ab_1 = f64x8::splat(param_c_ab_1);
    let param_c_ab_2 = f64x8::splat(param_c_ab_2);
    let param_c_ab_3 = f64x8::splat(param_c_ab_3);
    let param_c_ab_4 = f64x8::splat(param_c_ab_4);
    let param_c_ab_0 = f64x8::splat(param_c_ab_0);
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
            let t3 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t4 = ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold)) | (t3);
            let t5 = ((t3).select(zeta_threshold, f64x8::splat(1.0)));
            let t6 = f64x8::splat(M_CBRT3);
            let t7 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t8 = (simd::cbrt(t7));
            let t9 = t6 * t8;
            let t10 = f64x8::splat(M_CBRT4);
            let t11 = t10 * t10;
            let t12 = t9 * t11;
            let t13 = (simd::cbrt(v_rho));
            let t14 = f64x8::splat(1.0) / t13;
            let t15 = f64x8::splat(M_CBRT2);
            let t17 = (simd::cbrt(zeta_threshold));
            let t19 = ((t3).select(f64x8::splat(1.0) / t17, f64x8::splat(1.0)));
            let t21 = t12 * t14 * t15 * t19;
            let t23 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t21;
            let t24 = ((t21).sqrt());
            let t27 = ((t21) * (t21).sqrt());
            let t29 = t6 * t6;
            let t30 = t8 * t8;
            let t31 = t29 * t30;
            let t32 = t31 * t10;
            let t33 = t13 * t13;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t15 * t15;
            let t37 = t19 * t19;
            let t39 = t32 * t34 * t35 * t37;
            let t41 = f64x8::splat(3.79785) * t24 + f64x8::splat(0.8969) * t21 + f64x8::splat(0.204775) * t27 + f64x8::splat(0.123235) * t39;
            let t44 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t41;
            let t45 = (simd::ln(t44));
            let t47 = f64x8::splat(0.062182) * t23 * t45;
            let t49 = t17 * zeta_threshold;
            let t51 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t49, f64x8::splat(2.0) * t15));
            let t53 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t49, f64x8::splat(0.0)));
            let t57 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t15 - f64x8::splat(2.0));
            let t58 = (t51 + t53 - f64x8::splat(2.0)) * t57;
            let t60 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t21;
            let t65 = f64x8::splat(7.05945) * t24 + f64x8::splat(1.549425) * t21 + f64x8::splat(0.420775) * t27 + f64x8::splat(0.1562925) * t39;
            let t68 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t65;
            let t69 = (simd::ln(t68));
            let t73 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t21;
            let t78 = f64x8::splat(5.1785) * t24 + f64x8::splat(0.905775) * t21 + f64x8::splat(0.1100325) * t27 + f64x8::splat(0.1241775) * t39;
            let t81 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t78;
            let t82 = (simd::ln(t81));
            let t83 = t73 * t82;
            let t92 = ((t4).select(f64x8::splat(0.0), t5 * (-t47 + t58 * (-f64x8::splat(0.03109) * t60 * t69 + t47 - f64x8::splat(0.019751789702565206) * t83) + f64x8::splat(0.019751789702565206) * t58 * t83) / f64x8::splat(2.0)));
            let t94 = param_c_ss_1;
            let t95 = t94 * v_sigma;
            let t96 = v_rho * v_rho;
            let t98 = f64x8::splat(1.0) / t33 / t96;
            let t99 = t35 * t98;
            let t101 = v_sigma * t35 * t98;
            let t103 = f64x8::splat(1.0) + f64x8::splat(0.2) * t101;
            let t104 = f64x8::splat(1.0) / t103;
            let t108 = param_c_ss_2;
            let t109 = v_sigma * v_sigma;
            let t110 = t108 * t109;
            let t111 = t96 * t96;
            let t112 = t111 * v_rho;
            let t114 = f64x8::splat(1.0) / t13 / t112;
            let t115 = t15 * t114;
            let t116 = t103 * t103;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t115 * t117;
            let t121 = param_c_ss_3;
            let t122 = t109 * v_sigma;
            let t123 = t121 * t122;
            let t124 = t111 * t111;
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t116 * t103;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t125 * t127;
            let t131 = param_c_ss_4;
            let t132 = t109 * t109;
            let t133 = t131 * t132;
            let t134 = t124 * t96;
            let t136 = f64x8::splat(1.0) / t33 / t134;
            let t137 = t35 * t136;
            let t138 = t116 * t116;
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t137 * t139;
            let t143 = param_c_ss_0 + f64x8::splat(0.2) * t95 * t99 * t104 + f64x8::splat(0.08) * t110 * t118 + f64x8::splat(0.032) * t123 * t128 + f64x8::splat(0.0064) * t133 * t140;
            let t145 = f64x8::splat(2.0) * t92 * t143;
            let t147 = t9 * t11 * t14;
            let t149 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t147;
            let t150 = ((t147).sqrt());
            let t153 = ((t147) * (t147).sqrt());
            let t156 = t31 * t10 * t34;
            let t158 = f64x8::splat(3.79785) * t150 + f64x8::splat(0.8969) * t147 + f64x8::splat(0.204775) * t153 + f64x8::splat(0.123235) * t156;
            let t161 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t158;
            let t162 = (simd::ln(t161));
            let t165 = ((t3).select(t49, f64x8::splat(1.0)));
            let t168 = (f64x8::splat(2.0) * t165 - f64x8::splat(2.0)) * t57;
            let t170 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t147;
            let t175 = f64x8::splat(5.1785) * t150 + f64x8::splat(0.905775) * t147 + f64x8::splat(0.1100325) * t153 + f64x8::splat(0.1241775) * t156;
            let t178 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t175;
            let t179 = (simd::ln(t178));
            let t184 = -f64x8::splat(0.062182) * t149 * t162 + f64x8::splat(0.019751789702565206) * t168 * t170 * t179 - f64x8::splat(2.0) * t92;
            let t186 = param_c_ab_1;
            let t187 = t186 * v_sigma;
            let t189 = f64x8::splat(1.0) + f64x8::splat(0.006) * t101;
            let t190 = f64x8::splat(1.0) / t189;
            let t194 = param_c_ab_2;
            let t195 = t194 * t109;
            let t196 = t189 * t189;
            let t197 = f64x8::splat(1.0) / t196;
            let t198 = t115 * t197;
            let t201 = param_c_ab_3;
            let t202 = t201 * t122;
            let t203 = t196 * t189;
            let t204 = f64x8::splat(1.0) / t203;
            let t205 = t125 * t204;
            let t208 = param_c_ab_4;
            let t209 = t208 * t132;
            let t210 = t196 * t196;
            let t211 = f64x8::splat(1.0) / t210;
            let t212 = t137 * t211;
            let t215 = param_c_ab_0 + f64x8::splat(0.006) * t187 * t99 * t190 + f64x8::splat(7.2e-05) * t195 * t198 + f64x8::splat(8.64e-07) * t202 * t205 + f64x8::splat(5.184e-09) * t209 * t212;
            let t216 = t184 * t215;
            let tzk0 = t145 + t216;
            acc_zk = tzk0;
            let t218 = f64x8::splat(1.0) / t13 / v_rho;
            let t219 = t218 * t15;
            let t220 = t19 * t45;
            let t223 = f64x8::splat(0.0011073577833333333) * t12 * t219 * t220;
            let t224 = t41 * t41;
            let t225 = f64x8::splat(1.0) / t224;
            let t226 = t23 * t225;
            let t229 = f64x8::splat(1.0) / t24 * t6 * t8;
            let t230 = t11 * t218;
            let t231 = t15 * t19;
            let t232 = t230 * t231;
            let t233 = t229 * t232;
            let t235 = t219 * t19;
            let t236 = t12 * t235;
            let t238 = ((t21).sqrt());
            let t240 = t238 * t6 * t8;
            let t241 = t240 * t232;
            let t244 = f64x8::splat(1.0) / t33 / v_rho;
            let t247 = t32 * t244 * t35 * t37;
            let t249 = -f64x8::splat(0.632975) * t233 - f64x8::splat(0.29896666666666666) * t236 - f64x8::splat(0.1023875) * t241 - f64x8::splat(0.08215666666666667) * t247;
            let t250 = f64x8::splat(1.0) / t44;
            let t251 = t249 * t250;
            let t253 = f64x8::splat(1.0) * t226 * t251;
            let t254 = t19 * t69;
            let t258 = t65 * t65;
            let t259 = f64x8::splat(1.0) / t258;
            let t260 = t60 * t259;
            let t265 = -f64x8::splat(1.176575) * t233 - f64x8::splat(0.516475) * t236 - f64x8::splat(0.2103875) * t241 - f64x8::splat(0.104195) * t247;
            let t266 = f64x8::splat(1.0) / t68;
            let t267 = t265 * t266;
            let t270 = t19 * t82;
            let t274 = t78 * t78;
            let t275 = f64x8::splat(1.0) / t274;
            let t276 = t73 * t275;
            let t281 = -f64x8::splat(0.8630833333333333) * t233 - f64x8::splat(0.301925) * t236 - f64x8::splat(0.05501625) * t241 - f64x8::splat(0.082785) * t247;
            let t282 = f64x8::splat(1.0) / t81;
            let t283 = t281 * t282;
            let t288 = t58 * t9;
            let t289 = t231 * t82;
            let t293 = t58 * t73;
            let t295 = t275 * t281 * t282;
            let t301 = ((t4).select(f64x8::splat(0.0), t5 * (t223 + t253 + t58 * (f64x8::splat(0.0005323644333333333) * t12 * t219 * t254 + f64x8::splat(1.0) * t260 * t267 - t223 - t253 + f64x8::splat(0.0001831155503675316) * t12 * t219 * t270 + f64x8::splat(0.5848223397455204) * t276 * t283) - f64x8::splat(0.0001831155503675316) * t288 * t230 * t289 - f64x8::splat(0.5848223397455204) * t293 * t295) / f64x8::splat(2.0)));
            let t302 = t301 * t143;
            let t304 = t96 * v_rho;
            let t306 = f64x8::splat(1.0) / t33 / t304;
            let t307 = t35 * t306;
            let t311 = t94 * t109;
            let t312 = t111 * t96;
            let t314 = f64x8::splat(1.0) / t13 / t312;
            let t315 = t15 * t314;
            let t316 = t315 * t117;
            let t321 = t108 * t122;
            let t322 = t124 * v_rho;
            let t323 = f64x8::splat(1.0) / t322;
            let t324 = t323 * t127;
            let t329 = t121 * t132;
            let t330 = t124 * t304;
            let t332 = f64x8::splat(1.0) / t33 / t330;
            let t334 = t332 * t139 * t35;
            let t339 = t132 * v_sigma;
            let t340 = t131 * t339;
            let t341 = t124 * t312;
            let t344 = t15 / t13 / t341;
            let t346 = f64x8::splat(1.0) / t138 / t103;
            let t347 = t344 * t346;
            let t350 = -f64x8::splat(0.5333333333333333) * t95 * t307 * t104 + f64x8::splat(0.21333333333333335) * t311 * t316 - f64x8::splat(0.4266666666666667) * t110 * t316 + f64x8::splat(0.17066666666666666) * t321 * t324 - f64x8::splat(0.256) * t123 * t324 + f64x8::splat(0.0512) * t329 * t334 - f64x8::splat(0.06826666666666667) * t133 * t334 + f64x8::splat(0.027306666666666667) * t340 * t347;
            let t351 = t92 * t350;
            let t356 = t158 * t158;
            let t357 = f64x8::splat(1.0) / t356;
            let t358 = t149 * t357;
            let t360 = f64x8::splat(1.0) / t150 * t6;
            let t361 = t8 * t11;
            let t362 = t361 * t218;
            let t363 = t360 * t362;
            let t365 = t9 * t230;
            let t367 = ((t147).sqrt());
            let t368 = t367 * t6;
            let t369 = t368 * t362;
            let t372 = t31 * t10 * t244;
            let t374 = -f64x8::splat(0.632975) * t363 - f64x8::splat(0.29896666666666666) * t365 - f64x8::splat(0.1023875) * t369 - f64x8::splat(0.08215666666666667) * t372;
            let t375 = f64x8::splat(1.0) / t161;
            let t376 = t374 * t375;
            let t379 = t168 * t6;
            let t384 = t168 * t170;
            let t385 = t175 * t175;
            let t386 = f64x8::splat(1.0) / t385;
            let t391 = -f64x8::splat(0.8630833333333333) * t363 - f64x8::splat(0.301925) * t365 - f64x8::splat(0.05501625) * t369 - f64x8::splat(0.082785) * t372;
            let t393 = f64x8::splat(1.0) / t178;
            let t394 = t386 * t391 * t393;
            let t398 = f64x8::splat(0.0011073577833333333) * t9 * t230 * t162 + f64x8::splat(1.0) * t358 * t376 - f64x8::splat(0.0001831155503675316) * t379 * t361 * t218 * t179 - f64x8::splat(0.5848223397455204) * t384 * t394 - f64x8::splat(2.0) * t301;
            let t399 = t398 * t215;
            let t403 = t186 * t109;
            let t404 = t315 * t197;
            let t409 = t194 * t122;
            let t410 = t323 * t204;
            let t415 = t201 * t132;
            let t417 = t332 * t211 * t35;
            let t422 = t208 * t339;
            let t424 = f64x8::splat(1.0) / t210 / t189;
            let t425 = t344 * t424;
            let t428 = -f64x8::splat(0.016) * t187 * t307 * t190 + f64x8::splat(0.000192) * t403 * t404 - f64x8::splat(0.000384) * t195 * t404 + f64x8::splat(4.608e-06) * t409 * t410 - f64x8::splat(6.912e-06) * t202 * t410 + f64x8::splat(4.1472e-08) * t415 * t417 - f64x8::splat(5.5296e-08) * t209 * t417 + f64x8::splat(6.63552e-10) * t422 * t425;
            let t429 = t184 * t428;
            let tvrho0 = t145 + t216 + v_rho * (f64x8::splat(2.0) * t302 + f64x8::splat(2.0) * t351 + t399 + t429);
            acc_vrho = tvrho0;
            let t432 = t94 * t35;
            let t438 = t108 * v_sigma;
            let t443 = t121 * t109;
            let t448 = t131 * t122;
            let t451 = t124 * t112;
            let t454 = t15 / t13 / t451;
            let t455 = t454 * t346;
            let t458 = f64x8::splat(0.2) * t432 * t98 * t104 - f64x8::splat(0.08) * t95 * t118 + f64x8::splat(0.16) * t438 * t118 - f64x8::splat(0.064) * t110 * t128 + f64x8::splat(0.096) * t443 * t128 - f64x8::splat(0.0192) * t123 * t140 + f64x8::splat(0.0256) * t448 * t140 - f64x8::splat(0.01024) * t133 * t455;
            let t460 = f64x8::splat(2.0) * t92 * t458;
            let t461 = t186 * t35;
            let t467 = t194 * v_sigma;
            let t472 = t201 * t109;
            let t477 = t208 * t122;
            let t480 = t454 * t424;
            let t483 = f64x8::splat(0.006) * t461 * t98 * t190 - f64x8::splat(7.2e-05) * t187 * t198 + f64x8::splat(0.000144) * t467 * t198 - f64x8::splat(1.728e-06) * t195 * t205 + f64x8::splat(2.592e-06) * t472 * t205 - f64x8::splat(1.5552e-08) * t202 * t212 + f64x8::splat(2.0736e-08) * t477 * t212 - f64x8::splat(2.48832e-10) * t209 * t480;
            let t484 = t184 * t483;
            let tvsigma0 = v_rho * (t460 + t484);
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
