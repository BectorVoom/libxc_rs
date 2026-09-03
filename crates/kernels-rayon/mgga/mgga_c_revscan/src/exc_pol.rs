//! MGGA_C_REVSCAN exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_revscan.c`
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
pub fn mgga_c_revscan_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        {
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = v_rho0 + v_rho1;
            let t9 = (simd::cbrt(t8));
            let t12 = t5 * t7 / t9;
            let t14 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t12;
            let t15 = ((t12).sqrt());
            let t18 = ((t12) * (t12).sqrt());
            let t20 = t2 * t2;
            let t21 = t4 * t4;
            let t22 = t20 * t21;
            let t23 = t9 * t9;
            let t26 = t22 * t6 / t23;
            let t28 = f64x8::splat(3.79785) * t15 + f64x8::splat(0.8969) * t12 + f64x8::splat(0.204775) * t18 + f64x8::splat(0.123235) * t26;
            let t31 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t28;
            let t32 = (simd::ln(t31));
            let t34 = f64x8::splat(0.0621814) * t14 * t32;
            let t35 = v_rho0 - v_rho1;
            let t36 = t35 * t35;
            let t37 = t36 * t36;
            let t38 = t8 * t8;
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t37 * t40;
            let t42 = f64x8::splat(1.0) / t8;
            let t43 = t35 * t42;
            let t44 = f64x8::splat(1.0) + t43;
            let t45 = (t44).simd_le(zeta_threshold);
            let t46 = (simd::cbrt(zeta_threshold));
            let t47 = t46 * zeta_threshold;
            let t48 = (simd::cbrt(t44));
            let t49 = t48 * t44;
            let t50 = ((t45).select(t47, t49));
            let t51 = f64x8::splat(1.0) - t43;
            let t52 = (t51).simd_le(zeta_threshold);
            let t53 = (simd::cbrt(t51));
            let t54 = t53 * t51;
            let t55 = ((t52).select(t47, t54));
            let t56 = t50 + t55 - f64x8::splat(2.0);
            let t57 = f64x8::splat(M_CBRT2);
            let t58 = t57 - f64x8::splat(1.0);
            let t60 = f64x8::splat(1.0) / t58 / f64x8::splat(2.0);
            let t61 = t56 * t60;
            let t63 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t12;
            let t68 = f64x8::splat(7.05945) * t15 + f64x8::splat(1.549425) * t12 + f64x8::splat(0.420775) * t18 + f64x8::splat(0.1562925) * t26;
            let t71 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t68;
            let t72 = (simd::ln(t71));
            let t76 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t12;
            let t81 = f64x8::splat(5.1785) * t15 + f64x8::splat(0.905775) * t12 + f64x8::splat(0.1100325) * t18 + f64x8::splat(0.1241775) * t26;
            let t84 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t81;
            let t85 = (simd::ln(t84));
            let t86 = t76 * t85;
            let t88 = -f64x8::splat(0.0310907) * t63 * t72 + t34 - f64x8::splat(0.0197516734986138) * t86;
            let t89 = t61 * t88;
            let t90 = t41 * t89;
            let t92 = f64x8::splat(0.0197516734986138) * t61 * t86;
            let t93 = (simd::ln(f64x8::splat(2.0)));
            let t94 = f64x8::splat(1.0) - t93;
            let t95 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t97 = t94 / t95;
            let t98 = t46 * t46;
            let t99 = t48 * t48;
            let t100 = ((t45).select(t98, t99));
            let t101 = t53 * t53;
            let t102 = ((t52).select(t98, t101));
            let t104 = t100 / f64x8::splat(2.0) + t102 / f64x8::splat(2.0);
            let t105 = t104 * t104;
            let t106 = t105 * t104;
            let t108 = f64x8::splat(1.0) + f64x8::splat(0.025) * t12;
            let t110 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t12;
            let t111 = f64x8::splat(1.0) / t110;
            let t112 = t108 * t111;
            let t113 = f64x8::splat(1.0) / t94;
            let t115 = (-t34 + t90 + t92) * t113;
            let t116 = f64x8::splat(1.0) / t106;
            let t117 = t95 * t116;
            let t119 = (simd::exp(-t115 * t117));
            let t120 = t119 - f64x8::splat(1.0);
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t113 * t121;
            let t124 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t125 = t122 * t124;
            let t126 = t112 * t125;
            let t128 = f64x8::splat(1.0) / t9 / t38;
            let t129 = t128 * t57;
            let t130 = f64x8::splat(1.0) / t105;
            let t132 = f64x8::splat(1.0) / t4;
            let t133 = t20 * t132;
            let t134 = t133 * t6;
            let t135 = t129 * t130 * t134;
            let t138 = f64x8::splat(1.0) + f64x8::splat(0.054878743191129266) * t126 * t135;
            let t139 = ((t138).sqrt().sqrt());
            let t142 = t108 * t108;
            let t143 = t110 * t110;
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t142 * t144;
            let t146 = t94 * t94;
            let t147 = f64x8::splat(1.0) / t146;
            let t148 = t120 * t120;
            let t149 = f64x8::splat(1.0) / t148;
            let t150 = t147 * t149;
            let t151 = t124 * t124;
            let t152 = t150 * t151;
            let t153 = t145 * t152;
            let t155 = f64x8::splat(1.0) / t23 / t39;
            let t156 = t57 * t57;
            let t157 = t155 * t156;
            let t158 = t105 * t105;
            let t159 = f64x8::splat(1.0) / t158;
            let t160 = t157 * t159;
            let t161 = f64x8::splat(1.0) / t21;
            let t162 = t2 * t161;
            let t163 = t162 * t7;
            let t164 = t160 * t163;
            let t167 = f64x8::splat(1.0) + f64x8::splat(0.011293786703392187) * t153 * t164;
            let t168 = (simd::pow(t167, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t171 = f64x8::splat(1.0) - f64x8::splat(1.0) / t139 / f64x8::splat(2.0) - f64x8::splat(1.0) / t168 / f64x8::splat(2.0);
            let t174 = f64x8::splat(1.0) + f64x8::splat(1.0) * t171 * t120;
            let t175 = (simd::ln(t174));
            let t177 = t97 * t106 * t175;
            let t178 = (simd::cbrt(v_rho0));
            let t179 = t178 * t178;
            let t181 = f64x8::splat(1.0) / t179 / v_rho0;
            let t182 = v_tau0 * t181;
            let t183 = t44 / f64x8::splat(2.0);
            let t184 = (simd::cbrt(t183));
            let t185 = t184 * t184;
            let t186 = t185 * t183;
            let t188 = (simd::cbrt(v_rho1));
            let t189 = t188 * t188;
            let t191 = f64x8::splat(1.0) / t189 / v_rho1;
            let t192 = v_tau1 * t191;
            let t193 = t51 / f64x8::splat(2.0);
            let t194 = (simd::cbrt(t193));
            let t195 = t194 * t194;
            let t196 = t195 * t193;
            let t199 = f64x8::splat(1.0) / t23 / t38;
            let t203 = f64x8::splat(M_CBRT6);
            let t204 = (t182 * t186 + t192 * t196 - t124 * t199 / f64x8::splat(8.0)) * t203;
            let t205 = (simd::cbrt(t95));
            let t206 = t205 * t205;
            let t207 = f64x8::splat(1.0) / t206;
            let t208 = t186 + t196;
            let t209 = f64x8::splat(1.0) / t208;
            let t210 = t207 * t209;
            let t212 = f64x8::splat(5.0) / f64x8::splat(9.0) * t204 * t210;
            let t213 = (t212).simd_le(f64x8::splat(1.0));
            let t214 = (simd::ln(f64x8::splat(f64::EPSILON)));
            let t217 = t214 / (-t214 + f64x8::splat(1.131));
            let t218 = (-t217).simd_lt(t212);
            let t219 = (t212).simd_lt(-t217);
            let t220 = ((t219).select(t212, -t217));
            let t221 = f64x8::splat(1.0) - t220;
            let t222 = f64x8::splat(1.0) / t221;
            let t225 = (simd::exp(-f64x8::splat(1.131) * t220 * t222));
            let t226 = ((t218).select(f64x8::splat(0.0), t225));
            let t228 = (simd::ln(f64x8::splat(0.7299270072992701) * f64x8::splat(f64::EPSILON)));
            let t231 = (-t228 + f64x8::splat(1.7)) / t228;
            let t232 = (t212).simd_lt(-t231);
            let t233 = ((t232).select(-t231, t212));
            let t234 = f64x8::splat(1.0) - t233;
            let t237 = (simd::exp(f64x8::splat(1.7) / t234));
            let t239 = ((t232).select(f64x8::splat(0.0), -f64x8::splat(1.37) * t237));
            let t240 = ((t213).select(t226, t239));
            let t243 = f64x8::splat(1.0) + f64x8::splat(0.033115) * t15 + f64x8::splat(0.04168) * t12;
            let t244 = f64x8::splat(1.0) / t243;
            let t247 = (simd::exp(f64x8::splat(1.0) * t244));
            let t248 = t247 - f64x8::splat(1.0);
            let t249 = t203 * t207;
            let t250 = t156 * t124;
            let t254 = f64x8::splat(1.0) + f64x8::splat(0.04267528420875272) * t249 * t250 * t199;
            let t255 = ((t254).sqrt().sqrt());
            let t258 = t203 * t203;
            let t260 = f64x8::splat(1.0) / t205 / t95;
            let t261 = t258 * t260;
            let t262 = t57 * t151;
            let t263 = t39 * t8;
            let t265 = f64x8::splat(1.0) / t9 / t263;
            let t269 = f64x8::splat(1.0) + f64x8::splat(0.004552949705744548) * t261 * t262 * t265;
            let t270 = (simd::pow(t269, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t273 = f64x8::splat(1.0) - f64x8::splat(1.0) / t255 / f64x8::splat(2.0) - f64x8::splat(1.0) / t270 / f64x8::splat(2.0);
            let t275 = t248 * t273 + f64x8::splat(1.0);
            let t276 = (simd::ln(t275));
            let t278 = -f64x8::splat(0.030197) * t244 + f64x8::splat(0.030197) * t276;
            let t282 = f64x8::splat(1.0) - f64x8::splat(2.363) * t58 * t56 * t60;
            let t283 = t278 * t282;
            let t284 = t37 * t37;
            let t285 = t284 * t37;
            let t286 = t39 * t39;
            let t287 = t286 * t39;
            let t288 = f64x8::splat(1.0) / t287;
            let t290 = -t285 * t288 + f64x8::splat(1.0);
            let t292 = t283 * t290 - t177 + t34 - t90 - t92;
            let t293 = t240 * t292;
            let tzk0 = -t34 + t90 + t92 + t177 + t293;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
