//! MGGA_X_REGTPSS exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_regtpss.c`
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
pub fn mgga_x_regtpss_exc_pol(
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
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = f64x8::splat(1.0) / v_rho0;
            let t30 = v_sigma0 * t29;
            let t31 = f64x8::splat(1.0) / v_tau0;
            let t32 = t30 * t31;
            let t33 = ((t32) * (t32) * (t32));
            let t34 = v_sigma0 * v_sigma0;
            let t35 = v_rho0 * v_rho0;
            let t36 = f64x8::splat(1.0) / t35;
            let t37 = t34 * t36;
            let t38 = v_tau0 * v_tau0;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t37 * t39;
            let t42 = f64x8::splat(1.0) + t40 / f64x8::splat(64.0);
            let t43 = t42 * t42;
            let t44 = f64x8::splat(1.0) / t43;
            let t48 = f64x8::splat(M_CBRT6);
            let t49 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.0045938270703125) * t33 * t44) * t48;
            let t50 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t51 = (simd::cbrt(t50));
            let t52 = t51 * t51;
            let t53 = f64x8::splat(1.0) / t52;
            let t54 = t53 * v_sigma0;
            let t55 = (simd::cbrt(v_rho0));
            let t56 = t55 * t55;
            let t58 = f64x8::splat(1.0) / t56 / t35;
            let t59 = t54 * t58;
            let t63 = f64x8::splat(1.0) / t56 / v_rho0;
            let t65 = v_sigma0 * t58;
            let t67 = v_tau0 * t63 - t65 / f64x8::splat(8.0);
            let t68 = t67 * t48;
            let t69 = t68 * t53;
            let t71 = f64x8::splat(5.0) / f64x8::splat(9.0) * t69 - f64x8::splat(1.0);
            let t72 = t53 * t71;
            let t75 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t68 * t72;
            let t76 = ((t75).sqrt());
            let t77 = f64x8::splat(1.0) / t76;
            let t80 = t48 * t53;
            let t81 = t80 * t65;
            let t82 = t81 / f64x8::splat(36.0);
            let t83 = f64x8::splat(9.0) / f64x8::splat(20.0) * t71 * t77 + t82;
            let t84 = t83 * t83;
            let t87 = t48 * t48;
            let t89 = f64x8::splat(1.0) / t51 / t50;
            let t90 = t87 * t89;
            let t91 = t35 * t35;
            let t92 = t91 * v_rho0;
            let t94 = f64x8::splat(1.0) / t55 / t92;
            let t96 = t90 * t34 * t94;
            let t97 = f64x8::splat(50.0) * t96;
            let t98 = f64x8::splat(162.0) * t40 + t97;
            let t99 = ((t98).sqrt());
            let t102 = f64x8::splat(3.291178445357254e-05) * t96;
            let t104 = t34 * v_sigma0;
            let t105 = t91 * t91;
            let t106 = f64x8::splat(1.0) / t105;
            let t108 = f64x8::splat(1.3522126526770064e-06) * t104 * t106;
            let t109 = t49 * t59 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t84 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t83 * t99 + t102 + f64x8::splat(0.0020448759451792767) * t40 + t108;
            let t111 = f64x8::splat(1.0) + f64x8::splat(0.06134627835537829) * t81;
            let t112 = t111 * t111;
            let t113 = f64x8::splat(1.0) / t112;
            let t115 = f64x8::splat(0.804) + t109 * t113;
            let t117 = f64x8::splat(0.646416) / t115;
            let t118 = -t71;
            let t119 = t118 * t118;
            let t120 = t119 * t118;
            let t121 = t67 * t67;
            let t122 = t121 * t87;
            let t123 = t122 * t89;
            let t125 = f64x8::splat(1.0) + f64x8::splat(0.6714891975308642) * t123;
            let t126 = ((t125).sqrt());
            let t128 = f64x8::splat(1.0) / t126 / t125;
            let t129 = t120 * t128;
            let t131 = (simd::exp(-t81 / f64x8::splat(8.0)));
            let t133 = -f64x8::splat(0.45) + t82;
            let t134 = t133 * t133;
            let t136 = f64x8::splat(10368.0) + t97;
            let t137 = ((t136).sqrt());
            let t140 = f64x8::splat(0.029644443963477367) * t81 + f64x8::splat(146.0) / f64x8::splat(2025.0) * t134 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t133 * t137 + t102 + f64x8::splat(0.1308720604914737) + t108;
            let t142 = f64x8::splat(0.804) + t140 * t113;
            let t145 = -f64x8::splat(0.646416) / t142 + t117;
            let t146 = t131 * t145;
            let t148 = f64x8::splat(1.804) - t117 + t129 * t146;
            let t152 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t148));
            let t153 = (v_rho1).simd_le(dens_threshold);
            let t154 = -t17;
            let t156 = ((t15).select(t12, (t11).select(t16, t154 * t8)));
            let t157 = f64x8::splat(1.0) + t156;
            let t158 = (t157).simd_le(zeta_threshold);
            let t159 = (simd::cbrt(t157));
            let t161 = ((t158).select(t23, t159 * t157));
            let t162 = t161 * t27;
            let t163 = f64x8::splat(1.0) / v_rho1;
            let t164 = v_sigma2 * t163;
            let t165 = f64x8::splat(1.0) / v_tau1;
            let t166 = t164 * t165;
            let t167 = ((t166) * (t166) * (t166));
            let t168 = v_sigma2 * v_sigma2;
            let t169 = v_rho1 * v_rho1;
            let t170 = f64x8::splat(1.0) / t169;
            let t171 = t168 * t170;
            let t172 = v_tau1 * v_tau1;
            let t173 = f64x8::splat(1.0) / t172;
            let t174 = t171 * t173;
            let t176 = f64x8::splat(1.0) + t174 / f64x8::splat(64.0);
            let t177 = t176 * t176;
            let t178 = f64x8::splat(1.0) / t177;
            let t182 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.0045938270703125) * t167 * t178) * t48;
            let t183 = t53 * v_sigma2;
            let t184 = (simd::cbrt(v_rho1));
            let t185 = t184 * t184;
            let t187 = f64x8::splat(1.0) / t185 / t169;
            let t188 = t183 * t187;
            let t192 = f64x8::splat(1.0) / t185 / v_rho1;
            let t194 = v_sigma2 * t187;
            let t196 = v_tau1 * t192 - t194 / f64x8::splat(8.0);
            let t197 = t196 * t48;
            let t198 = t197 * t53;
            let t200 = f64x8::splat(5.0) / f64x8::splat(9.0) * t198 - f64x8::splat(1.0);
            let t201 = t53 * t200;
            let t204 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t197 * t201;
            let t205 = ((t204).sqrt());
            let t206 = f64x8::splat(1.0) / t205;
            let t209 = t80 * t194;
            let t210 = t209 / f64x8::splat(36.0);
            let t211 = f64x8::splat(9.0) / f64x8::splat(20.0) * t200 * t206 + t210;
            let t212 = t211 * t211;
            let t215 = t169 * t169;
            let t216 = t215 * v_rho1;
            let t218 = f64x8::splat(1.0) / t184 / t216;
            let t220 = t90 * t168 * t218;
            let t221 = f64x8::splat(50.0) * t220;
            let t222 = f64x8::splat(162.0) * t174 + t221;
            let t223 = ((t222).sqrt());
            let t226 = f64x8::splat(3.291178445357254e-05) * t220;
            let t228 = t168 * v_sigma2;
            let t229 = t215 * t215;
            let t230 = f64x8::splat(1.0) / t229;
            let t232 = f64x8::splat(1.3522126526770064e-06) * t228 * t230;
            let t233 = t182 * t188 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t212 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t211 * t223 + t226 + f64x8::splat(0.0020448759451792767) * t174 + t232;
            let t235 = f64x8::splat(1.0) + f64x8::splat(0.06134627835537829) * t209;
            let t236 = t235 * t235;
            let t237 = f64x8::splat(1.0) / t236;
            let t239 = f64x8::splat(0.804) + t233 * t237;
            let t241 = f64x8::splat(0.646416) / t239;
            let t242 = -t200;
            let t243 = t242 * t242;
            let t244 = t243 * t242;
            let t245 = t196 * t196;
            let t246 = t245 * t87;
            let t247 = t246 * t89;
            let t249 = f64x8::splat(1.0) + f64x8::splat(0.6714891975308642) * t247;
            let t250 = ((t249).sqrt());
            let t252 = f64x8::splat(1.0) / t250 / t249;
            let t253 = t244 * t252;
            let t255 = (simd::exp(-t209 / f64x8::splat(8.0)));
            let t257 = -f64x8::splat(0.45) + t210;
            let t258 = t257 * t257;
            let t260 = f64x8::splat(10368.0) + t221;
            let t261 = ((t260).sqrt());
            let t264 = f64x8::splat(0.029644443963477367) * t209 + f64x8::splat(146.0) / f64x8::splat(2025.0) * t258 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t257 * t261 + t226 + f64x8::splat(0.1308720604914737) + t232;
            let t266 = f64x8::splat(0.804) + t264 * t237;
            let t269 = -f64x8::splat(0.646416) / t266 + t241;
            let t270 = t255 * t269;
            let t272 = f64x8::splat(1.804) - t241 + t253 * t270;
            let t276 = ((t153).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t162 * t272));
            let tzk0 = t152 + t276;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
