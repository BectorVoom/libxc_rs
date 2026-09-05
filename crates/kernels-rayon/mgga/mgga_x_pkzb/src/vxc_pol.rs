//! MGGA_X_PKZB vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_pkzb.c`
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
pub fn mgga_x_pkzb_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
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
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t29 = f64x8::splat(M_CBRT6);
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t29 * t33;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t41 = t34 * v_sigma0 * t39;
            let t44 = f64x8::splat(1.0) / t37 / v_rho0;
            let t49 = t34 * v_tau0 * t44 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) - t41 / f64x8::splat(288.0);
            let t50 = t49 * t49;
            let t52 = t49 * t29;
            let t53 = t33 * v_sigma0;
            let t54 = t53 * t39;
            let t57 = t29 * t29;
            let t59 = f64x8::splat(1.0) / t31 / t30;
            let t60 = t57 * t59;
            let t61 = v_sigma0 * v_sigma0;
            let t62 = t35 * t35;
            let t63 = t62 * v_rho0;
            let t65 = f64x8::splat(1.0) / t36 / t63;
            let t69 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t41 + f64x8::splat(146.0) / f64x8::splat(2025.0) * t50 - f64x8::splat(73.0) / f64x8::splat(9720.0) * t52 * t54 + f64x8::splat(0.0002290923400091281) * t60 * t61 * t65;
            let t72 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t69;
            let t76 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t26 * t27 * t72));
            let t77 = (v_rho1).simd_le(dens_threshold);
            let t78 = -t17;
            let t80 = ((t15).select(t12, (t11).select(t16, t78 * t8)));
            let t81 = f64x8::splat(1.0) + t80;
            let t82 = (t81).simd_le(zeta_threshold);
            let t83 = (simd::cbrt(t81));
            let t85 = ((t82).select(t23, t83 * t81));
            let t87 = v_rho1 * v_rho1;
            let t88 = (simd::cbrt(v_rho1));
            let t89 = t88 * t88;
            let t91 = f64x8::splat(1.0) / t89 / t87;
            let t93 = t34 * v_sigma2 * t91;
            let t96 = f64x8::splat(1.0) / t89 / v_rho1;
            let t101 = t34 * v_tau1 * t96 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) - t93 / f64x8::splat(288.0);
            let t102 = t101 * t101;
            let t104 = t101 * t29;
            let t105 = t33 * v_sigma2;
            let t106 = t105 * t91;
            let t109 = v_sigma2 * v_sigma2;
            let t110 = t87 * t87;
            let t111 = t110 * v_rho1;
            let t113 = f64x8::splat(1.0) / t88 / t111;
            let t117 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t93 + f64x8::splat(146.0) / f64x8::splat(2025.0) * t102 - f64x8::splat(73.0) / f64x8::splat(9720.0) * t104 * t106 + f64x8::splat(0.0002290923400091281) * t60 * t109 * t113;
            let t120 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t117;
            let t124 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t85 * t27 * t120));
            let tzk0 = t76 + t124;
            acc_zk = tzk0;
            let t125 = t7 * t7;
            let t126 = f64x8::splat(1.0) / t125;
            let t127 = t17 * t126;
            let t129 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t127)));
            let t132 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t129));
            let t137 = t27 * t27;
            let t138 = f64x8::splat(1.0) / t137;
            let t142 = t6 * t26 * t138 * t72 / f64x8::splat(8.0);
            let t143 = t3 * t26;
            let t144 = t69 * t69;
            let t145 = f64x8::splat(1.0) / t144;
            let t146 = t27 * t145;
            let t147 = t35 * v_rho0;
            let t149 = f64x8::splat(1.0) / t37 / t147;
            let t151 = t34 * v_sigma0 * t149;
            let t157 = -f64x8::splat(5.0) / f64x8::splat(12.0) * t34 * v_tau0 * t39 + t151 / f64x8::splat(108.0);
            let t160 = t157 * t29;
            let t163 = t53 * t149;
            let t166 = t62 * t35;
            let t168 = f64x8::splat(1.0) / t36 / t166;
            let t172 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t151 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t49 * t157 - f64x8::splat(73.0) / f64x8::splat(9720.0) * t160 * t54 + f64x8::splat(73.0) / f64x8::splat(3645.0) * t52 * t163 - f64x8::splat(0.0012218258133820164) * t60 * t61 * t168;
            let t173 = t146 * t172;
            let t177 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t132 * t27 * t72 - t142 - f64x8::splat(0.1655109536374632) * t143 * t173));
            let t178 = t78 * t126;
            let t180 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t178)));
            let t183 = ((t82).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t83 * t180));
            let t191 = t6 * t85 * t138 * t120 / f64x8::splat(8.0);
            let t193 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t183 * t27 * t120 - t191));
            let tvrho0 = t76 + t124 + t7 * (t177 + t193);
            acc_vrho_0 = tvrho0;
            let t197 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t127)));
            let t200 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t197));
            let t206 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t200 * t27 * t72 - t142));
            let t208 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t178)));
            let t211 = ((t82).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t83 * t208));
            let t216 = t3 * t85;
            let t217 = t117 * t117;
            let t218 = f64x8::splat(1.0) / t217;
            let t219 = t27 * t218;
            let t220 = t87 * v_rho1;
            let t222 = f64x8::splat(1.0) / t89 / t220;
            let t224 = t34 * v_sigma2 * t222;
            let t230 = -f64x8::splat(5.0) / f64x8::splat(12.0) * t34 * v_tau1 * t91 + t224 / f64x8::splat(108.0);
            let t233 = t230 * t29;
            let t236 = t105 * t222;
            let t239 = t110 * t87;
            let t241 = f64x8::splat(1.0) / t88 / t239;
            let t245 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t224 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t101 * t230 - f64x8::splat(73.0) / f64x8::splat(9720.0) * t233 * t106 + f64x8::splat(73.0) / f64x8::splat(3645.0) * t104 * t236 - f64x8::splat(0.0012218258133820164) * t60 * t109 * t241;
            let t246 = t219 * t245;
            let t250 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t211 * t27 * t120 - t191 - f64x8::splat(0.1655109536374632) * t216 * t246));
            let tvrho1 = t76 + t124 + t7 * (t206 + t250);
            acc_vrho_1 = tvrho1;
            let t255 = t33 * t39;
            let t256 = t52 * t255;
            let t259 = t60 * t65 * v_sigma0;
            let t261 = f64x8::splat(5.0) / f64x8::splat(972.0) * t34 * t39 - f64x8::splat(146.0) / f64x8::splat(18225.0) * t256 + f64x8::splat(0.0004842620691357688) * t259;
            let t262 = t146 * t261;
            let t265 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t143 * t262));
            let tvsigma0 = t7 * t265;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t268 = t33 * t91;
            let t269 = t104 * t268;
            let t272 = t60 * t113 * v_sigma2;
            let t274 = f64x8::splat(5.0) / f64x8::splat(972.0) * t34 * t91 - f64x8::splat(146.0) / f64x8::splat(18225.0) * t269 + f64x8::splat(0.0004842620691357688) * t272;
            let t275 = t219 * t274;
            let t278 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t216 * t275));
            let tvsigma2 = t7 * t278;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t279 = t33 * t44;
            let t283 = f64x8::splat(1.0) / t36 / t62;
            let t287 = f64x8::splat(73.0) / f64x8::splat(2025.0) * t52 * t279 - f64x8::splat(73.0) / f64x8::splat(38880.0) * t60 * t283 * v_sigma0;
            let t288 = t146 * t287;
            let t291 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t143 * t288));
            let tvtau0 = t7 * t291;
            acc_vtau_0 = tvtau0;
            let t292 = t33 * t96;
            let t296 = f64x8::splat(1.0) / t88 / t110;
            let t300 = f64x8::splat(73.0) / f64x8::splat(2025.0) * t104 * t292 - f64x8::splat(73.0) / f64x8::splat(38880.0) * t60 * t296 * v_sigma2;
            let t301 = t219 * t300;
            let t304 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t216 * t301));
            let tvtau1 = t7 * t304;
            acc_vtau_1 = tvtau1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
