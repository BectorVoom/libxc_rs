//! LDA_C_PMGB06 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pmgb06.c`
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
pub fn lda_c_pmgb06_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
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
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t1 = v_rho0 - v_rho1;
            let t2 = v_rho0 + v_rho1;
            let t3 = f64x8::splat(1.0) / t2;
            let t4 = t1 * t3;
            let t5 = f64x8::splat(1.0) + t4;
            let t6 = (t5).simd_le(zeta_threshold);
            let t7 = (simd::cbrt(zeta_threshold));
            let t8 = t7 * t7;
            let t9 = (simd::cbrt(t5));
            let t10 = t9 * t9;
            let t11 = ((t6).select(t8, t10));
            let t12 = f64x8::splat(1.0) - t4;
            let t13 = (t12).simd_le(zeta_threshold);
            let t14 = (simd::cbrt(t12));
            let t15 = t14 * t14;
            let t16 = ((t13).select(t8, t15));
            let t18 = t11 / f64x8::splat(2.0) + t16 / f64x8::splat(2.0);
            let t19 = t18 * t18;
            let t20 = t19 * t18;
            let t21 = (simd::ln(f64x8::splat(2.0)));
            let t22 = t21 - f64x8::splat(1.0);
            let t23 = f64x8::splat(2.0) * t22;
            let t24 = t20 * t23;
            let t25 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t26 = f64x8::splat(1.0) / t25;
            let t27 = f64x8::splat(M_CBRT3);
            let t28 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t29 = (simd::cbrt(t28));
            let t30 = t27 * t29;
            let t31 = f64x8::splat(M_CBRT4);
            let t32 = t31 * t31;
            let t33 = (simd::cbrt(t2));
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t32 * t34;
            let t36 = t30 * t35;
            let t37 = ((t36).sqrt());
            let t38 = param_hyb_omega_0 * t37;
            let t39 = f64x8::splat(1.0) / t18;
            let t41 = f64x8::splat(2.923025) * t38 * t39;
            let t43 = (simd::cbrt(f64x8::splat(9.0)));
            let t44 = t43 * t43;
            let t52 = param_hyb_omega_0 * param_hyb_omega_0;
            let t53 = (f64x8::splat(3.44851) - f64x8::splat(M_PI) * t31 * t44 * t29 / t22 / f64x8::splat(12.0)) * t52;
            let t54 = t53 * t27;
            let t55 = t29 * t32;
            let t56 = f64x8::splat(1.0) / t19;
            let t61 = t52 * param_hyb_omega_0;
            let t62 = t37 * t36;
            let t63 = t61 * t62;
            let t64 = f64x8::splat(1.0) / t20;
            let t67 = f64x8::splat(1.0) + t41 + t54 * t55 * t34 * t56 / f64x8::splat(4.0) + f64x8::splat(0.48968) * t63 * t64;
            let t68 = t52 * t27;
            let t69 = t68 * t29;
            let t73 = f64x8::splat(1.0) + t41 + f64x8::splat(0.8621275) * t69 * t35 * t56;
            let t74 = f64x8::splat(1.0) / t73;
            let t76 = (simd::ln(t67 * t74));
            let t77 = t26 * t76;
            let t79 = t1 * t1;
            let t80 = t2 * t2;
            let t81 = f64x8::splat(1.0) / t80;
            let t82 = t79 * t81;
            let t83 = f64x8::splat(1.0) - t82;
            let t84 = t3 * t83;
            let t93 = (f64x8::splat(2.0) / f64x8::splat(45.0) * t31 * t44 * t29 * (t25 + f64x8::splat(6.0) * t21 - f64x8::splat(3.0)) * t28 - f64x8::splat(0.7524)) * t27;
            let t97 = t27 * t27;
            let t98 = t29 * t29;
            let t99 = t97 * t98;
            let t100 = t33 * t33;
            let t101 = f64x8::splat(1.0) / t100;
            let t102 = t31 * t101;
            let t103 = t99 * t102;
            let t106 = t29 * t28;
            let t107 = t27 * t106;
            let t109 = f64x8::splat(1.0) / t33 / t2;
            let t110 = t32 * t109;
            let t113 = f64x8::splat(1.0) - t93 * t55 * t34 / f64x8::splat(4.0) + f64x8::splat(0.0204825) * t103 - f64x8::splat(0.0030486129349252553) * t3 + f64x8::splat(0.0003485625) * t107 * t110;
            let t115 = (simd::exp(-f64x8::splat(0.1881) * t36));
            let t116 = t113 * t115;
            let t117 = f64x8::splat(M_SQRT2);
            let t118 = t116 * t117;
            let t122 = t97 * t98 * t26;
            let t123 = t122 * t31;
            let t125 = f64x8::splat(1.0) / t100 / t2;
            let t126 = zeta_threshold * zeta_threshold;
            let t127 = t5 * t5;
            let t128 = ((t6).select(t126, t127));
            let t129 = t128 * t44;
            let t130 = f64x8::splat(1.0) / t106;
            let t131 = t130 * t27;
            let t132 = t129 * t131;
            let t133 = f64x8::splat(1.0) / t5;
            let t134 = (simd::cbrt(t133));
            let t135 = t134 * t134;
            let t136 = f64x8::splat(1.0) / t135;
            let t137 = t100 * t136;
            let t138 = t30 * t32;
            let t139 = f64x8::splat(M_CBRT2);
            let t140 = t34 * t139;
            let t142 = t138 * t140 * t134;
            let t144 = f64x8::splat(1.0) - f64x8::splat(0.0056675) * t142;
            let t146 = t99 * t31;
            let t147 = t139 * t139;
            let t148 = t101 * t147;
            let t152 = f64x8::splat(1.0) + f64x8::splat(0.107975) * t142 + f64x8::splat(0.01) * t146 * t148 * t135;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t144 * t153;
            let t155 = t137 * t154;
            let t157 = t132 * t155 / f64x8::splat(30.0);
            let t158 = t12 * t12;
            let t159 = ((t13).select(t126, t158));
            let t160 = t159 * t44;
            let t161 = t160 * t131;
            let t162 = f64x8::splat(1.0) / t12;
            let t163 = (simd::cbrt(t162));
            let t164 = t163 * t163;
            let t165 = f64x8::splat(1.0) / t164;
            let t166 = t100 * t165;
            let t168 = t138 * t140 * t163;
            let t170 = f64x8::splat(1.0) - f64x8::splat(0.0056675) * t168;
            let t175 = f64x8::splat(1.0) + f64x8::splat(0.107975) * t168 + f64x8::splat(0.01) * t146 * t148 * t164;
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t170 * t176;
            let t178 = t166 * t177;
            let t180 = t161 * t178 / f64x8::splat(30.0);
            let t183 = -f64x8::splat(1.2375) * t36 + t103 / f64x8::splat(4.0);
            let t184 = t83 * t183;
            let t186 = (simd::exp(-f64x8::splat(0.0775) * t36));
            let t187 = t186 * f64x8::splat(M_PI);
            let t188 = t187 * t2;
            let t191 = t157 + t180 + f64x8::splat(4.0) / f64x8::splat(3.0) * t184 * t188;
            let t199 = t116 / f64x8::splat(2.0) - f64x8::splat(1.0) / f64x8::splat(2.0) + t82 / f64x8::splat(2.0);
            let t202 = t31 * t125;
            let t205 = -f64x8::splat(0.097) * t36 + f64x8::splat(0.169) * t103;
            let t206 = t83 * t205;
            let t208 = (simd::exp(-f64x8::splat(0.13675) * t36));
            let t209 = t206 * t208;
            let t211 = t27 / t98;
            let t213 = t211 * t32 * t100;
            let t216 = t8 * t126;
            let t217 = t10 * t127;
            let t218 = ((t6).select(t216, t217));
            let t219 = t15 * t158;
            let t220 = ((t13).select(t216, t219));
            let t223 = (t218 / f64x8::splat(2.0) + t220 / f64x8::splat(2.0)) * t44;
            let t224 = t131 * t100;
            let t227 = t157 + t180 + t209 * t213 / f64x8::splat(3.0) - t223 * t224 / f64x8::splat(15.0);
            let t232 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t36;
            let t235 = ((t36) * (t36).sqrt());
            let t238 = f64x8::splat(3.79785) * t37 + f64x8::splat(0.8969) * t36 + f64x8::splat(0.204775) * t235 + f64x8::splat(0.123235) * t103;
            let t241 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t238;
            let t242 = (simd::ln(t241));
            let t244 = f64x8::splat(0.0621814) * t232 * t242;
            let t245 = t79 * t79;
            let t246 = t80 * t80;
            let t247 = f64x8::splat(1.0) / t246;
            let t248 = t245 * t247;
            let t249 = t7 * zeta_threshold;
            let t250 = t9 * t5;
            let t251 = ((t6).select(t249, t250));
            let t252 = t14 * t12;
            let t253 = ((t13).select(t249, t252));
            let t254 = t251 + t253 - f64x8::splat(2.0);
            let t257 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t139 - f64x8::splat(2.0));
            let t258 = t254 * t257;
            let t260 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t36;
            let t265 = f64x8::splat(7.05945) * t37 + f64x8::splat(1.549425) * t36 + f64x8::splat(0.420775) * t235 + f64x8::splat(0.1562925) * t103;
            let t268 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t265;
            let t269 = (simd::ln(t268));
            let t273 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t36;
            let t278 = f64x8::splat(5.1785) * t37 + f64x8::splat(0.905775) * t36 + f64x8::splat(0.1100325) * t235 + f64x8::splat(0.1241775) * t103;
            let t281 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t278;
            let t282 = (simd::ln(t281));
            let t283 = t273 * t282;
            let t285 = -f64x8::splat(0.0310907) * t260 * t269 + t244 - f64x8::splat(0.0197516734986138) * t283;
            let t286 = t258 * t285;
            let t290 = -t244 + t248 * t286 + f64x8::splat(0.0197516734986138) * t258 * t283;
            let t295 = t52 * t52;
            let t297 = t122 * t202;
            let t299 = t115 * t117;
            let t301 = t299 * t295 * param_hyb_omega_0;
            let t302 = t83 * t113 * t301;
            let t305 = t125 * t83;
            let t312 = t295 * t52;
            let t315 = f64x8::splat(1.0) / t100 / t80;
            let t317 = t295 * t295;
            let t321 = t24 * t77 + (-f64x8::splat(0.031505407223141116) * t84 * t118 - f64x8::splat(0.005388405304614574) * t123 * t125 * t191 * t117) * t61 + (-f64x8::splat(0.0837628205355044) * t84 * t199 - f64x8::splat(0.011938374665504766) * t122 * t202 * t227 + f64x8::splat(0.42708890021612717) * t107 * t110 * t290) * t295 - f64x8::splat(0.01197423401025461) * t297 * t302 + (-f64x8::splat(0.031835665774679375) * t123 * t305 * t199 + f64x8::splat(0.05332506774217938) * t81 * t290) * t312 + f64x8::splat(0.020267214298646783) * t123 * t315 * t290 * t317;
            let t325 = f64x8::splat(1.0) + f64x8::splat(0.15403623315025) * t99 * t102 * t52;
            let t326 = t325 * t325;
            let t327 = t326 * t326;
            let t328 = f64x8::splat(1.0) / t327;
            let tzk0 = t321 * t328;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
