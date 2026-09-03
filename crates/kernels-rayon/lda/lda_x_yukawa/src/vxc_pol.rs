//! LDA_X_YUKAWA vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_yukawa.c`
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
pub fn lda_x_yukawa_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
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
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t3 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = t6 * t3 * t1;
            let t8 = f64x8::splat(M_CBRT2);
            let t9 = t8 * t8;
            let t10 = v_rho0 - v_rho1;
            let t11 = v_rho0 + v_rho1;
            let t12 = f64x8::splat(1.0) / t11;
            let t13 = t12 * t10;
            let t14 = f64x8::splat(1.0) + t13;
            let t15 = (t14).simd_le(zeta_threshold);
            let t16 = (simd::cbrt(zeta_threshold));
            let t17 = t16 * zeta_threshold;
            let t18 = (simd::cbrt(t14));
            let t20 = ((t15).select(t17, t18 * t14));
            let t21 = t20 * t9;
            let t22 = (simd::cbrt(t11));
            let t23 = (simd::cbrt(f64x8::splat(9.0)));
            let t24 = t23 * t23;
            let t25 = t3 * t3;
            let t26 = t25 * t24;
            let t27 = param_hyb_omega_0 * t26;
            let t28 = f64x8::splat(1.0) / t22;
            let t29 = t28 * t1;
            let t30 = ((t15).select(t16, t18));
            let t31 = f64x8::splat(1.0) / t30;
            let t34 = t31 * t29 * t27 / f64x8::splat(18.0);
            let t35 = (f64x8::splat(1.92)).simd_le(t34);
            let t36 = (f64x8::splat(1.92)).simd_lt(t34);
            let t37 = ((t36).select(t34, f64x8::splat(1.92)));
            let t38 = t37 * t37;
            let t41 = t38 * t38;
            let t42 = f64x8::splat(1.0) / t41;
            let t44 = t41 * t38;
            let t45 = f64x8::splat(1.0) / t44;
            let t47 = t41 * t41;
            let t48 = f64x8::splat(1.0) / t47;
            let t50 = t47 * t38;
            let t51 = f64x8::splat(1.0) / t50;
            let t53 = t47 * t41;
            let t54 = f64x8::splat(1.0) / t53;
            let t56 = t47 * t44;
            let t57 = f64x8::splat(1.0) / t56;
            let t59 = t47 * t47;
            let t60 = f64x8::splat(1.0) / t59;
            let t63 = f64x8::splat(1.0) / t59 / t38;
            let t66 = f64x8::splat(1.0) / t59 / t41;
            let t69 = f64x8::splat(1.0) / t59 / t44;
            let t72 = f64x8::splat(1.0) / t59 / t47;
            let t75 = f64x8::splat(1.0) / t59 / t50;
            let t78 = f64x8::splat(1.0) / t59 / t53;
            let t81 = f64x8::splat(1.0) / t59 / t56;
            let t83 = t59 * t59;
            let t84 = f64x8::splat(1.0) / t83;
            let t87 = f64x8::splat(1.0) / t83 / t38;
            let t90 = f64x8::splat(1.0) / t83 / t41;
            let t92 = f64x8::splat(1.0) / t38 / f64x8::splat(9.0) - t42 / f64x8::splat(30.0) + t45 / f64x8::splat(70.0) - t48 / f64x8::splat(135.0) + t51 / f64x8::splat(231.0) - t54 / f64x8::splat(364.0) + t57 / f64x8::splat(540.0) - t60 / f64x8::splat(765.0) + t63 / f64x8::splat(1045.0) - t66 / f64x8::splat(1386.0) + t69 / f64x8::splat(1794.0) - t72 / f64x8::splat(2275.0) + t75 / f64x8::splat(2835.0) - t78 / f64x8::splat(3480.0) + t81 / f64x8::splat(4216.0) - t84 / f64x8::splat(5049.0) + t87 / f64x8::splat(5985.0) - t90 / f64x8::splat(7030.0);
            let t93 = ((t36).select(f64x8::splat(1.92), t34));
            let t94 = (simd::atan2(f64x8::splat(1.0), t93));
            let t95 = t93 * t93;
            let t96 = t95 + f64x8::splat(3.0);
            let t97 = f64x8::splat(1.0) / t95;
            let t98 = f64x8::splat(1.0) + t97;
            let t99 = (simd::ln(t98));
            let t101 = -t99 * t96 + f64x8::splat(1.0);
            let t104 = t94 + t101 * t93 / f64x8::splat(4.0);
            let t108 = ((t35).select(t92, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t104 * t93));
            let t109 = t108 * t22;
            let t111 = t109 * t21 * t7;
            let t112 = f64x8::splat(1.0) - t13;
            let t113 = (t112).simd_le(zeta_threshold);
            let t114 = (simd::cbrt(t112));
            let t116 = ((t113).select(t17, t114 * t112));
            let t117 = t116 * t9;
            let t118 = ((t113).select(t16, t114));
            let t119 = f64x8::splat(1.0) / t118;
            let t122 = t119 * t29 * t27 / f64x8::splat(18.0);
            let t123 = (f64x8::splat(1.92)).simd_le(t122);
            let t124 = (f64x8::splat(1.92)).simd_lt(t122);
            let t125 = ((t124).select(t122, f64x8::splat(1.92)));
            let t126 = t125 * t125;
            let t129 = t126 * t126;
            let t130 = f64x8::splat(1.0) / t129;
            let t132 = t129 * t126;
            let t133 = f64x8::splat(1.0) / t132;
            let t135 = t129 * t129;
            let t136 = f64x8::splat(1.0) / t135;
            let t138 = t135 * t126;
            let t139 = f64x8::splat(1.0) / t138;
            let t141 = t135 * t129;
            let t142 = f64x8::splat(1.0) / t141;
            let t144 = t135 * t132;
            let t145 = f64x8::splat(1.0) / t144;
            let t147 = t135 * t135;
            let t148 = f64x8::splat(1.0) / t147;
            let t151 = f64x8::splat(1.0) / t147 / t126;
            let t154 = f64x8::splat(1.0) / t147 / t129;
            let t157 = f64x8::splat(1.0) / t147 / t132;
            let t160 = f64x8::splat(1.0) / t147 / t135;
            let t163 = f64x8::splat(1.0) / t147 / t138;
            let t166 = f64x8::splat(1.0) / t147 / t141;
            let t169 = f64x8::splat(1.0) / t147 / t144;
            let t171 = t147 * t147;
            let t172 = f64x8::splat(1.0) / t171;
            let t175 = f64x8::splat(1.0) / t171 / t126;
            let t178 = f64x8::splat(1.0) / t171 / t129;
            let t180 = f64x8::splat(1.0) / t126 / f64x8::splat(9.0) - t130 / f64x8::splat(30.0) + t133 / f64x8::splat(70.0) - t136 / f64x8::splat(135.0) + t139 / f64x8::splat(231.0) - t142 / f64x8::splat(364.0) + t145 / f64x8::splat(540.0) - t148 / f64x8::splat(765.0) + t151 / f64x8::splat(1045.0) - t154 / f64x8::splat(1386.0) + t157 / f64x8::splat(1794.0) - t160 / f64x8::splat(2275.0) + t163 / f64x8::splat(2835.0) - t166 / f64x8::splat(3480.0) + t169 / f64x8::splat(4216.0) - t172 / f64x8::splat(5049.0) + t175 / f64x8::splat(5985.0) - t178 / f64x8::splat(7030.0);
            let t181 = ((t124).select(f64x8::splat(1.92), t122));
            let t182 = (simd::atan2(f64x8::splat(1.0), t181));
            let t183 = t181 * t181;
            let t184 = t183 + f64x8::splat(3.0);
            let t185 = f64x8::splat(1.0) / t183;
            let t186 = f64x8::splat(1.0) + t185;
            let t187 = (simd::ln(t186));
            let t189 = -t187 * t184 + f64x8::splat(1.0);
            let t192 = t182 + t189 * t181 / f64x8::splat(4.0);
            let t196 = ((t123).select(t180, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t192 * t181));
            let t197 = t196 * t22;
            let t199 = t197 * t117 * t7;
            let tzk0 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t111 - f64x8::splat(3.0) / f64x8::splat(32.0) * t199;
            acc_zk = tzk0;
            let t201 = f64x8::splat(3.0) / f64x8::splat(32.0) * t111;
            let t202 = f64x8::splat(3.0) / f64x8::splat(32.0) * t199;
            let t203 = t11 * t11;
            let t204 = f64x8::splat(1.0) / t203;
            let t205 = t204 * t10;
            let t206 = t12 - t205;
            let t209 = ((t15).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t206 * t18));
            let t210 = t209 * t9;
            let t212 = t109 * t210 * t7;
            let t213 = f64x8::splat(3.0) / f64x8::splat(32.0) * t212;
            let t214 = t22 * t22;
            let t215 = f64x8::splat(1.0) / t214;
            let t216 = t108 * t215;
            let t218 = t216 * t21 * t7;
            let t219 = t218 / f64x8::splat(32.0);
            let t220 = t38 * t37;
            let t221 = f64x8::splat(1.0) / t220;
            let t223 = f64x8::splat(1.0) / t22 / t11;
            let t224 = t223 * t1;
            let t227 = t31 * t224 * t27 / f64x8::splat(54.0);
            let t228 = t30 * t30;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t18 * t18;
            let t231 = f64x8::splat(1.0) / t230;
            let t232 = t206 * t231;
            let t234 = ((t15).select(f64x8::splat(0.0), t232 / f64x8::splat(3.0)));
            let t235 = t234 * t229;
            let t239 = -t227 - t235 * t29 * t27 / f64x8::splat(18.0);
            let t240 = ((t36).select(t239, f64x8::splat(0.0)));
            let t243 = t41 * t37;
            let t244 = f64x8::splat(1.0) / t243;
            let t247 = t41 * t220;
            let t248 = f64x8::splat(1.0) / t247;
            let t251 = t47 * t37;
            let t252 = f64x8::splat(1.0) / t251;
            let t255 = t47 * t220;
            let t256 = f64x8::splat(1.0) / t255;
            let t259 = t47 * t243;
            let t260 = f64x8::splat(1.0) / t259;
            let t263 = t47 * t247;
            let t264 = f64x8::splat(1.0) / t263;
            let t268 = f64x8::splat(1.0) / t59 / t37;
            let t272 = f64x8::splat(1.0) / t59 / t220;
            let t276 = f64x8::splat(1.0) / t59 / t243;
            let t280 = f64x8::splat(1.0) / t59 / t247;
            let t284 = f64x8::splat(1.0) / t59 / t251;
            let t288 = f64x8::splat(1.0) / t59 / t255;
            let t292 = f64x8::splat(1.0) / t59 / t259;
            let t296 = f64x8::splat(1.0) / t59 / t263;
            let t300 = f64x8::splat(1.0) / t83 / t37;
            let t304 = f64x8::splat(1.0) / t83 / t220;
            let t308 = f64x8::splat(1.0) / t83 / t243;
            let t311 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t240 * t221 + f64x8::splat(2.0) / f64x8::splat(15.0) * t240 * t244 - f64x8::splat(3.0) / f64x8::splat(35.0) * t240 * t248 + f64x8::splat(8.0) / f64x8::splat(135.0) * t240 * t252 - f64x8::splat(10.0) / f64x8::splat(231.0) * t240 * t256 + f64x8::splat(3.0) / f64x8::splat(91.0) * t240 * t260 - f64x8::splat(7.0) / f64x8::splat(270.0) * t240 * t264 + f64x8::splat(16.0) / f64x8::splat(765.0) * t240 * t268 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t240 * t272 + f64x8::splat(10.0) / f64x8::splat(693.0) * t240 * t276 - f64x8::splat(11.0) / f64x8::splat(897.0) * t240 * t280 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t240 * t284 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t240 * t288 + f64x8::splat(7.0) / f64x8::splat(870.0) * t240 * t292 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t240 * t296 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t240 * t300 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t240 * t304 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t240 * t308;
            let t312 = ((t36).select(f64x8::splat(0.0), t239));
            let t315 = f64x8::splat(1.0) / t98;
            let t321 = t95 * t93;
            let t322 = f64x8::splat(1.0) / t321;
            let t323 = t322 * t96;
            let t324 = t315 * t312;
            let t327 = -f64x8::splat(2.0) * t99 * t312 * t93 + f64x8::splat(2.0) * t324 * t323;
            let t330 = -t315 * t97 * t312 + t101 * t312 / f64x8::splat(4.0) + t327 * t93 / f64x8::splat(4.0);
            let t334 = ((t35).select(t311, -f64x8::splat(8.0) / f64x8::splat(3.0) * t104 * t312 - f64x8::splat(8.0) / f64x8::splat(3.0) * t330 * t93));
            let t335 = t334 * t22;
            let t337 = t335 * t21 * t7;
            let t338 = f64x8::splat(3.0) / f64x8::splat(32.0) * t337;
            let t339 = -t206;
            let t342 = ((t113).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t339 * t114));
            let t343 = t342 * t9;
            let t345 = t197 * t343 * t7;
            let t346 = f64x8::splat(3.0) / f64x8::splat(32.0) * t345;
            let t347 = t196 * t215;
            let t349 = t347 * t117 * t7;
            let t350 = t349 / f64x8::splat(32.0);
            let t351 = t126 * t125;
            let t352 = f64x8::splat(1.0) / t351;
            let t355 = t119 * t224 * t27 / f64x8::splat(54.0);
            let t356 = t118 * t118;
            let t357 = f64x8::splat(1.0) / t356;
            let t358 = t114 * t114;
            let t359 = f64x8::splat(1.0) / t358;
            let t360 = t339 * t359;
            let t362 = ((t113).select(f64x8::splat(0.0), t360 / f64x8::splat(3.0)));
            let t363 = t362 * t357;
            let t367 = -t355 - t363 * t29 * t27 / f64x8::splat(18.0);
            let t368 = ((t124).select(t367, f64x8::splat(0.0)));
            let t371 = t129 * t125;
            let t372 = f64x8::splat(1.0) / t371;
            let t375 = t129 * t351;
            let t376 = f64x8::splat(1.0) / t375;
            let t379 = t135 * t125;
            let t380 = f64x8::splat(1.0) / t379;
            let t383 = t135 * t351;
            let t384 = f64x8::splat(1.0) / t383;
            let t387 = t135 * t371;
            let t388 = f64x8::splat(1.0) / t387;
            let t391 = t135 * t375;
            let t392 = f64x8::splat(1.0) / t391;
            let t396 = f64x8::splat(1.0) / t147 / t125;
            let t400 = f64x8::splat(1.0) / t147 / t351;
            let t404 = f64x8::splat(1.0) / t147 / t371;
            let t408 = f64x8::splat(1.0) / t147 / t375;
            let t412 = f64x8::splat(1.0) / t147 / t379;
            let t416 = f64x8::splat(1.0) / t147 / t383;
            let t420 = f64x8::splat(1.0) / t147 / t387;
            let t424 = f64x8::splat(1.0) / t147 / t391;
            let t428 = f64x8::splat(1.0) / t171 / t125;
            let t432 = f64x8::splat(1.0) / t171 / t351;
            let t436 = f64x8::splat(1.0) / t171 / t371;
            let t439 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t368 * t352 + f64x8::splat(2.0) / f64x8::splat(15.0) * t368 * t372 - f64x8::splat(3.0) / f64x8::splat(35.0) * t368 * t376 + f64x8::splat(8.0) / f64x8::splat(135.0) * t368 * t380 - f64x8::splat(10.0) / f64x8::splat(231.0) * t368 * t384 + f64x8::splat(3.0) / f64x8::splat(91.0) * t368 * t388 - f64x8::splat(7.0) / f64x8::splat(270.0) * t368 * t392 + f64x8::splat(16.0) / f64x8::splat(765.0) * t368 * t396 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t368 * t400 + f64x8::splat(10.0) / f64x8::splat(693.0) * t368 * t404 - f64x8::splat(11.0) / f64x8::splat(897.0) * t368 * t408 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t368 * t412 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t368 * t416 + f64x8::splat(7.0) / f64x8::splat(870.0) * t368 * t420 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t368 * t424 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t368 * t428 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t368 * t432 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t368 * t436;
            let t440 = ((t124).select(f64x8::splat(0.0), t367));
            let t443 = f64x8::splat(1.0) / t186;
            let t449 = t183 * t181;
            let t450 = f64x8::splat(1.0) / t449;
            let t451 = t450 * t184;
            let t452 = t443 * t440;
            let t455 = -f64x8::splat(2.0) * t187 * t440 * t181 + f64x8::splat(2.0) * t452 * t451;
            let t458 = -t443 * t185 * t440 + t189 * t440 / f64x8::splat(4.0) + t455 * t181 / f64x8::splat(4.0);
            let t462 = ((t123).select(t439, -f64x8::splat(8.0) / f64x8::splat(3.0) * t458 * t181 - f64x8::splat(8.0) / f64x8::splat(3.0) * t192 * t440));
            let t463 = t462 * t22;
            let t465 = t463 * t117 * t7;
            let t466 = f64x8::splat(3.0) / f64x8::splat(32.0) * t465;
            let tvrho0 = -t201 - t202 + (-t213 - t219 - t338 - t346 - t350 - t466) * t11;
            acc_vrho_0 = tvrho0;
            let t469 = -t12 - t205;
            let t472 = ((t15).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t469 * t18));
            let t473 = t472 * t9;
            let t475 = t109 * t473 * t7;
            let t476 = f64x8::splat(3.0) / f64x8::splat(32.0) * t475;
            let t477 = t469 * t231;
            let t479 = ((t15).select(f64x8::splat(0.0), t477 / f64x8::splat(3.0)));
            let t480 = t479 * t229;
            let t484 = -t227 - t480 * t29 * t27 / f64x8::splat(18.0);
            let t485 = ((t36).select(t484, f64x8::splat(0.0)));
            let t488 = t485 * t244;
            let t490 = t485 * t248;
            let t492 = t485 * t252;
            let t494 = t485 * t256;
            let t496 = t485 * t260;
            let t498 = t485 * t264;
            let t500 = t485 * t268;
            let t502 = t485 * t272;
            let t504 = t485 * t276;
            let t506 = t485 * t280;
            let t508 = t485 * t284;
            let t510 = t485 * t288;
            let t512 = t485 * t292;
            let t514 = t485 * t296;
            let t516 = t485 * t300;
            let t518 = t485 * t304;
            let t520 = t485 * t308;
            let t522 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t485 * t221 + f64x8::splat(2.0) / f64x8::splat(15.0) * t488 - f64x8::splat(3.0) / f64x8::splat(35.0) * t490 + f64x8::splat(8.0) / f64x8::splat(135.0) * t492 - f64x8::splat(10.0) / f64x8::splat(231.0) * t494 + f64x8::splat(3.0) / f64x8::splat(91.0) * t496 - f64x8::splat(7.0) / f64x8::splat(270.0) * t498 + f64x8::splat(16.0) / f64x8::splat(765.0) * t500 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t502 + f64x8::splat(10.0) / f64x8::splat(693.0) * t504 - f64x8::splat(11.0) / f64x8::splat(897.0) * t506 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t508 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t510 + f64x8::splat(7.0) / f64x8::splat(870.0) * t512 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t514 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t516 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t518 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t520;
            let t523 = ((t36).select(f64x8::splat(0.0), t484));
            let t525 = t97 * t523;
            let t531 = t315 * t523;
            let t534 = -f64x8::splat(2.0) * t99 * t523 * t93 + f64x8::splat(2.0) * t531 * t323;
            let t537 = -t315 * t525 + t101 * t523 / f64x8::splat(4.0) + t534 * t93 / f64x8::splat(4.0);
            let t541 = ((t35).select(t522, -f64x8::splat(8.0) / f64x8::splat(3.0) * t104 * t523 - f64x8::splat(8.0) / f64x8::splat(3.0) * t537 * t93));
            let t542 = t541 * t22;
            let t544 = t542 * t21 * t7;
            let t545 = f64x8::splat(3.0) / f64x8::splat(32.0) * t544;
            let t546 = -t469;
            let t549 = ((t113).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t546 * t114));
            let t550 = t549 * t9;
            let t552 = t197 * t550 * t7;
            let t553 = f64x8::splat(3.0) / f64x8::splat(32.0) * t552;
            let t554 = t546 * t359;
            let t556 = ((t113).select(f64x8::splat(0.0), t554 / f64x8::splat(3.0)));
            let t557 = t556 * t357;
            let t561 = -t355 - t557 * t29 * t27 / f64x8::splat(18.0);
            let t562 = ((t124).select(t561, f64x8::splat(0.0)));
            let t565 = t562 * t372;
            let t567 = t562 * t376;
            let t569 = t562 * t380;
            let t571 = t562 * t384;
            let t573 = t562 * t388;
            let t575 = t562 * t392;
            let t577 = t562 * t396;
            let t579 = t562 * t400;
            let t581 = t562 * t404;
            let t583 = t562 * t408;
            let t585 = t562 * t412;
            let t587 = t562 * t416;
            let t589 = t562 * t420;
            let t591 = t562 * t424;
            let t593 = t562 * t428;
            let t595 = t562 * t432;
            let t597 = t562 * t436;
            let t599 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t562 * t352 + f64x8::splat(2.0) / f64x8::splat(15.0) * t565 - f64x8::splat(3.0) / f64x8::splat(35.0) * t567 + f64x8::splat(8.0) / f64x8::splat(135.0) * t569 - f64x8::splat(10.0) / f64x8::splat(231.0) * t571 + f64x8::splat(3.0) / f64x8::splat(91.0) * t573 - f64x8::splat(7.0) / f64x8::splat(270.0) * t575 + f64x8::splat(16.0) / f64x8::splat(765.0) * t577 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t579 + f64x8::splat(10.0) / f64x8::splat(693.0) * t581 - f64x8::splat(11.0) / f64x8::splat(897.0) * t583 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t585 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t587 + f64x8::splat(7.0) / f64x8::splat(870.0) * t589 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t591 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t593 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t595 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t597;
            let t600 = ((t124).select(f64x8::splat(0.0), t561));
            let t602 = t185 * t600;
            let t608 = t443 * t600;
            let t611 = -f64x8::splat(2.0) * t187 * t600 * t181 + f64x8::splat(2.0) * t608 * t451;
            let t614 = -t443 * t602 + t189 * t600 / f64x8::splat(4.0) + t611 * t181 / f64x8::splat(4.0);
            let t618 = ((t123).select(t599, -f64x8::splat(8.0) / f64x8::splat(3.0) * t614 * t181 - f64x8::splat(8.0) / f64x8::splat(3.0) * t192 * t600));
            let t619 = t618 * t22;
            let t621 = t619 * t117 * t7;
            let t622 = f64x8::splat(3.0) / f64x8::splat(32.0) * t621;
            let tvrho1 = -t201 - t202 + (-t476 - t219 - t545 - t553 - t350 - t622) * t11;
            acc_vrho_1 = tvrho1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}
