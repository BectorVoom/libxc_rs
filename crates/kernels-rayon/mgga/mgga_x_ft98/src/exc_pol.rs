//! MGGA_X_FT98 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_ft98.c`
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
pub fn mgga_x_ft98_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_a: f64,
    param_a1: f64,
    param_a2: f64,
    param_b: f64,
    param_b1: f64,
    param_b2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_a1 = f64x8::splat(param_a1);
    let param_a2 = f64x8::splat(param_a2);
    let param_b = f64x8::splat(param_b);
    let param_b1 = f64x8::splat(param_b1);
    let param_b2 = f64x8::splat(param_b2);
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
            let t5 = f64x8::splat(1.0) / t4;
            let t6 = t3 * t5;
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
            let t30 = v_rho0 * v_rho0;
            let t31 = (simd::cbrt(v_rho0));
            let t32 = t31 * t31;
            let t34 = f64x8::splat(1.0) / t32 / t30;
            let t36 = param_a1 * v_sigma0 * t34 + f64x8::splat(1.0);
            let t37 = ((t36).sqrt());
            let t38 = param_a * t37;
            let t39 = param_b1 * v_sigma0;
            let t41 = t39 * t34 + f64x8::splat(1.0);
            let t42 = ((t41).sqrt().sqrt());
            let t43 = t42 * t42;
            let t44 = t43 * t42;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t45 * v_sigma0;
            let t49 = v_sigma0 * t34;
            let t51 = f64x8::splat(1.0) / t32 / v_rho0;
            let t53 = -v_lapl0 * t51 + t49;
            let t54 = t53 * t53;
            let t55 = param_a2 * t54;
            let t56 = f64x8::splat(1.0) + t49;
            let t57 = t56 * t56;
            let t58 = f64x8::splat(1.0) / t57;
            let t61 = param_b * (t55 * t58 + f64x8::splat(1.0));
            let t62 = param_b2 * param_b2;
            let t64 = ((t62 + f64x8::splat(1.0)).sqrt());
            let t65 = t64 - param_b2;
            let t66 = v_sigma0 * v_sigma0;
            let t67 = t30 * t30;
            let t68 = t67 * v_rho0;
            let t70 = f64x8::splat(1.0) / t31 / t68;
            let t71 = t66 * t70;
            let t72 = v_lapl0 * v_lapl0;
            let t73 = t30 * v_rho0;
            let t75 = f64x8::splat(1.0) / t31 / t73;
            let t76 = t72 * t75;
            let t77 = t71 - t76 - param_b2;
            let t78 = ((f64x8::splat(f64::EPSILON)).sqrt().sqrt());
            let t79 = f64x8::splat(1.0) / t78;
            let t80 = (t77).simd_lt(-t79);
            let t83 = f64x8::splat(2.0) * param_b2;
            let t86 = t77 * t77;
            let t87 = t86 * t77;
            let t88 = f64x8::splat(1.0) / t87;
            let t90 = t86 * t86;
            let t91 = t90 * t77;
            let t92 = f64x8::splat(1.0) / t91;
            let t97 = (((f64x8::splat(0.0)).simd_lt(t77)).select(t77, -t77));
            let t98 = (t97).simd_lt(t78);
            let t101 = t90 * t86;
            let t103 = t90 * t90;
            let t106 = (-t79).simd_lt(t77);
            let t107 = ((t106).select(t77, -t79));
            let t108 = t107 * t107;
            let t109 = f64x8::splat(1.0) + t108;
            let t110 = ((t109).sqrt());
            let t111 = t107 + t110;
            let t113 = ((t80).select(-f64x8::splat(2.0) * t71 + f64x8::splat(2.0) * t76 + t83 - f64x8::splat(1.0) / t77 / f64x8::splat(2.0) + t88 / f64x8::splat(8.0) - t92 / f64x8::splat(16.0), (t98).select(f64x8::splat(1.0) - t71 + t76 + param_b2 + t86 / f64x8::splat(2.0) - t90 / f64x8::splat(8.0) + t101 / f64x8::splat(16.0) - f64x8::splat(5.0) / f64x8::splat(128.0) * t103, f64x8::splat(1.0) / t111)));
            let t115 = t65 * t113 + f64x8::splat(1.0);
            let t116 = f64x8::splat(M_CBRT2);
            let t117 = t116 - f64x8::splat(1.0);
            let t118 = t117 * t65;
            let t120 = t118 * t113 + f64x8::splat(1.0);
            let t121 = t120 * t120;
            let t122 = t121 * t120;
            let t123 = f64x8::splat(1.0) / t122;
            let t124 = t115 * t123;
            let t125 = t124 * t54;
            let t127 = t38 * t46 * t34 + t61 * t125 + f64x8::splat(1.0);
            let t128 = t3 * t3;
            let t129 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t130 = (simd::cbrt(t129));
            let t131 = t130 * t130;
            let t132 = t128 * t131;
            let t133 = f64x8::splat(M_CBRT4);
            let t134 = t132 * t133;
            let t135 = param_b * v_sigma0;
            let t139 = f64x8::splat(1.0) + f64x8::splat(81.0) / f64x8::splat(4.0) * t134 * t135 * t34;
            let t140 = f64x8::splat(1.0) / t139;
            let t141 = t127 * t140;
            let t142 = ((t141).sqrt());
            let t146 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t142));
            let t147 = (v_rho1).simd_le(dens_threshold);
            let t148 = -t17;
            let t150 = ((t15).select(t12, (t11).select(t16, t148 * t8)));
            let t151 = f64x8::splat(1.0) + t150;
            let t152 = (t151).simd_le(zeta_threshold);
            let t153 = (simd::cbrt(t151));
            let t155 = ((t152).select(t23, t153 * t151));
            let t156 = t155 * t27;
            let t157 = param_a1 * v_sigma2;
            let t158 = v_rho1 * v_rho1;
            let t159 = (simd::cbrt(v_rho1));
            let t160 = t159 * t159;
            let t162 = f64x8::splat(1.0) / t160 / t158;
            let t164 = t157 * t162 + f64x8::splat(1.0);
            let t165 = ((t164).sqrt());
            let t166 = param_a * t165;
            let t169 = param_b1 * v_sigma2 * t162 + f64x8::splat(1.0);
            let t170 = ((t169).sqrt().sqrt());
            let t171 = t170 * t170;
            let t172 = t171 * t170;
            let t173 = f64x8::splat(1.0) / t172;
            let t174 = t173 * v_sigma2;
            let t177 = v_sigma2 * t162;
            let t179 = f64x8::splat(1.0) / t160 / v_rho1;
            let t181 = -v_lapl1 * t179 + t177;
            let t182 = t181 * t181;
            let t183 = param_a2 * t182;
            let t184 = f64x8::splat(1.0) + t177;
            let t185 = t184 * t184;
            let t186 = f64x8::splat(1.0) / t185;
            let t189 = param_b * (t183 * t186 + f64x8::splat(1.0));
            let t190 = v_sigma2 * v_sigma2;
            let t191 = t158 * t158;
            let t192 = t191 * v_rho1;
            let t194 = f64x8::splat(1.0) / t159 / t192;
            let t195 = t190 * t194;
            let t196 = v_lapl1 * v_lapl1;
            let t197 = t158 * v_rho1;
            let t199 = f64x8::splat(1.0) / t159 / t197;
            let t200 = t196 * t199;
            let t201 = t195 - t200 - param_b2;
            let t202 = (t201).simd_lt(-t79);
            let t207 = t201 * t201;
            let t208 = t207 * t201;
            let t209 = f64x8::splat(1.0) / t208;
            let t211 = t207 * t207;
            let t212 = t211 * t201;
            let t213 = f64x8::splat(1.0) / t212;
            let t218 = (((f64x8::splat(0.0)).simd_lt(t201)).select(t201, -t201));
            let t219 = (t218).simd_lt(t78);
            let t222 = t211 * t207;
            let t224 = t211 * t211;
            let t227 = (-t79).simd_lt(t201);
            let t228 = ((t227).select(t201, -t79));
            let t229 = t228 * t228;
            let t230 = f64x8::splat(1.0) + t229;
            let t231 = ((t230).sqrt());
            let t232 = t228 + t231;
            let t234 = ((t202).select(-f64x8::splat(2.0) * t195 + f64x8::splat(2.0) * t200 + t83 - f64x8::splat(1.0) / t201 / f64x8::splat(2.0) + t209 / f64x8::splat(8.0) - t213 / f64x8::splat(16.0), (t219).select(f64x8::splat(1.0) - t195 + t200 + param_b2 + t207 / f64x8::splat(2.0) - t211 / f64x8::splat(8.0) + t222 / f64x8::splat(16.0) - f64x8::splat(5.0) / f64x8::splat(128.0) * t224, f64x8::splat(1.0) / t232)));
            let t236 = t65 * t234 + f64x8::splat(1.0);
            let t238 = t118 * t234 + f64x8::splat(1.0);
            let t239 = t238 * t238;
            let t240 = t239 * t238;
            let t241 = f64x8::splat(1.0) / t240;
            let t242 = t236 * t241;
            let t243 = t242 * t182;
            let t245 = t166 * t174 * t162 + t189 * t243 + f64x8::splat(1.0);
            let t246 = param_b * v_sigma2;
            let t250 = f64x8::splat(1.0) + f64x8::splat(81.0) / f64x8::splat(4.0) * t134 * t246 * t162;
            let t251 = f64x8::splat(1.0) / t250;
            let t252 = t245 * t251;
            let t253 = ((t252).sqrt());
            let t257 = ((t147).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t156 * t253));
            let tzk0 = t146 + t257;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
