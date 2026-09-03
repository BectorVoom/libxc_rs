//! GGA_C_OP_B88 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_b88.c`
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
pub fn gga_c_op_b88_vxc_pol(
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
            let t50 = t41 * t49;
            let t51 = v_rho0 * v_rho0;
            let t52 = (simd::cbrt(v_rho0));
            let t53 = t52 * t52;
            let t55 = f64x8::splat(1.0) / t53 / t51;
            let t56 = v_sigma0 * t55;
            let t57 = ((v_sigma0).sqrt());
            let t59 = f64x8::splat(1.0) / t52 / v_rho0;
            let t60 = t57 * t59;
            let t61 = (simd::ln(t60 + ((t60 * t60 + f64x8::splat(1.0)).sqrt())));
            let t64 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t60 * t61;
            let t65 = f64x8::splat(1.0) / t64;
            let t69 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t40 * t56 * t65;
            let t70 = f64x8::splat(1.0) / t69;
            let t74 = ((t32).select(f64x8::splat(0.0), t40 * t50 * t70 / f64x8::splat(9.0)));
            let t78 = (t43 * t2 / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t79 = ((t44).select(t14, (t42).select(t17, -t28)));
            let t80 = f64x8::splat(1.0) + t79;
            let t81 = t80 * t2;
            let t82 = (simd::cbrt(t81));
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t41 * t83;
            let t85 = v_rho1 * v_rho1;
            let t86 = (simd::cbrt(v_rho1));
            let t87 = t86 * t86;
            let t89 = f64x8::splat(1.0) / t87 / t85;
            let t90 = v_sigma2 * t89;
            let t91 = ((v_sigma2).sqrt());
            let t93 = f64x8::splat(1.0) / t86 / v_rho1;
            let t94 = t91 * t93;
            let t95 = (simd::ln(t94 + ((t94 * t94 + f64x8::splat(1.0)).sqrt())));
            let t98 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t94 * t95;
            let t99 = f64x8::splat(1.0) / t98;
            let t103 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t40 * t90 * t99;
            let t104 = f64x8::splat(1.0) / t103;
            let t108 = ((t78).select(f64x8::splat(0.0), t40 * t84 * t104 / f64x8::splat(9.0)));
            let t109 = t74 + t108;
            let t110 = (t109).simd_eq(f64x8::splat(0.0));
            let t111 = ((t110).select(f64x8::splat(f64::EPSILON), t109));
            let t114 = f64x8::splat(3.6011538) / t111 + f64x8::splat(0.5764);
            let t115 = t111 * t111;
            let t116 = t115 * t115;
            let t117 = f64x8::splat(1.0) / t116;
            let t119 = t115 * t111;
            let t120 = f64x8::splat(1.0) / t119;
            let t122 = f64x8::splat(1.0) / t115;
            let t124 = f64x8::splat(31.390124030721) * t117 + f64x8::splat(14.9643497914092) * t120 + f64x8::splat(1.7833359087) * t122;
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t114 * t125;
            let tzk0 = ((t11).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t21 * t126));
            acc_zk = tzk0;
            let t129 = t2 * t2;
            let t130 = f64x8::splat(1.0) / t129;
            let t131 = t1 * t130;
            let t132 = t3 - t131;
            let t133 = ((t13).select(f64x8::splat(0.0), (t16).select(f64x8::splat(0.0), t132)));
            let t134 = t18 * t133;
            let t135 = t2 * t114;
            let t136 = t135 * t125;
            let t139 = t20 * t114;
            let t141 = f64x8::splat(0.25) * t139 * t125;
            let t143 = f64x8::splat(1.0) / t48 / t47;
            let t144 = t41 * t143;
            let t145 = ((t24).select(f64x8::splat(0.0), (t27).select(f64x8::splat(0.0), t132)));
            let t146 = ((t42).select(f64x8::splat(0.0), (t44).select(f64x8::splat(0.0), t145)));
            let t148 = t146 * t2 + t45 + f64x8::splat(1.0);
            let t153 = t69 * t69;
            let t154 = f64x8::splat(1.0) / t153;
            let t155 = t51 * v_rho0;
            let t157 = f64x8::splat(1.0) / t53 / t155;
            let t158 = v_sigma0 * t157;
            let t162 = t64 * t64;
            let t163 = f64x8::splat(1.0) / t162;
            let t165 = f64x8::splat(1.0) / t52 / t51;
            let t169 = t56 + f64x8::splat(1.0);
            let t170 = ((t169).sqrt());
            let t171 = f64x8::splat(1.0) / t170;
            let t174 = -f64x8::splat(0.0336) * t57 * t165 * t61 - f64x8::splat(0.0336) * t158 * t171;
            let t175 = t163 * t174;
            let t179 = -f64x8::splat(0.002488888888888889) * t40 * t158 * t65 - f64x8::splat(0.0009333333333333333) * t40 * t56 * t175;
            let t185 = ((t32).select(f64x8::splat(0.0), -t40 * t144 * t70 * t148 / f64x8::splat(27.0) - t40 * t50 * t154 * t179 / f64x8::splat(9.0)));
            let t187 = f64x8::splat(1.0) / t82 / t81;
            let t188 = t41 * t187;
            let t189 = ((t44).select(f64x8::splat(0.0), (t42).select(f64x8::splat(0.0), -t145)));
            let t191 = t189 * t2 + t79 + f64x8::splat(1.0);
            let t196 = ((t78).select(f64x8::splat(0.0), -t40 * t188 * t104 * t191 / f64x8::splat(27.0)));
            let t198 = ((t110).select(f64x8::splat(0.0), t185 + t196));
            let t199 = t122 * t198;
            let t200 = t199 * t125;
            let t203 = t124 * t124;
            let t204 = f64x8::splat(1.0) / t203;
            let t205 = t114 * t204;
            let t207 = f64x8::splat(1.0) / t116 / t111;
            let t208 = t207 * t198;
            let t210 = t117 * t198;
            let t212 = t120 * t198;
            let t214 = -f64x8::splat(125.560496122884) * t208 - f64x8::splat(44.8930493742276) * t210 - f64x8::splat(3.5666718174) * t212;
            let t215 = t205 * t214;
            let t219 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.5) * t134 * t136 - t141 + f64x8::splat(0.90028845) * t21 * t200 + f64x8::splat(0.25) * t21 * t215));
            let tvrho0 = t2 * t219 + tzk0;
            acc_vrho_0 = tvrho0;
            let t221 = -t3 - t131;
            let t222 = ((t13).select(f64x8::splat(0.0), (t16).select(f64x8::splat(0.0), t221)));
            let t223 = t18 * t222;
            let t226 = ((t24).select(f64x8::splat(0.0), (t27).select(f64x8::splat(0.0), t221)));
            let t227 = ((t42).select(f64x8::splat(0.0), (t44).select(f64x8::splat(0.0), t226)));
            let t229 = t2 * t227 + t45 + f64x8::splat(1.0);
            let t234 = ((t32).select(f64x8::splat(0.0), -t40 * t144 * t70 * t229 / f64x8::splat(27.0)));
            let t235 = ((t44).select(f64x8::splat(0.0), (t42).select(f64x8::splat(0.0), -t226)));
            let t237 = t2 * t235 + t79 + f64x8::splat(1.0);
            let t242 = t103 * t103;
            let t243 = f64x8::splat(1.0) / t242;
            let t244 = t85 * v_rho1;
            let t246 = f64x8::splat(1.0) / t87 / t244;
            let t247 = v_sigma2 * t246;
            let t251 = t98 * t98;
            let t252 = f64x8::splat(1.0) / t251;
            let t254 = f64x8::splat(1.0) / t86 / t85;
            let t258 = t90 + f64x8::splat(1.0);
            let t259 = ((t258).sqrt());
            let t260 = f64x8::splat(1.0) / t259;
            let t263 = -f64x8::splat(0.0336) * t91 * t254 * t95 - f64x8::splat(0.0336) * t247 * t260;
            let t264 = t252 * t263;
            let t268 = -f64x8::splat(0.002488888888888889) * t40 * t247 * t99 - f64x8::splat(0.0009333333333333333) * t40 * t90 * t264;
            let t274 = ((t78).select(f64x8::splat(0.0), -t40 * t188 * t104 * t237 / f64x8::splat(27.0) - t40 * t84 * t243 * t268 / f64x8::splat(9.0)));
            let t276 = ((t110).select(f64x8::splat(0.0), t234 + t274));
            let t277 = t122 * t276;
            let t278 = t277 * t125;
            let t281 = t207 * t276;
            let t283 = t117 * t276;
            let t285 = t120 * t276;
            let t287 = -f64x8::splat(125.560496122884) * t281 - f64x8::splat(44.8930493742276) * t283 - f64x8::splat(3.5666718174) * t285;
            let t288 = t205 * t287;
            let t292 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.5) * t223 * t136 - t141 + f64x8::splat(0.90028845) * t21 * t278 + f64x8::splat(0.25) * t21 * t288));
            let tvrho1 = t2 * t292 + tzk0;
            acc_vrho_1 = tvrho1;
            let t298 = f64x8::splat(1.0) / t57;
            let t304 = f64x8::splat(0.0126) * t298 * t59 * t61 + f64x8::splat(0.0126) * t55 * t171;
            let t305 = t163 * t304;
            let t309 = f64x8::splat(0.0009333333333333333) * t38 * t39 * t55 * t65 - f64x8::splat(0.0009333333333333333) * t40 * t56 * t305;
            let t314 = ((t32).select(f64x8::splat(0.0), -t40 * t50 * t154 * t309 / f64x8::splat(9.0)));
            let t315 = ((t110).select(f64x8::splat(0.0), t314));
            let t316 = t122 * t315;
            let t317 = t316 * t125;
            let t320 = t207 * t315;
            let t322 = t117 * t315;
            let t324 = t120 * t315;
            let t326 = -f64x8::splat(125.560496122884) * t320 - f64x8::splat(44.8930493742276) * t322 - f64x8::splat(3.5666718174) * t324;
            let t327 = t205 * t326;
            let t331 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.90028845) * t21 * t317 + f64x8::splat(0.25) * t21 * t327));
            let tvsigma0 = t2 * t331;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t336 = f64x8::splat(1.0) / t91;
            let t342 = f64x8::splat(0.0126) * t336 * t93 * t95 + f64x8::splat(0.0126) * t89 * t260;
            let t343 = t252 * t342;
            let t347 = f64x8::splat(0.0009333333333333333) * t38 * t39 * t89 * t99 - f64x8::splat(0.0009333333333333333) * t40 * t90 * t343;
            let t352 = ((t78).select(f64x8::splat(0.0), -t40 * t84 * t243 * t347 / f64x8::splat(9.0)));
            let t353 = ((t110).select(f64x8::splat(0.0), t352));
            let t354 = t122 * t353;
            let t355 = t354 * t125;
            let t358 = t207 * t353;
            let t360 = t117 * t353;
            let t362 = t120 * t353;
            let t364 = -f64x8::splat(125.560496122884) * t358 - f64x8::splat(44.8930493742276) * t360 - f64x8::splat(3.5666718174) * t362;
            let t365 = t205 * t364;
            let t369 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.90028845) * t21 * t355 + f64x8::splat(0.25) * t21 * t365));
            let tvsigma2 = t2 * t369;
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
