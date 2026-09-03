//! GGA_X_EV93 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ev93.c`
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
pub fn gga_x_ev93_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a1: f64,
    param_a2: f64,
    param_a3: f64,
    param_b1: f64,
    param_b2: f64,
    param_b3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a1 = f64x8::splat(param_a1);
    let param_a2 = f64x8::splat(param_a2);
    let param_a3 = f64x8::splat(param_a3);
    let param_b1 = f64x8::splat(param_b1);
    let param_b2 = f64x8::splat(param_b2);
    let param_b3 = f64x8::splat(param_b3);
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
            let t26 = t5 * t25;
            let t27 = (simd::cbrt(t6));
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = param_a1 * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t33 * v_sigma0;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t40 = t34 * t39;
            let t43 = t28 * t28;
            let t44 = param_a2 * t43;
            let t46 = f64x8::splat(1.0) / t31 / t30;
            let t47 = v_sigma0 * v_sigma0;
            let t48 = t46 * t47;
            let t49 = t35 * t35;
            let t50 = t49 * v_rho0;
            let t52 = f64x8::splat(1.0) / t36 / t50;
            let t53 = t48 * t52;
            let t56 = t30 * t30;
            let t57 = f64x8::splat(1.0) / t56;
            let t58 = param_a3 * t57;
            let t59 = t47 * v_sigma0;
            let t60 = t49 * t49;
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t59 * t61;
            let t65 = f64x8::splat(1.0) + t29 * t40 / f64x8::splat(24.0) + t44 * t53 / f64x8::splat(576.0) + t58 * t62 / f64x8::splat(2304.0);
            let t66 = t27 * t65;
            let t67 = param_b1 * t28;
            let t70 = param_b2 * t43;
            let t73 = param_b3 * t57;
            let t76 = f64x8::splat(1.0) + t67 * t40 / f64x8::splat(24.0) + t70 * t53 / f64x8::splat(576.0) + t73 * t62 / f64x8::splat(2304.0);
            let t77 = f64x8::splat(1.0) / t76;
            let t78 = t66 * t77;
            let t81 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t78));
            let t82 = (v_rho1).simd_le(dens_threshold);
            let t83 = -t16;
            let t85 = ((t14).select(t11, (t10).select(t15, t83 * t7)));
            let t86 = f64x8::splat(1.0) + t85;
            let t87 = (t86).simd_le(zeta_threshold);
            let t88 = (simd::cbrt(t86));
            let t90 = ((t87).select(t22, t88 * t86));
            let t91 = t5 * t90;
            let t92 = t33 * v_sigma2;
            let t93 = v_rho1 * v_rho1;
            let t94 = (simd::cbrt(v_rho1));
            let t95 = t94 * t94;
            let t97 = f64x8::splat(1.0) / t95 / t93;
            let t98 = t92 * t97;
            let t101 = v_sigma2 * v_sigma2;
            let t102 = t46 * t101;
            let t103 = t93 * t93;
            let t104 = t103 * v_rho1;
            let t106 = f64x8::splat(1.0) / t94 / t104;
            let t107 = t102 * t106;
            let t110 = t101 * v_sigma2;
            let t111 = t103 * t103;
            let t112 = f64x8::splat(1.0) / t111;
            let t113 = t110 * t112;
            let t116 = f64x8::splat(1.0) + t29 * t98 / f64x8::splat(24.0) + t44 * t107 / f64x8::splat(576.0) + t58 * t113 / f64x8::splat(2304.0);
            let t117 = t27 * t116;
            let t124 = f64x8::splat(1.0) + t67 * t98 / f64x8::splat(24.0) + t70 * t107 / f64x8::splat(576.0) + t73 * t113 / f64x8::splat(2304.0);
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t117 * t125;
            let t129 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t126));
            let tzk0 = t81 + t129;
            acc_zk = tzk0;
            let t130 = t6 * t6;
            let t131 = f64x8::splat(1.0) / t130;
            let t132 = t16 * t131;
            let t134 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t132)));
            let t137 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t134));
            let t138 = t5 * t137;
            let t141 = t27 * t27;
            let t142 = f64x8::splat(1.0) / t141;
            let t143 = t142 * t65;
            let t144 = t143 * t77;
            let t146 = t26 * t144 / f64x8::splat(8.0);
            let t147 = t35 * v_rho0;
            let t149 = f64x8::splat(1.0) / t37 / t147;
            let t150 = t34 * t149;
            let t153 = t49 * t35;
            let t155 = f64x8::splat(1.0) / t36 / t153;
            let t156 = t48 * t155;
            let t159 = t60 * v_rho0;
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = t59 * t160;
            let t164 = -t29 * t150 / f64x8::splat(9.0) - t44 * t156 / f64x8::splat(108.0) - t58 * t161 / f64x8::splat(288.0);
            let t165 = t27 * t164;
            let t166 = t165 * t77;
            let t169 = t76 * t76;
            let t170 = f64x8::splat(1.0) / t169;
            let t177 = -t67 * t150 / f64x8::splat(9.0) - t70 * t156 / f64x8::splat(108.0) - t73 * t161 / f64x8::splat(288.0);
            let t178 = t170 * t177;
            let t179 = t66 * t178;
            let t183 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t138 * t78 - t146 - f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t166 + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t179));
            let t184 = t83 * t131;
            let t186 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t184)));
            let t189 = ((t87).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t88 * t186));
            let t190 = t5 * t189;
            let t193 = t142 * t116;
            let t194 = t193 * t125;
            let t196 = t91 * t194 / f64x8::splat(8.0);
            let t198 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t190 * t126 - t196));
            let tvrho0 = t81 + t129 + t6 * (t183 + t198);
            acc_vrho_0 = tvrho0;
            let t202 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t132)));
            let t205 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t202));
            let t206 = t5 * t205;
            let t210 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t206 * t78 - t146));
            let t212 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t184)));
            let t215 = ((t87).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t88 * t212));
            let t216 = t5 * t215;
            let t219 = t93 * v_rho1;
            let t221 = f64x8::splat(1.0) / t95 / t219;
            let t222 = t92 * t221;
            let t225 = t103 * t93;
            let t227 = f64x8::splat(1.0) / t94 / t225;
            let t228 = t102 * t227;
            let t231 = t111 * v_rho1;
            let t232 = f64x8::splat(1.0) / t231;
            let t233 = t110 * t232;
            let t236 = -t29 * t222 / f64x8::splat(9.0) - t44 * t228 / f64x8::splat(108.0) - t58 * t233 / f64x8::splat(288.0);
            let t237 = t27 * t236;
            let t238 = t237 * t125;
            let t241 = t124 * t124;
            let t242 = f64x8::splat(1.0) / t241;
            let t249 = -t67 * t222 / f64x8::splat(9.0) - t70 * t228 / f64x8::splat(108.0) - t73 * t233 / f64x8::splat(288.0);
            let t250 = t242 * t249;
            let t251 = t117 * t250;
            let t255 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t216 * t126 - t196 - f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t238 + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t251));
            let tvrho1 = t81 + t129 + t6 * (t210 + t255);
            acc_vrho_1 = tvrho1;
            let t258 = t33 * t39;
            let t261 = t46 * v_sigma0;
            let t262 = t261 * t52;
            let t265 = t47 * t61;
            let t268 = t29 * t258 / f64x8::splat(24.0) + t44 * t262 / f64x8::splat(288.0) + t58 * t265 / f64x8::splat(768.0);
            let t269 = t27 * t268;
            let t270 = t269 * t77;
            let t278 = t67 * t258 / f64x8::splat(24.0) + t70 * t262 / f64x8::splat(288.0) + t73 * t265 / f64x8::splat(768.0);
            let t279 = t170 * t278;
            let t280 = t66 * t279;
            let t284 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t270 + f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t280));
            let tvsigma0 = t6 * t284;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t285 = t33 * t97;
            let t288 = t46 * v_sigma2;
            let t289 = t288 * t106;
            let t292 = t101 * t112;
            let t295 = t29 * t285 / f64x8::splat(24.0) + t44 * t289 / f64x8::splat(288.0) + t58 * t292 / f64x8::splat(768.0);
            let t296 = t27 * t295;
            let t297 = t296 * t125;
            let t305 = t67 * t285 / f64x8::splat(24.0) + t70 * t289 / f64x8::splat(288.0) + t73 * t292 / f64x8::splat(768.0);
            let t306 = t242 * t305;
            let t307 = t117 * t306;
            let t311 = ((t82).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t297 + f64x8::splat(3.0) / f64x8::splat(8.0) * t91 * t307));
            let tvsigma2 = t6 * t311;
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
