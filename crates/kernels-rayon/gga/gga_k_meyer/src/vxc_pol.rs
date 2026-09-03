//! GGA_K_MEYER vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_meyer.c`
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
pub fn gga_k_meyer_vxc_pol(
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
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = t2 * t2;
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 * t4 * f64x8::splat(M_PI);
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
            let t23 = t22 * t22;
            let t24 = t23 * zeta_threshold;
            let t25 = (simd::cbrt(t20));
            let t26 = t25 * t25;
            let t28 = ((t21).select(t24, t26 * t20));
            let t29 = (simd::cbrt(t7));
            let t30 = t29 * t29;
            let t31 = t28 * t30;
            let t32 = f64x8::splat(M_CBRT6);
            let t33 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t34 = (simd::cbrt(t33));
            let t35 = t34 * t34;
            let t36 = f64x8::splat(1.0) / t35;
            let t37 = t32 * t36;
            let t38 = v_rho0 * v_rho0;
            let t39 = (simd::cbrt(v_rho0));
            let t40 = t39 * t39;
            let t42 = f64x8::splat(1.0) / t40 / t38;
            let t46 = f64x8::splat(1.0) - t37 * v_sigma0 * t42 / f64x8::splat(864.0);
            let t47 = t32 * t32;
            let t48 = f64x8::splat(1.0) / t34;
            let t49 = t47 * t48;
            let t50 = ((v_sigma0).sqrt());
            let t51 = t39 * v_rho0;
            let t52 = f64x8::splat(1.0) / t51;
            let t55 = t49 * t50 * t52 / f64x8::splat(72.0);
            let t56 = f64x8::splat(1.0) + t55;
            let t57 = f64x8::splat(1.0) - t55;
            let t58 = ((t57).abs());
            let t59 = f64x8::splat(1.0) / t58;
            let t61 = (simd::ln(t56 * t59));
            let t63 = t46 * t61 * t32;
            let t64 = f64x8::splat(1.0) / t50;
            let t65 = t34 * t64;
            let t68 = f64x8::splat(3.0) * t63 * t65 * t51;
            let t69 = f64x8::splat(1.0) / f64x8::splat(2.0) - t68;
            let t70 = f64x8::splat(1.0) / f64x8::splat(2.0) + t68;
            let t71 = f64x8::splat(1.0) / t70;
            let t74 = f64x8::splat(20.0) * t69 * t71 + f64x8::splat(1.0);
            let t78 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t74));
            let t79 = (v_rho1).simd_le(dens_threshold);
            let t80 = -t17;
            let t82 = ((t15).select(t12, (t11).select(t16, t80 * t8)));
            let t83 = f64x8::splat(1.0) + t82;
            let t84 = (t83).simd_le(zeta_threshold);
            let t85 = (simd::cbrt(t83));
            let t86 = t85 * t85;
            let t88 = ((t84).select(t24, t86 * t83));
            let t89 = t88 * t30;
            let t90 = v_rho1 * v_rho1;
            let t91 = (simd::cbrt(v_rho1));
            let t92 = t91 * t91;
            let t94 = f64x8::splat(1.0) / t92 / t90;
            let t98 = f64x8::splat(1.0) - t37 * v_sigma2 * t94 / f64x8::splat(864.0);
            let t99 = ((v_sigma2).sqrt());
            let t100 = t91 * v_rho1;
            let t101 = f64x8::splat(1.0) / t100;
            let t104 = t49 * t99 * t101 / f64x8::splat(72.0);
            let t105 = f64x8::splat(1.0) + t104;
            let t106 = f64x8::splat(1.0) - t104;
            let t107 = ((t106).abs());
            let t108 = f64x8::splat(1.0) / t107;
            let t110 = (simd::ln(t105 * t108));
            let t112 = t98 * t110 * t32;
            let t113 = f64x8::splat(1.0) / t99;
            let t114 = t34 * t113;
            let t117 = f64x8::splat(3.0) * t112 * t114 * t100;
            let t118 = f64x8::splat(1.0) / f64x8::splat(2.0) - t117;
            let t119 = f64x8::splat(1.0) / f64x8::splat(2.0) + t117;
            let t120 = f64x8::splat(1.0) / t119;
            let t123 = f64x8::splat(20.0) * t118 * t120 + f64x8::splat(1.0);
            let t127 = ((t79).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t89 * t123));
            let tzk0 = t78 + t127;
            acc_zk = tzk0;
            let t128 = t7 * t7;
            let t129 = f64x8::splat(1.0) / t128;
            let t130 = t17 * t129;
            let t132 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t130)));
            let t135 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t132));
            let t136 = t135 * t30;
            let t140 = f64x8::splat(1.0) / t29;
            let t141 = t28 * t140;
            let t144 = t6 * t141 * t74 / f64x8::splat(10.0);
            let t146 = f64x8::splat(1.0) / t39 / t38;
            let t147 = t50 * t146;
            let t153 = t58 * t58;
            let t154 = f64x8::splat(1.0) / t153;
            let t155 = t56 * t154;
            let t156 = t155 * t47;
            let t157 = t48 * t50;
            let t158 = ((t57).abs()) / t57;
            let t159 = t146 * t158;
            let t163 = -t49 * t147 * t59 / f64x8::splat(54.0) - t156 * t157 * t159 / f64x8::splat(54.0);
            let t164 = t46 * t163;
            let t165 = f64x8::splat(1.0) / t56;
            let t166 = t165 * t58;
            let t167 = t164 * t166;
            let t168 = t32 * t34;
            let t170 = t168 * t64 * t51;
            let t176 = -t49 * t147 * t61 / f64x8::splat(108.0) - f64x8::splat(3.0) * t167 * t170 - f64x8::splat(4.0) * t63 * t65 * t39;
            let t178 = t70 * t70;
            let t179 = f64x8::splat(1.0) / t178;
            let t180 = t69 * t179;
            let t181 = -t176;
            let t184 = f64x8::splat(20.0) * t176 * t71 - f64x8::splat(20.0) * t180 * t181;
            let t189 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t136 * t74 + t144 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t184));
            let t190 = t80 * t129;
            let t192 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t190)));
            let t195 = ((t84).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t86 * t192));
            let t196 = t195 * t30;
            let t200 = t88 * t140;
            let t203 = t6 * t200 * t123 / f64x8::splat(10.0);
            let t205 = ((t79).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t196 * t123 + t203));
            let tvrho0 = t78 + t127 + t7 * (t189 + t205);
            acc_vrho_0 = tvrho0;
            let t209 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t130)));
            let t212 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t209));
            let t213 = t212 * t30;
            let t218 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t213 * t74 + t144));
            let t220 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t190)));
            let t223 = ((t84).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t86 * t220));
            let t224 = t223 * t30;
            let t229 = f64x8::splat(1.0) / t91 / t90;
            let t230 = t99 * t229;
            let t236 = t107 * t107;
            let t237 = f64x8::splat(1.0) / t236;
            let t238 = t105 * t237;
            let t239 = t238 * t47;
            let t240 = t48 * t99;
            let t241 = ((t106).abs()) / t106;
            let t242 = t229 * t241;
            let t246 = -t49 * t230 * t108 / f64x8::splat(54.0) - t239 * t240 * t242 / f64x8::splat(54.0);
            let t247 = t98 * t246;
            let t248 = f64x8::splat(1.0) / t105;
            let t249 = t248 * t107;
            let t250 = t247 * t249;
            let t252 = t168 * t113 * t100;
            let t258 = -t49 * t230 * t110 / f64x8::splat(108.0) - f64x8::splat(3.0) * t250 * t252 - f64x8::splat(4.0) * t112 * t114 * t91;
            let t260 = t119 * t119;
            let t261 = f64x8::splat(1.0) / t260;
            let t262 = t118 * t261;
            let t263 = -t258;
            let t266 = f64x8::splat(20.0) * t258 * t120 - f64x8::splat(20.0) * t262 * t263;
            let t271 = ((t79).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t224 * t123 + t203 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t89 * t266));
            let tvrho1 = t78 + t127 + t7 * (t218 + t271);
            acc_vrho_1 = tvrho1;
            let t274 = t52 * t61;
            let t281 = t48 * t64;
            let t282 = t52 * t158;
            let t286 = t49 * t64 * t52 * t59 / f64x8::splat(144.0) + t156 * t281 * t282 / f64x8::splat(144.0);
            let t287 = t46 * t286;
            let t288 = t287 * t166;
            let t291 = t50 * v_sigma0;
            let t292 = f64x8::splat(1.0) / t291;
            let t293 = t34 * t292;
            let t297 = t49 * t274 * t64 / f64x8::splat(288.0) - f64x8::splat(3.0) * t288 * t170 + f64x8::splat(3.0) / f64x8::splat(2.0) * t63 * t293 * t51;
            let t299 = -t297;
            let t302 = -f64x8::splat(20.0) * t180 * t299 + f64x8::splat(20.0) * t297 * t71;
            let t306 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t302));
            let tvsigma0 = t7 * t306;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t307 = t101 * t110;
            let t314 = t48 * t113;
            let t315 = t101 * t241;
            let t319 = t49 * t113 * t101 * t108 / f64x8::splat(144.0) + t239 * t314 * t315 / f64x8::splat(144.0);
            let t320 = t98 * t319;
            let t321 = t320 * t249;
            let t324 = t99 * v_sigma2;
            let t325 = f64x8::splat(1.0) / t324;
            let t326 = t34 * t325;
            let t330 = t49 * t307 * t113 / f64x8::splat(288.0) - f64x8::splat(3.0) * t321 * t252 + f64x8::splat(3.0) / f64x8::splat(2.0) * t112 * t326 * t100;
            let t332 = -t330;
            let t335 = f64x8::splat(20.0) * t330 * t120 - f64x8::splat(20.0) * t262 * t332;
            let t339 = ((t79).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t89 * t335));
            let tvsigma2 = t7 * t339;
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
