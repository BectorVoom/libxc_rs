//! MGGA_X_VT84 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_vt84.c`
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
pub fn mgga_x_vt84_exc_pol(
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
            let t29 = v_sigma0 * v_sigma0;
            let t30 = t29 * v_sigma0;
            let t31 = v_rho0 * v_rho0;
            let t32 = t31 * v_rho0;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t30 * t33;
            let t35 = v_tau0 * v_tau0;
            let t36 = t35 * v_tau0;
            let t37 = f64x8::splat(1.0) / t36;
            let t38 = f64x8::splat(1.0) / t31;
            let t39 = t29 * t38;
            let t40 = f64x8::splat(1.0) / t35;
            let t41 = t39 * t40;
            let t43 = f64x8::splat(1.0) + t41 / f64x8::splat(64.0);
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t37 * t45;
            let t50 = f64x8::splat(M_CBRT6);
            let t51 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.00419826171875) * t34 * t46) * t50;
            let t52 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t53 = (simd::cbrt(t52));
            let t54 = t53 * t53;
            let t55 = f64x8::splat(1.0) / t54;
            let t56 = t55 * v_sigma0;
            let t57 = (simd::cbrt(v_rho0));
            let t58 = t57 * t57;
            let t60 = f64x8::splat(1.0) / t58 / t31;
            let t61 = t56 * t60;
            let t65 = f64x8::splat(1.0) / t58 / v_rho0;
            let t67 = v_sigma0 * t60;
            let t69 = v_tau0 * t65 - t67 / f64x8::splat(8.0);
            let t70 = t69 * t50;
            let t73 = f64x8::splat(5.0) / f64x8::splat(9.0) * t70 * t55 - f64x8::splat(1.0);
            let t74 = t55 * t73;
            let t77 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t70 * t74;
            let t78 = ((t77).sqrt());
            let t79 = f64x8::splat(1.0) / t78;
            let t82 = t50 * t55;
            let t83 = t82 * t67;
            let t85 = f64x8::splat(9.0) / f64x8::splat(20.0) * t73 * t79 + t83 / f64x8::splat(36.0);
            let t86 = t85 * t85;
            let t89 = t50 * t50;
            let t91 = f64x8::splat(1.0) / t53 / t52;
            let t92 = t89 * t91;
            let t93 = t31 * t31;
            let t94 = t93 * v_rho0;
            let t96 = f64x8::splat(1.0) / t57 / t94;
            let t98 = t92 * t29 * t96;
            let t100 = f64x8::splat(162.0) * t41 + f64x8::splat(50.0) * t98;
            let t101 = ((t100).sqrt());
            let t106 = t93 * t93;
            let t107 = f64x8::splat(1.0) / t106;
            let t110 = t51 * t61 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t86 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t85 * t101 + f64x8::splat(2.6505934954444615e-05) * t98 + f64x8::splat(0.0019577914932045744) * t41 + f64x8::splat(1.0930269815274441e-06) * t30 * t107;
            let t112 = f64x8::splat(1.0) + f64x8::splat(0.05873374479613724) * t83;
            let t113 = t112 * t112;
            let t114 = f64x8::splat(1.0) / t113;
            let t115 = t110 * t114;
            let t117 = (simd::exp(-f64x8::splat(0.0001863) * t115));
            let t118 = f64x8::splat(1.0) + t115;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = t117 * t119;
            let t122 = t110 * t110;
            let t123 = t113 * t113;
            let t124 = f64x8::splat(1.0) / t123;
            let t127 = (simd::exp(-f64x8::splat(0.00150903) * t122 * t124));
            let t128 = f64x8::splat(1.0) - t127;
            let t129 = f64x8::splat(1.0) / t110;
            let t132 = f64x8::splat(10.0) / f64x8::splat(81.0) * t129 * t113 - f64x8::splat(1.0);
            let t134 = t115 * t120 + t128 * t132 + f64x8::splat(1.0);
            let t138 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t134));
            let t139 = (v_rho1).simd_le(dens_threshold);
            let t140 = -t17;
            let t142 = ((t15).select(t12, (t11).select(t16, t140 * t8)));
            let t143 = f64x8::splat(1.0) + t142;
            let t144 = (t143).simd_le(zeta_threshold);
            let t145 = (simd::cbrt(t143));
            let t147 = ((t144).select(t23, t145 * t143));
            let t148 = t147 * t27;
            let t149 = v_sigma2 * v_sigma2;
            let t150 = t149 * v_sigma2;
            let t151 = v_rho1 * v_rho1;
            let t152 = t151 * v_rho1;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t150 * t153;
            let t155 = v_tau1 * v_tau1;
            let t156 = t155 * v_tau1;
            let t157 = f64x8::splat(1.0) / t156;
            let t158 = f64x8::splat(1.0) / t151;
            let t159 = t149 * t158;
            let t160 = f64x8::splat(1.0) / t155;
            let t161 = t159 * t160;
            let t163 = f64x8::splat(1.0) + t161 / f64x8::splat(64.0);
            let t164 = t163 * t163;
            let t165 = f64x8::splat(1.0) / t164;
            let t166 = t157 * t165;
            let t170 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.00419826171875) * t154 * t166) * t50;
            let t171 = t55 * v_sigma2;
            let t172 = (simd::cbrt(v_rho1));
            let t173 = t172 * t172;
            let t175 = f64x8::splat(1.0) / t173 / t151;
            let t176 = t171 * t175;
            let t180 = f64x8::splat(1.0) / t173 / v_rho1;
            let t182 = v_sigma2 * t175;
            let t184 = v_tau1 * t180 - t182 / f64x8::splat(8.0);
            let t185 = t184 * t50;
            let t188 = f64x8::splat(5.0) / f64x8::splat(9.0) * t185 * t55 - f64x8::splat(1.0);
            let t189 = t55 * t188;
            let t192 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t185 * t189;
            let t193 = ((t192).sqrt());
            let t194 = f64x8::splat(1.0) / t193;
            let t197 = t82 * t182;
            let t199 = f64x8::splat(9.0) / f64x8::splat(20.0) * t188 * t194 + t197 / f64x8::splat(36.0);
            let t200 = t199 * t199;
            let t203 = t151 * t151;
            let t204 = t203 * v_rho1;
            let t206 = f64x8::splat(1.0) / t172 / t204;
            let t208 = t92 * t149 * t206;
            let t210 = f64x8::splat(162.0) * t161 + f64x8::splat(50.0) * t208;
            let t211 = ((t210).sqrt());
            let t216 = t203 * t203;
            let t217 = f64x8::splat(1.0) / t216;
            let t220 = t170 * t176 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t200 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t199 * t211 + f64x8::splat(2.6505934954444615e-05) * t208 + f64x8::splat(0.0019577914932045744) * t161 + f64x8::splat(1.0930269815274441e-06) * t150 * t217;
            let t222 = f64x8::splat(1.0) + f64x8::splat(0.05873374479613724) * t197;
            let t223 = t222 * t222;
            let t224 = f64x8::splat(1.0) / t223;
            let t225 = t220 * t224;
            let t227 = (simd::exp(-f64x8::splat(0.0001863) * t225));
            let t228 = f64x8::splat(1.0) + t225;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t227 * t229;
            let t232 = t220 * t220;
            let t233 = t223 * t223;
            let t234 = f64x8::splat(1.0) / t233;
            let t237 = (simd::exp(-f64x8::splat(0.00150903) * t232 * t234));
            let t238 = f64x8::splat(1.0) - t237;
            let t239 = f64x8::splat(1.0) / t220;
            let t242 = f64x8::splat(10.0) / f64x8::splat(81.0) * t239 * t223 - f64x8::splat(1.0);
            let t244 = t225 * t230 + t238 * t242 + f64x8::splat(1.0);
            let t248 = ((t139).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t148 * t244));
            let tzk0 = t138 + t248;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
