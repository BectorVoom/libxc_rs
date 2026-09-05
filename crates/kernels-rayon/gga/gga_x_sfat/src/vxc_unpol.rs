//! GGA_X_SFAT vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sfat.c`
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

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_sfat_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = t17 / t4 * t3;
            let t19 = (simd::cbrt(v_rho));
            let t20 = t3 * t3;
            let t22 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t25 * t24;
            let t27 = t24 * t20;
            let t28 = t25 * t27;
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t29 * t29;
            let t31 = t30 * v_sigma;
            let t32 = v_rho * v_rho;
            let t33 = t19 * t19;
            let t35 = f64x8::splat(1.0) / t33 / t32;
            let t36 = ((v_sigma).sqrt());
            let t37 = t29 * t36;
            let t39 = f64x8::splat(1.0) / t19 / v_rho;
            let t41 = (simd::ln(t39 * t37 + ((((t39 * t37) * (t39 * t37)) + f64x8::splat(1.0)).sqrt())));
            let t42 = t41 * t39;
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t42 * t37;
            let t46 = f64x8::splat(1.0) / t45;
            let t51 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t46 * t35 * t31 * t28;
            let t54 = f64x8::splat(1.0) / t51 * t26 * t20 * f64x8::splat(M_PI);
            let t55 = ((t54).sqrt());
            let t57 = f64x8::splat(1.0) / t55 * param_hyb_omega_0;
            let t58 = v_rho * t11;
            let t59 = (simd::cbrt(t58));
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t60 * t29;
            let t63 = t61 * t57 / f64x8::splat(2.0);
            let t64 = (f64x8::splat(1.92)).simd_le(t63);
            let t65 = (f64x8::splat(1.92)).simd_lt(t63);
            let t66 = ((t65).select(t63, f64x8::splat(1.92)));
            let t67 = t66 * t66;
            let t68 = t67 * t67;
            let t69 = t68 * t68;
            let t70 = t69 * t69;
            let t71 = t70 * t70;
            let t73 = f64x8::splat(1.0) / t71 / t67;
            let t76 = f64x8::splat(1.0) / t71 / t68;
            let t78 = f64x8::splat(1.0) / t68;
            let t80 = t68 * t67;
            let t81 = f64x8::splat(1.0) / t80;
            let t83 = f64x8::splat(1.0) / t69;
            let t85 = t69 * t67;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = t69 * t68;
            let t89 = f64x8::splat(1.0) / t88;
            let t91 = t69 * t80;
            let t92 = f64x8::splat(1.0) / t91;
            let t94 = f64x8::splat(1.0) / t70;
            let t97 = f64x8::splat(1.0) / t70 / t67;
            let t100 = f64x8::splat(1.0) / t70 / t68;
            let t103 = f64x8::splat(1.0) / t70 / t80;
            let t106 = f64x8::splat(1.0) / t70 / t69;
            let t109 = f64x8::splat(1.0) / t70 / t85;
            let t112 = f64x8::splat(1.0) / t70 / t88;
            let t115 = f64x8::splat(1.0) / t70 / t91;
            let t117 = f64x8::splat(1.0) / t71;
            let t121 = t73 / f64x8::splat(5985.0) - t76 / f64x8::splat(7030.0) - t78 / f64x8::splat(30.0) + t81 / f64x8::splat(70.0) - t83 / f64x8::splat(135.0) + t86 / f64x8::splat(231.0) - t89 / f64x8::splat(364.0) + t92 / f64x8::splat(540.0) - t94 / f64x8::splat(765.0) + t97 / f64x8::splat(1045.0) - t100 / f64x8::splat(1386.0) + t103 / f64x8::splat(1794.0) - t106 / f64x8::splat(2275.0) + t109 / f64x8::splat(2835.0) - t112 / f64x8::splat(3480.0) + t115 / f64x8::splat(4216.0) - t117 / f64x8::splat(5049.0) + f64x8::splat(1.0) / t67 / f64x8::splat(9.0);
            let t122 = ((t65).select(f64x8::splat(1.92), t63));
            let t123 = (simd::atan2(f64x8::splat(1.0), t122));
            let t124 = t122 * t122;
            let t125 = t124 + f64x8::splat(3.0);
            let t126 = f64x8::splat(1.0) / t124;
            let t127 = f64x8::splat(1.0) + t126;
            let t128 = (simd::ln(t127));
            let t130 = -t125 * t128 + f64x8::splat(1.0);
            let t133 = t123 + t130 * t122 / f64x8::splat(4.0);
            let t137 = ((t64).select(t121, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t133 * t122));
            let t138 = t137 * t19;
            let t142 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t51 * t138 * t18));
            let tzk0 = f64x8::splat(2.0) * t142;
            acc_zk = tzk0;
            let t143 = f64x8::splat(1.0) / t33;
            let t144 = t137 * t143;
            let t148 = t67 * t66;
            let t150 = f64x8::splat(1.0) / t71 / t148;
            let t153 = f64x8::splat(1.0) / t55 / t54 * param_hyb_omega_0;
            let t155 = f64x8::splat(M_PI) * t61 * t153;
            let t156 = t51 * t51;
            let t157 = f64x8::splat(1.0) / t156;
            let t158 = t157 * t25;
            let t159 = t32 * v_rho;
            let t161 = f64x8::splat(1.0) / t33 / t159;
            let t166 = v_sigma * t25;
            let t167 = t166 * t27;
            let t168 = t35 * t30;
            let t169 = t45 * t45;
            let t170 = f64x8::splat(1.0) / t169;
            let t173 = t41 / t19 / t32;
            let t177 = t31 * t35 + f64x8::splat(1.0);
            let t178 = ((t177).sqrt());
            let t179 = f64x8::splat(1.0) / t178;
            let t180 = t179 * t161;
            let t183 = -f64x8::splat(0.0336) * t173 * t37 - f64x8::splat(0.0336) * t180 * t31;
            let t184 = t183 * t170;
            let t185 = t184 * t168;
            let t188 = -f64x8::splat(0.002488888888888889) * t46 * t161 * t31 * t28 - f64x8::splat(0.0009333333333333333) * t185 * t167;
            let t194 = f64x8::splat(1.0) / t59 / t58;
            let t195 = t194 * t29;
            let t199 = t188 * t158 * t27 * t155 / f64x8::splat(4.0) - t11 * t195 * t57 / f64x8::splat(6.0);
            let t200 = ((t65).select(t199, f64x8::splat(0.0)));
            let t203 = t68 * t66;
            let t205 = f64x8::splat(1.0) / t71 / t203;
            let t208 = f64x8::splat(1.0) / t203;
            let t211 = t68 * t148;
            let t212 = f64x8::splat(1.0) / t211;
            let t215 = t69 * t66;
            let t216 = f64x8::splat(1.0) / t215;
            let t219 = t69 * t148;
            let t220 = f64x8::splat(1.0) / t219;
            let t223 = t69 * t203;
            let t224 = f64x8::splat(1.0) / t223;
            let t227 = t69 * t211;
            let t228 = f64x8::splat(1.0) / t227;
            let t232 = f64x8::splat(1.0) / t70 / t66;
            let t236 = f64x8::splat(1.0) / t70 / t148;
            let t240 = f64x8::splat(1.0) / t70 / t203;
            let t244 = f64x8::splat(1.0) / t70 / t211;
            let t248 = f64x8::splat(1.0) / t70 / t215;
            let t252 = f64x8::splat(1.0) / t70 / t219;
            let t256 = f64x8::splat(1.0) / t70 / t223;
            let t260 = f64x8::splat(1.0) / t70 / t227;
            let t264 = f64x8::splat(1.0) / t71 / t66;
            let t267 = f64x8::splat(1.0) / t148;
            let t270 = -f64x8::splat(34.0) / f64x8::splat(5985.0) * t200 * t150 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t200 * t205 + f64x8::splat(2.0) / f64x8::splat(15.0) * t200 * t208 - f64x8::splat(3.0) / f64x8::splat(35.0) * t200 * t212 + f64x8::splat(8.0) / f64x8::splat(135.0) * t200 * t216 - f64x8::splat(10.0) / f64x8::splat(231.0) * t200 * t220 + f64x8::splat(3.0) / f64x8::splat(91.0) * t200 * t224 - f64x8::splat(7.0) / f64x8::splat(270.0) * t200 * t228 + f64x8::splat(16.0) / f64x8::splat(765.0) * t200 * t232 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t200 * t236 + f64x8::splat(10.0) / f64x8::splat(693.0) * t200 * t240 - f64x8::splat(11.0) / f64x8::splat(897.0) * t200 * t244 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t200 * t248 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t200 * t252 + f64x8::splat(7.0) / f64x8::splat(870.0) * t200 * t256 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t200 * t260 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t200 * t264 - f64x8::splat(2.0) / f64x8::splat(9.0) * t200 * t267;
            let t271 = ((t65).select(f64x8::splat(0.0), t199));
            let t274 = f64x8::splat(1.0) / t127;
            let t280 = t124 * t122;
            let t281 = f64x8::splat(1.0) / t280;
            let t282 = t281 * t125;
            let t283 = t274 * t271;
            let t286 = -f64x8::splat(2.0) * t122 * t128 * t271 + f64x8::splat(2.0) * t282 * t283;
            let t289 = -t274 * t126 * t271 + t130 * t271 / f64x8::splat(4.0) + t286 * t122 / f64x8::splat(4.0);
            let t293 = ((t64).select(t270, -f64x8::splat(8.0) / f64x8::splat(3.0) * t289 * t122 - f64x8::splat(8.0) / f64x8::splat(3.0) * t133 * t271));
            let t294 = t293 * t19;
            let t302 = ((t2).select(f64x8::splat(0.0), -t51 * t144 * t18 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t51 * t294 * t18 - f64x8::splat(3.0) / f64x8::splat(8.0) * t188 * t138 * t18));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t302 + f64x8::splat(2.0) * t142;
            acc_vrho = tvrho0;
            let t309 = t29 / t36;
            let t314 = f64x8::splat(0.0126) * t42 * t309 + f64x8::splat(0.0126) * t179 * t168;
            let t315 = t314 * t170;
            let t316 = t315 * t168;
            let t319 = f64x8::splat(0.0009333333333333333) * t46 * t168 * t28 - f64x8::splat(0.0009333333333333333) * t316 * t167;
            let t323 = t319 * t158 * t27 * t155 / f64x8::splat(4.0);
            let t324 = ((t65).select(t323, f64x8::splat(0.0)));
            let t325 = t324 * t150;
            let t327 = t324 * t205;
            let t329 = t324 * t208;
            let t331 = t324 * t212;
            let t333 = t324 * t216;
            let t335 = t324 * t220;
            let t337 = t324 * t224;
            let t339 = t324 * t228;
            let t341 = t324 * t232;
            let t343 = t324 * t236;
            let t345 = t324 * t240;
            let t347 = t324 * t244;
            let t349 = t324 * t248;
            let t351 = t324 * t252;
            let t353 = t324 * t256;
            let t355 = t324 * t260;
            let t357 = t324 * t264;
            let t361 = -f64x8::splat(34.0) / f64x8::splat(5985.0) * t325 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t327 + f64x8::splat(2.0) / f64x8::splat(15.0) * t329 - f64x8::splat(3.0) / f64x8::splat(35.0) * t331 + f64x8::splat(8.0) / f64x8::splat(135.0) * t333 - f64x8::splat(10.0) / f64x8::splat(231.0) * t335 + f64x8::splat(3.0) / f64x8::splat(91.0) * t337 - f64x8::splat(7.0) / f64x8::splat(270.0) * t339 + f64x8::splat(16.0) / f64x8::splat(765.0) * t341 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t343 + f64x8::splat(10.0) / f64x8::splat(693.0) * t345 - f64x8::splat(11.0) / f64x8::splat(897.0) * t347 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t349 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t351 + f64x8::splat(7.0) / f64x8::splat(870.0) * t353 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t355 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t357 - f64x8::splat(2.0) / f64x8::splat(9.0) * t324 * t267;
            let t362 = ((t65).select(f64x8::splat(0.0), t323));
            let t364 = t126 * t362;
            let t370 = t274 * t362;
            let t373 = -f64x8::splat(2.0) * t122 * t128 * t362 + f64x8::splat(2.0) * t282 * t370;
            let t376 = -t274 * t364 + t130 * t362 / f64x8::splat(4.0) + t373 * t122 / f64x8::splat(4.0);
            let t380 = ((t64).select(t361, -f64x8::splat(8.0) / f64x8::splat(3.0) * t376 * t122 - f64x8::splat(8.0) / f64x8::splat(3.0) * t133 * t362));
            let t381 = t380 * t19;
            let t388 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t319 * t138 * t18 - f64x8::splat(3.0) / f64x8::splat(8.0) * t51 * t381 * t18));
            let tvsigma0 = f64x8::splat(2.0) * t388 * v_rho;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
