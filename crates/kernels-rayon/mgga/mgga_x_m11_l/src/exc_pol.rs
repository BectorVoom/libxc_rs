//! MGGA_X_M11_L exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_m11_l.c`
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

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_m11_l_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_a_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_b_9: f64,
    param_b_10: f64,
    param_b_11: f64,
    param_b_0: f64,
    param_c_1: f64,
    param_c_2: f64,
    param_c_3: f64,
    param_c_4: f64,
    param_c_5: f64,
    param_c_6: f64,
    param_c_7: f64,
    param_c_8: f64,
    param_c_9: f64,
    param_c_10: f64,
    param_c_11: f64,
    param_c_0: f64,
    param_d_1: f64,
    param_d_2: f64,
    param_d_3: f64,
    param_d_4: f64,
    param_d_5: f64,
    param_d_6: f64,
    param_d_7: f64,
    param_d_8: f64,
    param_d_9: f64,
    param_d_10: f64,
    param_d_11: f64,
    param_d_0: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a_1 = f64x8::splat(param_a_1);
    let param_a_2 = f64x8::splat(param_a_2);
    let param_a_3 = f64x8::splat(param_a_3);
    let param_a_4 = f64x8::splat(param_a_4);
    let param_a_5 = f64x8::splat(param_a_5);
    let param_a_6 = f64x8::splat(param_a_6);
    let param_a_7 = f64x8::splat(param_a_7);
    let param_a_8 = f64x8::splat(param_a_8);
    let param_a_9 = f64x8::splat(param_a_9);
    let param_a_10 = f64x8::splat(param_a_10);
    let param_a_11 = f64x8::splat(param_a_11);
    let param_a_0 = f64x8::splat(param_a_0);
    let param_b_1 = f64x8::splat(param_b_1);
    let param_b_2 = f64x8::splat(param_b_2);
    let param_b_3 = f64x8::splat(param_b_3);
    let param_b_4 = f64x8::splat(param_b_4);
    let param_b_5 = f64x8::splat(param_b_5);
    let param_b_6 = f64x8::splat(param_b_6);
    let param_b_7 = f64x8::splat(param_b_7);
    let param_b_8 = f64x8::splat(param_b_8);
    let param_b_9 = f64x8::splat(param_b_9);
    let param_b_10 = f64x8::splat(param_b_10);
    let param_b_11 = f64x8::splat(param_b_11);
    let param_b_0 = f64x8::splat(param_b_0);
    let param_c_1 = f64x8::splat(param_c_1);
    let param_c_2 = f64x8::splat(param_c_2);
    let param_c_3 = f64x8::splat(param_c_3);
    let param_c_4 = f64x8::splat(param_c_4);
    let param_c_5 = f64x8::splat(param_c_5);
    let param_c_6 = f64x8::splat(param_c_6);
    let param_c_7 = f64x8::splat(param_c_7);
    let param_c_8 = f64x8::splat(param_c_8);
    let param_c_9 = f64x8::splat(param_c_9);
    let param_c_10 = f64x8::splat(param_c_10);
    let param_c_11 = f64x8::splat(param_c_11);
    let param_c_0 = f64x8::splat(param_c_0);
    let param_d_1 = f64x8::splat(param_d_1);
    let param_d_2 = f64x8::splat(param_d_2);
    let param_d_3 = f64x8::splat(param_d_3);
    let param_d_4 = f64x8::splat(param_d_4);
    let param_d_5 = f64x8::splat(param_d_5);
    let param_d_6 = f64x8::splat(param_d_6);
    let param_d_7 = f64x8::splat(param_d_7);
    let param_d_8 = f64x8::splat(param_d_8);
    let param_d_9 = f64x8::splat(param_d_9);
    let param_d_10 = f64x8::splat(param_d_10);
    let param_d_11 = f64x8::splat(param_d_11);
    let param_d_0 = f64x8::splat(param_d_0);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
            let t18 = t17 * t8;
            let t19 = ((t11).select(t12, (t15).select(t16, t18)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = (simd::cbrt(f64x8::splat(9.0)));
            let t30 = t29 * t29;
            let t32 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t33 = t32 * t32;
            let t34 = t30 * t33;
            let t35 = t34 * param_hyb_omega_0;
            let t36 = f64x8::splat(1.0) / t27;
            let t37 = t3 * t36;
            let t39 = (f64x8::splat(1.0) + t18).simd_le(zeta_threshold);
            let t41 = (f64x8::splat(1.0) - t18).simd_le(zeta_threshold);
            let t42 = ((t39).select(t12, (t41).select(t16, t18)));
            let t43 = f64x8::splat(1.0) + t42;
            let t44 = (t43).simd_le(zeta_threshold);
            let t45 = (simd::cbrt(t43));
            let t46 = ((t44).select(t22, t45));
            let t47 = f64x8::splat(1.0) / t46;
            let t50 = t35 * t37 * t47 / f64x8::splat(18.0);
            let t51 = (f64x8::splat(1.35)).simd_le(t50);
            let t52 = (f64x8::splat(1.35)).simd_lt(t50);
            let t53 = ((t52).select(t50, f64x8::splat(1.35)));
            let t54 = t53 * t53;
            let t57 = t54 * t54;
            let t58 = f64x8::splat(1.0) / t57;
            let t60 = t57 * t54;
            let t61 = f64x8::splat(1.0) / t60;
            let t63 = t57 * t57;
            let t64 = f64x8::splat(1.0) / t63;
            let t67 = f64x8::splat(1.0) / t63 / t54;
            let t70 = f64x8::splat(1.0) / t63 / t57;
            let t73 = f64x8::splat(1.0) / t63 / t60;
            let t75 = t63 * t63;
            let t76 = f64x8::splat(1.0) / t75;
            let t79 = ((t52).select(f64x8::splat(1.35), t50));
            let t80 = ((f64x8::splat(M_PI)).sqrt());
            let t81 = f64x8::splat(1.0) / t79;
            let t83 = (simd::erf(t81 / f64x8::splat(2.0)));
            let t85 = t79 * t79;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = (simd::exp(-t86 / f64x8::splat(4.0)));
            let t89 = t88 - f64x8::splat(1.0);
            let t92 = t88 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t85 * t89;
            let t95 = f64x8::splat(2.0) * t79 * t92 + t80 * t83;
            let t99 = ((t51).select(f64x8::splat(1.0) / t54 / f64x8::splat(36.0) - t58 / f64x8::splat(960.0) + t61 / f64x8::splat(26880.0) - t64 / f64x8::splat(829440.0) + t67 / f64x8::splat(28385280.0) - t70 / f64x8::splat(1073479680.0) + t73 / f64x8::splat(44590694400.0) - t76 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t79 * t95));
            let t100 = f64x8::splat(M_CBRT6);
            let t101 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t102 = (simd::cbrt(t101));
            let t103 = t102 * t102;
            let t104 = f64x8::splat(1.0) / t103;
            let t105 = t100 * t104;
            let t106 = v_rho0 * v_rho0;
            let t107 = (simd::cbrt(v_rho0));
            let t108 = t107 * t107;
            let t110 = f64x8::splat(1.0) / t108 / t106;
            let t112 = t105 * v_sigma0 * t110;
            let t114 = f64x8::splat(0.804) + f64x8::splat(0.00914625) * t112;
            let t117 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t114;
            let t118 = param_a_0;
            let t119 = param_a_1;
            let t120 = t100 * t100;
            let t122 = f64x8::splat(3.0) / f64x8::splat(10.0) * t120 * t103;
            let t124 = f64x8::splat(1.0) / t108 / v_rho0;
            let t125 = v_tau0 * t124;
            let t126 = t122 - t125;
            let t127 = t119 * t126;
            let t128 = t122 + t125;
            let t129 = f64x8::splat(1.0) / t128;
            let t131 = param_a_2;
            let t132 = t126 * t126;
            let t133 = t131 * t132;
            let t134 = t128 * t128;
            let t135 = f64x8::splat(1.0) / t134;
            let t137 = param_a_3;
            let t138 = t132 * t126;
            let t139 = t137 * t138;
            let t140 = t134 * t128;
            let t141 = f64x8::splat(1.0) / t140;
            let t143 = param_a_4;
            let t144 = t132 * t132;
            let t145 = t143 * t144;
            let t146 = t134 * t134;
            let t147 = f64x8::splat(1.0) / t146;
            let t149 = param_a_5;
            let t150 = t144 * t126;
            let t151 = t149 * t150;
            let t152 = t146 * t128;
            let t153 = f64x8::splat(1.0) / t152;
            let t155 = param_a_6;
            let t156 = t144 * t132;
            let t157 = t155 * t156;
            let t158 = t146 * t134;
            let t159 = f64x8::splat(1.0) / t158;
            let t161 = param_a_7;
            let t162 = t144 * t138;
            let t163 = t161 * t162;
            let t164 = t146 * t140;
            let t165 = f64x8::splat(1.0) / t164;
            let t167 = param_a_8;
            let t168 = t144 * t144;
            let t169 = t167 * t168;
            let t170 = t146 * t146;
            let t171 = f64x8::splat(1.0) / t170;
            let t173 = param_a_9;
            let t174 = t168 * t126;
            let t175 = t173 * t174;
            let t177 = f64x8::splat(1.0) / t170 / t128;
            let t179 = param_a_10;
            let t180 = t168 * t132;
            let t181 = t179 * t180;
            let t183 = f64x8::splat(1.0) / t170 / t134;
            let t185 = param_a_11;
            let t186 = t168 * t138;
            let t187 = t185 * t186;
            let t189 = f64x8::splat(1.0) / t170 / t140;
            let t191 = t127 * t129 + t133 * t135 + t139 * t141 + t145 * t147 + t151 * t153 + t157 * t159 + t163 * t165 + t169 * t171 + t175 * t177 + t181 * t183 + t187 * t189 + t118;
            let t194 = (simd::exp(-f64x8::splat(0.009318900220671557) * t112));
            let t196 = f64x8::splat(1.552) - f64x8::splat(0.552) * t194;
            let t197 = param_b_0;
            let t198 = param_b_1;
            let t199 = t198 * t126;
            let t201 = param_b_2;
            let t202 = t201 * t132;
            let t204 = param_b_3;
            let t205 = t204 * t138;
            let t207 = param_b_4;
            let t208 = t207 * t144;
            let t210 = param_b_5;
            let t211 = t210 * t150;
            let t213 = param_b_6;
            let t214 = t213 * t156;
            let t216 = param_b_7;
            let t217 = t216 * t162;
            let t219 = param_b_8;
            let t220 = t219 * t168;
            let t222 = param_b_9;
            let t223 = t222 * t174;
            let t225 = param_b_10;
            let t226 = t225 * t180;
            let t228 = param_b_11;
            let t229 = t228 * t186;
            let t231 = t199 * t129 + t202 * t135 + t205 * t141 + t208 * t147 + t211 * t153 + t214 * t159 + t217 * t165 + t220 * t171 + t223 * t177 + t226 * t183 + t229 * t189 + t197;
            let t233 = t117 * t191 + t196 * t231;
            let t235 = f64x8::splat(1.0) - t99;
            let t236 = param_c_0;
            let t237 = param_c_1;
            let t238 = t237 * t126;
            let t240 = param_c_2;
            let t241 = t240 * t132;
            let t243 = param_c_3;
            let t244 = t243 * t138;
            let t246 = param_c_4;
            let t247 = t246 * t144;
            let t249 = param_c_5;
            let t250 = t249 * t150;
            let t252 = param_c_6;
            let t253 = t252 * t156;
            let t255 = param_c_7;
            let t256 = t255 * t162;
            let t258 = param_c_8;
            let t259 = t258 * t168;
            let t261 = param_c_9;
            let t262 = t261 * t174;
            let t264 = param_c_10;
            let t265 = t264 * t180;
            let t267 = param_c_11;
            let t268 = t267 * t186;
            let t270 = t238 * t129 + t241 * t135 + t244 * t141 + t247 * t147 + t250 * t153 + t253 * t159 + t256 * t165 + t259 * t171 + t262 * t177 + t265 * t183 + t268 * t189 + t236;
            let t272 = param_d_0;
            let t273 = param_d_1;
            let t274 = t273 * t126;
            let t276 = param_d_2;
            let t277 = t276 * t132;
            let t279 = param_d_3;
            let t280 = t279 * t138;
            let t282 = param_d_4;
            let t283 = t282 * t144;
            let t285 = param_d_5;
            let t286 = t285 * t150;
            let t288 = param_d_6;
            let t289 = t288 * t156;
            let t291 = param_d_7;
            let t292 = t291 * t162;
            let t294 = param_d_8;
            let t295 = t294 * t168;
            let t297 = param_d_9;
            let t298 = t297 * t174;
            let t300 = param_d_10;
            let t301 = t300 * t180;
            let t303 = param_d_11;
            let t304 = t303 * t186;
            let t306 = t274 * t129 + t277 * t135 + t280 * t141 + t283 * t147 + t286 * t153 + t289 * t159 + t292 * t165 + t295 * t171 + t298 * t177 + t301 * t183 + t304 * t189 + t272;
            let t308 = t117 * t270 + t196 * t306;
            let t310 = t99 * t233 + t235 * t308;
            let t314 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t310));
            let t315 = (v_rho1).simd_le(dens_threshold);
            let t316 = -t17;
            let t318 = ((t15).select(t12, (t11).select(t16, t316 * t8)));
            let t319 = f64x8::splat(1.0) + t318;
            let t320 = (t319).simd_le(zeta_threshold);
            let t321 = (simd::cbrt(t319));
            let t323 = ((t320).select(t23, t321 * t319));
            let t324 = t323 * t27;
            let t325 = ((t41).select(t12, (t39).select(t16, -t18)));
            let t326 = f64x8::splat(1.0) + t325;
            let t327 = (t326).simd_le(zeta_threshold);
            let t328 = (simd::cbrt(t326));
            let t329 = ((t327).select(t22, t328));
            let t330 = f64x8::splat(1.0) / t329;
            let t333 = t35 * t37 * t330 / f64x8::splat(18.0);
            let t334 = (f64x8::splat(1.35)).simd_le(t333);
            let t335 = (f64x8::splat(1.35)).simd_lt(t333);
            let t336 = ((t335).select(t333, f64x8::splat(1.35)));
            let t337 = t336 * t336;
            let t340 = t337 * t337;
            let t341 = f64x8::splat(1.0) / t340;
            let t343 = t340 * t337;
            let t344 = f64x8::splat(1.0) / t343;
            let t346 = t340 * t340;
            let t347 = f64x8::splat(1.0) / t346;
            let t350 = f64x8::splat(1.0) / t346 / t337;
            let t353 = f64x8::splat(1.0) / t346 / t340;
            let t356 = f64x8::splat(1.0) / t346 / t343;
            let t358 = t346 * t346;
            let t359 = f64x8::splat(1.0) / t358;
            let t362 = ((t335).select(f64x8::splat(1.35), t333));
            let t363 = f64x8::splat(1.0) / t362;
            let t365 = (simd::erf(t363 / f64x8::splat(2.0)));
            let t367 = t362 * t362;
            let t368 = f64x8::splat(1.0) / t367;
            let t370 = (simd::exp(-t368 / f64x8::splat(4.0)));
            let t371 = t370 - f64x8::splat(1.0);
            let t374 = t370 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t367 * t371;
            let t377 = f64x8::splat(2.0) * t362 * t374 + t80 * t365;
            let t381 = ((t334).select(f64x8::splat(1.0) / t337 / f64x8::splat(36.0) - t341 / f64x8::splat(960.0) + t344 / f64x8::splat(26880.0) - t347 / f64x8::splat(829440.0) + t350 / f64x8::splat(28385280.0) - t353 / f64x8::splat(1073479680.0) + t356 / f64x8::splat(44590694400.0) - t359 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t362 * t377));
            let t382 = v_rho1 * v_rho1;
            let t383 = (simd::cbrt(v_rho1));
            let t384 = t383 * t383;
            let t386 = f64x8::splat(1.0) / t384 / t382;
            let t388 = t105 * v_sigma2 * t386;
            let t390 = f64x8::splat(0.804) + f64x8::splat(0.00914625) * t388;
            let t393 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t390;
            let t395 = f64x8::splat(1.0) / t384 / v_rho1;
            let t396 = v_tau1 * t395;
            let t397 = t122 - t396;
            let t398 = t119 * t397;
            let t399 = t122 + t396;
            let t400 = f64x8::splat(1.0) / t399;
            let t402 = t397 * t397;
            let t403 = t131 * t402;
            let t404 = t399 * t399;
            let t405 = f64x8::splat(1.0) / t404;
            let t407 = t402 * t397;
            let t408 = t137 * t407;
            let t409 = t404 * t399;
            let t410 = f64x8::splat(1.0) / t409;
            let t412 = t402 * t402;
            let t413 = t143 * t412;
            let t414 = t404 * t404;
            let t415 = f64x8::splat(1.0) / t414;
            let t417 = t412 * t397;
            let t418 = t149 * t417;
            let t419 = t414 * t399;
            let t420 = f64x8::splat(1.0) / t419;
            let t422 = t412 * t402;
            let t423 = t155 * t422;
            let t424 = t414 * t404;
            let t425 = f64x8::splat(1.0) / t424;
            let t427 = t412 * t407;
            let t428 = t161 * t427;
            let t429 = t414 * t409;
            let t430 = f64x8::splat(1.0) / t429;
            let t432 = t412 * t412;
            let t433 = t167 * t432;
            let t434 = t414 * t414;
            let t435 = f64x8::splat(1.0) / t434;
            let t437 = t432 * t397;
            let t438 = t173 * t437;
            let t440 = f64x8::splat(1.0) / t434 / t399;
            let t442 = t432 * t402;
            let t443 = t179 * t442;
            let t445 = f64x8::splat(1.0) / t434 / t404;
            let t447 = t432 * t407;
            let t448 = t185 * t447;
            let t450 = f64x8::splat(1.0) / t434 / t409;
            let t452 = t398 * t400 + t403 * t405 + t408 * t410 + t413 * t415 + t418 * t420 + t423 * t425 + t428 * t430 + t433 * t435 + t438 * t440 + t443 * t445 + t448 * t450 + t118;
            let t455 = (simd::exp(-f64x8::splat(0.009318900220671557) * t388));
            let t457 = f64x8::splat(1.552) - f64x8::splat(0.552) * t455;
            let t458 = t198 * t397;
            let t460 = t201 * t402;
            let t462 = t204 * t407;
            let t464 = t207 * t412;
            let t466 = t210 * t417;
            let t468 = t213 * t422;
            let t470 = t216 * t427;
            let t472 = t219 * t432;
            let t474 = t222 * t437;
            let t476 = t225 * t442;
            let t478 = t228 * t447;
            let t480 = t458 * t400 + t460 * t405 + t462 * t410 + t464 * t415 + t466 * t420 + t468 * t425 + t470 * t430 + t472 * t435 + t474 * t440 + t476 * t445 + t478 * t450 + t197;
            let t482 = t393 * t452 + t457 * t480;
            let t484 = f64x8::splat(1.0) - t381;
            let t485 = t237 * t397;
            let t487 = t240 * t402;
            let t489 = t243 * t407;
            let t491 = t246 * t412;
            let t493 = t249 * t417;
            let t495 = t252 * t422;
            let t497 = t255 * t427;
            let t499 = t258 * t432;
            let t501 = t261 * t437;
            let t503 = t264 * t442;
            let t505 = t267 * t447;
            let t507 = t485 * t400 + t487 * t405 + t489 * t410 + t491 * t415 + t493 * t420 + t495 * t425 + t497 * t430 + t499 * t435 + t501 * t440 + t503 * t445 + t505 * t450 + t236;
            let t509 = t273 * t397;
            let t511 = t276 * t402;
            let t513 = t279 * t407;
            let t515 = t282 * t412;
            let t517 = t285 * t417;
            let t519 = t288 * t422;
            let t521 = t291 * t427;
            let t523 = t294 * t432;
            let t525 = t297 * t437;
            let t527 = t300 * t442;
            let t529 = t303 * t447;
            let t531 = t509 * t400 + t511 * t405 + t513 * t410 + t515 * t415 + t517 * t420 + t519 * t425 + t521 * t430 + t523 * t435 + t525 * t440 + t527 * t445 + t529 * t450 + t272;
            let t533 = t393 * t507 + t457 * t531;
            let t535 = t381 * t482 + t484 * t533;
            let t539 = ((t315).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t324 * t535));
            let tzk0 = t314 + t539;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
