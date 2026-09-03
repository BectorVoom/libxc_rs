//! MGGA_X_RSCAN exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rscan.c`
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
pub fn mgga_x_rscan_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_alphar: f64,
    param_c2: f64,
    param_d: f64,
    param_k1: f64,
    param_taur: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alphar = f64x8::splat(param_alphar);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_k1 = f64x8::splat(param_k1);
    let param_taur = f64x8::splat(param_taur);
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
            let t27 = t6 * t26;
            let t28 = (simd::cbrt(t7));
            let t29 = f64x8::splat(M_CBRT6);
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t29 * t33;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t38 = t37 * t35;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = v_sigma0 * t39;
            let t41 = t34 * t40;
            let t45 = f64x8::splat(100.0) / f64x8::splat(6561.0) / param_k1 - f64x8::splat(73.0) / f64x8::splat(648.0);
            let t46 = t29 * t29;
            let t47 = t45 * t46;
            let t48 = t31 * t30;
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = t47 * t49;
            let t51 = v_sigma0 * v_sigma0;
            let t52 = t35 * t35;
            let t53 = t52 * v_rho0;
            let t55 = f64x8::splat(1.0) / t36 / t53;
            let t56 = t51 * t55;
            let t57 = t45 * t29;
            let t58 = t33 * v_sigma0;
            let t59 = t58 * t39;
            let t62 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t57 * t59));
            let t66 = ((f64x8::splat(146.0)).sqrt());
            let t67 = t66 * t29;
            let t70 = t20 * t20;
            let t71 = t70 * t70;
            let t72 = t71 * t20;
            let t73 = t7 * t7;
            let t74 = t73 * t73;
            let t75 = t74 * t7;
            let t76 = t72 * t75;
            let t77 = t37 * v_rho0;
            let t78 = f64x8::splat(1.0) / t77;
            let t81 = v_tau0 * t78 - t40 / f64x8::splat(8.0);
            let t82 = (f64x8::splat(0.0)).simd_lt(t81);
            let t83 = ((t82).select(t81, f64x8::splat(0.0)));
            let t84 = t83 * t83;
            let t85 = t84 * t83;
            let t86 = f64x8::splat(M_CBRT2);
            let t87 = t20 * t7;
            let t88 = (simd::cbrt(t87));
            let t89 = t88 * t88;
            let t92 = t46 * t32;
            let t95 = param_taur / f64x8::splat(2.0);
            let t96 = f64x8::splat(3.0) / f64x8::splat(40.0) * t86 * t89 * t87 * t92 + t95;
            let t97 = t96 * t96;
            let t98 = t97 * t96;
            let t99 = f64x8::splat(1.0) / t98;
            let t100 = t85 * t99;
            let t101 = t86 * t86;
            let t102 = t70 * t20;
            let t103 = t73 * t7;
            let t104 = t102 * t103;
            let t105 = t88 * t104;
            let t106 = t101 * t105;
            let t107 = f64x8::splat(1.0) / t97;
            let t108 = t84 * t107;
            let t111 = t106 * t108 / f64x8::splat(16.0) + param_alphar;
            let t112 = f64x8::splat(1.0) / t111;
            let t113 = t100 * t112;
            let t115 = t76 * t113 / f64x8::splat(32.0);
            let t116 = f64x8::splat(1.0) - t115;
            let t118 = t116 * t116;
            let t120 = (simd::exp(-t118 / f64x8::splat(2.0)));
            let t123 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t59 + t66 * t116 * t120 / f64x8::splat(100.0);
            let t124 = t123 * t123;
            let t125 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t41 + t50 * t56 * t62 / f64x8::splat(576.0) + t124;
            let t130 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t125);
            let t131 = (t115).simd_le(f64x8::splat(2.5));
            let t132 = (f64x8::splat(2.5)).simd_lt(t115);
            let t133 = ((t132).select(f64x8::splat(2.5), t115));
            let t135 = t133 * t133;
            let t137 = t135 * t133;
            let t139 = t135 * t135;
            let t141 = t139 * t133;
            let t143 = t139 * t135;
            let t148 = ((t132).select(t115, f64x8::splat(2.5)));
            let t149 = f64x8::splat(1.0) - t148;
            let t152 = (simd::exp(param_c2 / t149));
            let t154 = ((t131).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t133 - f64x8::splat(0.4445555) * t135 - f64x8::splat(0.663086601049) * t137 + f64x8::splat(1.45129704449) * t139 - f64x8::splat(0.887998041597) * t141 + f64x8::splat(0.234528941479) * t143 - f64x8::splat(0.023185843322) * t139 * t137, -param_d * t152));
            let t155 = f64x8::splat(1.0) - t154;
            let t158 = t130 * t155 + f64x8::splat(1.174) * t154;
            let t159 = t28 * t158;
            let t160 = ((f64x8::splat(3.0)).sqrt());
            let t161 = f64x8::splat(1.0) / t31;
            let t162 = t46 * t161;
            let t163 = ((v_sigma0).sqrt());
            let t164 = t36 * v_rho0;
            let t165 = f64x8::splat(1.0) / t164;
            let t167 = t162 * t163 * t165;
            let t168 = ((t167).sqrt());
            let t172 = (simd::exp(-f64x8::splat(9.8958) * t160 / t168));
            let t173 = f64x8::splat(1.0) - t172;
            let t174 = t159 * t173;
            let t177 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t174));
            let t178 = (v_rho1).simd_le(dens_threshold);
            let t179 = -t17;
            let t181 = ((t15).select(t12, (t11).select(t16, t179 * t8)));
            let t182 = f64x8::splat(1.0) + t181;
            let t183 = (t182).simd_le(zeta_threshold);
            let t184 = (simd::cbrt(t182));
            let t186 = ((t183).select(t23, t184 * t182));
            let t187 = t6 * t186;
            let t188 = v_rho1 * v_rho1;
            let t189 = (simd::cbrt(v_rho1));
            let t190 = t189 * t189;
            let t191 = t190 * t188;
            let t192 = f64x8::splat(1.0) / t191;
            let t193 = v_sigma2 * t192;
            let t194 = t34 * t193;
            let t196 = v_sigma2 * v_sigma2;
            let t197 = t188 * t188;
            let t198 = t197 * v_rho1;
            let t200 = f64x8::splat(1.0) / t189 / t198;
            let t201 = t196 * t200;
            let t202 = t33 * v_sigma2;
            let t203 = t202 * t192;
            let t206 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t57 * t203));
            let t212 = t182 * t182;
            let t213 = t212 * t212;
            let t214 = t213 * t182;
            let t215 = t214 * t75;
            let t216 = t190 * v_rho1;
            let t217 = f64x8::splat(1.0) / t216;
            let t220 = v_tau1 * t217 - t193 / f64x8::splat(8.0);
            let t221 = (f64x8::splat(0.0)).simd_lt(t220);
            let t222 = ((t221).select(t220, f64x8::splat(0.0)));
            let t223 = t222 * t222;
            let t224 = t223 * t222;
            let t225 = t182 * t7;
            let t226 = (simd::cbrt(t225));
            let t227 = t226 * t226;
            let t232 = f64x8::splat(3.0) / f64x8::splat(40.0) * t86 * t227 * t225 * t92 + t95;
            let t233 = t232 * t232;
            let t234 = t233 * t232;
            let t235 = f64x8::splat(1.0) / t234;
            let t236 = t224 * t235;
            let t237 = t212 * t182;
            let t238 = t237 * t103;
            let t239 = t226 * t238;
            let t240 = t101 * t239;
            let t241 = f64x8::splat(1.0) / t233;
            let t242 = t223 * t241;
            let t245 = t240 * t242 / f64x8::splat(16.0) + param_alphar;
            let t246 = f64x8::splat(1.0) / t245;
            let t247 = t236 * t246;
            let t249 = t215 * t247 / f64x8::splat(32.0);
            let t250 = f64x8::splat(1.0) - t249;
            let t252 = t250 * t250;
            let t254 = (simd::exp(-t252 / f64x8::splat(2.0)));
            let t257 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t67 * t203 + t66 * t250 * t254 / f64x8::splat(100.0);
            let t258 = t257 * t257;
            let t259 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t194 + t50 * t201 * t206 / f64x8::splat(576.0) + t258;
            let t264 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t259);
            let t265 = (t249).simd_le(f64x8::splat(2.5));
            let t266 = (f64x8::splat(2.5)).simd_lt(t249);
            let t267 = ((t266).select(f64x8::splat(2.5), t249));
            let t269 = t267 * t267;
            let t271 = t269 * t267;
            let t273 = t269 * t269;
            let t275 = t273 * t267;
            let t277 = t273 * t269;
            let t282 = ((t266).select(t249, f64x8::splat(2.5)));
            let t283 = f64x8::splat(1.0) - t282;
            let t286 = (simd::exp(param_c2 / t283));
            let t288 = ((t265).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t267 - f64x8::splat(0.4445555) * t269 - f64x8::splat(0.663086601049) * t271 + f64x8::splat(1.45129704449) * t273 - f64x8::splat(0.887998041597) * t275 + f64x8::splat(0.234528941479) * t277 - f64x8::splat(0.023185843322) * t273 * t271, -param_d * t286));
            let t289 = f64x8::splat(1.0) - t288;
            let t292 = t264 * t289 + f64x8::splat(1.174) * t288;
            let t293 = t28 * t292;
            let t294 = ((v_sigma2).sqrt());
            let t295 = t189 * v_rho1;
            let t296 = f64x8::splat(1.0) / t295;
            let t298 = t162 * t294 * t296;
            let t299 = ((t298).sqrt());
            let t303 = (simd::exp(-f64x8::splat(9.8958) * t160 / t299));
            let t304 = f64x8::splat(1.0) - t303;
            let t305 = t293 * t304;
            let t308 = ((t178).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t187 * t305));
            let tzk0 = t177 + t308;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
