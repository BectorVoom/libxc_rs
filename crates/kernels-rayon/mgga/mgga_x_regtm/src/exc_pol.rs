//! MGGA_X_REGTM exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_regtm.c`
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
pub fn mgga_x_regtm_exc_pol(
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
            let t29 = (simd::cbrt(v_rho0));
            let t30 = t29 * t29;
            let t32 = f64x8::splat(1.0) / t30 / v_rho0;
            let t33 = v_tau0 * t32;
            let t34 = v_rho0 * v_rho0;
            let t36 = f64x8::splat(1.0) / t30 / t34;
            let t37 = v_sigma0 * t36;
            let t39 = t33 - t37 / f64x8::splat(8.0);
            let t40 = f64x8::splat(M_CBRT6);
            let t41 = t39 * t40;
            let t42 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t43 = (simd::cbrt(t42));
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t40 * t45;
            let t47 = t46 * t37;
            let t49 = t41 * t45;
            let t51 = f64x8::splat(1.0) - f64x8::splat(5.0) / f64x8::splat(9.0) * t49;
            let t52 = t51 * t51;
            let t53 = t52 * t51;
            let t54 = t39 * t39;
            let t55 = t40 * t40;
            let t56 = t54 * t55;
            let t58 = f64x8::splat(1.0) / t43 / t42;
            let t61 = f64x8::splat(1.0) + f64x8::splat(0.6714891975308642) * t56 * t58;
            let t62 = ((t61).sqrt());
            let t64 = f64x8::splat(1.0) / t62 / t61;
            let t65 = t53 * t64;
            let t67 = (simd::exp(-t47 / f64x8::splat(8.0)));
            let t69 = t47 / f64x8::splat(24.0) + t65 * t67;
            let t71 = t45 / t69;
            let t74 = f64x8::splat(1.0) + t41 * t71 / f64x8::splat(3.0);
            let t75 = t74 * t74;
            let t77 = t75 * t74;
            let t78 = f64x8::splat(1.0) / t77;
            let t80 = f64x8::splat(1.0) / t75 + f64x8::splat(3.0) * t78;
            let t81 = f64x8::splat(1.0) + t78;
            let t82 = t81 * t81;
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t80 * t83;
            let t86 = t55 * t58;
            let t87 = v_sigma0 * v_sigma0;
            let t88 = t34 * t34;
            let t89 = t88 * v_rho0;
            let t91 = f64x8::splat(1.0) / t29 / t89;
            let t95 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t47 + f64x8::splat(0.002689949046226295) * t86 * t87 * t91;
            let t96 = (simd::pow(t95, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t101 = f64x8::splat(0.256337604) * t55 * t44;
            let t107 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t47 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.14554132) * t33 + t101 + f64x8::splat(0.011867481666666667) * t37) * t40 * t45;
            let t108 = t96 * t96;
            let t109 = f64x8::splat(1.0) / t108;
            let t112 = f64x8::splat(1.0) / t96 + f64x8::splat(7.0) / f64x8::splat(9.0) * t107 * t109;
            let t114 = f64x8::splat(1.0) - t84;
            let t117 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t47) * t40;
            let t118 = t45 * v_sigma0;
            let t124 = t49 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) + t47 / f64x8::splat(36.0);
            let t125 = t124 * t124;
            let t127 = f64x8::splat(1.0) / v_rho0;
            let t128 = v_sigma0 * t127;
            let t129 = f64x8::splat(1.0) / v_tau0;
            let t131 = t128 * t129 / f64x8::splat(8.0);
            let t132 = (t131).simd_lt(f64x8::splat(1.0));
            let t133 = ((t132).select(t131, f64x8::splat(1.0)));
            let t134 = t124 * t133;
            let t135 = f64x8::splat(1.0) - t133;
            let t138 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t117 * t118 * t36 + f64x8::splat(292.0) / f64x8::splat(405.0) * t125 - f64x8::splat(146.0) / f64x8::splat(135.0) * t134 * t135;
            let t139 = (simd::pow(t138, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t141 = t84 * t112 + t114 * t139;
            let t145 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t141));
            let t146 = (v_rho1).simd_le(dens_threshold);
            let t147 = -t17;
            let t149 = ((t15).select(t12, (t11).select(t16, t147 * t8)));
            let t150 = f64x8::splat(1.0) + t149;
            let t151 = (t150).simd_le(zeta_threshold);
            let t152 = (simd::cbrt(t150));
            let t154 = ((t151).select(t23, t152 * t150));
            let t155 = t154 * t27;
            let t156 = (simd::cbrt(v_rho1));
            let t157 = t156 * t156;
            let t159 = f64x8::splat(1.0) / t157 / v_rho1;
            let t160 = v_tau1 * t159;
            let t161 = v_rho1 * v_rho1;
            let t163 = f64x8::splat(1.0) / t157 / t161;
            let t164 = v_sigma2 * t163;
            let t166 = t160 - t164 / f64x8::splat(8.0);
            let t167 = t166 * t40;
            let t168 = t46 * t164;
            let t170 = t167 * t45;
            let t172 = f64x8::splat(1.0) - f64x8::splat(5.0) / f64x8::splat(9.0) * t170;
            let t173 = t172 * t172;
            let t174 = t173 * t172;
            let t175 = t166 * t166;
            let t176 = t175 * t55;
            let t179 = f64x8::splat(1.0) + f64x8::splat(0.6714891975308642) * t176 * t58;
            let t180 = ((t179).sqrt());
            let t182 = f64x8::splat(1.0) / t180 / t179;
            let t183 = t174 * t182;
            let t185 = (simd::exp(-t168 / f64x8::splat(8.0)));
            let t187 = t168 / f64x8::splat(24.0) + t183 * t185;
            let t189 = t45 / t187;
            let t192 = f64x8::splat(1.0) + t167 * t189 / f64x8::splat(3.0);
            let t193 = t192 * t192;
            let t195 = t193 * t192;
            let t196 = f64x8::splat(1.0) / t195;
            let t198 = f64x8::splat(1.0) / t193 + f64x8::splat(3.0) * t196;
            let t199 = f64x8::splat(1.0) + t196;
            let t200 = t199 * t199;
            let t201 = f64x8::splat(1.0) / t200;
            let t202 = t198 * t201;
            let t204 = v_sigma2 * v_sigma2;
            let t205 = t161 * t161;
            let t206 = t205 * v_rho1;
            let t208 = f64x8::splat(1.0) / t156 / t206;
            let t212 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t168 + f64x8::splat(0.002689949046226295) * t86 * t204 * t208;
            let t213 = (simd::pow(t212, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t222 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t168 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.14554132) * t160 + t101 + f64x8::splat(0.011867481666666667) * t164) * t40 * t45;
            let t223 = t213 * t213;
            let t224 = f64x8::splat(1.0) / t223;
            let t227 = f64x8::splat(1.0) / t213 + f64x8::splat(7.0) / f64x8::splat(9.0) * t222 * t224;
            let t229 = f64x8::splat(1.0) - t202;
            let t232 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t168) * t40;
            let t233 = t45 * v_sigma2;
            let t239 = t170 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) + t168 / f64x8::splat(36.0);
            let t240 = t239 * t239;
            let t242 = f64x8::splat(1.0) / v_rho1;
            let t243 = v_sigma2 * t242;
            let t244 = f64x8::splat(1.0) / v_tau1;
            let t246 = t243 * t244 / f64x8::splat(8.0);
            let t247 = (t246).simd_lt(f64x8::splat(1.0));
            let t248 = ((t247).select(t246, f64x8::splat(1.0)));
            let t249 = t239 * t248;
            let t250 = f64x8::splat(1.0) - t248;
            let t253 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t232 * t233 * t163 + f64x8::splat(292.0) / f64x8::splat(405.0) * t240 - f64x8::splat(146.0) / f64x8::splat(135.0) * t249 * t250;
            let t254 = (simd::pow(t253, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t256 = t202 * t227 + t229 * t254;
            let t260 = ((t146).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t155 * t256));
            let tzk0 = t145 + t260;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
