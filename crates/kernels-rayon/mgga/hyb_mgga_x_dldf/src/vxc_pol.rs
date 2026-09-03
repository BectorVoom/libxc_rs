//! HYB_MGGA_X_DLDF vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_dldf.c`
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
pub fn hyb_mgga_x_dldf_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = v_rho0 + v_rho1;
            let t5 = f64x8::splat(1.0) / t4;
            let t8 = (f64x8::splat(2.0) * v_rho0 * t5).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t12 = (f64x8::splat(2.0) * v_rho1 * t5).simd_le(zeta_threshold);
            let t13 = -t9;
            let t14 = v_rho0 - v_rho1;
            let t16 = ((t8).select(t9, (t12).select(t13, t14 * t5)));
            let t17 = f64x8::splat(1.0) + t16;
            let t18 = (t17).simd_le(zeta_threshold);
            let t19 = (simd::cbrt(zeta_threshold));
            let t20 = t19 * zeta_threshold;
            let t21 = (simd::cbrt(t17));
            let t23 = ((t18).select(t20, t21 * t17));
            let t24 = t3 * t23;
            let t25 = (simd::cbrt(t4));
            let t26 = f64x8::splat(M_CBRT6);
            let t27 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t28 = (simd::cbrt(t27));
            let t29 = t28 * t28;
            let t30 = f64x8::splat(1.0) / t29;
            let t31 = t26 * t30;
            let t32 = v_rho0 * v_rho0;
            let t33 = (simd::cbrt(v_rho0));
            let t34 = t33 * t33;
            let t36 = f64x8::splat(1.0) / t34 / t32;
            let t40 = f64x8::splat(4.8827323) + f64x8::splat(0.0146297) * t31 * v_sigma0 * t36;
            let t43 = f64x8::splat(5.8827323) - f64x8::splat(23.84107471346329) / t40;
            let t44 = t25 * t43;
            let t45 = t26 * t26;
            let t47 = f64x8::splat(3.0) / f64x8::splat(10.0) * t45 * t29;
            let t49 = f64x8::splat(1.0) / t34 / v_rho0;
            let t50 = v_tau0 * t49;
            let t51 = t47 - t50;
            let t52 = t47 + t50;
            let t53 = f64x8::splat(1.0) / t52;
            let t56 = t51 * t51;
            let t57 = t52 * t52;
            let t58 = f64x8::splat(1.0) / t57;
            let t61 = t56 * t51;
            let t62 = t57 * t52;
            let t63 = f64x8::splat(1.0) / t62;
            let t66 = t56 * t56;
            let t67 = t57 * t57;
            let t68 = f64x8::splat(1.0) / t67;
            let t71 = f64x8::splat(1.0) - f64x8::splat(0.1637571) * t51 * t53 - f64x8::splat(0.1880028) * t56 * t58 - f64x8::splat(0.4490609) * t61 * t63 - f64x8::splat(0.0082359) * t66 * t68;
            let t72 = t44 * t71;
            let t75 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.09872727257880975) * t24 * t72));
            let t76 = (v_rho1).simd_le(dens_threshold);
            let t77 = -t14;
            let t79 = ((t12).select(t9, (t8).select(t13, t77 * t5)));
            let t80 = f64x8::splat(1.0) + t79;
            let t81 = (t80).simd_le(zeta_threshold);
            let t82 = (simd::cbrt(t80));
            let t84 = ((t81).select(t20, t82 * t80));
            let t85 = t3 * t84;
            let t86 = v_rho1 * v_rho1;
            let t87 = (simd::cbrt(v_rho1));
            let t88 = t87 * t87;
            let t90 = f64x8::splat(1.0) / t88 / t86;
            let t94 = f64x8::splat(4.8827323) + f64x8::splat(0.0146297) * t31 * v_sigma2 * t90;
            let t97 = f64x8::splat(5.8827323) - f64x8::splat(23.84107471346329) / t94;
            let t98 = t25 * t97;
            let t100 = f64x8::splat(1.0) / t88 / v_rho1;
            let t101 = v_tau1 * t100;
            let t102 = t47 - t101;
            let t103 = t47 + t101;
            let t104 = f64x8::splat(1.0) / t103;
            let t107 = t102 * t102;
            let t108 = t103 * t103;
            let t109 = f64x8::splat(1.0) / t108;
            let t112 = t107 * t102;
            let t113 = t108 * t103;
            let t114 = f64x8::splat(1.0) / t113;
            let t117 = t107 * t107;
            let t118 = t108 * t108;
            let t119 = f64x8::splat(1.0) / t118;
            let t122 = f64x8::splat(1.0) - f64x8::splat(0.1637571) * t102 * t104 - f64x8::splat(0.1880028) * t107 * t109 - f64x8::splat(0.4490609) * t112 * t114 - f64x8::splat(0.0082359) * t117 * t119;
            let t123 = t98 * t122;
            let t126 = ((t76).select(f64x8::splat(0.0), -f64x8::splat(0.09872727257880975) * t85 * t123));
            let tzk0 = t75 + t126;
            acc_zk = tzk0;
            let t127 = t4 * t4;
            let t128 = f64x8::splat(1.0) / t127;
            let t129 = t14 * t128;
            let t131 = ((t8).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), t5 - t129)));
            let t134 = ((t18).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t21 * t131));
            let t135 = t3 * t134;
            let t138 = t25 * t25;
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t139 * t43;
            let t141 = t140 * t71;
            let t143 = f64x8::splat(0.03290909085960325) * t24 * t141;
            let t144 = t40 * t40;
            let t145 = f64x8::splat(1.0) / t144;
            let t146 = t25 * t145;
            let t147 = t24 * t146;
            let t148 = t32 * v_rho0;
            let t150 = f64x8::splat(1.0) / t34 / t148;
            let t151 = v_sigma0 * t150;
            let t153 = t31 * t151 * t71;
            let t156 = v_tau0 * t36;
            let t159 = t51 * t58;
            let t162 = t56 * t63;
            let t165 = t61 * t68;
            let t169 = f64x8::splat(1.0) / t67 / t52;
            let t170 = t66 * t169;
            let t173 = -f64x8::splat(0.2729285) * t156 * t53 - f64x8::splat(0.8996045) * t159 * t156 - f64x8::splat(2.8719805) * t162 * t156 - f64x8::splat(2.3002105) * t165 * t156 - f64x8::splat(0.054906) * t170 * t156;
            let t174 = t44 * t173;
            let t178 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.09872727257880975) * t135 * t72 - t143 + f64x8::splat(0.09182630750283849) * t147 * t153 - f64x8::splat(0.09872727257880975) * t24 * t174));
            let t179 = t77 * t128;
            let t181 = ((t12).select(f64x8::splat(0.0), (t8).select(f64x8::splat(0.0), -t5 - t179)));
            let t184 = ((t81).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t82 * t181));
            let t185 = t3 * t184;
            let t188 = t139 * t97;
            let t189 = t188 * t122;
            let t191 = f64x8::splat(0.03290909085960325) * t85 * t189;
            let t193 = ((t76).select(f64x8::splat(0.0), -f64x8::splat(0.09872727257880975) * t185 * t123 - t191));
            let tvrho0 = t75 + t126 + t4 * (t178 + t193);
            acc_vrho_0 = tvrho0;
            let t197 = ((t8).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), -t5 - t129)));
            let t200 = ((t18).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t21 * t197));
            let t201 = t3 * t200;
            let t205 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.09872727257880975) * t201 * t72 - t143));
            let t207 = ((t12).select(f64x8::splat(0.0), (t8).select(f64x8::splat(0.0), t5 - t179)));
            let t210 = ((t81).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t82 * t207));
            let t211 = t3 * t210;
            let t214 = t94 * t94;
            let t215 = f64x8::splat(1.0) / t214;
            let t216 = t25 * t215;
            let t217 = t85 * t216;
            let t218 = t86 * v_rho1;
            let t220 = f64x8::splat(1.0) / t88 / t218;
            let t221 = v_sigma2 * t220;
            let t223 = t31 * t221 * t122;
            let t226 = v_tau1 * t90;
            let t229 = t102 * t109;
            let t232 = t107 * t114;
            let t235 = t112 * t119;
            let t239 = f64x8::splat(1.0) / t118 / t103;
            let t240 = t117 * t239;
            let t243 = -f64x8::splat(0.2729285) * t226 * t104 - f64x8::splat(0.8996045) * t229 * t226 - f64x8::splat(2.8719805) * t232 * t226 - f64x8::splat(2.3002105) * t235 * t226 - f64x8::splat(0.054906) * t240 * t226;
            let t244 = t98 * t243;
            let t248 = ((t76).select(f64x8::splat(0.0), -f64x8::splat(0.09872727257880975) * t211 * t123 - t191 + f64x8::splat(0.09182630750283849) * t217 * t223 - f64x8::splat(0.09872727257880975) * t85 * t244));
            let tvrho1 = t75 + t126 + t4 * (t205 + t248);
            acc_vrho_1 = tvrho1;
            let t252 = t31 * t36 * t71;
            let t255 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.03443486531356443) * t147 * t252));
            let tvsigma0 = t4 * t255;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t257 = t31 * t90 * t122;
            let t260 = ((t76).select(f64x8::splat(0.0), -f64x8::splat(0.03443486531356443) * t217 * t257));
            let tvsigma2 = t4 * t260;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t271 = f64x8::splat(0.1637571) * t49 * t53 + f64x8::splat(0.5397627) * t159 * t49 + f64x8::splat(1.7231883) * t162 * t49 + f64x8::splat(1.3801263) * t165 * t49 + f64x8::splat(0.0329436) * t170 * t49;
            let t272 = t44 * t271;
            let t275 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.09872727257880975) * t24 * t272));
            let tvtau0 = t4 * t275;
            acc_vtau_0 = tvtau0;
            let t286 = f64x8::splat(0.1637571) * t100 * t104 + f64x8::splat(0.5397627) * t229 * t100 + f64x8::splat(1.7231883) * t232 * t100 + f64x8::splat(1.3801263) * t235 * t100 + f64x8::splat(0.0329436) * t240 * t100;
            let t287 = t98 * t286;
            let t290 = ((t76).select(f64x8::splat(0.0), -f64x8::splat(0.09872727257880975) * t85 * t287));
            let tvtau1 = t4 * t290;
            acc_vtau_1 = tvtau1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
