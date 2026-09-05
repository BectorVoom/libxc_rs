//! GGA_X_HCTH_A vxc pol kernel — explicit SIMD (bit-exact).
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
pub fn gga_x_hcth_a_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
