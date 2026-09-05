//! GGA_C_P86 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_p86.c`
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
pub fn gga_c_p86_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_aa: f64,
    param_bb: f64,
    param_ftilde: f64,
    param_malpha: f64,
    param_mbeta: f64,
    param_mgamma: f64,
    param_mdelta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_aa = f64x8::splat(param_aa);
    let param_bb = f64x8::splat(param_bb);
    let param_ftilde = f64x8::splat(param_ftilde);
    let param_malpha = f64x8::splat(param_malpha);
    let param_mbeta = f64x8::splat(param_mbeta);
    let param_mgamma = f64x8::splat(param_mgamma);
    let param_mdelta = f64x8::splat(param_mdelta);
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
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = t6 * t9;
            let t11 = t4 * t10;
            let t12 = t11 / f64x8::splat(4.0);
            let t13 = (f64x8::splat(1.0)).simd_le(t12);
            let t14 = ((t11).sqrt());
            let t17 = f64x8::splat(1.0) + f64x8::splat(0.52645) * t14 + f64x8::splat(0.08335) * t11;
            let t20 = (simd::ln(t12));
            let t23 = t4 * t10 * t20;
            let t27 = ((t13).select(-f64x8::splat(0.1423) / t17, f64x8::splat(0.0311) * t20 - f64x8::splat(0.048) + f64x8::splat(0.0005) * t23 - f64x8::splat(0.0029) * t11));
            let t30 = f64x8::splat(1.0) + f64x8::splat(0.69905) * t14 + f64x8::splat(0.065275) * t11;
            let t37 = ((t13).select(-f64x8::splat(0.0843) / t30, f64x8::splat(0.01555) * t20 - f64x8::splat(0.0269) + f64x8::splat(0.000175) * t23 - f64x8::splat(0.0012) * t11));
            let t38 = t37 - t27;
            let t39 = v_rho0 - v_rho1;
            let t40 = f64x8::splat(1.0) / t7;
            let t41 = t39 * t40;
            let t42 = f64x8::splat(1.0) + t41;
            let t43 = (t42).simd_le(zeta_threshold);
            let t44 = (simd::cbrt(zeta_threshold));
            let t45 = t44 * zeta_threshold;
            let t46 = (simd::cbrt(t42));
            let t47 = t46 * t42;
            let t48 = ((t43).select(t45, t47));
            let t49 = f64x8::splat(1.0) - t41;
            let t50 = (t49).simd_le(zeta_threshold);
            let t51 = (simd::cbrt(t49));
            let t52 = t51 * t49;
            let t53 = ((t50).select(t45, t52));
            let t54 = t48 + t53 - f64x8::splat(2.0);
            let t56 = f64x8::splat(M_CBRT2);
            let t59 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t56 - f64x8::splat(2.0));
            let t60 = t38 * t54 * t59;
            let t62 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t63 = t7 * t7;
            let t65 = f64x8::splat(1.0) / t8 / t63;
            let t66 = t62 * t65;
            let t67 = param_aa + param_bb;
            let t68 = param_ftilde * t67;
            let t69 = param_malpha * t1;
            let t70 = t3 * t6;
            let t71 = t70 * t9;
            let t74 = t1 * t1;
            let t75 = param_mbeta * t74;
            let t76 = t3 * t3;
            let t77 = t76 * t5;
            let t78 = t8 * t8;
            let t79 = f64x8::splat(1.0) / t78;
            let t80 = t77 * t79;
            let t83 = param_bb + t69 * t71 / f64x8::splat(4.0) + t75 * t80 / f64x8::splat(4.0);
            let t84 = param_mgamma * t1;
            let t87 = param_mdelta * t74;
            let t92 = f64x8::splat(1.0) + t84 * t71 / f64x8::splat(4.0) + t87 * t80 / f64x8::splat(4.0) + f64x8::splat(2387.32414637843) * param_mbeta * t40;
            let t93 = f64x8::splat(1.0) / t92;
            let t95 = t83 * t93 + param_aa;
            let t96 = f64x8::splat(1.0) / t95;
            let t97 = ((t62).sqrt());
            let t98 = t96 * t97;
            let t99 = (simd::pow(t7, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t101 = f64x8::splat(1.0) / t99 / t7;
            let t104 = (simd::exp(-t68 * t98 * t101));
            let t105 = t66 * t104;
            let t106 = t44 * t44;
            let t107 = t106 * zeta_threshold;
            let t108 = t46 * t46;
            let t109 = t108 * t42;
            let t110 = ((t43).select(t107, t109));
            let t111 = t51 * t51;
            let t112 = t111 * t49;
            let t113 = ((t50).select(t107, t112));
            let t114 = t110 + t113;
            let t115 = ((t114).sqrt());
            let t116 = f64x8::splat(1.0) / t115;
            let t117 = t95 * t116;
            let t118 = f64x8::splat(M_SQRT2);
            let t119 = t117 * t118;
            let t120 = t105 * t119;
            let tzk0 = t27 + t60 + t120;
            acc_zk = tzk0;
            let t121 = t17 * t17;
            let t122 = f64x8::splat(1.0) / t121;
            let t124 = f64x8::splat(1.0) / t14 * t1;
            let t126 = f64x8::splat(1.0) / t8 / t7;
            let t127 = t70 * t126;
            let t128 = t124 * t127;
            let t130 = t6 * t126;
            let t131 = t4 * t130;
            let t133 = -f64x8::splat(0.08774166666666666) * t128 - f64x8::splat(0.027783333333333333) * t131;
            let t138 = t4 * t130 * t20;
            let t142 = ((t13).select(f64x8::splat(0.1423) * t122 * t133, -f64x8::splat(0.010366666666666666) * t40 - f64x8::splat(0.00016666666666666666) * t138 + f64x8::splat(0.0008) * t131));
            let t143 = t30 * t30;
            let t144 = f64x8::splat(1.0) / t143;
            let t147 = -f64x8::splat(0.11650833333333334) * t128 - f64x8::splat(0.021758333333333334) * t131;
            let t154 = ((t13).select(f64x8::splat(0.0843) * t144 * t147, -f64x8::splat(0.005183333333333333) * t40 - f64x8::splat(5.833333333333333e-05) * t138 + f64x8::splat(0.00034166666666666666) * t131));
            let t155 = t154 - t142;
            let t157 = t155 * t54 * t59;
            let t158 = f64x8::splat(1.0) / t63;
            let t159 = t39 * t158;
            let t160 = t40 - t159;
            let t163 = ((t43).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t46 * t160));
            let t164 = -t160;
            let t167 = ((t50).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t51 * t164));
            let t168 = t163 + t167;
            let t170 = t38 * t168 * t59;
            let t171 = t63 * t7;
            let t173 = f64x8::splat(1.0) / t8 / t171;
            let t174 = t62 * t173;
            let t175 = t174 * t104;
            let t176 = t175 * t119;
            let t177 = f64x8::splat(7.0) / f64x8::splat(3.0) * t176;
            let t178 = t95 * t95;
            let t179 = f64x8::splat(1.0) / t178;
            let t180 = t68 * t179;
            let t181 = t97 * t101;
            let t186 = t77 / t78 / t7;
            let t189 = -t69 * t127 / f64x8::splat(12.0) - t75 * t186 / f64x8::splat(6.0);
            let t191 = t92 * t92;
            let t192 = f64x8::splat(1.0) / t191;
            let t193 = t83 * t192;
            let t200 = -t84 * t127 / f64x8::splat(12.0) - t87 * t186 / f64x8::splat(6.0) - f64x8::splat(2387.32414637843) * param_mbeta * t158;
            let t202 = t189 * t93 - t193 * t200;
            let t206 = f64x8::splat(1.0) / t99 / t63;
            let t210 = t180 * t181 * t202 + f64x8::splat(7.0) / f64x8::splat(6.0) * t68 * t98 * t206;
            let t211 = t66 * t210;
            let t212 = t104 * t95;
            let t213 = t116 * t118;
            let t214 = t212 * t213;
            let t215 = t211 * t214;
            let t216 = t202 * t116;
            let t217 = t216 * t118;
            let t218 = t105 * t217;
            let t220 = f64x8::splat(1.0) / t115 / t114;
            let t221 = t95 * t220;
            let t224 = ((t43).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t108 * t160));
            let t227 = ((t50).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t111 * t164));
            let t228 = t224 + t227;
            let t229 = t118 * t228;
            let t230 = t221 * t229;
            let t231 = t105 * t230;
            let t232 = t231 / f64x8::splat(2.0);
            let tvrho0 = t27 + t60 + t120 + t7 * (t142 + t157 + t170 - t177 + t215 + t218 - t232);
            acc_vrho_0 = tvrho0;
            let t235 = -t40 - t159;
            let t238 = ((t43).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t46 * t235));
            let t239 = -t235;
            let t242 = ((t50).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t51 * t239));
            let t243 = t238 + t242;
            let t245 = t38 * t243 * t59;
            let t248 = ((t43).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t108 * t235));
            let t251 = ((t50).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t111 * t239));
            let t252 = t248 + t251;
            let t253 = t118 * t252;
            let t254 = t221 * t253;
            let t255 = t105 * t254;
            let t256 = t255 / f64x8::splat(2.0);
            let tvrho1 = t27 + t60 + t120 + t7 * (t142 + t157 + t245 - t177 + t215 + t218 - t256);
            acc_vrho_1 = tvrho1;
            let t259 = t65 * t104;
            let t260 = t259 * t119;
            let t261 = ((t7).sqrt());
            let t263 = f64x8::splat(1.0) / t261 / t171;
            let t264 = t97 * t263;
            let t267 = t67 * t104 * t213;
            let t268 = t264 * param_ftilde * t267;
            let t269 = t268 / f64x8::splat(2.0);
            let tvsigma0 = t7 * (t260 - t269);
            acc_vsigma_0 = tvsigma0;
            let t271 = f64x8::splat(2.0) * t260;
            let tvsigma1 = t7 * (t271 - t268);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
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
