//! GGA_C_REVTCA vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_revtca.c`
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
pub fn gga_c_revtca_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
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
            let t21 = f64x8::splat(M_CBRT3);
            let t22 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t21 * t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t25 * t25;
            let t27 = (simd::cbrt(t2));
            let t32 = f64x8::splat(4.88827) + f64x8::splat(0.79425925) * t24 * t26 / t27;
            let t33 = (simd::atan(t32));
            let t35 = -f64x8::splat(0.655868) * t33 + f64x8::splat(0.897889);
            let t36 = t20 * t35;
            let t37 = t21 * t21;
            let t38 = f64x8::splat(1.0) / t23;
            let t39 = t37 * t38;
            let t40 = t36 * t39;
            let t41 = t25 * t27;
            let t42 = f64x8::splat(M_CBRT6);
            let t43 = t42 * t42;
            let t44 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t45 = (simd::cbrt(t44));
            let t46 = f64x8::splat(1.0) / t45;
            let t47 = t43 * t46;
            let t48 = f64x8::splat(M_CBRT2);
            let t50 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t51 = ((t50).sqrt());
            let t52 = t48 * t51;
            let t53 = t27 * t2;
            let t54 = f64x8::splat(1.0) / t53;
            let t56 = t47 * t52 * t54;
            let t57 = (simd::pow(t56, f64x8::splat(2.3)));
            let t59 = f64x8::splat(1.0) + f64x8::splat(0.004712150703442276) * t57;
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t1 * t1;
            let t62 = t61 * t61;
            let t63 = t2 * t2;
            let t64 = t63 * t63;
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t62 * t65;
            let t67 = f64x8::splat(M_CBRTPI);
            let t69 = (simd::cbrt(f64x8::splat(9.0)));
            let t71 = t67 * f64x8::splat(M_PI) * t69 * t47;
            let t73 = t3 * t37 * t38;
            let t76 = t71 * t52 * t73 / f64x8::splat(36.0);
            let t77 = ((f64x8::splat(f64::EPSILON)).sqrt().sqrt());
            let t78 = (t76).simd_le(t77);
            let t79 = t67 * t67;
            let t81 = t69 * t69;
            let t83 = t45 * t45;
            let t84 = f64x8::splat(1.0) / t83;
            let t85 = t42 * t84;
            let t86 = t79 * t44 * t81 * t85;
            let t87 = t48 * t48;
            let t88 = t87 * t50;
            let t89 = f64x8::splat(1.0) / t63;
            let t91 = t23 * t23;
            let t92 = f64x8::splat(1.0) / t91;
            let t97 = t44 * t44;
            let t104 = t67 * t97 * f64x8::splat(M_PI) * t69 * t43 / t45 / t44;
            let t105 = t50 * t50;
            let t106 = t48 * t105;
            let t107 = t65 * t37;
            let t109 = f64x8::splat(1.0) / t23 / t22;
            let t110 = t107 * t109;
            let t114 = t97 * t44;
            let t115 = t105 * t50;
            let t116 = t114 * t115;
            let t117 = t64 * t63;
            let t118 = f64x8::splat(1.0) / t117;
            let t122 = (t77).simd_lt(t76);
            let t123 = ((t122).select(t76, t77));
            let t124 = (simd::sin(t123));
            let t125 = f64x8::splat(1.0) / t123;
            let t126 = t124 * t125;
            let t127 = ((t78).select(f64x8::splat(1.0) - t86 * t88 * t89 * t21 * t92 / f64x8::splat(432.0) + t104 * t106 * t110 / f64x8::splat(34560.0) - t116 * t118 / f64x8::splat(322560.0), t126));
            let t128 = t127 * t127;
            let t129 = f64x8::splat(1.0) - t128;
            let t131 = -t66 * t129 + f64x8::splat(1.0);
            let t132 = t60 * t131;
            let t134 = t40 * t41 * t132;
            let tzk0 = t134 / f64x8::splat(3.0);
            acc_zk = tzk0;
            let t135 = f64x8::splat(4.0) / f64x8::splat(9.0) * t134;
            let t137 = t35 * t37;
            let t138 = t53 * t19 * t137;
            let t139 = t38 * t25;
            let t140 = f64x8::splat(1.0) / t9;
            let t141 = t1 * t89;
            let t142 = t3 - t141;
            let t145 = ((t6).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t140 * t142));
            let t146 = f64x8::splat(1.0) / t14;
            let t147 = -t142;
            let t150 = ((t13).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t146 * t147));
            let t152 = t145 / f64x8::splat(2.0) + t150 / f64x8::splat(2.0);
            let t153 = t132 * t152;
            let t156 = t32 * t32;
            let t157 = t156 + f64x8::splat(1.0);
            let t158 = f64x8::splat(1.0) / t157;
            let t159 = t20 * t158;
            let t161 = f64x8::splat(0.6945723010386666) * t159 * t132;
            let t162 = t3 * t20;
            let t164 = t39 * t25;
            let t165 = t162 * t35 * t164;
            let t166 = t59 * t59;
            let t167 = f64x8::splat(1.0) / t166;
            let t168 = t167 * t131;
            let t169 = (simd::pow(t56, f64x8::splat(1.3)));
            let t170 = t168 * t169;
            let t171 = t47 * t52;
            let t172 = t170 * t171;
            let t174 = f64x8::splat(0.004816865163518771) * t165 * t172;
            let t176 = t53 * t20 * t137;
            let t177 = t61 * t1;
            let t178 = t177 * t65;
            let t180 = f64x8::splat(4.0) * t178 * t129;
            let t181 = t64 * t2;
            let t182 = f64x8::splat(1.0) / t181;
            let t183 = t62 * t182;
            let t185 = f64x8::splat(4.0) * t183 * t129;
            let t186 = t63 * t2;
            let t187 = f64x8::splat(1.0) / t186;
            let t193 = t182 * t37;
            let t194 = t193 * t109;
            let t198 = t64 * t186;
            let t199 = f64x8::splat(1.0) / t198;
            let t204 = t89 * t37 * t38;
            let t208 = ((t122).select(-t71 * t52 * t204 / f64x8::splat(36.0), f64x8::splat(0.0)));
            let t209 = (simd::cos(t123));
            let t211 = t208 * t209 * t125;
            let t212 = t123 * t123;
            let t213 = f64x8::splat(1.0) / t212;
            let t214 = t124 * t213;
            let t215 = t214 * t208;
            let t217 = ((t78).select(t86 * t88 * t187 * t21 * t92 / f64x8::splat(216.0) - t104 * t106 * t194 / f64x8::splat(8640.0) + t116 * t199 / f64x8::splat(53760.0), t211 - t215));
            let t218 = t127 * t217;
            let t220 = f64x8::splat(2.0) * t66 * t218;
            let t221 = -t180 + t185 + t220;
            let t222 = t60 * t221;
            let t223 = t139 * t222;
            let tvrho0 = t135 + t138 * t139 * t153 + t161 + t174 + t176 * t223 / f64x8::splat(3.0);
            acc_vrho_0 = tvrho0;
            let t226 = -t3 - t141;
            let t229 = ((t6).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t140 * t226));
            let t230 = -t226;
            let t233 = ((t13).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t146 * t230));
            let t235 = t229 / f64x8::splat(2.0) + t233 / f64x8::splat(2.0);
            let t236 = t132 * t235;
            let t237 = t139 * t236;
            let t239 = t180 + t185 + t220;
            let t240 = t60 * t239;
            let t241 = t139 * t240;
            let tvrho1 = t135 + t138 * t237 + t161 + t174 + t176 * t241 / f64x8::splat(3.0);
            acc_vrho_1 = tvrho1;
            let t244 = t36 * t37;
            let t245 = t139 * t167;
            let t246 = t244 * t245;
            let t247 = t131 * t169;
            let t248 = t247 * t43;
            let t249 = t46 * t48;
            let t250 = f64x8::splat(1.0) / t51;
            let t251 = t249 * t250;
            let t252 = t248 * t251;
            let t253 = t246 * t252;
            let t255 = t27 * t27;
            let t257 = f64x8::splat(1.0) / t255 / t63;
            let t258 = t257 * t20;
            let t259 = t137 * t38;
            let t260 = t258 * t259;
            let t261 = t25 * t60;
            let t262 = t62 * t127;
            let t264 = t21 * t92;
            let t266 = t86 * t87 * t89 * t264;
            let t268 = t48 * t50;
            let t270 = t104 * t268 * t110;
            let t272 = t114 * t105;
            let t273 = t272 * t118;
            let t276 = t48 * t250;
            let t278 = t71 * t276 * t73;
            let t280 = ((t122).select(t278 / f64x8::splat(72.0), f64x8::splat(0.0)));
            let t281 = t280 * t209;
            let t282 = t281 * t125;
            let t283 = t214 * t280;
            let t285 = ((t78).select(-t266 / f64x8::splat(432.0) + t270 / f64x8::splat(17280.0) - t273 / f64x8::splat(107520.0), t282 - t283));
            let t287 = t261 * t262 * t285;
            let tvsigma0 = -f64x8::splat(0.001806324436319539) * t253 + f64x8::splat(2.0) / f64x8::splat(3.0) * t260 * t287;
            acc_vsigma_0 = tvsigma0;
            let t296 = ((t122).select(t278 / f64x8::splat(36.0), f64x8::splat(0.0)));
            let t297 = t296 * t209;
            let t301 = ((t78).select(-t266 / f64x8::splat(216.0) + t270 / f64x8::splat(8640.0) - t273 / f64x8::splat(53760.0), t297 * t125 - t214 * t296));
            let t302 = t262 * t301;
            let t303 = t261 * t302;
            let tvsigma1 = -f64x8::splat(0.003612648872639078) * t253 + f64x8::splat(2.0) / f64x8::splat(3.0) * t260 * t303;
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
