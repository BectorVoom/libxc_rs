//! GGA_C_OP_PBE vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_pbe.c`
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
pub fn gga_c_op_pbe_vxc_pol(
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
            let t5 = ((t4).abs());
            let t11 = ((f64x8::splat(1.0) - t5).simd_le(zeta_threshold)) | (((v_rho0).simd_le(dens_threshold)) & ((v_rho1).simd_le(dens_threshold)));
            let t13 = (f64x8::splat(1.0) + t4).simd_le(zeta_threshold);
            let t14 = zeta_threshold - f64x8::splat(1.0);
            let t16 = (f64x8::splat(1.0) - t4).simd_le(zeta_threshold);
            let t17 = -t14;
            let t18 = ((t13).select(t14, (t16).select(t17, t4)));
            let t19 = t18 * t18;
            let t20 = f64x8::splat(1.0) - t19;
            let t21 = t20 * t2;
            let t24 = (f64x8::splat(2.0) * v_rho0 * t3).simd_le(zeta_threshold);
            let t27 = (f64x8::splat(2.0) * v_rho1 * t3).simd_le(zeta_threshold);
            let t28 = ((t24).select(t14, (t27).select(t17, t4)));
            let t29 = f64x8::splat(1.0) + t28;
            let t32 = (t29 * t2 / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t33 = f64x8::splat(M_CBRT3);
            let t34 = t33 * t33;
            let t36 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t38 = t34 / t36;
            let t39 = f64x8::splat(M_CBRT4);
            let t40 = t38 * t39;
            let t41 = f64x8::splat(M_CBRT2);
            let t42 = (t29).simd_le(zeta_threshold);
            let t43 = f64x8::splat(1.0) - t28;
            let t44 = (t43).simd_le(zeta_threshold);
            let t45 = ((t42).select(t14, (t44).select(t17, t28)));
            let t46 = f64x8::splat(1.0) + t45;
            let t47 = t46 * t2;
            let t48 = (simd::cbrt(t47));
            let t49 = f64x8::splat(1.0) / t48;
            let t51 = f64x8::splat(M_CBRT6);
            let t52 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t53 = (simd::cbrt(t52));
            let t54 = t53 * t53;
            let t55 = f64x8::splat(1.0) / t54;
            let t56 = t51 * t55;
            let t57 = v_rho0 * v_rho0;
            let t58 = (simd::cbrt(v_rho0));
            let t59 = t58 * t58;
            let t61 = f64x8::splat(1.0) / t59 / t57;
            let t65 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t56 * v_sigma0 * t61;
            let t68 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t65;
            let t69 = f64x8::splat(1.0) / t68;
            let t73 = ((t32).select(f64x8::splat(0.0), t40 * t41 * t49 * t69 / f64x8::splat(9.0)));
            let t77 = (t43 * t2 / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t78 = ((t44).select(t14, (t42).select(t17, -t28)));
            let t79 = f64x8::splat(1.0) + t78;
            let t80 = t79 * t2;
            let t81 = (simd::cbrt(t80));
            let t82 = f64x8::splat(1.0) / t81;
            let t84 = v_rho1 * v_rho1;
            let t85 = (simd::cbrt(v_rho1));
            let t86 = t85 * t85;
            let t88 = f64x8::splat(1.0) / t86 / t84;
            let t92 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t56 * v_sigma2 * t88;
            let t95 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t92;
            let t96 = f64x8::splat(1.0) / t95;
            let t100 = ((t77).select(f64x8::splat(0.0), t40 * t41 * t82 * t96 / f64x8::splat(9.0)));
            let t101 = t73 + t100;
            let t102 = (t101).simd_eq(f64x8::splat(0.0));
            let t103 = ((t102).select(f64x8::splat(f64::EPSILON), t101));
            let t106 = f64x8::splat(3.61925846) / t103 + f64x8::splat(0.5764);
            let t107 = t103 * t103;
            let t108 = t107 * t107;
            let t109 = f64x8::splat(1.0) / t108;
            let t111 = t107 * t103;
            let t112 = f64x8::splat(1.0) / t111;
            let t114 = f64x8::splat(1.0) / t107;
            let t116 = f64x8::splat(32.02615087407435) * t109 + f64x8::splat(15.19118443242906) * t112 + f64x8::splat(1.801312286343) * t114;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t106 * t117;
            let tzk0 = ((t11).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t21 * t118));
            acc_zk = tzk0;
            let t121 = t2 * t2;
            let t122 = f64x8::splat(1.0) / t121;
            let t123 = t1 * t122;
            let t124 = t3 - t123;
            let t125 = ((t13).select(f64x8::splat(0.0), (t16).select(f64x8::splat(0.0), t124)));
            let t126 = t18 * t125;
            let t127 = t2 * t106;
            let t128 = t127 * t117;
            let t131 = t20 * t106;
            let t133 = f64x8::splat(0.25) * t131 * t117;
            let t135 = f64x8::splat(1.0) / t48 / t47;
            let t136 = t41 * t135;
            let t137 = ((t24).select(f64x8::splat(0.0), (t27).select(f64x8::splat(0.0), t124)));
            let t138 = ((t42).select(f64x8::splat(0.0), (t44).select(f64x8::splat(0.0), t137)));
            let t140 = t138 * t2 + t45 + f64x8::splat(1.0);
            let t145 = t39 * t41;
            let t147 = t38 * t145 * t49;
            let t148 = t68 * t68;
            let t149 = f64x8::splat(1.0) / t148;
            let t150 = t65 * t65;
            let t151 = f64x8::splat(1.0) / t150;
            let t152 = t149 * t151;
            let t153 = t152 * t51;
            let t154 = t55 * v_sigma0;
            let t155 = t57 * v_rho0;
            let t157 = f64x8::splat(1.0) / t59 / t155;
            let t158 = t154 * t157;
            let t163 = ((t32).select(f64x8::splat(0.0), -t40 * t136 * t69 * t140 / f64x8::splat(27.0) + f64x8::splat(0.001751827044870964) * t147 * t153 * t158));
            let t165 = f64x8::splat(1.0) / t81 / t80;
            let t166 = t41 * t165;
            let t167 = ((t44).select(f64x8::splat(0.0), (t42).select(f64x8::splat(0.0), -t137)));
            let t169 = t167 * t2 + t78 + f64x8::splat(1.0);
            let t174 = ((t77).select(f64x8::splat(0.0), -t40 * t166 * t96 * t169 / f64x8::splat(27.0)));
            let t176 = ((t102).select(f64x8::splat(0.0), t163 + t174));
            let t177 = t114 * t176;
            let t178 = t177 * t117;
            let t181 = t116 * t116;
            let t182 = f64x8::splat(1.0) / t181;
            let t183 = t106 * t182;
            let t185 = f64x8::splat(1.0) / t108 / t103;
            let t186 = t185 * t176;
            let t188 = t109 * t176;
            let t190 = t112 * t176;
            let t192 = -f64x8::splat(128.1046034962974) * t186 - f64x8::splat(45.57355329728718) * t188 - f64x8::splat(3.602624572686) * t190;
            let t193 = t183 * t192;
            let t197 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.5) * t126 * t128 - t133 + f64x8::splat(0.904814615) * t21 * t178 + f64x8::splat(0.25) * t21 * t193));
            let tvrho0 = t2 * t197 + tzk0;
            acc_vrho_0 = tvrho0;
            let t199 = -t3 - t123;
            let t200 = ((t13).select(f64x8::splat(0.0), (t16).select(f64x8::splat(0.0), t199)));
            let t201 = t18 * t200;
            let t204 = ((t24).select(f64x8::splat(0.0), (t27).select(f64x8::splat(0.0), t199)));
            let t205 = ((t42).select(f64x8::splat(0.0), (t44).select(f64x8::splat(0.0), t204)));
            let t207 = t205 * t2 + t45 + f64x8::splat(1.0);
            let t212 = ((t32).select(f64x8::splat(0.0), -t40 * t136 * t69 * t207 / f64x8::splat(27.0)));
            let t213 = ((t44).select(f64x8::splat(0.0), (t42).select(f64x8::splat(0.0), -t204)));
            let t215 = t213 * t2 + t78 + f64x8::splat(1.0);
            let t221 = t38 * t145 * t82;
            let t222 = t95 * t95;
            let t223 = f64x8::splat(1.0) / t222;
            let t224 = t92 * t92;
            let t225 = f64x8::splat(1.0) / t224;
            let t226 = t223 * t225;
            let t227 = t226 * t51;
            let t228 = t55 * v_sigma2;
            let t229 = t84 * v_rho1;
            let t231 = f64x8::splat(1.0) / t86 / t229;
            let t232 = t228 * t231;
            let t237 = ((t77).select(f64x8::splat(0.0), -t40 * t166 * t96 * t215 / f64x8::splat(27.0) + f64x8::splat(0.001751827044870964) * t221 * t227 * t232));
            let t239 = ((t102).select(f64x8::splat(0.0), t212 + t237));
            let t240 = t114 * t239;
            let t241 = t240 * t117;
            let t244 = t185 * t239;
            let t246 = t109 * t239;
            let t248 = t112 * t239;
            let t250 = -f64x8::splat(128.1046034962974) * t244 - f64x8::splat(45.57355329728718) * t246 - f64x8::splat(3.602624572686) * t248;
            let t251 = t183 * t250;
            let t255 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.5) * t201 * t128 - t133 + f64x8::splat(0.904814615) * t21 * t241 + f64x8::splat(0.25) * t21 * t251));
            let tvrho1 = t2 * t255 + tzk0;
            acc_vrho_1 = tvrho1;
            let t261 = ((t32).select(f64x8::splat(0.0), -f64x8::splat(0.0006569351418266115) * t147 * t152 * t56 * t61));
            let t262 = ((t102).select(f64x8::splat(0.0), t261));
            let t263 = t114 * t262;
            let t264 = t263 * t117;
            let t267 = t185 * t262;
            let t269 = t109 * t262;
            let t271 = t112 * t262;
            let t273 = -f64x8::splat(128.1046034962974) * t267 - f64x8::splat(45.57355329728718) * t269 - f64x8::splat(3.602624572686) * t271;
            let t274 = t183 * t273;
            let t278 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.904814615) * t21 * t264 + f64x8::splat(0.25) * t21 * t274));
            let tvsigma0 = t2 * t278;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t283 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(0.0006569351418266115) * t221 * t226 * t56 * t88));
            let t284 = ((t102).select(f64x8::splat(0.0), t283));
            let t285 = t114 * t284;
            let t286 = t285 * t117;
            let t289 = t185 * t284;
            let t291 = t109 * t284;
            let t293 = t112 * t284;
            let t295 = -f64x8::splat(128.1046034962974) * t289 - f64x8::splat(45.57355329728718) * t291 - f64x8::splat(3.602624572686) * t293;
            let t296 = t183 * t295;
            let t300 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.904814615) * t21 * t286 + f64x8::splat(0.25) * t21 * t296));
            let tvsigma2 = t2 * t300;
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
