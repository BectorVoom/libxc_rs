//! GGA_X_HCTH_A kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_hcth_a.c`
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
pub fn gga_x_hcth_a_kxc_pol(
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
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = t2 * t2;
            let t30 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t32 = t28 / t30;
            let t33 = f64x8::splat(M_CBRT4);
            let t34 = t32 * t33;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t40 = v_sigma0 * t39;
            let t41 = ((v_sigma0).sqrt());
            let t43 = f64x8::splat(1.0) / t36 / v_rho0;
            let t44 = t41 * t43;
            let t45 = (simd::ln(t44 + ((t44 * t44 + f64x8::splat(1.0)).sqrt())));
            let t48 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t44 * t45;
            let t51 = t48 * t48;
            let t52 = f64x8::splat(1.0) / t51;
            let t54 = -f64x8::splat(2.51173) / t48 + f64x8::splat(3.7198333333333333) * t52;
            let t58 = f64x8::splat(1.09878) + f64x8::splat(0.0009333333333333333) * t34 * t40 * t54;
            let t62 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t58));
            let t63 = (v_rho1).simd_le(dens_threshold);
            let t64 = -t16;
            let t66 = ((t14).select(t11, (t10).select(t15, t64 * t7)));
            let t67 = f64x8::splat(1.0) + t66;
            let t68 = (t67).simd_le(zeta_threshold);
            let t69 = (simd::cbrt(t67));
            let t71 = ((t68).select(t22, t69 * t67));
            let t72 = t71 * t26;
            let t73 = v_rho1 * v_rho1;
            let t74 = (simd::cbrt(v_rho1));
            let t75 = t74 * t74;
            let t77 = f64x8::splat(1.0) / t75 / t73;
            let t78 = v_sigma2 * t77;
            let t79 = ((v_sigma2).sqrt());
            let t81 = f64x8::splat(1.0) / t74 / v_rho1;
            let t82 = t79 * t81;
            let t83 = (simd::ln(t82 + ((t82 * t82 + f64x8::splat(1.0)).sqrt())));
            let t86 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t82 * t83;
            let t89 = t86 * t86;
            let t90 = f64x8::splat(1.0) / t89;
            let t92 = -f64x8::splat(2.51173) / t86 + f64x8::splat(3.7198333333333333) * t90;
            let t96 = f64x8::splat(1.09878) + f64x8::splat(0.0009333333333333333) * t34 * t78 * t92;
            let t100 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t72 * t96));
            let tzk0 = t62 + t100;
            acc_zk = tzk0;
            let t101 = t6 * t6;
            let t102 = f64x8::splat(1.0) / t101;
            let t103 = t16 * t102;
            let t105 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t103)));
            let t108 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t105));
            let t109 = t108 * t26;
            let t113 = t26 * t26;
            let t114 = f64x8::splat(1.0) / t113;
            let t115 = t25 * t114;
            let t118 = t5 * t115 * t58 / f64x8::splat(8.0);
            let t119 = t35 * v_rho0;
            let t121 = f64x8::splat(1.0) / t37 / t119;
            let t122 = v_sigma0 * t121;
            let t127 = f64x8::splat(1.0) / t36 / t35;
            let t131 = t40 + f64x8::splat(1.0);
            let t132 = ((t131).sqrt());
            let t133 = f64x8::splat(1.0) / t132;
            let t136 = -f64x8::splat(0.0336) * t41 * t127 * t45 - f64x8::splat(0.0336) * t122 * t133;
            let t140 = f64x8::splat(1.0) / t51 / t48;
            let t141 = t140 * t136;
            let t143 = f64x8::splat(2.51173) * t52 * t136 - f64x8::splat(7.439666666666667) * t141;
            let t147 = -f64x8::splat(0.002488888888888889) * t34 * t122 * t54 + f64x8::splat(0.0009333333333333333) * t34 * t40 * t143;
            let t152 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t109 * t58 - t118 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t147));
            let t153 = t64 * t102;
            let t155 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t153)));
            let t158 = ((t68).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t69 * t155));
            let t159 = t158 * t26;
            let t163 = t71 * t114;
            let t166 = t5 * t163 * t96 / f64x8::splat(8.0);
            let t168 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t159 * t96 - t166));
            let tvrho0 = t62 + t100 + t6 * (t152 + t168);
            acc_vrho_0 = tvrho0;
            let t172 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t103)));
            let t175 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t172));
            let t176 = t175 * t26;
            let t181 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t176 * t58 - t118));
            let t183 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t153)));
            let t186 = ((t68).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t69 * t183));
            let t187 = t186 * t26;
            let t191 = t73 * v_rho1;
            let t193 = f64x8::splat(1.0) / t75 / t191;
            let t194 = v_sigma2 * t193;
            let t199 = f64x8::splat(1.0) / t74 / t73;
            let t203 = t78 + f64x8::splat(1.0);
            let t204 = ((t203).sqrt());
            let t205 = f64x8::splat(1.0) / t204;
            let t208 = -f64x8::splat(0.0336) * t79 * t199 * t83 - f64x8::splat(0.0336) * t194 * t205;
            let t212 = f64x8::splat(1.0) / t89 / t86;
            let t213 = t212 * t208;
            let t215 = f64x8::splat(2.51173) * t90 * t208 - f64x8::splat(7.439666666666667) * t213;
            let t219 = -f64x8::splat(0.002488888888888889) * t34 * t194 * t92 + f64x8::splat(0.0009333333333333333) * t34 * t78 * t215;
            let t224 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t187 * t96 - t166 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t72 * t219));
            let tvrho1 = t62 + t100 + t6 * (t181 + t224);
            acc_vrho_1 = tvrho1;
            let t227 = t33 * t39;
            let t231 = f64x8::splat(1.0) / t41;
            let t237 = f64x8::splat(0.0126) * t231 * t43 * t45 + f64x8::splat(0.0126) * t39 * t133;
            let t240 = t140 * t237;
            let t242 = f64x8::splat(2.51173) * t52 * t237 - f64x8::splat(7.439666666666667) * t240;
            let t246 = f64x8::splat(0.0009333333333333333) * t32 * t227 * t54 + f64x8::splat(0.0009333333333333333) * t34 * t40 * t242;
            let t250 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t246));
            let tvsigma0 = t6 * t250;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t251 = t33 * t77;
            let t255 = f64x8::splat(1.0) / t79;
            let t261 = f64x8::splat(0.0126) * t255 * t81 * t83 + f64x8::splat(0.0126) * t77 * t205;
            let t264 = t212 * t261;
            let t266 = f64x8::splat(2.51173) * t90 * t261 - f64x8::splat(7.439666666666667) * t264;
            let t270 = f64x8::splat(0.0009333333333333333) * t32 * t251 * t92 + f64x8::splat(0.0009333333333333333) * t34 * t78 * t266;
            let t274 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t72 * t270));
            let tvsigma2 = t6 * t274;
            acc_vsigma_2 = tvsigma2;
            let t277 = t23 * t23;
            let t278 = f64x8::splat(1.0) / t277;
            let t279 = t105 * t105;
            let t282 = t101 * t6;
            let t283 = f64x8::splat(1.0) / t282;
            let t284 = t16 * t283;
            let t287 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t102 + f64x8::splat(2.0) * t284)));
            let t291 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t278 * t279 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t287));
            let t292 = t291 * t26;
            let t296 = t108 * t114;
            let t298 = t5 * t296 * t58;
            let t304 = f64x8::splat(1.0) / t113 / t6;
            let t305 = t25 * t304;
            let t308 = t5 * t305 * t58 / f64x8::splat(12.0);
            let t310 = t5 * t115 * t147;
            let t312 = t35 * t35;
            let t314 = f64x8::splat(1.0) / t37 / t312;
            let t315 = v_sigma0 * t314;
            let t322 = t136 * t136;
            let t326 = f64x8::splat(1.0) / t36 / t119;
            let t332 = v_sigma0 * v_sigma0;
            let t335 = f64x8::splat(1.0) / t36 / t312 / t119;
            let t338 = f64x8::splat(1.0) / t132 / t131;
            let t341 = f64x8::splat(0.0784) * t41 * t326 * t45 + f64x8::splat(0.168) * t315 * t133 - f64x8::splat(0.0448) * t332 * t335 * t338;
            let t344 = t51 * t51;
            let t345 = f64x8::splat(1.0) / t344;
            let t346 = t345 * t322;
            let t350 = -f64x8::splat(5.02346) * t140 * t322 + f64x8::splat(2.51173) * t52 * t341 + f64x8::splat(22.319) * t346 - f64x8::splat(7.439666666666667) * t140 * t341;
            let t354 = f64x8::splat(0.009125925925925926) * t34 * t315 * t54 - f64x8::splat(0.004977777777777778) * t34 * t122 * t143 + f64x8::splat(0.0009333333333333333) * t34 * t40 * t350;
            let t359 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t292 * t58 - t298 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t109 * t147 + t308 - t310 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t354));
            let t360 = t69 * t69;
            let t361 = f64x8::splat(1.0) / t360;
            let t362 = t155 * t155;
            let t365 = t64 * t283;
            let t368 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t102 + f64x8::splat(2.0) * t365)));
            let t372 = ((t68).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t361 * t362 + f64x8::splat(4.0) / f64x8::splat(3.0) * t69 * t368));
            let t373 = t372 * t26;
            let t377 = t158 * t114;
            let t379 = t5 * t377 * t96;
            let t381 = t71 * t304;
            let t384 = t5 * t381 * t96 / f64x8::splat(12.0);
            let t386 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t373 * t96 - t379 / f64x8::splat(4.0) + t384));
            let tv2rho20 = f64x8::splat(2.0) * t152 + f64x8::splat(2.0) * t168 + t6 * (t359 + t386);
            acc_v2rho2_0 = tv2rho20;
            let t389 = t278 * t172;
            let t393 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t284)));
            let t397 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t389 * t105 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t393));
            let t398 = t397 * t26;
            let t402 = t175 * t114;
            let t404 = t5 * t402 * t58;
            let t412 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t398 * t58 - t404 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t176 * t147 - t298 / f64x8::splat(8.0) + t308 - t310 / f64x8::splat(8.0)));
            let t413 = t361 * t183;
            let t417 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t365)));
            let t421 = ((t68).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t413 * t155 + f64x8::splat(4.0) / f64x8::splat(3.0) * t69 * t417));
            let t422 = t421 * t26;
            let t426 = t186 * t114;
            let t428 = t5 * t426 * t96;
            let t435 = t5 * t163 * t219;
            let t438 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t422 * t96 - t428 / f64x8::splat(8.0) - t379 / f64x8::splat(8.0) + t384 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t159 * t219 - t435 / f64x8::splat(8.0)));
            let tv2rho21 = t152 + t168 + t181 + t224 + t6 * (t412 + t438);
            acc_v2rho2_1 = tv2rho21;
            let t443 = t172 * t172;
            let t448 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t102 + f64x8::splat(2.0) * t284)));
            let t452 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t278 * t443 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t448));
            let t453 = t452 * t26;
            let t459 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t453 * t58 - t404 / f64x8::splat(4.0) + t308));
            let t460 = t183 * t183;
            let t465 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t102 + f64x8::splat(2.0) * t365)));
            let t469 = ((t68).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t361 * t460 + f64x8::splat(4.0) / f64x8::splat(3.0) * t69 * t465));
            let t470 = t469 * t26;
            let t479 = t73 * t73;
            let t481 = f64x8::splat(1.0) / t75 / t479;
            let t482 = v_sigma2 * t481;
            let t489 = t208 * t208;
            let t493 = f64x8::splat(1.0) / t74 / t191;
            let t499 = v_sigma2 * v_sigma2;
            let t502 = f64x8::splat(1.0) / t74 / t479 / t191;
            let t505 = f64x8::splat(1.0) / t204 / t203;
            let t508 = f64x8::splat(0.0784) * t79 * t493 * t83 + f64x8::splat(0.168) * t482 * t205 - f64x8::splat(0.0448) * t499 * t502 * t505;
            let t511 = t89 * t89;
            let t512 = f64x8::splat(1.0) / t511;
            let t513 = t512 * t489;
            let t517 = -f64x8::splat(5.02346) * t212 * t489 + f64x8::splat(2.51173) * t90 * t508 + f64x8::splat(22.319) * t513 - f64x8::splat(7.439666666666667) * t212 * t508;
            let t521 = f64x8::splat(0.009125925925925926) * t34 * t482 * t92 - f64x8::splat(0.004977777777777778) * t34 * t194 * t215 + f64x8::splat(0.0009333333333333333) * t34 * t78 * t517;
            let t526 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t470 * t96 - t428 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t187 * t219 + t384 - t435 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t72 * t521));
            let tv2rho22 = f64x8::splat(2.0) * t181 + f64x8::splat(2.0) * t224 + t6 * (t459 + t526);
            acc_v2rho2_2 = tv2rho22;
            let t534 = t5 * t115 * t246 / f64x8::splat(8.0);
            let t535 = t33 * t121;
            let t552 = t312 * t35;
            let t554 = f64x8::splat(1.0) / t36 / t552;
            let t555 = t554 * t338;
            let t558 = -f64x8::splat(0.0168) * t231 * t127 * t45 - f64x8::splat(0.0504) * t121 * t133 + f64x8::splat(0.0168) * t555 * v_sigma0;
            let t561 = t345 * t237;
            let t564 = t140 * t558;
            let t566 = -f64x8::splat(5.02346) * t240 * t136 + f64x8::splat(2.51173) * t52 * t558 + f64x8::splat(22.319) * t561 * t136 - f64x8::splat(7.439666666666667) * t564;
            let t570 = -f64x8::splat(0.002488888888888889) * t32 * t535 * t54 + f64x8::splat(0.0009333333333333333) * t32 * t227 * t143 - f64x8::splat(0.002488888888888889) * t34 * t122 * t242 + f64x8::splat(0.0009333333333333333) * t34 * t40 * t566;
            let t575 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t109 * t246 - t534 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t570));
            let tv2rhosigma0 = t6 * t575 + t250;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t582 = t5 * t163 * t270 / f64x8::splat(8.0);
            let t584 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t159 * t270 - t582));
            let tv2rhosigma2 = t6 * t584 + t274;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t590 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t176 * t246 - t534));
            let tv2rhosigma3 = t6 * t590 + t250;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t595 = t33 * t193;
            let t612 = t479 * t73;
            let t614 = f64x8::splat(1.0) / t74 / t612;
            let t615 = t614 * t505;
            let t618 = -f64x8::splat(0.0168) * t255 * t199 * t83 - f64x8::splat(0.0504) * t193 * t205 + f64x8::splat(0.0168) * t615 * v_sigma2;
            let t621 = t512 * t261;
            let t624 = t212 * t618;
            let t626 = -f64x8::splat(5.02346) * t264 * t208 + f64x8::splat(2.51173) * t90 * t618 + f64x8::splat(22.319) * t621 * t208 - f64x8::splat(7.439666666666667) * t624;
            let t630 = -f64x8::splat(0.002488888888888889) * t32 * t595 * t92 + f64x8::splat(0.0009333333333333333) * t32 * t251 * t215 - f64x8::splat(0.002488888888888889) * t34 * t194 * t266 + f64x8::splat(0.0009333333333333333) * t34 * t78 * t626;
            let t635 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t187 * t270 - t582 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t72 * t630));
            let tv2rhosigma5 = t6 * t635 + t274;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t640 = t237 * t237;
            let t644 = f64x8::splat(1.0) / t41 / v_sigma0;
            let t648 = f64x8::splat(1.0) / v_sigma0;
            let t652 = t312 * v_rho0;
            let t654 = f64x8::splat(1.0) / t36 / t652;
            let t657 = -f64x8::splat(0.0063) * t644 * t43 * t45 + f64x8::splat(0.0063) * t648 * t39 * t133 - f64x8::splat(0.0063) * t654 * t338;
            let t660 = t345 * t640;
            let t662 = t140 * t657;
            let t664 = -f64x8::splat(5.02346) * t140 * t640 + f64x8::splat(2.51173) * t52 * t657 + f64x8::splat(22.319) * t660 - f64x8::splat(7.439666666666667) * t662;
            let t668 = f64x8::splat(0.0018666666666666666) * t32 * t227 * t242 + f64x8::splat(0.0009333333333333333) * t34 * t40 * t664;
            let t672 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t668));
            let tv2sigma20 = t6 * t672;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t676 = t261 * t261;
            let t680 = f64x8::splat(1.0) / t79 / v_sigma2;
            let t684 = f64x8::splat(1.0) / v_sigma2;
            let t688 = t479 * v_rho1;
            let t690 = f64x8::splat(1.0) / t74 / t688;
            let t693 = -f64x8::splat(0.0063) * t680 * t81 * t83 + f64x8::splat(0.0063) * t684 * t77 * t205 - f64x8::splat(0.0063) * t690 * t505;
            let t696 = t512 * t676;
            let t698 = t212 * t693;
            let t700 = -f64x8::splat(5.02346) * t212 * t676 + f64x8::splat(2.51173) * t90 * t693 + f64x8::splat(22.319) * t696 - f64x8::splat(7.439666666666667) * t698;
            let t704 = f64x8::splat(0.0018666666666666666) * t32 * t251 * t266 + f64x8::splat(0.0009333333333333333) * t34 * t78 * t700;
            let t708 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t72 * t704));
            let tv2sigma25 = t6 * t708;
            acc_v2sigma2_5 = tv2sigma25;
            let t712 = f64x8::splat(1.0) / t277 / t19;
            let t713 = t279 * t105;
            let t716 = t278 * t105;
            let t719 = t101 * t101;
            let t720 = f64x8::splat(1.0) / t719;
            let t721 = t16 * t720;
            let t724 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t283 - f64x8::splat(6.0) * t721)));
            let t728 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t712 * t713 + f64x8::splat(4.0) / f64x8::splat(3.0) * t716 * t287 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t724));
            let t729 = t728 * t26;
            let t733 = t291 * t114;
            let t735 = t5 * t733 * t58;
            let t740 = t108 * t304;
            let t742 = t5 * t740 * t58;
            let t745 = t5 * t296 * t147;
            let t751 = f64x8::splat(1.0) / t113 / t101;
            let t752 = t25 * t751;
            let t755 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t752 * t58;
            let t757 = t5 * t305 * t147;
            let t760 = t5 * t115 * t354;
            let t763 = f64x8::splat(1.0) / t37 / t652;
            let t764 = v_sigma0 * t763;
            let t774 = t322 * t136;
            let t780 = f64x8::splat(1.0) / t36 / t312;
            let t786 = t312 * t312;
            let t788 = f64x8::splat(1.0) / t36 / t786;
            let t792 = t332 * v_sigma0;
            let t793 = t786 * t119;
            let t794 = f64x8::splat(1.0) / t793;
            let t796 = t131 * t131;
            let t798 = f64x8::splat(1.0) / t132 / t796;
            let t801 = -f64x8::splat(0.2613333333333333) * t41 * t780 * t45 - f64x8::splat(0.8885333333333333) * t764 * t133 + f64x8::splat(0.5525333333333333) * t332 * t788 * t338 - f64x8::splat(0.1792) * t792 * t794 * t798;
            let t805 = f64x8::splat(1.0) / t344 / t48;
            let t808 = t345 * t136;
            let t813 = f64x8::splat(15.07038) * t345 * t774 - f64x8::splat(15.07038) * t141 * t341 + f64x8::splat(2.51173) * t52 * t801 - f64x8::splat(89.276) * t805 * t774 + f64x8::splat(66.957) * t808 * t341 - f64x8::splat(7.439666666666667) * t140 * t801;
            let t817 = -f64x8::splat(0.042587654320987656) * t34 * t764 * t54 + f64x8::splat(0.02737777777777778) * t34 * t315 * t143 - f64x8::splat(0.007466666666666667) * t34 * t122 * t350 + f64x8::splat(0.0009333333333333333) * t34 * t40 * t813;
            let t822 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t729 * t58 - f64x8::splat(3.0) / f64x8::splat(8.0) * t735 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t292 * t147 + t742 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t745 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t109 * t354 - t755 + t757 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t760 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t817));
            let t824 = f64x8::splat(1.0) / t360 / t67;
            let t825 = t362 * t155;
            let t828 = t361 * t155;
            let t831 = t64 * t720;
            let t834 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t283 - f64x8::splat(6.0) * t831)));
            let t838 = ((t68).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t824 * t825 + f64x8::splat(4.0) / f64x8::splat(3.0) * t828 * t368 + f64x8::splat(4.0) / f64x8::splat(3.0) * t69 * t834));
            let t839 = t838 * t26;
            let t843 = t372 * t114;
            let t845 = t5 * t843 * t96;
            let t847 = t158 * t304;
            let t849 = t5 * t847 * t96;
            let t851 = t71 * t751;
            let t854 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t851 * t96;
            let t856 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t839 * t96 - f64x8::splat(3.0) / f64x8::splat(8.0) * t845 + t849 / f64x8::splat(4.0) - t854));
            let tv3rho30 = f64x8::splat(3.0) * t359 + f64x8::splat(3.0) * t386 + t6 * (t822 + t856);
            acc_v3rho3_0 = tv3rho30;
            let t859 = f64x8::splat(2.0) * t412;
            let t860 = f64x8::splat(2.0) * t438;
            let t861 = t712 * t172;
            let t864 = t278 * t393;
            let t869 = f64x8::splat(2.0) * t283;
            let t870 = f64x8::splat(6.0) * t721;
            let t872 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t869 - t870)));
            let t876 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t861 * t279 + f64x8::splat(8.0) / f64x8::splat(9.0) * t864 * t105 + f64x8::splat(4.0) / f64x8::splat(9.0) * t389 * t287 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t872));
            let t877 = t876 * t26;
            let t881 = t397 * t114;
            let t884 = t5 * t881 * t58 / f64x8::splat(4.0);
            let t888 = t175 * t304;
            let t890 = t5 * t888 * t58;
            let t894 = t5 * t402 * t147 / f64x8::splat(4.0);
            let t903 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t877 * t58 - t884 - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t398 * t147 + t890 / f64x8::splat(12.0) - t894 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t176 * t354 - t735 / f64x8::splat(8.0) + t742 / f64x8::splat(6.0) - t745 / f64x8::splat(4.0) - t755 + t757 / f64x8::splat(6.0) - t760 / f64x8::splat(8.0);
            let t904 = ((t1).select(f64x8::splat(0.0), t903));
            let t905 = t824 * t183;
            let t908 = t361 * t417;
            let t913 = f64x8::splat(6.0) * t831;
            let t915 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t869 - t913)));
            let t919 = ((t68).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t905 * t362 + f64x8::splat(8.0) / f64x8::splat(9.0) * t908 * t155 + f64x8::splat(4.0) / f64x8::splat(9.0) * t413 * t368 + f64x8::splat(4.0) / f64x8::splat(3.0) * t69 * t915));
            let t920 = t919 * t26;
            let t924 = t421 * t114;
            let t927 = t5 * t924 * t96 / f64x8::splat(4.0);
            let t928 = t186 * t304;
            let t930 = t5 * t928 * t96;
            let t939 = t5 * t377 * t219 / f64x8::splat(4.0);
            let t941 = t5 * t381 * t219;
            let t944 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t920 * t96 - t927 + t930 / f64x8::splat(12.0) - t845 / f64x8::splat(8.0) + t849 / f64x8::splat(6.0) - t854 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t373 * t219 - t939 + t941 / f64x8::splat(12.0)));
            let tv3rho31 = t359 + t386 + t859 + t860 + t6 * (t904 + t944);
            acc_v3rho3_1 = tv3rho31;
            let t947 = t712 * t443;
            let t952 = t278 * t448;
            let t956 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t869 - t870)));
            let t960 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t947 * t105 + f64x8::splat(8.0) / f64x8::splat(9.0) * t389 * t393 + f64x8::splat(4.0) / f64x8::splat(9.0) * t952 * t105 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t956));
            let t961 = t960 * t26;
            let t965 = t452 * t114;
            let t967 = t5 * t965 * t58;
            let t976 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t961 * t58 - t967 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t453 * t147 - t884 + t890 / f64x8::splat(6.0) - t894 + t742 / f64x8::splat(12.0) - t755 + t757 / f64x8::splat(12.0)));
            let t977 = t824 * t460;
            let t982 = t361 * t465;
            let t986 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t869 - t913)));
            let t990 = ((t68).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t977 * t155 + f64x8::splat(8.0) / f64x8::splat(9.0) * t413 * t417 + f64x8::splat(4.0) / f64x8::splat(9.0) * t982 * t155 + f64x8::splat(4.0) / f64x8::splat(3.0) * t69 * t986));
            let t991 = t990 * t26;
            let t995 = t469 * t114;
            let t997 = t5 * t995 * t96;
            let t1004 = t5 * t426 * t219;
            let t1012 = t5 * t163 * t521;
            let t1014 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t991 * t96 - t997 / f64x8::splat(8.0) - t927 + t930 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t422 * t219 - t1004 / f64x8::splat(4.0) + t849 / f64x8::splat(12.0) - t854 - t939 + t941 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t159 * t521 - t1012 / f64x8::splat(8.0);
            let t1015 = ((t63).select(f64x8::splat(0.0), t1014));
            let tv3rho32 = t859 + t860 + t459 + t526 + t6 * (t976 + t1015);
            acc_v3rho3_2 = tv3rho32;
            let t1020 = t443 * t172;
            let t1027 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t283 - f64x8::splat(6.0) * t721)));
            let t1031 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t712 * t1020 + f64x8::splat(4.0) / f64x8::splat(3.0) * t389 * t448 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t1027));
            let t1032 = t1031 * t26;
            let t1039 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1032 * t58 - f64x8::splat(3.0) / f64x8::splat(8.0) * t967 + t890 / f64x8::splat(4.0) - t755));
            let t1040 = t460 * t183;
            let t1047 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t283 - f64x8::splat(6.0) * t831)));
            let t1051 = ((t68).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t824 * t1040 + f64x8::splat(4.0) / f64x8::splat(3.0) * t413 * t465 + f64x8::splat(4.0) / f64x8::splat(3.0) * t69 * t1047));
            let t1052 = t1051 * t26;
            let t1068 = f64x8::splat(1.0) / t75 / t688;
            let t1069 = v_sigma2 * t1068;
            let t1079 = t489 * t208;
            let t1085 = f64x8::splat(1.0) / t74 / t479;
            let t1091 = t479 * t479;
            let t1093 = f64x8::splat(1.0) / t74 / t1091;
            let t1097 = t499 * v_sigma2;
            let t1098 = t1091 * t191;
            let t1099 = f64x8::splat(1.0) / t1098;
            let t1101 = t203 * t203;
            let t1103 = f64x8::splat(1.0) / t204 / t1101;
            let t1106 = -f64x8::splat(0.2613333333333333) * t79 * t1085 * t83 - f64x8::splat(0.8885333333333333) * t1069 * t205 + f64x8::splat(0.5525333333333333) * t499 * t1093 * t505 - f64x8::splat(0.1792) * t1097 * t1099 * t1103;
            let t1110 = f64x8::splat(1.0) / t511 / t86;
            let t1113 = t512 * t208;
            let t1118 = f64x8::splat(15.07038) * t512 * t1079 - f64x8::splat(15.07038) * t213 * t508 + f64x8::splat(2.51173) * t90 * t1106 - f64x8::splat(89.276) * t1110 * t1079 + f64x8::splat(66.957) * t1113 * t508 - f64x8::splat(7.439666666666667) * t212 * t1106;
            let t1122 = -f64x8::splat(0.042587654320987656) * t34 * t1069 * t92 + f64x8::splat(0.02737777777777778) * t34 * t482 * t215 - f64x8::splat(0.007466666666666667) * t34 * t194 * t517 + f64x8::splat(0.0009333333333333333) * t34 * t78 * t1118;
            let t1127 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t1052 * t96 - f64x8::splat(3.0) / f64x8::splat(8.0) * t997 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t470 * t219 + t930 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t1004 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t187 * t521 - t854 + t941 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t1012 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t72 * t1122));
            let tv3rho33 = f64x8::splat(3.0) * t459 + f64x8::splat(3.0) * t526 + t6 * (t1039 + t1127);
            acc_v3rho3_3 = tv3rho33;
            let t1135 = t5 * t296 * t246;
            let t1142 = t5 * t305 * t246 / f64x8::splat(12.0);
            let t1144 = t5 * t115 * t570;
            let t1146 = t33 * t314;
            let t1173 = t335 * t338;
            let t1176 = t786 * t35;
            let t1178 = f64x8::splat(1.0) / t1176 * t798;
            let t1181 = f64x8::splat(0.0392) * t231 * t326 * t45 + f64x8::splat(0.2072) * t314 * t133 - f64x8::splat(0.1736) * t1173 * v_sigma0 + f64x8::splat(0.0672) * t1178 * t332;
            let t1184 = t805 * t237;
            let t1187 = t345 * t558;
            let t1192 = t140 * t1181;
            let t1194 = f64x8::splat(15.07038) * t561 * t322 - f64x8::splat(10.04692) * t564 * t136 - f64x8::splat(5.02346) * t240 * t341 + f64x8::splat(2.51173) * t52 * t1181 - f64x8::splat(89.276) * t1184 * t322 + f64x8::splat(44.638) * t1187 * t136 + f64x8::splat(22.319) * t561 * t341 - f64x8::splat(7.439666666666667) * t1192;
            let t1198 = f64x8::splat(0.009125925925925926) * t32 * t1146 * t54 - f64x8::splat(0.004977777777777778) * t32 * t535 * t143 + f64x8::splat(0.0009333333333333333) * t32 * t227 * t350 + f64x8::splat(0.009125925925925926) * t34 * t315 * t242 - f64x8::splat(0.004977777777777778) * t34 * t122 * t566 + f64x8::splat(0.0009333333333333333) * t34 * t40 * t1194;
            let t1203 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t292 * t246 - t1135 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t109 * t570 + t1142 - t1144 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1198));
            let tv3rho2sigma0 = t6 * t1203 + f64x8::splat(2.0) * t575;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t1210 = t5 * t377 * t270;
            let t1214 = t5 * t381 * t270 / f64x8::splat(12.0);
            let t1216 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t373 * t270 - t1210 / f64x8::splat(4.0) + t1214));
            let tv3rho2sigma2 = t6 * t1216 + f64x8::splat(2.0) * t584;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t1222 = t5 * t402 * t246;
            let t1230 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t398 * t246 - t1222 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t176 * t570 - t1135 / f64x8::splat(8.0) + t1142 - t1144 / f64x8::splat(8.0)));
            let tv3rho2sigma3 = t6 * t1230 + t575 + t590;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1236 = t5 * t426 * t270;
            let t1243 = t5 * t163 * t630;
            let t1246 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t422 * t270 - t1236 / f64x8::splat(8.0) - t1210 / f64x8::splat(8.0) + t1214 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t159 * t630 - t1243 / f64x8::splat(8.0)));
            let tv3rho2sigma5 = t6 * t1246 + t584 + t635;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1254 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t453 * t246 - t1222 / f64x8::splat(4.0) + t1142));
            let tv3rho2sigma6 = t6 * t1254 + f64x8::splat(2.0) * t590;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1265 = t33 * t481;
            let t1292 = t502 * t505;
            let t1295 = t1091 * t73;
            let t1297 = f64x8::splat(1.0) / t1295 * t1103;
            let t1300 = f64x8::splat(0.0392) * t255 * t493 * t83 + f64x8::splat(0.2072) * t481 * t205 - f64x8::splat(0.1736) * t1292 * v_sigma2 + f64x8::splat(0.0672) * t1297 * t499;
            let t1303 = t1110 * t261;
            let t1306 = t512 * t618;
            let t1311 = t212 * t1300;
            let t1313 = f64x8::splat(15.07038) * t621 * t489 - f64x8::splat(10.04692) * t624 * t208 - f64x8::splat(5.02346) * t264 * t508 + f64x8::splat(2.51173) * t90 * t1300 - f64x8::splat(89.276) * t1303 * t489 + f64x8::splat(44.638) * t1306 * t208 + f64x8::splat(22.319) * t621 * t508 - f64x8::splat(7.439666666666667) * t1311;
            let t1317 = f64x8::splat(0.009125925925925926) * t32 * t1265 * t92 - f64x8::splat(0.004977777777777778) * t32 * t595 * t215 + f64x8::splat(0.0009333333333333333) * t32 * t251 * t517 + f64x8::splat(0.009125925925925926) * t34 * t482 * t266 - f64x8::splat(0.004977777777777778) * t34 * t194 * t626 + f64x8::splat(0.0009333333333333333) * t34 * t78 * t1313;
            let t1322 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t470 * t270 - t1236 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t187 * t630 + t1214 - t1243 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t72 * t1317));
            let tv3rho2sigma8 = t6 * t1322 + f64x8::splat(2.0) * t635;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1329 = t5 * t115 * t668 / f64x8::splat(8.0);
            let t1352 = t786 * v_rho0;
            let t1354 = f64x8::splat(1.0) / t1352 * t798;
            let t1357 = f64x8::splat(0.0084) * t644 * t127 * t45 - f64x8::splat(0.0084) * t648 * t121 * t133 + f64x8::splat(0.042) * t555 - f64x8::splat(0.0252) * t1354 * v_sigma0;
            let t1360 = t805 * t640;
            let t1365 = t345 * t657;
            let t1368 = t140 * t1357;
            let t1370 = f64x8::splat(15.07038) * t660 * t136 - f64x8::splat(10.04692) * t240 * t558 - f64x8::splat(5.02346) * t662 * t136 + f64x8::splat(2.51173) * t52 * t1357 - f64x8::splat(89.276) * t1360 * t136 + f64x8::splat(44.638) * t561 * t558 + f64x8::splat(22.319) * t1365 * t136 - f64x8::splat(7.439666666666667) * t1368;
            let t1374 = -f64x8::splat(0.004977777777777778) * t32 * t535 * t242 + f64x8::splat(0.0018666666666666666) * t32 * t227 * t566 - f64x8::splat(0.002488888888888889) * t34 * t122 * t664 + f64x8::splat(0.0009333333333333333) * t34 * t40 * t1370;
            let t1379 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t109 * t668 - t1329 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1374));
            let tv3rhosigma20 = t6 * t1379 + t672;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1386 = t5 * t163 * t704 / f64x8::splat(8.0);
            let t1388 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t159 * t704 - t1386));
            let tv3rhosigma25 = t6 * t1388 + t708;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1394 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t176 * t668 - t1329));
            let tv3rhosigma26 = t6 * t1394 + t672;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1421 = t1091 * v_rho1;
            let t1423 = f64x8::splat(1.0) / t1421 * t1103;
            let t1426 = f64x8::splat(0.0084) * t680 * t199 * t83 - f64x8::splat(0.0084) * t684 * t193 * t205 + f64x8::splat(0.042) * t615 - f64x8::splat(0.0252) * t1423 * v_sigma2;
            let t1429 = t1110 * t676;
            let t1434 = t512 * t693;
            let t1437 = t212 * t1426;
            let t1439 = f64x8::splat(15.07038) * t696 * t208 - f64x8::splat(10.04692) * t264 * t618 - f64x8::splat(5.02346) * t698 * t208 + f64x8::splat(2.51173) * t90 * t1426 - f64x8::splat(89.276) * t1429 * t208 + f64x8::splat(44.638) * t621 * t618 + f64x8::splat(22.319) * t1434 * t208 - f64x8::splat(7.439666666666667) * t1437;
            let t1443 = -f64x8::splat(0.004977777777777778) * t32 * t595 * t266 + f64x8::splat(0.0018666666666666666) * t32 * t251 * t626 - f64x8::splat(0.002488888888888889) * t34 * t194 * t700 + f64x8::splat(0.0009333333333333333) * t34 * t78 * t1439;
            let t1448 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t187 * t704 - t1386 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t72 * t1443));
            let tv3rhosigma211 = t6 * t1448 + t708;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1453 = t640 * t237;
            let t1459 = f64x8::splat(1.0) / t41 / t332;
            let t1463 = f64x8::splat(1.0) / t332;
            let t1470 = f64x8::splat(1.0) / t786;
            let t1473 = f64x8::splat(0.00945) * t1459 * t43 * t45 - f64x8::splat(0.00945) * t1463 * t39 * t133 - f64x8::splat(0.00315) * t648 * t654 * t338 + f64x8::splat(0.00945) * t1470 * t798;
            let t1476 = t805 * t1453;
            let t1480 = t140 * t1473;
            let t1482 = f64x8::splat(15.07038) * t345 * t1453 - f64x8::splat(15.07038) * t240 * t657 + f64x8::splat(2.51173) * t52 * t1473 - f64x8::splat(89.276) * t1476 + f64x8::splat(66.957) * t561 * t657 - f64x8::splat(7.439666666666667) * t1480;
            let t1486 = f64x8::splat(0.0028) * t32 * t227 * t664 + f64x8::splat(0.0009333333333333333) * t34 * t40 * t1482;
            let t1490 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1486));
            let tv3sigma30 = t6 * t1490;
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
            let t1494 = t676 * t261;
            let t1500 = f64x8::splat(1.0) / t79 / t499;
            let t1504 = f64x8::splat(1.0) / t499;
            let t1511 = f64x8::splat(1.0) / t1091;
            let t1514 = f64x8::splat(0.00945) * t1500 * t81 * t83 - f64x8::splat(0.00945) * t1504 * t77 * t205 - f64x8::splat(0.00315) * t684 * t690 * t505 + f64x8::splat(0.00945) * t1511 * t1103;
            let t1517 = t1110 * t1494;
            let t1521 = t212 * t1514;
            let t1523 = f64x8::splat(15.07038) * t512 * t1494 - f64x8::splat(15.07038) * t264 * t693 + f64x8::splat(2.51173) * t90 * t1514 - f64x8::splat(89.276) * t1517 + f64x8::splat(66.957) * t621 * t693 - f64x8::splat(7.439666666666667) * t1521;
            let t1527 = f64x8::splat(0.0028) * t32 * t251 * t700 + f64x8::splat(0.0009333333333333333) * t34 * t78 * t1523;
            let t1531 = ((t63).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t72 * t1527));
            let tv3sigma39 = t6 * t1531;
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
