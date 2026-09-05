//! GGA_X_2D_B88 vxc pol kernel — explicit SIMD (bit-exact).
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
pub fn gga_x_2d_b88_vxc_pol(
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
