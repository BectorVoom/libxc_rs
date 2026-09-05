//! GGA_X_2D_B88 kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b88.c`
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
pub fn gga_x_2d_b88_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v2rhosigma_0 = V_ZERO;
        let mut acc_v2rhosigma_1 = V_ZERO;
        let mut acc_v2rhosigma_2 = V_ZERO;
        let mut acc_v2rhosigma_3 = V_ZERO;
        let mut acc_v2rhosigma_4 = V_ZERO;
        let mut acc_v2rhosigma_5 = V_ZERO;
        let mut acc_v2sigma2_0 = V_ZERO;
        let mut acc_v2sigma2_1 = V_ZERO;
        let mut acc_v2sigma2_2 = V_ZERO;
        let mut acc_v2sigma2_3 = V_ZERO;
        let mut acc_v2sigma2_4 = V_ZERO;
        let mut acc_v2sigma2_5 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v3rho2sigma_0 = V_ZERO;
        let mut acc_v3rho2sigma_1 = V_ZERO;
        let mut acc_v3rho2sigma_2 = V_ZERO;
        let mut acc_v3rho2sigma_3 = V_ZERO;
        let mut acc_v3rho2sigma_4 = V_ZERO;
        let mut acc_v3rho2sigma_5 = V_ZERO;
        let mut acc_v3rho2sigma_6 = V_ZERO;
        let mut acc_v3rho2sigma_7 = V_ZERO;
        let mut acc_v3rho2sigma_8 = V_ZERO;
        let mut acc_v3rhosigma2_0 = V_ZERO;
        let mut acc_v3rhosigma2_1 = V_ZERO;
        let mut acc_v3rhosigma2_2 = V_ZERO;
        let mut acc_v3rhosigma2_3 = V_ZERO;
        let mut acc_v3rhosigma2_4 = V_ZERO;
        let mut acc_v3rhosigma2_5 = V_ZERO;
        let mut acc_v3rhosigma2_6 = V_ZERO;
        let mut acc_v3rhosigma2_7 = V_ZERO;
        let mut acc_v3rhosigma2_8 = V_ZERO;
        let mut acc_v3rhosigma2_9 = V_ZERO;
        let mut acc_v3rhosigma2_10 = V_ZERO;
        let mut acc_v3rhosigma2_11 = V_ZERO;
        let mut acc_v3sigma3_0 = V_ZERO;
        let mut acc_v3sigma3_1 = V_ZERO;
        let mut acc_v3sigma3_2 = V_ZERO;
        let mut acc_v3sigma3_3 = V_ZERO;
        let mut acc_v3sigma3_4 = V_ZERO;
        let mut acc_v3sigma3_5 = V_ZERO;
        let mut acc_v3sigma3_6 = V_ZERO;
        let mut acc_v3sigma3_7 = V_ZERO;
        let mut acc_v3sigma3_8 = V_ZERO;
        let mut acc_v3sigma3_9 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = ((f64x8::splat(M_PI)).sqrt());
            let t3 = f64x8::splat(1.0) / t2;
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
            let t19 = ((zeta_threshold).sqrt());
            let t20 = t19 * zeta_threshold;
            let t21 = ((t17).sqrt());
            let t22 = t21 * t17;
            let t23 = ((t18).select(t20, t22));
            let t24 = t3 * t23;
            let t25 = f64x8::splat(M_SQRT2);
            let t26 = ((t4).sqrt());
            let t27 = t25 * t26;
            let t28 = v_rho0 * v_rho0;
            let t29 = t28 * v_rho0;
            let t30 = f64x8::splat(1.0) / t29;
            let t31 = v_sigma0 * t30;
            let t32 = ((v_sigma0).sqrt());
            let t33 = ((v_rho0).sqrt());
            let t35 = f64x8::splat(1.0) / t33 / v_rho0;
            let t36 = t32 * t35;
            let t37 = (simd::ln(t36 + ((t36 * t36 + f64x8::splat(1.0)).sqrt())));
            let t40 = f64x8::splat(1.0) + f64x8::splat(0.056) * t36 * t37;
            let t41 = f64x8::splat(1.0) / t40;
            let t44 = f64x8::splat(1.0) + f64x8::splat(0.004652691358626979) * t31 * t41;
            let t45 = t27 * t44;
            let t48 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t24 * t45));
            let t49 = (v_rho1).simd_le(dens_threshold);
            let t50 = -t14;
            let t52 = ((t12).select(t9, (t8).select(t13, t50 * t5)));
            let t53 = f64x8::splat(1.0) + t52;
            let t54 = (t53).simd_le(zeta_threshold);
            let t55 = ((t53).sqrt());
            let t56 = t55 * t53;
            let t57 = ((t54).select(t20, t56));
            let t58 = t3 * t57;
            let t59 = v_rho1 * v_rho1;
            let t60 = t59 * v_rho1;
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = v_sigma2 * t61;
            let t63 = ((v_sigma2).sqrt());
            let t64 = ((v_rho1).sqrt());
            let t66 = f64x8::splat(1.0) / t64 / v_rho1;
            let t67 = t63 * t66;
            let t68 = (simd::ln(t67 + ((t67 * t67 + f64x8::splat(1.0)).sqrt())));
            let t71 = f64x8::splat(1.0) + f64x8::splat(0.056) * t67 * t68;
            let t72 = f64x8::splat(1.0) / t71;
            let t75 = f64x8::splat(1.0) + f64x8::splat(0.004652691358626979) * t62 * t72;
            let t76 = t27 * t75;
            let t79 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t58 * t76));
            let tzk0 = t48 + t79;
            acc_zk = tzk0;
            let t80 = t4 * t4;
            let t81 = f64x8::splat(1.0) / t80;
            let t82 = t14 * t81;
            let t84 = ((t8).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), t5 - t82)));
            let t87 = ((t18).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(2.0) * t21 * t84));
            let t88 = t3 * t87;
            let t92 = t25 / t26;
            let t93 = t92 * t44;
            let t95 = t24 * t93 / f64x8::splat(3.0);
            let t96 = t28 * t28;
            let t97 = f64x8::splat(1.0) / t96;
            let t98 = v_sigma0 * t97;
            let t101 = t40 * t40;
            let t102 = f64x8::splat(1.0) / t101;
            let t104 = f64x8::splat(1.0) / t33 / t28;
            let t108 = t31 + f64x8::splat(1.0);
            let t109 = ((t108).sqrt());
            let t110 = f64x8::splat(1.0) / t109;
            let t113 = -f64x8::splat(0.084) * t32 * t104 * t37 - f64x8::splat(0.084) * t98 * t110;
            let t114 = t102 * t113;
            let t117 = -f64x8::splat(0.01395807407588094) * t98 * t41 - f64x8::splat(0.004652691358626979) * t31 * t114;
            let t118 = t27 * t117;
            let t122 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t88 * t45 - t95 - f64x8::splat(2.0) / f64x8::splat(3.0) * t24 * t118));
            let t123 = t50 * t81;
            let t125 = ((t12).select(f64x8::splat(0.0), (t8).select(f64x8::splat(0.0), -t5 - t123)));
            let t128 = ((t54).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(2.0) * t55 * t125));
            let t129 = t3 * t128;
            let t132 = t92 * t75;
            let t134 = t58 * t132 / f64x8::splat(3.0);
            let t136 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t129 * t76 - t134));
            let tvrho0 = t48 + t79 + t4 * (t122 + t136);
            acc_vrho_0 = tvrho0;
            let t140 = ((t8).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), -t5 - t82)));
            let t143 = ((t18).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(2.0) * t21 * t140));
            let t144 = t3 * t143;
            let t148 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t144 * t45 - t95));
            let t150 = ((t12).select(f64x8::splat(0.0), (t8).select(f64x8::splat(0.0), t5 - t123)));
            let t153 = ((t54).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(2.0) * t55 * t150));
            let t154 = t3 * t153;
            let t157 = t59 * t59;
            let t158 = f64x8::splat(1.0) / t157;
            let t159 = v_sigma2 * t158;
            let t162 = t71 * t71;
            let t163 = f64x8::splat(1.0) / t162;
            let t165 = f64x8::splat(1.0) / t64 / t59;
            let t169 = t62 + f64x8::splat(1.0);
            let t170 = ((t169).sqrt());
            let t171 = f64x8::splat(1.0) / t170;
            let t174 = -f64x8::splat(0.084) * t63 * t165 * t68 - f64x8::splat(0.084) * t159 * t171;
            let t175 = t163 * t174;
            let t178 = -f64x8::splat(0.01395807407588094) * t159 * t72 - f64x8::splat(0.004652691358626979) * t62 * t175;
            let t179 = t27 * t178;
            let t183 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t154 * t76 - t134 - f64x8::splat(2.0) / f64x8::splat(3.0) * t58 * t179));
            let tvrho1 = t48 + t79 + t4 * (t148 + t183);
            acc_vrho_1 = tvrho1;
            let t188 = f64x8::splat(1.0) / t32;
            let t194 = f64x8::splat(0.028) * t188 * t35 * t37 + f64x8::splat(0.028) * t30 * t110;
            let t195 = t102 * t194;
            let t198 = f64x8::splat(0.004652691358626979) * t30 * t41 - f64x8::splat(0.004652691358626979) * t31 * t195;
            let t199 = t27 * t198;
            let t202 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t24 * t199));
            let tvsigma0 = t4 * t202;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t205 = f64x8::splat(1.0) / t63;
            let t211 = f64x8::splat(0.028) * t205 * t66 * t68 + f64x8::splat(0.028) * t61 * t171;
            let t212 = t163 * t211;
            let t215 = f64x8::splat(0.004652691358626979) * t61 * t72 - f64x8::splat(0.004652691358626979) * t62 * t212;
            let t216 = t27 * t215;
            let t219 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t58 * t216));
            let tvsigma2 = t4 * t219;
            acc_vsigma_2 = tvsigma2;
            let t222 = f64x8::splat(1.0) / t21;
            let t223 = t84 * t84;
            let t226 = t80 * t4;
            let t227 = f64x8::splat(1.0) / t226;
            let t228 = t14 * t227;
            let t231 = ((t8).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t81 + f64x8::splat(2.0) * t228)));
            let t235 = ((t18).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t222 * t223 + f64x8::splat(3.0) / f64x8::splat(2.0) * t21 * t231));
            let t236 = t3 * t235;
            let t239 = t88 * t93;
            let t245 = t25 / t26 / t4;
            let t246 = t245 * t44;
            let t248 = t24 * t246 / f64x8::splat(6.0);
            let t249 = t92 * t117;
            let t250 = t24 * t249;
            let t252 = t96 * v_rho0;
            let t253 = f64x8::splat(1.0) / t252;
            let t254 = v_sigma0 * t253;
            let t260 = f64x8::splat(1.0) / t101 / t40;
            let t261 = t113 * t113;
            let t262 = t260 * t261;
            let t266 = f64x8::splat(1.0) / t33 / t29;
            let t272 = v_sigma0 * v_sigma0;
            let t273 = t96 * t96;
            let t274 = f64x8::splat(1.0) / t273;
            let t277 = f64x8::splat(1.0) / t109 / t108;
            let t280 = f64x8::splat(0.21) * t32 * t266 * t37 + f64x8::splat(0.462) * t254 * t110 - f64x8::splat(0.126) * t272 * t274 * t277;
            let t281 = t102 * t280;
            let t284 = f64x8::splat(0.05583229630352376) * t254 * t41 + f64x8::splat(0.02791614815176188) * t98 * t114 + f64x8::splat(0.009305382717253959) * t31 * t262 - f64x8::splat(0.004652691358626979) * t31 * t281;
            let t285 = t27 * t284;
            let t289 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t236 * t45 - f64x8::splat(2.0) / f64x8::splat(3.0) * t239 - f64x8::splat(4.0) / f64x8::splat(3.0) * t88 * t118 + t248 - f64x8::splat(2.0) / f64x8::splat(3.0) * t250 - f64x8::splat(2.0) / f64x8::splat(3.0) * t24 * t285));
            let t290 = f64x8::splat(1.0) / t55;
            let t291 = t125 * t125;
            let t294 = t50 * t227;
            let t297 = ((t12).select(f64x8::splat(0.0), (t8).select(f64x8::splat(0.0), f64x8::splat(2.0) * t81 + f64x8::splat(2.0) * t294)));
            let t301 = ((t54).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t290 * t291 + f64x8::splat(3.0) / f64x8::splat(2.0) * t55 * t297));
            let t302 = t3 * t301;
            let t305 = t129 * t132;
            let t307 = t245 * t75;
            let t309 = t58 * t307 / f64x8::splat(6.0);
            let t311 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t302 * t76 - f64x8::splat(2.0) / f64x8::splat(3.0) * t305 + t309));
            let tv2rho20 = f64x8::splat(2.0) * t122 + f64x8::splat(2.0) * t136 + t4 * (t289 + t311);
            acc_v2rho2_0 = tv2rho20;
            let t314 = t222 * t140;
            let t318 = ((t8).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), f64x8::splat(2.0) * t228)));
            let t322 = ((t18).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t314 * t84 + f64x8::splat(3.0) / f64x8::splat(2.0) * t21 * t318));
            let t323 = t3 * t322;
            let t326 = t144 * t93;
            let t333 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t323 * t45 - t326 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t144 * t118 - t239 / f64x8::splat(3.0) + t248 - t250 / f64x8::splat(3.0)));
            let t334 = t290 * t150;
            let t338 = ((t12).select(f64x8::splat(0.0), (t8).select(f64x8::splat(0.0), f64x8::splat(2.0) * t294)));
            let t342 = ((t54).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t334 * t125 + f64x8::splat(3.0) / f64x8::splat(2.0) * t55 * t338));
            let t343 = t3 * t342;
            let t346 = t154 * t132;
            let t351 = t92 * t178;
            let t352 = t58 * t351;
            let t355 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t343 * t76 - t346 / f64x8::splat(3.0) - t305 / f64x8::splat(3.0) + t309 - f64x8::splat(2.0) / f64x8::splat(3.0) * t129 * t179 - t352 / f64x8::splat(3.0)));
            let tv2rho21 = t122 + t136 + t148 + t183 + t4 * (t333 + t355);
            acc_v2rho2_1 = tv2rho21;
            let t360 = t140 * t140;
            let t365 = ((t8).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), f64x8::splat(2.0) * t81 + f64x8::splat(2.0) * t228)));
            let t369 = ((t18).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t222 * t360 + f64x8::splat(3.0) / f64x8::splat(2.0) * t21 * t365));
            let t370 = t3 * t369;
            let t375 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t370 * t45 - f64x8::splat(2.0) / f64x8::splat(3.0) * t326 + t248));
            let t376 = t150 * t150;
            let t381 = ((t12).select(f64x8::splat(0.0), (t8).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t81 + f64x8::splat(2.0) * t294)));
            let t385 = ((t54).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t290 * t376 + f64x8::splat(3.0) / f64x8::splat(2.0) * t55 * t381));
            let t386 = t3 * t385;
            let t393 = t157 * v_rho1;
            let t394 = f64x8::splat(1.0) / t393;
            let t395 = v_sigma2 * t394;
            let t401 = f64x8::splat(1.0) / t162 / t71;
            let t402 = t174 * t174;
            let t403 = t401 * t402;
            let t407 = f64x8::splat(1.0) / t64 / t60;
            let t413 = v_sigma2 * v_sigma2;
            let t414 = t157 * t157;
            let t415 = f64x8::splat(1.0) / t414;
            let t418 = f64x8::splat(1.0) / t170 / t169;
            let t421 = f64x8::splat(0.21) * t63 * t407 * t68 + f64x8::splat(0.462) * t395 * t171 - f64x8::splat(0.126) * t413 * t415 * t418;
            let t422 = t163 * t421;
            let t425 = f64x8::splat(0.05583229630352376) * t395 * t72 + f64x8::splat(0.02791614815176188) * t159 * t175 + f64x8::splat(0.009305382717253959) * t62 * t403 - f64x8::splat(0.004652691358626979) * t62 * t422;
            let t426 = t27 * t425;
            let t430 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t386 * t76 - f64x8::splat(2.0) / f64x8::splat(3.0) * t346 - f64x8::splat(4.0) / f64x8::splat(3.0) * t154 * t179 + t309 - f64x8::splat(2.0) / f64x8::splat(3.0) * t352 - f64x8::splat(2.0) / f64x8::splat(3.0) * t58 * t426));
            let tv2rho22 = f64x8::splat(2.0) * t148 + f64x8::splat(2.0) * t183 + t4 * (t375 + t430);
            acc_v2rho2_2 = tv2rho22;
            let t435 = t92 * t198;
            let t437 = t24 * t435 / f64x8::splat(3.0);
            let t440 = t30 * t102;
            let t445 = t260 * t194;
            let t446 = t445 * t113;
            let t454 = t96 * t29;
            let t455 = f64x8::splat(1.0) / t454;
            let t456 = t455 * t277;
            let t459 = -f64x8::splat(0.042) * t188 * t104 * t37 - f64x8::splat(0.126) * t97 * t110 + f64x8::splat(0.042) * t456 * v_sigma0;
            let t460 = t102 * t459;
            let t463 = -f64x8::splat(0.01395807407588094) * t97 * t41 - f64x8::splat(0.004652691358626979) * t440 * t113 + f64x8::splat(0.01395807407588094) * t98 * t195 + f64x8::splat(0.009305382717253959) * t31 * t446 - f64x8::splat(0.004652691358626979) * t31 * t460;
            let t464 = t27 * t463;
            let t468 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t88 * t199 - t437 - f64x8::splat(2.0) / f64x8::splat(3.0) * t24 * t464));
            let tv2rhosigma0 = t4 * t468 + t202;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t472 = t92 * t215;
            let t474 = t58 * t472 / f64x8::splat(3.0);
            let t476 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t129 * t216 - t474));
            let tv2rhosigma2 = t4 * t476 + t219;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t481 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t144 * t199 - t437));
            let tv2rhosigma3 = t4 * t481 + t202;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t487 = t61 * t163;
            let t492 = t401 * t211;
            let t493 = t492 * t174;
            let t501 = t157 * t60;
            let t502 = f64x8::splat(1.0) / t501;
            let t503 = t502 * t418;
            let t506 = -f64x8::splat(0.042) * t205 * t165 * t68 - f64x8::splat(0.126) * t158 * t171 + f64x8::splat(0.042) * t503 * v_sigma2;
            let t507 = t163 * t506;
            let t510 = -f64x8::splat(0.01395807407588094) * t158 * t72 - f64x8::splat(0.004652691358626979) * t487 * t174 + f64x8::splat(0.01395807407588094) * t159 * t212 + f64x8::splat(0.009305382717253959) * t62 * t493 - f64x8::splat(0.004652691358626979) * t62 * t507;
            let t511 = t27 * t510;
            let t515 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t154 * t216 - t474 - f64x8::splat(2.0) / f64x8::splat(3.0) * t58 * t511));
            let tv2rhosigma5 = t4 * t515 + t219;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t519 = t194 * t194;
            let t520 = t260 * t519;
            let t524 = f64x8::splat(1.0) / t32 / v_sigma0;
            let t528 = f64x8::splat(1.0) / v_sigma0;
            let t532 = t96 * t28;
            let t533 = f64x8::splat(1.0) / t532;
            let t536 = -f64x8::splat(0.014) * t524 * t35 * t37 + f64x8::splat(0.014) * t528 * t30 * t110 - f64x8::splat(0.014) * t533 * t277;
            let t537 = t102 * t536;
            let t540 = -f64x8::splat(0.009305382717253959) * t440 * t194 + f64x8::splat(0.009305382717253959) * t31 * t520 - f64x8::splat(0.004652691358626979) * t31 * t537;
            let t541 = t27 * t540;
            let t544 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t24 * t541));
            let tv2sigma20 = t4 * t544;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t547 = t211 * t211;
            let t548 = t401 * t547;
            let t552 = f64x8::splat(1.0) / t63 / v_sigma2;
            let t556 = f64x8::splat(1.0) / v_sigma2;
            let t560 = t157 * t59;
            let t561 = f64x8::splat(1.0) / t560;
            let t564 = -f64x8::splat(0.014) * t552 * t66 * t68 + f64x8::splat(0.014) * t556 * t61 * t171 - f64x8::splat(0.014) * t561 * t418;
            let t565 = t163 * t564;
            let t568 = -f64x8::splat(0.009305382717253959) * t487 * t211 + f64x8::splat(0.009305382717253959) * t62 * t548 - f64x8::splat(0.004652691358626979) * t62 * t565;
            let t569 = t27 * t568;
            let t572 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t58 * t569));
            let tv2sigma25 = t4 * t572;
            acc_v2sigma2_5 = tv2sigma25;
            let t575 = f64x8::splat(1.0) / t22;
            let t576 = t223 * t84;
            let t579 = t222 * t84;
            let t582 = t80 * t80;
            let t583 = f64x8::splat(1.0) / t582;
            let t584 = t14 * t583;
            let t587 = ((t8).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), f64x8::splat(6.0) * t227 - f64x8::splat(6.0) * t584)));
            let t591 = ((t18).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t575 * t576 + f64x8::splat(9.0) / f64x8::splat(4.0) * t579 * t231 + f64x8::splat(3.0) / f64x8::splat(2.0) * t21 * t587));
            let t592 = t3 * t591;
            let t595 = t236 * t93;
            let t598 = t88 * t246;
            let t600 = t88 * t249;
            let t606 = t25 / t26 / t80;
            let t607 = t606 * t44;
            let t609 = t24 * t607 / f64x8::splat(4.0);
            let t610 = t245 * t117;
            let t611 = t24 * t610;
            let t613 = t92 * t284;
            let t614 = t24 * t613;
            let t615 = v_sigma0 * t533;
            let t624 = t101 * t101;
            let t625 = f64x8::splat(1.0) / t624;
            let t626 = t261 * t113;
            let t627 = t625 * t626;
            let t630 = t260 * t113;
            let t631 = t630 * t280;
            let t635 = f64x8::splat(1.0) / t33 / t96;
            let t642 = f64x8::splat(1.0) / t273 / v_rho0;
            let t646 = t272 * v_sigma0;
            let t648 = f64x8::splat(1.0) / t273 / t96;
            let t650 = t108 * t108;
            let t652 = f64x8::splat(1.0) / t109 / t650;
            let t655 = -f64x8::splat(0.735) * t32 * t635 * t37 - f64x8::splat(2.625) * t615 * t110 + f64x8::splat(1.701) * t272 * t642 * t277 - f64x8::splat(0.567) * t646 * t648 * t652;
            let t656 = t102 * t655;
            let t659 = -f64x8::splat(0.27916148151761877) * t615 * t41 - f64x8::splat(0.16749688891057127) * t254 * t114 - f64x8::splat(0.08374844445528563) * t98 * t262 + f64x8::splat(0.04187422222764282) * t98 * t281 - f64x8::splat(0.02791614815176188) * t31 * t627 + f64x8::splat(0.02791614815176188) * t31 * t631 - f64x8::splat(0.004652691358626979) * t31 * t656;
            let t660 = t27 * t659;
            let t664 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t592 * t45 - t595 - f64x8::splat(2.0) * t236 * t118 + t598 / f64x8::splat(2.0) - f64x8::splat(2.0) * t600 - f64x8::splat(2.0) * t88 * t285 - t609 + t611 / f64x8::splat(2.0) - t614 - f64x8::splat(2.0) / f64x8::splat(3.0) * t24 * t660));
            let t665 = f64x8::splat(1.0) / t56;
            let t666 = t291 * t125;
            let t669 = t290 * t125;
            let t672 = t50 * t583;
            let t675 = ((t12).select(f64x8::splat(0.0), (t8).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t227 - f64x8::splat(6.0) * t672)));
            let t679 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t665 * t666 + f64x8::splat(9.0) / f64x8::splat(4.0) * t669 * t297 + f64x8::splat(3.0) / f64x8::splat(2.0) * t55 * t675));
            let t680 = t3 * t679;
            let t683 = t302 * t132;
            let t684 = t129 * t307;
            let t686 = t606 * t75;
            let t688 = t58 * t686 / f64x8::splat(4.0);
            let t690 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t680 * t76 - t683 + t684 / f64x8::splat(2.0) - t688));
            let tv3rho30 = f64x8::splat(3.0) * t289 + f64x8::splat(3.0) * t311 + t4 * (t664 + t690);
            acc_v3rho3_0 = tv3rho30;
            let t693 = f64x8::splat(2.0) * t333;
            let t694 = f64x8::splat(2.0) * t355;
            let t695 = t575 * t140;
            let t698 = t222 * t318;
            let t703 = f64x8::splat(2.0) * t227;
            let t704 = f64x8::splat(6.0) * t584;
            let t706 = ((t8).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), t703 - t704)));
            let t710 = ((t18).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t695 * t223 + f64x8::splat(3.0) / f64x8::splat(2.0) * t698 * t84 + f64x8::splat(3.0) / f64x8::splat(4.0) * t314 * t231 + f64x8::splat(3.0) / f64x8::splat(2.0) * t21 * t706));
            let t711 = t3 * t710;
            let t715 = f64x8::splat(2.0) / f64x8::splat(3.0) * t323 * t93;
            let t718 = t144 * t246;
            let t721 = f64x8::splat(2.0) / f64x8::splat(3.0) * t144 * t249;
            let t729 = -f64x8::splat(2.0) / f64x8::splat(3.0) * t711 * t45 - t715 - f64x8::splat(4.0) / f64x8::splat(3.0) * t323 * t118 + t718 / f64x8::splat(6.0) - t721 - f64x8::splat(2.0) / f64x8::splat(3.0) * t144 * t285 - t595 / f64x8::splat(3.0) + t598 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t600 - t609 + t611 / f64x8::splat(3.0) - t614 / f64x8::splat(3.0);
            let t730 = ((t1).select(f64x8::splat(0.0), t729));
            let t731 = t665 * t150;
            let t734 = t290 * t338;
            let t739 = f64x8::splat(6.0) * t672;
            let t741 = ((t12).select(f64x8::splat(0.0), (t8).select(f64x8::splat(0.0), -t703 - t739)));
            let t745 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t731 * t291 + f64x8::splat(3.0) / f64x8::splat(2.0) * t734 * t125 + f64x8::splat(3.0) / f64x8::splat(4.0) * t334 * t297 + f64x8::splat(3.0) / f64x8::splat(2.0) * t55 * t741));
            let t746 = t3 * t745;
            let t750 = f64x8::splat(2.0) / f64x8::splat(3.0) * t343 * t132;
            let t751 = t154 * t307;
            let t758 = f64x8::splat(2.0) / f64x8::splat(3.0) * t129 * t351;
            let t759 = t245 * t178;
            let t760 = t58 * t759;
            let t763 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t746 * t76 - t750 + t751 / f64x8::splat(6.0) - t683 / f64x8::splat(3.0) + t684 / f64x8::splat(3.0) - t688 - f64x8::splat(2.0) / f64x8::splat(3.0) * t302 * t179 - t758 + t760 / f64x8::splat(6.0)));
            let tv3rho31 = t289 + t311 + t693 + t694 + t4 * (t730 + t763);
            acc_v3rho3_1 = tv3rho31;
            let t766 = t575 * t360;
            let t771 = t222 * t365;
            let t775 = ((t8).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), -t703 - t704)));
            let t779 = ((t18).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t766 * t84 + f64x8::splat(3.0) / f64x8::splat(2.0) * t314 * t318 + f64x8::splat(3.0) / f64x8::splat(4.0) * t771 * t84 + f64x8::splat(3.0) / f64x8::splat(2.0) * t21 * t775));
            let t780 = t3 * t779;
            let t783 = t370 * t93;
            let t791 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t780 * t45 - t783 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t370 * t118 - t715 + t718 / f64x8::splat(3.0) - t721 + t598 / f64x8::splat(6.0) - t609 + t611 / f64x8::splat(6.0)));
            let t792 = t665 * t376;
            let t797 = t290 * t381;
            let t801 = ((t12).select(f64x8::splat(0.0), (t8).select(f64x8::splat(0.0), t703 - t739)));
            let t805 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t792 * t125 + f64x8::splat(3.0) / f64x8::splat(2.0) * t334 * t338 + f64x8::splat(3.0) / f64x8::splat(4.0) * t797 * t125 + f64x8::splat(3.0) / f64x8::splat(2.0) * t55 * t801));
            let t806 = t3 * t805;
            let t809 = t386 * t132;
            let t814 = t154 * t351;
            let t820 = t92 * t425;
            let t821 = t58 * t820;
            let t823 = -f64x8::splat(2.0) / f64x8::splat(3.0) * t806 * t76 - t809 / f64x8::splat(3.0) - t750 + t751 / f64x8::splat(3.0) - f64x8::splat(4.0) / f64x8::splat(3.0) * t343 * t179 - f64x8::splat(2.0) / f64x8::splat(3.0) * t814 + t684 / f64x8::splat(6.0) - t688 - t758 + t760 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t129 * t426 - t821 / f64x8::splat(3.0);
            let t824 = ((t49).select(f64x8::splat(0.0), t823));
            let tv3rho32 = t693 + t694 + t375 + t430 + t4 * (t791 + t824);
            acc_v3rho3_2 = tv3rho32;
            let t829 = t360 * t140;
            let t836 = ((t8).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t227 - f64x8::splat(6.0) * t584)));
            let t840 = ((t18).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t575 * t829 + f64x8::splat(9.0) / f64x8::splat(4.0) * t314 * t365 + f64x8::splat(3.0) / f64x8::splat(2.0) * t21 * t836));
            let t841 = t3 * t840;
            let t846 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t841 * t45 - t783 + t718 / f64x8::splat(2.0) - t609));
            let t847 = t376 * t150;
            let t854 = ((t12).select(f64x8::splat(0.0), (t8).select(f64x8::splat(0.0), f64x8::splat(6.0) * t227 - f64x8::splat(6.0) * t672)));
            let t858 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t665 * t847 + f64x8::splat(9.0) / f64x8::splat(4.0) * t334 * t381 + f64x8::splat(3.0) / f64x8::splat(2.0) * t55 * t854));
            let t859 = t3 * t858;
            let t869 = v_sigma2 * t561;
            let t878 = t162 * t162;
            let t879 = f64x8::splat(1.0) / t878;
            let t880 = t402 * t174;
            let t881 = t879 * t880;
            let t884 = t401 * t174;
            let t885 = t884 * t421;
            let t889 = f64x8::splat(1.0) / t64 / t157;
            let t896 = f64x8::splat(1.0) / t414 / v_rho1;
            let t900 = t413 * v_sigma2;
            let t902 = f64x8::splat(1.0) / t414 / t157;
            let t904 = t169 * t169;
            let t906 = f64x8::splat(1.0) / t170 / t904;
            let t909 = -f64x8::splat(0.735) * t63 * t889 * t68 - f64x8::splat(2.625) * t869 * t171 + f64x8::splat(1.701) * t413 * t896 * t418 - f64x8::splat(0.567) * t900 * t902 * t906;
            let t910 = t163 * t909;
            let t913 = -f64x8::splat(0.27916148151761877) * t869 * t72 - f64x8::splat(0.16749688891057127) * t395 * t175 - f64x8::splat(0.08374844445528563) * t159 * t403 + f64x8::splat(0.04187422222764282) * t159 * t422 - f64x8::splat(0.02791614815176188) * t62 * t881 + f64x8::splat(0.02791614815176188) * t62 * t885 - f64x8::splat(0.004652691358626979) * t62 * t910;
            let t914 = t27 * t913;
            let t918 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t859 * t76 - t809 - f64x8::splat(2.0) * t386 * t179 + t751 / f64x8::splat(2.0) - f64x8::splat(2.0) * t814 - f64x8::splat(2.0) * t154 * t426 - t688 + t760 / f64x8::splat(2.0) - t821 - f64x8::splat(2.0) / f64x8::splat(3.0) * t58 * t914));
            let tv3rho33 = f64x8::splat(3.0) * t375 + f64x8::splat(3.0) * t430 + t4 * (t846 + t918);
            acc_v3rho3_3 = tv3rho33;
            let t924 = t88 * t435;
            let t928 = t245 * t198;
            let t930 = t24 * t928 / f64x8::splat(6.0);
            let t931 = t92 * t463;
            let t932 = t24 * t931;
            let t936 = t97 * t102;
            let t939 = t30 * t260;
            let t951 = t625 * t194 * t261;
            let t954 = t260 * t459;
            let t955 = t954 * t113;
            let t958 = t445 * t280;
            let t966 = t274 * t277;
            let t971 = f64x8::splat(1.0) / t273 / t29 * t652;
            let t974 = f64x8::splat(0.105) * t188 * t266 * t37 + f64x8::splat(0.567) * t253 * t110 - f64x8::splat(0.483) * t966 * v_sigma0 + f64x8::splat(0.189) * t971 * t272;
            let t975 = t102 * t974;
            let t978 = f64x8::splat(0.05583229630352376) * t253 * t41 + f64x8::splat(0.02791614815176188) * t936 * t113 + f64x8::splat(0.009305382717253959) * t939 * t261 - f64x8::splat(0.004652691358626979) * t440 * t280 - f64x8::splat(0.05583229630352376) * t254 * t195 - f64x8::splat(0.05583229630352376) * t98 * t446 + f64x8::splat(0.02791614815176188) * t98 * t460 - f64x8::splat(0.02791614815176188) * t31 * t951 + f64x8::splat(0.018610765434507917) * t31 * t955 + f64x8::splat(0.009305382717253959) * t31 * t958 - f64x8::splat(0.004652691358626979) * t31 * t975;
            let t979 = t27 * t978;
            let t983 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t236 * t199 - f64x8::splat(2.0) / f64x8::splat(3.0) * t924 - f64x8::splat(4.0) / f64x8::splat(3.0) * t88 * t464 + t930 - f64x8::splat(2.0) / f64x8::splat(3.0) * t932 - f64x8::splat(2.0) / f64x8::splat(3.0) * t24 * t979));
            let tv3rho2sigma0 = t4 * t983 + f64x8::splat(2.0) * t468;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t988 = t129 * t472;
            let t990 = t245 * t215;
            let t992 = t58 * t990 / f64x8::splat(6.0);
            let t994 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t302 * t216 - f64x8::splat(2.0) / f64x8::splat(3.0) * t988 + t992));
            let tv3rho2sigma2 = t4 * t994 + f64x8::splat(2.0) * t476;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t998 = t144 * t435;
            let t1005 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t323 * t199 - t998 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t144 * t464 - t924 / f64x8::splat(3.0) + t930 - t932 / f64x8::splat(3.0)));
            let tv3rho2sigma3 = t4 * t1005 + t468 + t481;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1009 = t154 * t472;
            let t1014 = t92 * t510;
            let t1015 = t58 * t1014;
            let t1018 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t343 * t216 - t1009 / f64x8::splat(3.0) - t988 / f64x8::splat(3.0) + t992 - f64x8::splat(2.0) / f64x8::splat(3.0) * t129 * t511 - t1015 / f64x8::splat(3.0)));
            let tv3rho2sigma5 = t4 * t1018 + t476 + t515;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1025 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t370 * t199 - f64x8::splat(2.0) / f64x8::splat(3.0) * t998 + t930));
            let tv3rho2sigma6 = t4 * t1025 + f64x8::splat(2.0) * t481;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1036 = t158 * t163;
            let t1039 = t61 * t401;
            let t1051 = t879 * t211 * t402;
            let t1054 = t401 * t506;
            let t1055 = t1054 * t174;
            let t1058 = t492 * t421;
            let t1066 = t415 * t418;
            let t1071 = f64x8::splat(1.0) / t414 / t60 * t906;
            let t1074 = f64x8::splat(0.105) * t205 * t407 * t68 + f64x8::splat(0.567) * t394 * t171 - f64x8::splat(0.483) * t1066 * v_sigma2 + f64x8::splat(0.189) * t1071 * t413;
            let t1075 = t163 * t1074;
            let t1078 = f64x8::splat(0.05583229630352376) * t394 * t72 + f64x8::splat(0.02791614815176188) * t1036 * t174 + f64x8::splat(0.009305382717253959) * t1039 * t402 - f64x8::splat(0.004652691358626979) * t487 * t421 - f64x8::splat(0.05583229630352376) * t395 * t212 - f64x8::splat(0.05583229630352376) * t159 * t493 + f64x8::splat(0.02791614815176188) * t159 * t507 - f64x8::splat(0.02791614815176188) * t62 * t1051 + f64x8::splat(0.018610765434507917) * t62 * t1055 + f64x8::splat(0.009305382717253959) * t62 * t1058 - f64x8::splat(0.004652691358626979) * t62 * t1075;
            let t1079 = t27 * t1078;
            let t1083 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t386 * t216 - f64x8::splat(2.0) / f64x8::splat(3.0) * t1009 - f64x8::splat(4.0) / f64x8::splat(3.0) * t154 * t511 + t992 - f64x8::splat(2.0) / f64x8::splat(3.0) * t1015 - f64x8::splat(2.0) / f64x8::splat(3.0) * t58 * t1079));
            let tv3rho2sigma8 = t4 * t1083 + f64x8::splat(2.0) * t515;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1087 = t92 * t540;
            let t1089 = t24 * t1087 / f64x8::splat(3.0);
            let t1092 = t194 * t113;
            let t1099 = t625 * t519;
            let t1100 = t1099 * t113;
            let t1103 = t445 * t459;
            let t1108 = t260 * t536;
            let t1109 = t1108 * t113;
            let t1120 = f64x8::splat(1.0) / t273 / t28;
            let t1121 = t1120 * t652;
            let t1124 = f64x8::splat(0.021) * t524 * t104 * t37 - f64x8::splat(0.021) * t528 * t97 * t110 + f64x8::splat(0.105) * t456 - f64x8::splat(0.063) * t1121 * v_sigma0;
            let t1125 = t102 * t1124;
            let t1128 = f64x8::splat(0.02791614815176188) * t936 * t194 + f64x8::splat(0.018610765434507917) * t939 * t1092 - f64x8::splat(0.009305382717253959) * t440 * t459 - f64x8::splat(0.02791614815176188) * t98 * t520 - f64x8::splat(0.02791614815176188) * t31 * t1100 + f64x8::splat(0.018610765434507917) * t31 * t1103 + f64x8::splat(0.01395807407588094) * t98 * t537 + f64x8::splat(0.009305382717253959) * t31 * t1109 - f64x8::splat(0.004652691358626979) * t31 * t1125;
            let t1129 = t27 * t1128;
            let t1133 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t88 * t541 - t1089 - f64x8::splat(2.0) / f64x8::splat(3.0) * t24 * t1129));
            let tv3rhosigma20 = t4 * t1133 + t544;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1137 = t92 * t568;
            let t1139 = t58 * t1137 / f64x8::splat(3.0);
            let t1141 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t129 * t569 - t1139));
            let tv3rhosigma25 = t4 * t1141 + t572;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1146 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t144 * t541 - t1089));
            let tv3rhosigma26 = t4 * t1146 + t544;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1152 = t211 * t174;
            let t1159 = t879 * t547;
            let t1160 = t1159 * t174;
            let t1163 = t492 * t506;
            let t1168 = t401 * t564;
            let t1169 = t1168 * t174;
            let t1180 = f64x8::splat(1.0) / t414 / t59;
            let t1181 = t1180 * t906;
            let t1184 = f64x8::splat(0.021) * t552 * t165 * t68 - f64x8::splat(0.021) * t556 * t158 * t171 + f64x8::splat(0.105) * t503 - f64x8::splat(0.063) * t1181 * v_sigma2;
            let t1185 = t163 * t1184;
            let t1188 = f64x8::splat(0.02791614815176188) * t1036 * t211 + f64x8::splat(0.018610765434507917) * t1039 * t1152 - f64x8::splat(0.009305382717253959) * t487 * t506 - f64x8::splat(0.02791614815176188) * t159 * t548 - f64x8::splat(0.02791614815176188) * t62 * t1160 + f64x8::splat(0.018610765434507917) * t62 * t1163 + f64x8::splat(0.01395807407588094) * t159 * t565 + f64x8::splat(0.009305382717253959) * t62 * t1169 - f64x8::splat(0.004652691358626979) * t62 * t1185;
            let t1189 = t27 * t1188;
            let t1193 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t154 * t569 - t1139 - f64x8::splat(2.0) / f64x8::splat(3.0) * t58 * t1189));
            let tv3rhosigma211 = t4 * t1193 + t572;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1199 = t519 * t194;
            let t1200 = t625 * t1199;
            let t1203 = t445 * t536;
            let t1207 = f64x8::splat(1.0) / t32 / t272;
            let t1211 = f64x8::splat(1.0) / t272;
            let t1220 = f64x8::splat(0.021) * t1207 * t35 * t37 - f64x8::splat(0.021) * t1211 * t30 * t110 - f64x8::splat(0.007) * t528 * t533 * t277 + f64x8::splat(0.021) * t642 * t652;
            let t1221 = t102 * t1220;
            let t1224 = f64x8::splat(0.02791614815176188) * t939 * t519 - f64x8::splat(0.01395807407588094) * t440 * t536 - f64x8::splat(0.02791614815176188) * t31 * t1200 + f64x8::splat(0.02791614815176188) * t31 * t1203 - f64x8::splat(0.004652691358626979) * t31 * t1221;
            let t1225 = t27 * t1224;
            let t1228 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t24 * t1225));
            let tv3sigma30 = t4 * t1228;
            acc_v3sigma3_0 = tv3sigma30;
            let tv3sigma31 = f64x8::splat(0.0);
            acc_v3sigma3_1 = tv3sigma31;
            let tv3sigma32 = f64x8::splat(0.0);
            acc_v3sigma3_2 = tv3sigma32;
            let tv3sigma33 = f64x8::splat(0.0);
            acc_v3sigma3_3 = tv3sigma33;
            let tv3sigma34 = f64x8::splat(0.0);
            acc_v3sigma3_4 = tv3sigma34;
            let tv3sigma35 = f64x8::splat(0.0);
            acc_v3sigma3_5 = tv3sigma35;
            let tv3sigma36 = f64x8::splat(0.0);
            acc_v3sigma3_6 = tv3sigma36;
            let tv3sigma37 = f64x8::splat(0.0);
            acc_v3sigma3_7 = tv3sigma37;
            let tv3sigma38 = f64x8::splat(0.0);
            acc_v3sigma3_8 = tv3sigma38;
            let t1233 = t547 * t211;
            let t1234 = t879 * t1233;
            let t1237 = t492 * t564;
            let t1241 = f64x8::splat(1.0) / t63 / t413;
            let t1245 = f64x8::splat(1.0) / t413;
            let t1254 = f64x8::splat(0.021) * t1241 * t66 * t68 - f64x8::splat(0.021) * t1245 * t61 * t171 - f64x8::splat(0.007) * t556 * t561 * t418 + f64x8::splat(0.021) * t896 * t906;
            let t1255 = t163 * t1254;
            let t1258 = f64x8::splat(0.02791614815176188) * t1039 * t547 - f64x8::splat(0.01395807407588094) * t487 * t564 - f64x8::splat(0.02791614815176188) * t62 * t1234 + f64x8::splat(0.02791614815176188) * t62 * t1237 - f64x8::splat(0.004652691358626979) * t62 * t1255;
            let t1259 = t27 * t1258;
            let t1262 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t58 * t1259));
            let tv3sigma39 = t4 * t1262;
            acc_v3sigma3_9 = tv3sigma39;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v2rhosigma, ip, m, 6, 0, acc_v2rhosigma_0);
        store_strided(v2rhosigma, ip, m, 6, 1, acc_v2rhosigma_1);
        store_strided(v2rhosigma, ip, m, 6, 2, acc_v2rhosigma_2);
        store_strided(v2rhosigma, ip, m, 6, 3, acc_v2rhosigma_3);
        store_strided(v2rhosigma, ip, m, 6, 4, acc_v2rhosigma_4);
        store_strided(v2rhosigma, ip, m, 6, 5, acc_v2rhosigma_5);
        store_strided(v2sigma2, ip, m, 6, 0, acc_v2sigma2_0);
        store_strided(v2sigma2, ip, m, 6, 1, acc_v2sigma2_1);
        store_strided(v2sigma2, ip, m, 6, 2, acc_v2sigma2_2);
        store_strided(v2sigma2, ip, m, 6, 3, acc_v2sigma2_3);
        store_strided(v2sigma2, ip, m, 6, 4, acc_v2sigma2_4);
        store_strided(v2sigma2, ip, m, 6, 5, acc_v2sigma2_5);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v3rho2sigma, ip, m, 9, 0, acc_v3rho2sigma_0);
        store_strided(v3rho2sigma, ip, m, 9, 1, acc_v3rho2sigma_1);
        store_strided(v3rho2sigma, ip, m, 9, 2, acc_v3rho2sigma_2);
        store_strided(v3rho2sigma, ip, m, 9, 3, acc_v3rho2sigma_3);
        store_strided(v3rho2sigma, ip, m, 9, 4, acc_v3rho2sigma_4);
        store_strided(v3rho2sigma, ip, m, 9, 5, acc_v3rho2sigma_5);
        store_strided(v3rho2sigma, ip, m, 9, 6, acc_v3rho2sigma_6);
        store_strided(v3rho2sigma, ip, m, 9, 7, acc_v3rho2sigma_7);
        store_strided(v3rho2sigma, ip, m, 9, 8, acc_v3rho2sigma_8);
        store_strided(v3rhosigma2, ip, m, 12, 0, acc_v3rhosigma2_0);
        store_strided(v3rhosigma2, ip, m, 12, 1, acc_v3rhosigma2_1);
        store_strided(v3rhosigma2, ip, m, 12, 2, acc_v3rhosigma2_2);
        store_strided(v3rhosigma2, ip, m, 12, 3, acc_v3rhosigma2_3);
        store_strided(v3rhosigma2, ip, m, 12, 4, acc_v3rhosigma2_4);
        store_strided(v3rhosigma2, ip, m, 12, 5, acc_v3rhosigma2_5);
        store_strided(v3rhosigma2, ip, m, 12, 6, acc_v3rhosigma2_6);
        store_strided(v3rhosigma2, ip, m, 12, 7, acc_v3rhosigma2_7);
        store_strided(v3rhosigma2, ip, m, 12, 8, acc_v3rhosigma2_8);
        store_strided(v3rhosigma2, ip, m, 12, 9, acc_v3rhosigma2_9);
        store_strided(v3rhosigma2, ip, m, 12, 10, acc_v3rhosigma2_10);
        store_strided(v3rhosigma2, ip, m, 12, 11, acc_v3rhosigma2_11);
        store_strided(v3sigma3, ip, m, 10, 0, acc_v3sigma3_0);
        store_strided(v3sigma3, ip, m, 10, 1, acc_v3sigma3_1);
        store_strided(v3sigma3, ip, m, 10, 2, acc_v3sigma3_2);
        store_strided(v3sigma3, ip, m, 10, 3, acc_v3sigma3_3);
        store_strided(v3sigma3, ip, m, 10, 4, acc_v3sigma3_4);
        store_strided(v3sigma3, ip, m, 10, 5, acc_v3sigma3_5);
        store_strided(v3sigma3, ip, m, 10, 6, acc_v3sigma3_6);
        store_strided(v3sigma3, ip, m, 10, 7, acc_v3sigma3_7);
        store_strided(v3sigma3, ip, m, 10, 8, acc_v3sigma3_8);
        store_strided(v3sigma3, ip, m, 10, 9, acc_v3sigma3_9);
        ip += 8;
    }
}
