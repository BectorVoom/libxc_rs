//! GGA_K_LLP vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_llp.c`
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
pub fn gga_k_llp_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_beta = f64x8::splat(param_beta);
    let param_gamma = f64x8::splat(param_gamma);
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
            let t3 = t2 * t2;
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 * t4 * f64x8::splat(M_PI);
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * t22;
            let t24 = t23 * zeta_threshold;
            let t25 = (simd::cbrt(t20));
            let t26 = t25 * t25;
            let t28 = ((t21).select(t24, t26 * t20));
            let t29 = (simd::cbrt(t7));
            let t30 = t29 * t29;
            let t31 = t28 * t30;
            let t32 = param_beta * t3;
            let t34 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = t32 * t35;
            let t37 = f64x8::splat(M_CBRT4);
            let t38 = t37 * v_sigma0;
            let t39 = v_rho0 * v_rho0;
            let t40 = (simd::cbrt(v_rho0));
            let t41 = t40 * t40;
            let t43 = f64x8::splat(1.0) / t41 / t39;
            let t44 = param_gamma * param_beta;
            let t45 = ((v_sigma0).sqrt());
            let t47 = f64x8::splat(1.0) / t40 / v_rho0;
            let t48 = t45 * t47;
            let t49 = (simd::ln(t48 + ((t48 * t48 + f64x8::splat(1.0)).sqrt())));
            let t52 = f64x8::splat(1.0) + t44 * t48 * t49;
            let t53 = f64x8::splat(1.0) / t52;
            let t58 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t36 * t38 * t43 * t53;
            let t62 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t58));
            let t63 = (v_rho1).simd_le(dens_threshold);
            let t64 = -t17;
            let t66 = ((t15).select(t12, (t11).select(t16, t64 * t8)));
            let t67 = f64x8::splat(1.0) + t66;
            let t68 = (t67).simd_le(zeta_threshold);
            let t69 = (simd::cbrt(t67));
            let t70 = t69 * t69;
            let t72 = ((t68).select(t24, t70 * t67));
            let t73 = t72 * t30;
            let t74 = t37 * v_sigma2;
            let t75 = v_rho1 * v_rho1;
            let t76 = (simd::cbrt(v_rho1));
            let t77 = t76 * t76;
            let t79 = f64x8::splat(1.0) / t77 / t75;
            let t80 = ((v_sigma2).sqrt());
            let t82 = f64x8::splat(1.0) / t76 / v_rho1;
            let t83 = t80 * t82;
            let t84 = (simd::ln(t83 + ((t83 * t83 + f64x8::splat(1.0)).sqrt())));
            let t87 = f64x8::splat(1.0) + t44 * t83 * t84;
            let t88 = f64x8::splat(1.0) / t87;
            let t93 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t36 * t74 * t79 * t88;
            let t97 = ((t63).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t73 * t93));
            let tzk0 = t62 + t97;
            acc_zk = tzk0;
            let t98 = t7 * t7;
            let t99 = f64x8::splat(1.0) / t98;
            let t100 = t17 * t99;
            let t102 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t100)));
            let t105 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t102));
            let t106 = t105 * t30;
            let t110 = f64x8::splat(1.0) / t29;
            let t111 = t28 * t110;
            let t114 = t6 * t111 * t58 / f64x8::splat(10.0);
            let t115 = t39 * v_rho0;
            let t117 = f64x8::splat(1.0) / t41 / t115;
            let t122 = t35 * t37;
            let t123 = t32 * t122;
            let t124 = v_sigma0 * t43;
            let t125 = t52 * t52;
            let t126 = f64x8::splat(1.0) / t125;
            let t128 = f64x8::splat(1.0) / t40 / t39;
            let t132 = v_sigma0 * t117;
            let t133 = t124 + f64x8::splat(1.0);
            let t134 = ((t133).sqrt());
            let t135 = f64x8::splat(1.0) / t134;
            let t139 = -f64x8::splat(4.0) / f64x8::splat(3.0) * t44 * t45 * t128 * t49 - f64x8::splat(4.0) / f64x8::splat(3.0) * t44 * t132 * t135;
            let t140 = t126 * t139;
            let t144 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t36 * t38 * t117 * t53 - f64x8::splat(2.0) / f64x8::splat(9.0) * t123 * t124 * t140;
            let t149 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t106 * t58 + t114 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t144));
            let t150 = t64 * t99;
            let t152 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t150)));
            let t155 = ((t68).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t70 * t152));
            let t156 = t155 * t30;
            let t160 = t72 * t110;
            let t163 = t6 * t160 * t93 / f64x8::splat(10.0);
            let t165 = ((t63).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t156 * t93 + t163));
            let tvrho0 = t62 + t97 + t7 * (t149 + t165);
            acc_vrho_0 = tvrho0;
            let t169 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t100)));
            let t172 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t169));
            let t173 = t172 * t30;
            let t178 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t173 * t58 + t114));
            let t180 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t150)));
            let t183 = ((t68).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t70 * t180));
            let t184 = t183 * t30;
            let t188 = t75 * v_rho1;
            let t190 = f64x8::splat(1.0) / t77 / t188;
            let t195 = v_sigma2 * t79;
            let t196 = t87 * t87;
            let t197 = f64x8::splat(1.0) / t196;
            let t199 = f64x8::splat(1.0) / t76 / t75;
            let t203 = v_sigma2 * t190;
            let t204 = t195 + f64x8::splat(1.0);
            let t205 = ((t204).sqrt());
            let t206 = f64x8::splat(1.0) / t205;
            let t210 = -f64x8::splat(4.0) / f64x8::splat(3.0) * t44 * t80 * t199 * t84 - f64x8::splat(4.0) / f64x8::splat(3.0) * t44 * t203 * t206;
            let t211 = t197 * t210;
            let t215 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t36 * t74 * t190 * t88 - f64x8::splat(2.0) / f64x8::splat(9.0) * t123 * t195 * t211;
            let t220 = ((t63).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t184 * t93 + t163 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t73 * t215));
            let tvrho1 = t62 + t97 + t7 * (t178 + t220);
            acc_vrho_1 = tvrho1;
            let t223 = t37 * t43;
            let t226 = f64x8::splat(1.0) / t45;
            let t233 = t44 * t226 * t47 * t49 / f64x8::splat(2.0) + t44 * t43 * t135 / f64x8::splat(2.0);
            let t234 = t126 * t233;
            let t238 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t123 * t124 * t234 + f64x8::splat(2.0) / f64x8::splat(9.0) * t36 * t223 * t53;
            let t242 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t238));
            let tvsigma0 = t7 * t242;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t243 = t37 * t79;
            let t246 = f64x8::splat(1.0) / t80;
            let t253 = t44 * t246 * t82 * t84 / f64x8::splat(2.0) + t44 * t79 * t206 / f64x8::splat(2.0);
            let t254 = t197 * t253;
            let t258 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t123 * t195 * t254 + f64x8::splat(2.0) / f64x8::splat(9.0) * t36 * t243 * t88;
            let t262 = ((t63).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t73 * t258));
            let tvsigma2 = t7 * t262;
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
