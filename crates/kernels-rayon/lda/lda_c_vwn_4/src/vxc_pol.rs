//! LDA_C_VWN_4 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_4.c`
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
pub fn lda_c_vwn_4_vxc_pol(
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
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = t6 * t9;
            let t11 = t4 * t10;
            let t12 = t11 / f64x8::splat(4.0);
            let t13 = ((t11).sqrt());
            let t15 = t12 + f64x8::splat(1.86372) * t13 + f64x8::splat(12.9352);
            let t16 = f64x8::splat(1.0) / t15;
            let t20 = (simd::ln(t4 * t10 * t16 / f64x8::splat(4.0)));
            let t21 = f64x8::splat(0.0310907) * t20;
            let t22 = t13 + f64x8::splat(3.72744);
            let t25 = (simd::atan(f64x8::splat(6.15199081975908) / t22));
            let t26 = f64x8::splat(0.038783294878113016) * t25;
            let t27 = t13 / f64x8::splat(2.0);
            let t28 = t27 + f64x8::splat(0.10498);
            let t29 = t28 * t28;
            let t31 = (simd::ln(t29 * t16));
            let t32 = f64x8::splat(0.0009690227711544374) * t31;
            let t33 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t34 = f64x8::splat(1.0) / t33;
            let t36 = t12 + f64x8::splat(0.534175) * t13 + f64x8::splat(11.4813);
            let t37 = f64x8::splat(1.0) / t36;
            let t41 = (simd::ln(t4 * t10 * t37 / f64x8::splat(4.0)));
            let t42 = t13 + f64x8::splat(1.06835);
            let t45 = (simd::atan(f64x8::splat(6.692072046645942) / t42));
            let t47 = t27 + f64x8::splat(0.228344);
            let t48 = t47 * t47;
            let t50 = (simd::ln(t48 * t37));
            let t53 = t34 * (t41 + f64x8::splat(0.32323836906055065) * t45 + f64x8::splat(0.021608710360898266) * t50);
            let t54 = v_rho0 - v_rho1;
            let t55 = f64x8::splat(1.0) / t7;
            let t56 = t54 * t55;
            let t57 = f64x8::splat(1.0) + t56;
            let t58 = (t57).simd_le(zeta_threshold);
            let t59 = (simd::cbrt(zeta_threshold));
            let t60 = t59 * zeta_threshold;
            let t61 = (simd::cbrt(t57));
            let t63 = ((t58).select(t60, t61 * t57));
            let t64 = f64x8::splat(1.0) - t56;
            let t65 = (t64).simd_le(zeta_threshold);
            let t66 = (simd::cbrt(t64));
            let t68 = ((t65).select(t60, t66 * t64));
            let t69 = t63 + t68 - f64x8::splat(2.0);
            let t70 = t53 * t69;
            let t71 = f64x8::splat(M_CBRT2);
            let t72 = t71 - f64x8::splat(1.0);
            let t74 = f64x8::splat(1.0) / t72 / f64x8::splat(2.0);
            let t75 = t54 * t54;
            let t76 = t75 * t75;
            let t77 = t7 * t7;
            let t78 = t77 * t77;
            let t79 = f64x8::splat(1.0) / t78;
            let t83 = f64x8::splat(9.0) * t72;
            let t84 = t74 * (-t76 * t79 + f64x8::splat(1.0)) * t83;
            let t86 = t70 * t84 / f64x8::splat(24.0);
            let t88 = t12 + f64x8::splat(3.53021) * t13 + f64x8::splat(18.0578);
            let t89 = f64x8::splat(1.0) / t88;
            let t93 = (simd::ln(t4 * t10 * t89 / f64x8::splat(4.0)));
            let t95 = t13 + f64x8::splat(7.06042);
            let t98 = (simd::atan(f64x8::splat(4.730926909560113) / t95));
            let t100 = t27 + f64x8::splat(0.325);
            let t101 = t100 * t100;
            let t103 = (simd::ln(t101 * t89));
            let t105 = f64x8::splat(0.01554535) * t93 + f64x8::splat(0.05249139316978094) * t98 + f64x8::splat(0.0022478670955426118) * t103 - t21 - t26 - t32;
            let t106 = t105 * t69;
            let t107 = t74 * t76;
            let t108 = t107 * t79;
            let t109 = t106 * t108;
            let tzk0 = t21 + t26 + t32 - t86 + t109;
            acc_zk = tzk0;
            let t111 = f64x8::splat(1.0) / t8 / t7;
            let t112 = t6 * t111;
            let t116 = t4 * t6;
            let t117 = t15 * t15;
            let t118 = f64x8::splat(1.0) / t117;
            let t119 = t9 * t118;
            let t120 = t4 * t112;
            let t121 = t120 / f64x8::splat(12.0);
            let t122 = f64x8::splat(1.0) / t13;
            let t123 = t122 * t1;
            let t124 = t3 * t6;
            let t126 = t123 * t124 * t111;
            let t128 = -t121 - f64x8::splat(0.31062) * t126;
            let t133 = t1 * t1;
            let t135 = f64x8::splat(1.0) / t3;
            let t136 = (-t4 * t112 * t16 / f64x8::splat(12.0) - t116 * t119 * t128 / f64x8::splat(4.0)) * t133 * t135;
            let t137 = t5 * t8;
            let t138 = t137 * t15;
            let t139 = t136 * t138;
            let t140 = f64x8::splat(0.010363566666666667) * t139;
            let t141 = t22 * t22;
            let t142 = f64x8::splat(1.0) / t141;
            let t144 = t142 * t122 * t1;
            let t146 = f64x8::splat(37.8469910464) * t142 + f64x8::splat(1.0);
            let t147 = f64x8::splat(1.0) / t146;
            let t150 = t144 * t124 * t111 * t147;
            let t151 = f64x8::splat(0.03976574567502677) * t150;
            let t152 = t28 * t16;
            let t153 = t152 * t122;
            let t156 = t29 * t118;
            let t158 = -t153 * t120 / f64x8::splat(6.0) - t156 * t128;
            let t159 = f64x8::splat(1.0) / t29;
            let t160 = t158 * t159;
            let t161 = t160 * t15;
            let t162 = f64x8::splat(0.0009690227711544374) * t161;
            let t166 = t36 * t36;
            let t167 = f64x8::splat(1.0) / t166;
            let t168 = t9 * t167;
            let t170 = -t121 - f64x8::splat(0.08902916666666667) * t126;
            let t176 = (-t4 * t112 * t37 / f64x8::splat(12.0) - t116 * t168 * t170 / f64x8::splat(4.0)) * t133 * t135;
            let t177 = t137 * t36;
            let t180 = t42 * t42;
            let t181 = f64x8::splat(1.0) / t180;
            let t183 = t181 * t122 * t1;
            let t185 = f64x8::splat(44.7838282775) * t181 + f64x8::splat(1.0);
            let t186 = f64x8::splat(1.0) / t185;
            let t191 = t47 * t37;
            let t192 = t191 * t122;
            let t195 = t48 * t167;
            let t197 = -t192 * t120 / f64x8::splat(6.0) - t195 * t170;
            let t198 = f64x8::splat(1.0) / t48;
            let t199 = t197 * t198;
            let t203 = t34 * (t176 * t177 / f64x8::splat(3.0) + f64x8::splat(0.36052240899892257) * t183 * t124 * t111 * t186 + f64x8::splat(0.021608710360898266) * t199 * t36);
            let t204 = t203 * t69;
            let t205 = t204 * t84;
            let t206 = t205 / f64x8::splat(24.0);
            let t207 = f64x8::splat(1.0) / t77;
            let t208 = t54 * t207;
            let t209 = t55 - t208;
            let t212 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t61 * t209));
            let t213 = -t209;
            let t216 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t213));
            let t217 = t212 + t216;
            let t218 = t53 * t217;
            let t219 = t218 * t84;
            let t220 = t219 / f64x8::splat(24.0);
            let t221 = t75 * t54;
            let t222 = t221 * t79;
            let t223 = t78 * t7;
            let t224 = f64x8::splat(1.0) / t223;
            let t225 = t76 * t224;
            let t229 = t74 * (-f64x8::splat(4.0) * t222 + f64x8::splat(4.0) * t225) * t83;
            let t230 = t70 * t229;
            let t231 = t230 / f64x8::splat(24.0);
            let t235 = t88 * t88;
            let t236 = f64x8::splat(1.0) / t235;
            let t237 = t9 * t236;
            let t239 = -t121 - f64x8::splat(0.5883683333333334) * t126;
            let t245 = (-t4 * t112 * t89 / f64x8::splat(12.0) - t116 * t237 * t239 / f64x8::splat(4.0)) * t133 * t135;
            let t246 = t137 * t88;
            let t249 = t95 * t95;
            let t250 = f64x8::splat(1.0) / t249;
            let t252 = t250 * t122 * t1;
            let t254 = f64x8::splat(22.3816694236) * t250 + f64x8::splat(1.0);
            let t255 = f64x8::splat(1.0) / t254;
            let t260 = t100 * t89;
            let t261 = t260 * t122;
            let t264 = t101 * t236;
            let t266 = -t261 * t120 / f64x8::splat(6.0) - t264 * t239;
            let t267 = f64x8::splat(1.0) / t101;
            let t268 = t266 * t267;
            let t271 = f64x8::splat(0.005181783333333334) * t245 * t246 + f64x8::splat(0.041388824077869424) * t252 * t124 * t111 * t255 + f64x8::splat(0.0022478670955426118) * t268 * t88 - t140 - t151 - t162;
            let t272 = t271 * t69;
            let t273 = t272 * t108;
            let t274 = t105 * t217;
            let t275 = t274 * t108;
            let t276 = t74 * t221;
            let t277 = t276 * t79;
            let t278 = t106 * t277;
            let t279 = f64x8::splat(4.0) * t278;
            let t280 = t107 * t224;
            let t281 = t106 * t280;
            let t282 = f64x8::splat(4.0) * t281;
            let tvrho0 = t21 + t26 + t32 - t86 + t109 + t7 * (t140 + t151 + t162 - t206 - t220 - t231 + t273 + t275 + t279 - t282);
            acc_vrho_0 = tvrho0;
            let t285 = -t55 - t208;
            let t288 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t61 * t285));
            let t289 = -t285;
            let t292 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t289));
            let t293 = t288 + t292;
            let t294 = t53 * t293;
            let t295 = t294 * t84;
            let t296 = t295 / f64x8::splat(24.0);
            let t300 = t74 * (f64x8::splat(4.0) * t222 + f64x8::splat(4.0) * t225) * t83;
            let t301 = t70 * t300;
            let t302 = t301 / f64x8::splat(24.0);
            let t303 = t105 * t293;
            let t304 = t303 * t108;
            let tvrho1 = t21 + t26 + t32 - t86 + t109 + t7 * (t140 + t151 + t162 - t206 - t296 - t302 + t273 + t304 - t279 - t282);
            acc_vrho_1 = tvrho1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}
