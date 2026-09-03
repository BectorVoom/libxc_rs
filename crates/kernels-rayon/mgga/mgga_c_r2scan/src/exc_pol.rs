//! MGGA_C_R2SCAN exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_r2scan.c`
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
pub fn mgga_c_r2scan_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_eta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_eta = f64x8::splat(param_eta);
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
            let t11 = t7 / t9;
            let t12 = t5 * t11;
            let t14 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t12;
            let t15 = ((t12).sqrt());
            let t17 = f64x8::splat(0.8969) * t12;
            let t18 = ((t12) * (t12).sqrt());
            let t19 = f64x8::splat(0.204775) * t18;
            let t20 = t2 * t2;
            let t21 = t4 * t4;
            let t22 = t20 * t21;
            let t23 = t9 * t9;
            let t26 = t22 * t6 / t23;
            let t27 = f64x8::splat(0.123235) * t26;
            let t28 = f64x8::splat(3.79785) * t15 + t17 + t19 + t27;
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
            let t65 = f64x8::splat(1.549425) * t12;
            let t66 = f64x8::splat(0.420775) * t18;
            let t67 = f64x8::splat(0.1562925) * t26;
            let t68 = f64x8::splat(7.05945) * t15 + t65 + t66 + t67;
            let t71 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t68;
            let t72 = (simd::ln(t71));
            let t76 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t12;
            let t78 = f64x8::splat(0.905775) * t12;
            let t79 = f64x8::splat(0.1100325) * t18;
            let t80 = f64x8::splat(0.1241775) * t26;
            let t81 = f64x8::splat(5.1785) * t15 + t78 + t79 + t80;
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
            let t108 = f64x8::splat(1.0) / t94;
            let t109 = (-t34 + t90 + t92) * t108;
            let t110 = f64x8::splat(1.0) / t106;
            let t111 = t95 * t110;
            let t113 = (simd::exp(-t109 * t111));
            let t114 = t113 - f64x8::splat(1.0);
            let t116 = f64x8::splat(1.0) + f64x8::splat(0.025) * t12;
            let t118 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t12;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = t116 * t119;
            let t122 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t124 = f64x8::splat(1.0) / t9 / t38;
            let t125 = t122 * t124;
            let t128 = f64x8::splat(1.0) / t105;
            let t129 = t128 * t20;
            let t130 = f64x8::splat(1.0) / t4;
            let t132 = t6 * t108;
            let t133 = f64x8::splat(1.0) / t114;
            let t134 = t132 * t133;
            let t135 = t129 * t130 * t134;
            let t138 = t98 * zeta_threshold;
            let t139 = t99 * t44;
            let t140 = ((t45).select(t138, t139));
            let t141 = t101 * t51;
            let t142 = ((t52).select(t138, t141));
            let t144 = t140 / f64x8::splat(2.0) + t142 / f64x8::splat(2.0);
            let t146 = t108 / t144;
            let t147 = t110 * t133;
            let t148 = ((f64x8::splat(4.0)).sqrt());
            let t149 = t148 * t15;
            let t151 = f64x8::splat(0.03138525) * t12;
            let t152 = f64x8::splat(1.0) + f64x8::splat(0.022225) * t149 + t151;
            let t153 = t152 * t152;
            let t154 = f64x8::splat(1.0) / t153;
            let t158 = f64x8::splat(1.0) - f64x8::splat(2.363) * t58 * t56 * t60;
            let t159 = t154 * t158;
            let t160 = t37 * t37;
            let t161 = t160 * t37;
            let t162 = t39 * t39;
            let t163 = t162 * t39;
            let t164 = f64x8::splat(1.0) / t163;
            let t166 = -t161 * t164 + f64x8::splat(1.0);
            let t167 = f64x8::splat(1.0) / t15;
            let t168 = t148 * t167;
            let t170 = f64x8::splat(0.04445) * t168 + f64x8::splat(0.125541);
            let t171 = t166 * t170;
            let t175 = f64x8::splat(1.898925) * t149 + t17 + t19 + t27;
            let t178 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t175;
            let t179 = (simd::ln(t178));
            let t180 = f64x8::splat(0.01328816518) * t179;
            let t181 = t175 * t175;
            let t182 = f64x8::splat(1.0) / t181;
            let t183 = t14 * t182;
            let t185 = ((t12).sqrt());
            let t188 = f64x8::splat(3.79785) * t168 + f64x8::splat(3.5876) + f64x8::splat(1.22865) * t185 + f64x8::splat(0.24647) * t12;
            let t189 = f64x8::splat(1.0) / t178;
            let t190 = t188 * t189;
            let t192 = f64x8::splat(1.0) * t183 * t190;
            let t194 = f64x8::splat(3.529725) * t149 + t65 + t66 + t67;
            let t197 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t194;
            let t198 = (simd::ln(t197));
            let t200 = t194 * t194;
            let t201 = f64x8::splat(1.0) / t200;
            let t202 = t63 * t201;
            let t206 = f64x8::splat(7.05945) * t168 + f64x8::splat(6.1977) + f64x8::splat(2.52465) * t185 + f64x8::splat(0.312585) * t12;
            let t207 = f64x8::splat(1.0) / t197;
            let t208 = t206 * t207;
            let t212 = f64x8::splat(2.58925) * t149 + t78 + t79 + t80;
            let t215 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t212;
            let t216 = (simd::ln(t215));
            let t218 = t212 * t212;
            let t219 = f64x8::splat(1.0) / t218;
            let t220 = t76 * t219;
            let t224 = f64x8::splat(5.1785) * t168 + f64x8::splat(3.6231) + f64x8::splat(0.660195) * t185 + f64x8::splat(0.248355) * t12;
            let t225 = f64x8::splat(1.0) / t215;
            let t226 = t224 * t225;
            let t229 = -f64x8::splat(0.006388517036) * t198 + f64x8::splat(1.0) * t202 * t208 + t180 - t192 - f64x8::splat(0.0021973736767207856) * t216 + f64x8::splat(0.5848223622634646) * t220 * t226;
            let t230 = t61 * t229;
            let t234 = t61 * t76;
            let t236 = t219 * t224 * t225;
            let t239 = f64x8::splat(0.0285764) * t159 * t171 + t180 - t192 - t41 * t230 - f64x8::splat(0.0021973736767207856) * t61 * t216 + f64x8::splat(0.5848223622634646) * t234 * t236;
            let t244 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t15 + t151;
            let t245 = f64x8::splat(1.0) / t244;
            let t246 = t245 * t158;
            let t252 = f64x8::splat(5.0) * t5 * t11 * t239 - f64x8::splat(45.0) * param_eta * (-f64x8::splat(0.0285764) * t246 * t166 + t34 - t90 - t92);
            let t253 = t147 * t252;
            let t254 = t146 * t253;
            let t255 = f64x8::splat(M_CBRT6);
            let t256 = (simd::cbrt(t95));
            let t257 = t256 * t256;
            let t258 = f64x8::splat(1.0) / t257;
            let t259 = t255 * t258;
            let t260 = t57 * t57;
            let t261 = t259 * t260;
            let t263 = f64x8::splat(1.0) / t23 / t38;
            let t264 = t122 * t263;
            let t265 = t255 * t255;
            let t267 = f64x8::splat(1.0) / t256 / t95;
            let t268 = t265 * t267;
            let t269 = t122 * t122;
            let t270 = t57 * t269;
            let t271 = t39 * t8;
            let t273 = f64x8::splat(1.0) / t9 / t271;
            let t277 = (simd::exp(-f64x8::splat(0.2044460407889637) * t268 * t270 * t273));
            let t279 = t261 * t264 * t277;
            let t282 = f64x8::splat(1.0) + f64x8::splat(0.027439371595564633) * t120 * t125 * t57 * t135 + f64x8::splat(0.043341108700271344) * t254 * t279;
            let t283 = ((t282).sqrt().sqrt());
            let t285 = f64x8::splat(1.0) - f64x8::splat(1.0) / t283;
            let t287 = t114 * t285 + f64x8::splat(1.0);
            let t288 = (simd::ln(t287));
            let t290 = t97 * t106 * t288;
            let t291 = (simd::cbrt(v_rho0));
            let t292 = t291 * t291;
            let t294 = f64x8::splat(1.0) / t292 / v_rho0;
            let t295 = v_tau0 * t294;
            let t296 = t44 / f64x8::splat(2.0);
            let t297 = (simd::cbrt(t296));
            let t298 = t297 * t297;
            let t299 = t298 * t296;
            let t301 = (simd::cbrt(v_rho1));
            let t302 = t301 * t301;
            let t304 = f64x8::splat(1.0) / t302 / v_rho1;
            let t305 = v_tau1 * t304;
            let t306 = t51 / f64x8::splat(2.0);
            let t307 = (simd::cbrt(t306));
            let t308 = t307 * t307;
            let t309 = t308 * t306;
            let t312 = t295 * t299 + t305 * t309 - t264 / f64x8::splat(8.0);
            let t313 = t265 * t257;
            let t317 = param_eta * t122;
            let t320 = f64x8::splat(3.0) / f64x8::splat(10.0) * t313 * (t299 + t309) + t317 * t263 / f64x8::splat(8.0);
            let t321 = f64x8::splat(1.0) / t320;
            let t322 = t312 * t321;
            let t323 = (t322).simd_le(f64x8::splat(0.0));
            let t324 = (f64x8::splat(0.0)).simd_lt(t322);
            let t325 = ((t324).select(f64x8::splat(0.0), t322));
            let t326 = f64x8::splat(1.0) - t325;
            let t327 = f64x8::splat(1.0) / t326;
            let t330 = (simd::exp(-f64x8::splat(0.64) * t325 * t327));
            let t331 = (t322).simd_le(f64x8::splat(2.5));
            let t332 = (f64x8::splat(2.5)).simd_lt(t322);
            let t333 = ((t332).select(f64x8::splat(2.5), t322));
            let t335 = t333 * t333;
            let t337 = t335 * t333;
            let t339 = t335 * t335;
            let t341 = t339 * t333;
            let t343 = t339 * t335;
            let t348 = ((t332).select(t322, f64x8::splat(2.5)));
            let t349 = f64x8::splat(1.0) - t348;
            let t352 = (simd::exp(f64x8::splat(1.5) / t349));
            let t354 = ((t323).select(t330, (t331).select(f64x8::splat(1.0) - f64x8::splat(0.64) * t333 - f64x8::splat(0.4352) * t335 - f64x8::splat(1.535685604549) * t337 + f64x8::splat(3.061560252175) * t339 - f64x8::splat(1.915710236206) * t341 + f64x8::splat(0.516884468372) * t343 - f64x8::splat(0.051848879792) * t339 * t337, -f64x8::splat(0.7) * t352)));
            let t357 = (simd::exp(f64x8::splat(1.0) * t245));
            let t358 = t357 - f64x8::splat(1.0);
            let t359 = t260 * t122;
            let t360 = t359 * t263;
            let t363 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t259 * t360;
            let t364 = ((t363).sqrt().sqrt());
            let t366 = f64x8::splat(1.0) - f64x8::splat(1.0) / t364;
            let t368 = t358 * t366 + f64x8::splat(1.0);
            let t369 = (simd::ln(t368));
            let t371 = -f64x8::splat(0.0285764) * t245 + f64x8::splat(0.0285764) * t369;
            let t372 = t371 * t158;
            let t374 = t372 * t166 - t290 + t34 - t90 - t92;
            let t375 = t354 * t374;
            let tzk0 = -t34 + t90 + t92 + t290 + t375;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
