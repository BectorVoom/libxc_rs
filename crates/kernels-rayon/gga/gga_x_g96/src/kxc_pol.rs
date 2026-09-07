//! GGA_X_G96 kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_g96.c`
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
pub fn gga_x_g96_kxc_pol(
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
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
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
            let t28 = t2 * t2;
            let t30 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = t28 * t31;
            let t33 = f64x8::splat(M_CBRT4);
            let t34 = ((v_sigma0).sqrt());
            let t35 = (simd::cbrt(v_rho0));
            let t37 = f64x8::splat(1.0) / t35 / v_rho0;
            let t38 = t34 * t37;
            let t39 = ((t38).sqrt());
            let t40 = t39 * t38;
            let t44 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(1233.0) * t32 * t33 * t40;
            let t48 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t25 * t26 * t44));
            let t49 = (v_rho1).simd_le(dens_threshold);
            let t50 = -t16;
            let t52 = ((t14).select(t11, (t10).select(t15, t50 * t7)));
            let t53 = f64x8::splat(1.0) + t52;
            let t54 = (t53).simd_le(zeta_threshold);
            let t55 = (simd::cbrt(t53));
            let t57 = ((t54).select(t22, t55 * t53));
            let t59 = ((v_sigma2).sqrt());
            let t60 = (simd::cbrt(v_rho1));
            let t62 = f64x8::splat(1.0) / t60 / v_rho1;
            let t63 = t59 * t62;
            let t64 = ((t63).sqrt());
            let t65 = t64 * t63;
            let t69 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(1233.0) * t32 * t33 * t65;
            let t73 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t57 * t26 * t69));
            let tzk0 = t48 + t73;
            acc_zk = tzk0;
            let t74 = t6 * t6;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t16 * t75;
            let t78 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t76)));
            let t81 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t78));
            let t86 = t26 * t26;
            let t87 = f64x8::splat(1.0) / t86;
            let t91 = t5 * t25 * t87 * t44 / f64x8::splat(8.0);
            let t92 = t4 * t25;
            let t93 = t26 * t31;
            let t94 = t92 * t93;
            let t95 = t33 * t39;
            let t96 = v_rho0 * v_rho0;
            let t98 = f64x8::splat(1.0) / t35 / t96;
            let t100 = t95 * t34 * t98;
            let t104 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t81 * t26 * t44 - t91 + t94 * t100 / f64x8::splat(274.0)));
            let t105 = t50 * t75;
            let t107 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t105)));
            let t110 = ((t54).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * t107));
            let t118 = t5 * t57 * t87 * t69 / f64x8::splat(8.0);
            let t120 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t110 * t26 * t69 - t118));
            let tvrho0 = t48 + t73 + t6 * (t104 + t120);
            acc_vrho_0 = tvrho0;
            let t124 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t76)));
            let t127 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t124));
            let t133 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t127 * t26 * t44 - t91));
            let t135 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t105)));
            let t138 = ((t54).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * t135));
            let t143 = t4 * t57;
            let t144 = t143 * t93;
            let t145 = t33 * t64;
            let t146 = v_rho1 * v_rho1;
            let t148 = f64x8::splat(1.0) / t60 / t146;
            let t150 = t145 * t59 * t148;
            let t154 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t138 * t26 * t69 - t118 + t144 * t150 / f64x8::splat(274.0)));
            let tvrho1 = t48 + t73 + t6 * (t133 + t154);
            acc_vrho_1 = tvrho1;
            let t157 = f64x8::splat(1.0) / t34;
            let t159 = t95 * t157 * t37;
            let t162 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t94 * t159));
            let tvsigma0 = t6 * t162;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t163 = f64x8::splat(1.0) / t59;
            let t165 = t145 * t163 * t62;
            let t168 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t144 * t165));
            let tvsigma2 = t6 * t168;
            acc_vsigma_2 = tvsigma2;
            let t171 = t23 * t23;
            let t172 = f64x8::splat(1.0) / t171;
            let t173 = t78 * t78;
            let t176 = t74 * t6;
            let t177 = f64x8::splat(1.0) / t176;
            let t178 = t16 * t177;
            let t181 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t75 + f64x8::splat(2.0) * t178)));
            let t185 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t172 * t173 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t181));
            let t192 = t5 * t81 * t87 * t44;
            let t194 = t4 * t81;
            let t195 = t194 * t93;
            let t199 = f64x8::splat(1.0) / t86 / t6;
            let t203 = t5 * t25 * t199 * t44 / f64x8::splat(12.0);
            let t204 = t87 * t31;
            let t205 = t92 * t204;
            let t206 = t205 * t100;
            let t208 = f64x8::splat(1.0) / t39;
            let t209 = t33 * t208;
            let t210 = t96 * t96;
            let t211 = t35 * t35;
            let t213 = f64x8::splat(1.0) / t211 / t210;
            let t215 = t209 * v_sigma0 * t213;
            let t218 = t96 * v_rho0;
            let t220 = f64x8::splat(1.0) / t35 / t218;
            let t222 = t95 * t34 * t220;
            let t226 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t185 * t26 * t44 - t192 / f64x8::splat(4.0) + t195 * t100 / f64x8::splat(137.0) + t203 + t206 / f64x8::splat(411.0) - t94 * t215 / f64x8::splat(411.0) - f64x8::splat(7.0) / f64x8::splat(822.0) * t94 * t222));
            let t227 = t55 * t55;
            let t228 = f64x8::splat(1.0) / t227;
            let t229 = t107 * t107;
            let t232 = t50 * t177;
            let t235 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t75 + f64x8::splat(2.0) * t232)));
            let t239 = ((t54).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t228 * t229 + f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * t235));
            let t246 = t5 * t110 * t87 * t69;
            let t251 = t5 * t57 * t199 * t69 / f64x8::splat(12.0);
            let t253 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t239 * t26 * t69 - t246 / f64x8::splat(4.0) + t251));
            let tv2rho20 = f64x8::splat(2.0) * t104 + f64x8::splat(2.0) * t120 + t6 * (t226 + t253);
            acc_v2rho2_0 = tv2rho20;
            let t256 = t172 * t124;
            let t260 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t178)));
            let t264 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t256 * t78 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t260));
            let t271 = t5 * t127 * t87 * t44;
            let t273 = t4 * t127;
            let t274 = t273 * t93;
            let t280 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t264 * t26 * t44 - t271 / f64x8::splat(8.0) + t274 * t100 / f64x8::splat(274.0) - t192 / f64x8::splat(8.0) + t203 + t206 / f64x8::splat(822.0)));
            let t281 = t228 * t135;
            let t285 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t232)));
            let t289 = ((t54).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t281 * t107 + f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * t285));
            let t296 = t5 * t138 * t87 * t69;
            let t299 = t4 * t110;
            let t300 = t299 * t93;
            let t303 = t143 * t204;
            let t304 = t303 * t150;
            let t307 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t289 * t26 * t69 - t296 / f64x8::splat(8.0) - t246 / f64x8::splat(8.0) + t251 + t300 * t150 / f64x8::splat(274.0) + t304 / f64x8::splat(822.0)));
            let tv2rho21 = t104 + t120 + t133 + t154 + t6 * (t280 + t307);
            acc_v2rho2_1 = tv2rho21;
            let t312 = t124 * t124;
            let t317 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t75 + f64x8::splat(2.0) * t178)));
            let t321 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t172 * t312 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t317));
            let t328 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t321 * t26 * t44 - t271 / f64x8::splat(4.0) + t203));
            let t329 = t135 * t135;
            let t334 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t75 + f64x8::splat(2.0) * t232)));
            let t338 = ((t54).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t228 * t329 + f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * t334));
            let t344 = t4 * t138;
            let t345 = t344 * t93;
            let t349 = f64x8::splat(1.0) / t64;
            let t350 = t33 * t349;
            let t351 = t146 * t146;
            let t352 = t60 * t60;
            let t354 = f64x8::splat(1.0) / t352 / t351;
            let t356 = t350 * v_sigma2 * t354;
            let t359 = t146 * v_rho1;
            let t361 = f64x8::splat(1.0) / t60 / t359;
            let t363 = t145 * t59 * t361;
            let t367 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t338 * t26 * t69 - t296 / f64x8::splat(4.0) + t345 * t150 / f64x8::splat(137.0) + t251 + t304 / f64x8::splat(411.0) - t144 * t356 / f64x8::splat(411.0) - f64x8::splat(7.0) / f64x8::splat(822.0) * t144 * t363));
            let tv2rho22 = f64x8::splat(2.0) * t133 + f64x8::splat(2.0) * t154 + t6 * (t328 + t367);
            acc_v2rho2_2 = tv2rho22;
            let t373 = t205 * t159 / f64x8::splat(2192.0);
            let t374 = t92 * t26;
            let t375 = t31 * t33;
            let t377 = f64x8::splat(1.0) / t211 / t218;
            let t379 = t375 * t208 * t377;
            let t383 = t95 * t157 * t98;
            let t387 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t195 * t159 - t373 + t374 * t379 / f64x8::splat(1096.0) + t94 * t383 / f64x8::splat(548.0)));
            let tv2rhosigma0 = t6 * t387 + t162;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t392 = t303 * t165 / f64x8::splat(2192.0);
            let t394 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t300 * t165 - t392));
            let tv2rhosigma2 = t6 * t394 + t168;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t399 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t274 * t159 - t373));
            let tv2rhosigma3 = t6 * t399 + t162;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t403 = t143 * t26;
            let t405 = f64x8::splat(1.0) / t352 / t359;
            let t407 = t375 * t349 * t405;
            let t411 = t145 * t163 * t148;
            let t415 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t345 * t165 - t392 + t403 * t407 / f64x8::splat(1096.0) + t144 * t411 / f64x8::splat(548.0)));
            let tv2rhosigma5 = t6 * t415 + t168;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t417 = f64x8::splat(1.0) / v_sigma0;
            let t419 = f64x8::splat(1.0) / t211 / t96;
            let t421 = t209 * t417 * t419;
            let t424 = t34 * v_sigma0;
            let t425 = f64x8::splat(1.0) / t424;
            let t427 = t95 * t425 * t37;
            let t431 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8768.0) * t94 * t421 + f64x8::splat(3.0) / f64x8::splat(4384.0) * t94 * t427));
            let tv2sigma20 = t6 * t431;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t432 = f64x8::splat(1.0) / v_sigma2;
            let t434 = f64x8::splat(1.0) / t352 / t146;
            let t436 = t350 * t432 * t434;
            let t439 = t59 * v_sigma2;
            let t440 = f64x8::splat(1.0) / t439;
            let t442 = t145 * t440 * t62;
            let t446 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8768.0) * t144 * t436 + f64x8::splat(3.0) / f64x8::splat(4384.0) * t144 * t442));
            let tv2sigma25 = t6 * t446;
            acc_v2sigma2_5 = tv2sigma25;
            let t449 = t205 * t215;
            let t452 = t33 / t40;
            let t453 = t210 * t218;
            let t454 = f64x8::splat(1.0) / t453;
            let t456 = t452 * t424 * t454;
            let t461 = t5 * t185 * t87 * t44;
            let t465 = t5 * t81 * t199 * t44;
            let t468 = f64x8::splat(1.0) / t86 / t74;
            let t472 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t25 * t468 * t44;
            let t473 = t4 * t185;
            let t474 = t473 * t93;
            let t477 = t194 * t204;
            let t478 = t477 * t100;
            let t482 = t199 * t31;
            let t483 = t92 * t482;
            let t484 = t483 * t100;
            let t487 = f64x8::splat(1.0) / t171 / t19;
            let t488 = t173 * t78;
            let t491 = t172 * t78;
            let t494 = t74 * t74;
            let t495 = f64x8::splat(1.0) / t494;
            let t496 = t16 * t495;
            let t499 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t177 - f64x8::splat(6.0) * t496)));
            let t503 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t487 * t488 + f64x8::splat(4.0) / f64x8::splat(3.0) * t491 * t181 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t499));
            let t510 = t205 * t222;
            let t512 = t210 * v_rho0;
            let t514 = f64x8::splat(1.0) / t211 / t512;
            let t516 = t209 * v_sigma0 * t514;
            let t520 = f64x8::splat(1.0) / t35 / t210;
            let t522 = t95 * t34 * t520;
            let t525 = -t449 / f64x8::splat(411.0) - f64x8::splat(2.0) / f64x8::splat(1233.0) * t94 * t456 - f64x8::splat(3.0) / f64x8::splat(8.0) * t461 + t465 / f64x8::splat(4.0) - t472 + f64x8::splat(3.0) / f64x8::splat(274.0) * t474 * t100 + t478 / f64x8::splat(137.0) - t195 * t215 / f64x8::splat(137.0) - t484 / f64x8::splat(411.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t503 * t26 * t44 - f64x8::splat(7.0) / f64x8::splat(274.0) * t195 * t222 - f64x8::splat(7.0) / f64x8::splat(822.0) * t510 + f64x8::splat(7.0) / f64x8::splat(411.0) * t94 * t516 + f64x8::splat(35.0) / f64x8::splat(1233.0) * t94 * t522;
            let t526 = ((t1).select(f64x8::splat(0.0), t525));
            let t528 = f64x8::splat(1.0) / t227 / t53;
            let t529 = t229 * t107;
            let t532 = t228 * t107;
            let t535 = t50 * t495;
            let t538 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t177 - f64x8::splat(6.0) * t535)));
            let t542 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t528 * t529 + f64x8::splat(4.0) / f64x8::splat(3.0) * t532 * t235 + f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * t538));
            let t549 = t5 * t239 * t87 * t69;
            let t553 = t5 * t110 * t199 * t69;
            let t558 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t57 * t468 * t69;
            let t560 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t542 * t26 * t69 - f64x8::splat(3.0) / f64x8::splat(8.0) * t549 + t553 / f64x8::splat(4.0) - t558));
            let tv3rho30 = f64x8::splat(3.0) * t226 + f64x8::splat(3.0) * t253 + t6 * (t526 + t560);
            acc_v3rho3_0 = tv3rho30;
            let t563 = f64x8::splat(2.0) * t280;
            let t564 = f64x8::splat(2.0) * t307;
            let t565 = t487 * t124;
            let t568 = t172 * t260;
            let t573 = f64x8::splat(2.0) * t177;
            let t574 = f64x8::splat(6.0) * t496;
            let t576 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t573 - t574)));
            let t580 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t565 * t173 + f64x8::splat(8.0) / f64x8::splat(9.0) * t568 * t78 + f64x8::splat(4.0) / f64x8::splat(9.0) * t256 * t181 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t576));
            let t588 = t5 * t264 * t87 * t44 / f64x8::splat(4.0);
            let t589 = t4 * t264;
            let t590 = t589 * t93;
            let t595 = t5 * t127 * t199 * t44;
            let t597 = t273 * t204;
            let t599 = t597 * t100 / f64x8::splat(411.0);
            let t610 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t580 * t26 * t44 - t588 + t590 * t100 / f64x8::splat(137.0) + t595 / f64x8::splat(12.0) + t599 - t274 * t215 / f64x8::splat(411.0) - f64x8::splat(7.0) / f64x8::splat(822.0) * t274 * t222 - t461 / f64x8::splat(8.0) + t465 / f64x8::splat(6.0) + t478 / f64x8::splat(411.0) - t472 - f64x8::splat(2.0) / f64x8::splat(1233.0) * t484 - t449 / f64x8::splat(1233.0) - f64x8::splat(7.0) / f64x8::splat(2466.0) * t510;
            let t611 = ((t1).select(f64x8::splat(0.0), t610));
            let t612 = t528 * t135;
            let t615 = t228 * t285;
            let t620 = f64x8::splat(6.0) * t535;
            let t622 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t573 - t620)));
            let t626 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t612 * t229 + f64x8::splat(8.0) / f64x8::splat(9.0) * t615 * t107 + f64x8::splat(4.0) / f64x8::splat(9.0) * t281 * t235 + f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * t622));
            let t634 = t5 * t289 * t87 * t69 / f64x8::splat(4.0);
            let t637 = t5 * t138 * t199 * t69;
            let t641 = t4 * t239;
            let t642 = t641 * t93;
            let t645 = t299 * t204;
            let t647 = t645 * t150 / f64x8::splat(411.0);
            let t648 = t143 * t482;
            let t649 = t648 * t150;
            let t652 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t626 * t26 * t69 - t634 + t637 / f64x8::splat(12.0) - t549 / f64x8::splat(8.0) + t553 / f64x8::splat(6.0) - t558 + t642 * t150 / f64x8::splat(274.0) + t647 - t649 / f64x8::splat(1233.0)));
            let tv3rho31 = t226 + t253 + t563 + t564 + t6 * (t611 + t652);
            acc_v3rho3_1 = tv3rho31;
            let t655 = t487 * t312;
            let t660 = t172 * t317;
            let t664 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t573 - t574)));
            let t668 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t655 * t78 + f64x8::splat(8.0) / f64x8::splat(9.0) * t256 * t260 + f64x8::splat(4.0) / f64x8::splat(9.0) * t660 * t78 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t664));
            let t675 = t5 * t321 * t87 * t44;
            let t677 = t4 * t321;
            let t678 = t677 * t93;
            let t685 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t668 * t26 * t44 - t675 / f64x8::splat(8.0) + t678 * t100 / f64x8::splat(274.0) - t588 + t595 / f64x8::splat(6.0) + t599 + t465 / f64x8::splat(12.0) - t472 - t484 / f64x8::splat(1233.0)));
            let t686 = t528 * t329;
            let t691 = t228 * t334;
            let t695 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t573 - t620)));
            let t699 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t686 * t107 + f64x8::splat(8.0) / f64x8::splat(9.0) * t281 * t285 + f64x8::splat(4.0) / f64x8::splat(9.0) * t691 * t107 + f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * t695));
            let t706 = t5 * t338 * t87 * t69;
            let t709 = t4 * t289;
            let t710 = t709 * t93;
            let t713 = t344 * t204;
            let t714 = t713 * t150;
            let t720 = t303 * t356;
            let t724 = t303 * t363;
            let t726 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t699 * t26 * t69 - t706 / f64x8::splat(8.0) - t634 + t637 / f64x8::splat(6.0) + t710 * t150 / f64x8::splat(137.0) + t714 / f64x8::splat(411.0) + t553 / f64x8::splat(12.0) - t558 + t647 - f64x8::splat(2.0) / f64x8::splat(1233.0) * t649 - t300 * t356 / f64x8::splat(411.0) - t720 / f64x8::splat(1233.0) - f64x8::splat(7.0) / f64x8::splat(822.0) * t300 * t363 - f64x8::splat(7.0) / f64x8::splat(2466.0) * t724;
            let t727 = ((t49).select(f64x8::splat(0.0), t726));
            let tv3rho32 = t563 + t564 + t328 + t367 + t6 * (t685 + t727);
            acc_v3rho3_2 = tv3rho32;
            let t732 = t312 * t124;
            let t739 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t177 - f64x8::splat(6.0) * t496)));
            let t743 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t487 * t732 + f64x8::splat(4.0) / f64x8::splat(3.0) * t256 * t317 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t739));
            let t751 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t743 * t26 * t44 - f64x8::splat(3.0) / f64x8::splat(8.0) * t675 + t595 / f64x8::splat(4.0) - t472));
            let t752 = t329 * t135;
            let t759 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t177 - f64x8::splat(6.0) * t535)));
            let t763 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t528 * t752 + f64x8::splat(4.0) / f64x8::splat(3.0) * t281 * t334 + f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * t759));
            let t768 = t4 * t338;
            let t769 = t768 * t93;
            let t775 = t33 / t65;
            let t776 = t351 * t359;
            let t777 = f64x8::splat(1.0) / t776;
            let t779 = t775 * t439 * t777;
            let t790 = t351 * v_rho1;
            let t792 = f64x8::splat(1.0) / t352 / t790;
            let t794 = t350 * v_sigma2 * t792;
            let t798 = f64x8::splat(1.0) / t60 / t351;
            let t800 = t145 * t59 * t798;
            let t803 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t763 * t26 * t69 + f64x8::splat(3.0) / f64x8::splat(274.0) * t769 * t150 - t345 * t356 / f64x8::splat(137.0) - f64x8::splat(2.0) / f64x8::splat(1233.0) * t144 * t779 - f64x8::splat(3.0) / f64x8::splat(8.0) * t706 + t637 / f64x8::splat(4.0) - t558 - t649 / f64x8::splat(411.0) + t714 / f64x8::splat(137.0) - t720 / f64x8::splat(411.0) - f64x8::splat(7.0) / f64x8::splat(822.0) * t724 - f64x8::splat(7.0) / f64x8::splat(274.0) * t345 * t363 + f64x8::splat(7.0) / f64x8::splat(411.0) * t144 * t794 + f64x8::splat(35.0) / f64x8::splat(1233.0) * t144 * t800;
            let t804 = ((t49).select(f64x8::splat(0.0), t803));
            let tv3rho33 = f64x8::splat(3.0) * t328 + f64x8::splat(3.0) * t367 + t6 * (t751 + t804);
            acc_v3rho3_3 = tv3rho33;
            let t810 = t477 * t159;
            let t812 = t194 * t26;
            let t818 = t483 * t159 / f64x8::splat(3288.0);
            let t819 = t92 * t87;
            let t820 = t819 * t379;
            let t822 = t205 * t383;
            let t824 = t210 * t96;
            let t825 = f64x8::splat(1.0) / t824;
            let t827 = t452 * t825 * t34;
            let t831 = t375 * t208 * t213;
            let t835 = t95 * t157 * t220;
            let t839 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t474 * t159 - t810 / f64x8::splat(1096.0) + t812 * t379 / f64x8::splat(548.0) + t195 * t383 / f64x8::splat(274.0) + t818 + t820 / f64x8::splat(1644.0) + t822 / f64x8::splat(822.0) + t94 * t827 / f64x8::splat(1644.0) - f64x8::splat(5.0) / f64x8::splat(1096.0) * t374 * t831 - f64x8::splat(7.0) / f64x8::splat(1644.0) * t94 * t835));
            let tv3rho2sigma0 = t6 * t839 + f64x8::splat(2.0) * t387;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t844 = t645 * t165;
            let t847 = t648 * t165 / f64x8::splat(3288.0);
            let t849 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t642 * t165 - t844 / f64x8::splat(1096.0) + t847));
            let tv3rho2sigma2 = t6 * t849 + f64x8::splat(2.0) * t394;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t853 = t597 * t159;
            let t855 = t273 * t26;
            let t864 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t590 * t159 - t853 / f64x8::splat(2192.0) + t855 * t379 / f64x8::splat(1096.0) + t274 * t383 / f64x8::splat(548.0) - t810 / f64x8::splat(2192.0) + t818 + t820 / f64x8::splat(3288.0) + t822 / f64x8::splat(1644.0)));
            let tv3rho2sigma3 = t6 * t864 + t387 + t399;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t868 = t713 * t165;
            let t871 = t299 * t26;
            let t874 = t143 * t87;
            let t875 = t874 * t407;
            let t879 = t303 * t411;
            let t882 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t710 * t165 - t868 / f64x8::splat(2192.0) - t844 / f64x8::splat(2192.0) + t847 + t871 * t407 / f64x8::splat(1096.0) + t875 / f64x8::splat(3288.0) + t300 * t411 / f64x8::splat(548.0) + t879 / f64x8::splat(1644.0)));
            let tv3rho2sigma5 = t6 * t882 + t394 + t415;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t889 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t678 * t159 - t853 / f64x8::splat(1096.0) + t818));
            let tv3rho2sigma6 = t6 * t889 + f64x8::splat(2.0) * t399;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t895 = t344 * t26;
            let t902 = t351 * t146;
            let t903 = f64x8::splat(1.0) / t902;
            let t905 = t775 * t903 * t59;
            let t909 = t375 * t349 * t354;
            let t913 = t145 * t163 * t361;
            let t917 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t769 * t165 - t868 / f64x8::splat(1096.0) + t895 * t407 / f64x8::splat(548.0) + t345 * t411 / f64x8::splat(274.0) + t847 + t875 / f64x8::splat(1644.0) + t879 / f64x8::splat(822.0) + t144 * t905 / f64x8::splat(1644.0) - f64x8::splat(5.0) / f64x8::splat(1096.0) * t403 * t909 - f64x8::splat(7.0) / f64x8::splat(1644.0) * t144 * t913));
            let tv3rho2sigma8 = t6 * t917 + f64x8::splat(2.0) * t415;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t922 = t205 * t421 / f64x8::splat(8768.0);
            let t925 = t452 * t157 / t512;
            let t929 = t209 * t417 * t377;
            let t935 = t205 * t427 / f64x8::splat(4384.0);
            let t937 = t95 * t425 * t98;
            let t941 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8768.0) * t195 * t421 - t922 - t94 * t925 / f64x8::splat(4384.0) + t94 * t929 / f64x8::splat(2192.0) + f64x8::splat(3.0) / f64x8::splat(4384.0) * t195 * t427 + t935 - t94 * t937 / f64x8::splat(1096.0)));
            let tv3rhosigma20 = t6 * t941 + t431;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t946 = t303 * t436 / f64x8::splat(8768.0);
            let t950 = t303 * t442 / f64x8::splat(4384.0);
            let t952 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8768.0) * t300 * t436 - t946 + f64x8::splat(3.0) / f64x8::splat(4384.0) * t300 * t442 + t950));
            let tv3rhosigma25 = t6 * t952 + t446;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t959 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8768.0) * t274 * t421 - t922 + f64x8::splat(3.0) / f64x8::splat(4384.0) * t274 * t427 + t935));
            let tv3rhosigma26 = t6 * t959 + t431;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t965 = t775 * t163 / t790;
            let t969 = t350 * t432 * t405;
            let t975 = t145 * t440 * t148;
            let t979 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8768.0) * t345 * t436 - t946 - t144 * t965 / f64x8::splat(4384.0) + t144 * t969 / f64x8::splat(2192.0) + f64x8::splat(3.0) / f64x8::splat(4384.0) * t345 * t442 + t950 - t144 * t975 / f64x8::splat(1096.0)));
            let tv3rhosigma211 = t6 * t979 + t446;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t981 = f64x8::splat(1.0) / t210;
            let t983 = t452 * t425 * t981;
            let t986 = v_sigma0 * v_sigma0;
            let t987 = f64x8::splat(1.0) / t986;
            let t989 = t209 * t987 * t419;
            let t993 = f64x8::splat(1.0) / t34 / t986;
            let t995 = t95 * t993 * t37;
            let t999 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(35072.0) * t94 * t983 + f64x8::splat(9.0) / f64x8::splat(17536.0) * t94 * t989 - f64x8::splat(9.0) / f64x8::splat(8768.0) * t94 * t995));
            let tv3sigma30 = t6 * t999;
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
            let t1000 = f64x8::splat(1.0) / t351;
            let t1002 = t775 * t440 * t1000;
            let t1005 = v_sigma2 * v_sigma2;
            let t1006 = f64x8::splat(1.0) / t1005;
            let t1008 = t350 * t1006 * t434;
            let t1012 = f64x8::splat(1.0) / t59 / t1005;
            let t1014 = t145 * t1012 * t62;
            let t1018 = ((t49).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(35072.0) * t144 * t1002 + f64x8::splat(9.0) / f64x8::splat(17536.0) * t144 * t1008 - f64x8::splat(9.0) / f64x8::splat(8768.0) * t144 * t1014));
            let tv3sigma39 = t6 * t1018;
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
