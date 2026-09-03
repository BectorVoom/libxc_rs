//! MGGA_C_SCAN exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_scan.c`
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
pub fn mgga_c_scan_exc_pol(
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
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = v_rho0 + v_rho1;
            let t9 = (simd::cbrt(t8));
            let t12 = t5 * t7 / t9;
            let t14 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t12;
            let t15 = ((t12).sqrt());
            let t18 = ((t12) * (t12).sqrt());
            let t20 = t2 * t2;
            let t21 = t4 * t4;
            let t22 = t20 * t21;
            let t23 = t9 * t9;
            let t26 = t22 * t6 / t23;
            let t28 = f64x8::splat(3.79785) * t15 + f64x8::splat(0.8969) * t12 + f64x8::splat(0.204775) * t18 + f64x8::splat(0.123235) * t26;
            let t31 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t28;
            let t32 = (simd::ln(t31));
            let t34 = f64x8::splat(0.0621814) * t14 * t32;
            let t35 = v_rho0 - v_rho1;
            let t36 = t35 * t35;
            let t37 = t36 * t36;
            let t38 = t8 * t8;
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t37 * t40;
            let t42 = f64x8::splat(1.0) / t8;
            let t43 = t35 * t42;
            let t44 = f64x8::splat(1.0) + t43;
            let t45 = (t44).simd_le(zeta_threshold);
            let t46 = (simd::cbrt(zeta_threshold));
            let t47 = t46 * zeta_threshold;
            let t48 = (simd::cbrt(t44));
            let t49 = t48 * t44;
            let t50 = ((t45).select(t47, t49));
            let t51 = f64x8::splat(1.0) - t43;
            let t52 = (t51).simd_le(zeta_threshold);
            let t53 = (simd::cbrt(t51));
            let t54 = t53 * t51;
            let t55 = ((t52).select(t47, t54));
            let t56 = t50 + t55 - f64x8::splat(2.0);
            let t57 = f64x8::splat(M_CBRT2);
            let t58 = t57 - f64x8::splat(1.0);
            let t60 = f64x8::splat(1.0) / t58 / f64x8::splat(2.0);
            let t61 = t56 * t60;
            let t63 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t12;
            let t68 = f64x8::splat(7.05945) * t15 + f64x8::splat(1.549425) * t12 + f64x8::splat(0.420775) * t18 + f64x8::splat(0.1562925) * t26;
            let t71 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t68;
            let t72 = (simd::ln(t71));
            let t76 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t12;
            let t81 = f64x8::splat(5.1785) * t15 + f64x8::splat(0.905775) * t12 + f64x8::splat(0.1100325) * t18 + f64x8::splat(0.1241775) * t26;
            let t84 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t81;
            let t85 = (simd::ln(t84));
            let t86 = t76 * t85;
            let t88 = -f64x8::splat(0.0310907) * t63 * t72 + t34 - f64x8::splat(0.0197516734986138) * t86;
            let t89 = t61 * t88;
            let t90 = t41 * t89;
            let t92 = f64x8::splat(0.0197516734986138) * t61 * t86;
            let t93 = (simd::ln(f64x8::splat(2.0)));
            let t94 = f64x8::splat(1.0) - t93;
            let t95 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t97 = t94 / t95;
            let t98 = t46 * t46;
            let t99 = t48 * t48;
            let t100 = ((t45).select(t98, t99));
            let t101 = t53 * t53;
            let t102 = ((t52).select(t98, t101));
            let t104 = t100 / f64x8::splat(2.0) + t102 / f64x8::splat(2.0);
            let t105 = t104 * t104;
            let t106 = t105 * t104;
            let t108 = f64x8::splat(1.0) + f64x8::splat(0.025) * t12;
            let t110 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t12;
            let t111 = f64x8::splat(1.0) / t110;
            let t112 = t108 * t111;
            let t113 = f64x8::splat(1.0) / t94;
            let t115 = (-t34 + t90 + t92) * t113;
            let t116 = f64x8::splat(1.0) / t106;
            let t117 = t95 * t116;
            let t119 = (simd::exp(-t115 * t117));
            let t120 = t119 - f64x8::splat(1.0);
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t113 * t121;
            let t124 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t125 = t122 * t124;
            let t126 = t112 * t125;
            let t128 = f64x8::splat(1.0) / t9 / t38;
            let t129 = t128 * t57;
            let t130 = f64x8::splat(1.0) / t105;
            let t132 = f64x8::splat(1.0) / t4;
            let t133 = t20 * t132;
            let t134 = t133 * t6;
            let t138 = f64x8::splat(1.0) + f64x8::splat(0.027439371595564633) * t126 * t129 * t130 * t134;
            let t139 = ((t138).sqrt().sqrt());
            let t141 = f64x8::splat(1.0) - f64x8::splat(1.0) / t139;
            let t144 = f64x8::splat(1.0) + f64x8::splat(1.0) * t141 * t120;
            let t145 = (simd::ln(t144));
            let t147 = t97 * t106 * t145;
            let t148 = (simd::cbrt(v_rho0));
            let t149 = t148 * t148;
            let t151 = f64x8::splat(1.0) / t149 / v_rho0;
            let t152 = v_tau0 * t151;
            let t153 = t44 / f64x8::splat(2.0);
            let t154 = (simd::cbrt(t153));
            let t155 = t154 * t154;
            let t156 = t155 * t153;
            let t158 = (simd::cbrt(v_rho1));
            let t159 = t158 * t158;
            let t161 = f64x8::splat(1.0) / t159 / v_rho1;
            let t162 = v_tau1 * t161;
            let t163 = t51 / f64x8::splat(2.0);
            let t164 = (simd::cbrt(t163));
            let t165 = t164 * t164;
            let t166 = t165 * t163;
            let t169 = f64x8::splat(1.0) / t23 / t38;
            let t173 = f64x8::splat(M_CBRT6);
            let t174 = (t152 * t156 + t162 * t166 - t124 * t169 / f64x8::splat(8.0)) * t173;
            let t175 = (simd::cbrt(t95));
            let t176 = t175 * t175;
            let t177 = f64x8::splat(1.0) / t176;
            let t178 = t156 + t166;
            let t179 = f64x8::splat(1.0) / t178;
            let t180 = t177 * t179;
            let t182 = f64x8::splat(5.0) / f64x8::splat(9.0) * t174 * t180;
            let t183 = (t182).simd_le(f64x8::splat(1.0));
            let t184 = (simd::ln(f64x8::splat(f64::EPSILON)));
            let t187 = t184 / (-t184 + f64x8::splat(0.64));
            let t188 = (-t187).simd_lt(t182);
            let t189 = (t182).simd_lt(-t187);
            let t190 = ((t189).select(t182, -t187));
            let t191 = f64x8::splat(1.0) - t190;
            let t192 = f64x8::splat(1.0) / t191;
            let t195 = (simd::exp(-f64x8::splat(0.64) * t190 * t192));
            let t196 = ((t188).select(f64x8::splat(0.0), t195));
            let t198 = (simd::ln(f64x8::splat(1.4285714285714286) * f64x8::splat(f64::EPSILON)));
            let t201 = (-t198 + f64x8::splat(1.5)) / t198;
            let t202 = (t182).simd_lt(-t201);
            let t203 = ((t202).select(-t201, t182));
            let t204 = f64x8::splat(1.0) - t203;
            let t207 = (simd::exp(f64x8::splat(1.5) / t204));
            let t209 = ((t202).select(f64x8::splat(0.0), -f64x8::splat(0.7) * t207));
            let t210 = ((t183).select(t196, t209));
            let t213 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t15 + f64x8::splat(0.03138525) * t12;
            let t214 = f64x8::splat(1.0) / t213;
            let t217 = (simd::exp(f64x8::splat(1.0) * t214));
            let t218 = t217 - f64x8::splat(1.0);
            let t219 = t173 * t177;
            let t220 = t57 * t57;
            let t221 = t220 * t124;
            let t225 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t219 * t221 * t169;
            let t226 = ((t225).sqrt().sqrt());
            let t228 = f64x8::splat(1.0) - f64x8::splat(1.0) / t226;
            let t230 = t218 * t228 + f64x8::splat(1.0);
            let t231 = (simd::ln(t230));
            let t233 = -f64x8::splat(0.0285764) * t214 + f64x8::splat(0.0285764) * t231;
            let t237 = f64x8::splat(1.0) - f64x8::splat(2.363) * t58 * t56 * t60;
            let t238 = t233 * t237;
            let t239 = t37 * t37;
            let t240 = t239 * t37;
            let t241 = t39 * t39;
            let t242 = t241 * t39;
            let t243 = f64x8::splat(1.0) / t242;
            let t245 = -t240 * t243 + f64x8::splat(1.0);
            let t247 = t238 * t245 - t147 + t34 - t90 - t92;
            let t248 = t210 * t247;
            let tzk0 = -t34 + t90 + t92 + t147 + t248;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
