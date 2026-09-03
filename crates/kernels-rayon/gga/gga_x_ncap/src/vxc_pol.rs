//! GGA_X_NCAP vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ncap.c`
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
pub fn gga_x_ncap_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_mu: f64,
    param_zeta: f64,
    param_alpha: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_mu = f64x8::splat(param_mu);
    let param_zeta = f64x8::splat(param_zeta);
    let param_alpha = f64x8::splat(param_alpha);
    let param_beta = f64x8::splat(param_beta);
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
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = t28 * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t29 * t32;
            let t34 = ((v_sigma0).sqrt());
            let t35 = (simd::cbrt(v_rho0));
            let t37 = f64x8::splat(1.0) / t35 / v_rho0;
            let t38 = t34 * t37;
            let t40 = t33 * t38 / f64x8::splat(12.0);
            let t41 = (simd::tanh(t40));
            let t42 = param_mu * t41;
            let t43 = (simd::ln(t40 + ((t40 * t40 + f64x8::splat(1.0)).sqrt())));
            let t44 = f64x8::splat(1.0) - param_zeta;
            let t46 = t44 * t29 * t32;
            let t47 = f64x8::splat(1.0) + t40;
            let t48 = (simd::ln(t47));
            let t51 = param_zeta * t29;
            let t52 = t32 * t34;
            let t58 = f64x8::splat(1.0) + param_alpha * (t51 * t52 * t37 / f64x8::splat(12.0) + t46 * t38 * t48 / f64x8::splat(12.0));
            let t59 = t43 * t58;
            let t60 = param_beta * t41;
            let t62 = t60 * t43 + f64x8::splat(1.0);
            let t63 = f64x8::splat(1.0) / t62;
            let t64 = t59 * t63;
            let t66 = t42 * t64 + f64x8::splat(1.0);
            let t70 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t66));
            let t71 = (v_rho1).simd_le(dens_threshold);
            let t72 = -t16;
            let t74 = ((t14).select(t11, (t10).select(t15, t72 * t7)));
            let t75 = f64x8::splat(1.0) + t74;
            let t76 = (t75).simd_le(zeta_threshold);
            let t77 = (simd::cbrt(t75));
            let t79 = ((t76).select(t22, t77 * t75));
            let t80 = t79 * t26;
            let t81 = ((v_sigma2).sqrt());
            let t82 = (simd::cbrt(v_rho1));
            let t84 = f64x8::splat(1.0) / t82 / v_rho1;
            let t85 = t81 * t84;
            let t87 = t33 * t85 / f64x8::splat(12.0);
            let t88 = (simd::tanh(t87));
            let t89 = param_mu * t88;
            let t90 = (simd::ln(t87 + ((t87 * t87 + f64x8::splat(1.0)).sqrt())));
            let t91 = f64x8::splat(1.0) + t87;
            let t92 = (simd::ln(t91));
            let t95 = t32 * t81;
            let t101 = f64x8::splat(1.0) + param_alpha * (t46 * t85 * t92 / f64x8::splat(12.0) + t51 * t95 * t84 / f64x8::splat(12.0));
            let t102 = t90 * t101;
            let t103 = param_beta * t88;
            let t105 = t103 * t90 + f64x8::splat(1.0);
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t102 * t106;
            let t109 = t89 * t107 + f64x8::splat(1.0);
            let t113 = ((t71).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t80 * t109));
            let tzk0 = t70 + t113;
            acc_zk = tzk0;
            let t114 = t6 * t6;
            let t115 = f64x8::splat(1.0) / t114;
            let t116 = t16 * t115;
            let t118 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t116)));
            let t121 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t118));
            let t122 = t121 * t26;
            let t126 = t26 * t26;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t25 * t127;
            let t131 = t5 * t128 * t66 / f64x8::splat(8.0);
            let t132 = param_mu * t29;
            let t133 = t132 * t52;
            let t134 = v_rho0 * v_rho0;
            let t136 = f64x8::splat(1.0) / t35 / t134;
            let t137 = t41 * t41;
            let t138 = f64x8::splat(1.0) - t137;
            let t140 = t136 * t138 * t64;
            let t143 = t42 * t33;
            let t144 = t34 * t136;
            let t145 = t31 * t31;
            let t146 = f64x8::splat(1.0) / t145;
            let t147 = t28 * t146;
            let t148 = t35 * t35;
            let t150 = f64x8::splat(1.0) / t148 / t134;
            let t154 = f64x8::splat(6.0) * t147 * v_sigma0 * t150 + f64x8::splat(144.0);
            let t155 = ((t154).sqrt());
            let t156 = f64x8::splat(1.0) / t155;
            let t157 = t156 * t58;
            let t158 = t157 * t63;
            let t162 = t42 * t43;
            let t166 = t44 * t28;
            let t167 = t166 * t146;
            let t168 = t134 * v_rho0;
            let t170 = f64x8::splat(1.0) / t148 / t168;
            let t172 = f64x8::splat(1.0) / t47;
            let t176 = t52 * t136;
            let t179 = -t46 * t144 * t48 / f64x8::splat(9.0) - t167 * v_sigma0 * t170 * t172 / f64x8::splat(18.0) - t51 * t176 / f64x8::splat(9.0);
            let t180 = param_alpha * t179;
            let t181 = t180 * t63;
            let t183 = t62 * t62;
            let t184 = f64x8::splat(1.0) / t183;
            let t185 = t58 * t184;
            let t186 = param_beta * t29;
            let t187 = t186 * t32;
            let t188 = t138 * t43;
            let t192 = t60 * t29;
            let t193 = t136 * t156;
            let t197 = -t187 * t144 * t188 / f64x8::splat(9.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t192 * t52 * t193;
            let t198 = t185 * t197;
            let t200 = -t133 * t140 / f64x8::splat(9.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t143 * t144 * t158 + t162 * t181 - t162 * t198;
            let t205 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t122 * t66 - t131 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t200));
            let t206 = t72 * t115;
            let t208 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t206)));
            let t211 = ((t76).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t77 * t208));
            let t212 = t211 * t26;
            let t216 = t79 * t127;
            let t219 = t5 * t216 * t109 / f64x8::splat(8.0);
            let t221 = ((t71).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t212 * t109 - t219));
            let tvrho0 = t70 + t113 + t6 * (t205 + t221);
            acc_vrho_0 = tvrho0;
            let t225 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t116)));
            let t228 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t225));
            let t229 = t228 * t26;
            let t234 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t229 * t66 - t131));
            let t236 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t206)));
            let t239 = ((t76).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t77 * t236));
            let t240 = t239 * t26;
            let t244 = t132 * t95;
            let t245 = v_rho1 * v_rho1;
            let t247 = f64x8::splat(1.0) / t82 / t245;
            let t248 = t88 * t88;
            let t249 = f64x8::splat(1.0) - t248;
            let t251 = t247 * t249 * t107;
            let t254 = t89 * t33;
            let t255 = t81 * t247;
            let t256 = t82 * t82;
            let t258 = f64x8::splat(1.0) / t256 / t245;
            let t262 = f64x8::splat(6.0) * t147 * v_sigma2 * t258 + f64x8::splat(144.0);
            let t263 = ((t262).sqrt());
            let t264 = f64x8::splat(1.0) / t263;
            let t265 = t264 * t101;
            let t266 = t265 * t106;
            let t270 = t89 * t90;
            let t274 = t245 * v_rho1;
            let t276 = f64x8::splat(1.0) / t256 / t274;
            let t278 = f64x8::splat(1.0) / t91;
            let t282 = t95 * t247;
            let t285 = -t46 * t255 * t92 / f64x8::splat(9.0) - t167 * v_sigma2 * t276 * t278 / f64x8::splat(18.0) - t51 * t282 / f64x8::splat(9.0);
            let t286 = param_alpha * t285;
            let t287 = t286 * t106;
            let t289 = t105 * t105;
            let t290 = f64x8::splat(1.0) / t289;
            let t291 = t101 * t290;
            let t292 = t249 * t90;
            let t296 = t103 * t29;
            let t297 = t247 * t264;
            let t301 = -t187 * t255 * t292 / f64x8::splat(9.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t296 * t95 * t297;
            let t302 = t291 * t301;
            let t304 = -t244 * t251 / f64x8::splat(9.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t254 * t255 * t266 + t270 * t287 - t270 * t302;
            let t309 = ((t71).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t240 * t109 - t219 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t80 * t304));
            let tvrho1 = t70 + t113 + t6 * (t234 + t309);
            acc_vrho_1 = tvrho1;
            let t312 = f64x8::splat(1.0) / t34;
            let t313 = t32 * t312;
            let t314 = t132 * t313;
            let t316 = t37 * t138 * t64;
            let t319 = t312 * t37;
            let t330 = t313 * t37;
            let t333 = t46 * t319 * t48 / f64x8::splat(24.0) + t166 * t146 * t150 * t172 / f64x8::splat(48.0) + t51 * t330 / f64x8::splat(24.0);
            let t334 = param_alpha * t333;
            let t335 = t334 * t63;
            let t340 = t37 * t156;
            let t344 = t187 * t319 * t188 / f64x8::splat(24.0) + t192 * t313 * t340 / f64x8::splat(2.0);
            let t345 = t185 * t344;
            let t347 = t314 * t316 / f64x8::splat(24.0) + t143 * t319 * t158 / f64x8::splat(2.0) + t162 * t335 - t162 * t345;
            let t351 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t347));
            let tvsigma0 = t6 * t351;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t352 = f64x8::splat(1.0) / t81;
            let t353 = t32 * t352;
            let t354 = t132 * t353;
            let t356 = t84 * t249 * t107;
            let t359 = t352 * t84;
            let t370 = t353 * t84;
            let t373 = t46 * t359 * t92 / f64x8::splat(24.0) + t166 * t146 * t258 * t278 / f64x8::splat(48.0) + t51 * t370 / f64x8::splat(24.0);
            let t374 = param_alpha * t373;
            let t375 = t374 * t106;
            let t380 = t84 * t264;
            let t384 = t187 * t359 * t292 / f64x8::splat(24.0) + t296 * t353 * t380 / f64x8::splat(2.0);
            let t385 = t291 * t384;
            let t387 = t354 * t356 / f64x8::splat(24.0) + t254 * t359 * t266 / f64x8::splat(2.0) + t270 * t375 - t270 * t385;
            let t391 = ((t71).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t80 * t387));
            let tvsigma2 = t6 * t391;
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
