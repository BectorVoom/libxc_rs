//! MGGA_X_2D_JS17 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_2d_js17.c`
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
pub fn mgga_x_2d_js17_vxc_pol(
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
            let t3 = ((f64x8::splat(M_PI)).sqrt());
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = v_rho0 + v_rho1;
            let t6 = f64x8::splat(1.0) / t5;
            let t9 = (f64x8::splat(2.0) * v_rho0 * t6).simd_le(zeta_threshold);
            let t10 = zeta_threshold - f64x8::splat(1.0);
            let t13 = (f64x8::splat(2.0) * v_rho1 * t6).simd_le(zeta_threshold);
            let t14 = -t10;
            let t15 = v_rho0 - v_rho1;
            let t17 = ((t9).select(t10, (t13).select(t14, t15 * t6)));
            let t18 = f64x8::splat(1.0) + t17;
            let t19 = (t18).simd_le(zeta_threshold);
            let t20 = ((zeta_threshold).sqrt());
            let t21 = t20 * zeta_threshold;
            let t22 = ((t18).sqrt());
            let t23 = t22 * t18;
            let t24 = ((t19).select(t21, t23));
            let t25 = t4 * t24;
            let t26 = f64x8::splat(M_SQRT2);
            let t27 = ((t5).sqrt());
            let t28 = t26 * t27;
            let t29 = v_rho0 * v_rho0;
            let t30 = t29 * v_rho0;
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = v_sigma0 * t31;
            let t34 = v_sigma0 * v_sigma0;
            let t35 = t29 * t29;
            let t37 = f64x8::splat(1.0) / t35 / t29;
            let t40 = f64x8::splat(1.0) + f64x8::splat(0.41252961249419273) * t32 + f64x8::splat(0.0006302988192022548) * t34 * t37;
            let t41 = (simd::pow(t40, f64x8::splat(1.0) / f64x8::splat(15.0)));
            let t44 = f64x8::splat(1.0) / t29;
            let t48 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t51 = f64x8::splat(1.0) + f64x8::splat(0.02793851343876014) * t32 + (-f64x8::splat(0.0772) * v_tau0 * t44 - f64x8::splat(11.596246802930645)) * t48 / f64x8::splat(4.0);
            let t52 = (simd::pow(t40, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t53 = f64x8::splat(1.0) / t52;
            let t56 = f64x8::splat(1.0) / t41 + f64x8::splat(2.0) / f64x8::splat(5.0) * t51 * t53;
            let t57 = t28 * t56;
            let t60 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t25 * t57));
            let t61 = (v_rho1).simd_le(dens_threshold);
            let t62 = -t15;
            let t64 = ((t13).select(t10, (t9).select(t14, t62 * t6)));
            let t65 = f64x8::splat(1.0) + t64;
            let t66 = (t65).simd_le(zeta_threshold);
            let t67 = ((t65).sqrt());
            let t68 = t67 * t65;
            let t69 = ((t66).select(t21, t68));
            let t70 = t4 * t69;
            let t71 = v_rho1 * v_rho1;
            let t72 = t71 * v_rho1;
            let t73 = f64x8::splat(1.0) / t72;
            let t74 = v_sigma2 * t73;
            let t76 = v_sigma2 * v_sigma2;
            let t77 = t71 * t71;
            let t79 = f64x8::splat(1.0) / t77 / t71;
            let t82 = f64x8::splat(1.0) + f64x8::splat(0.41252961249419273) * t74 + f64x8::splat(0.0006302988192022548) * t76 * t79;
            let t83 = (simd::pow(t82, f64x8::splat(1.0) / f64x8::splat(15.0)));
            let t86 = f64x8::splat(1.0) / t71;
            let t92 = f64x8::splat(1.0) + f64x8::splat(0.02793851343876014) * t74 + (-f64x8::splat(0.0772) * v_tau1 * t86 - f64x8::splat(11.596246802930645)) * t48 / f64x8::splat(4.0);
            let t93 = (simd::pow(t82, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t94 = f64x8::splat(1.0) / t93;
            let t97 = f64x8::splat(1.0) / t83 + f64x8::splat(2.0) / f64x8::splat(5.0) * t92 * t94;
            let t98 = t28 * t97;
            let t101 = ((t61).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t70 * t98));
            let tzk0 = t60 + t101;
            acc_zk = tzk0;
            let t102 = t5 * t5;
            let t103 = f64x8::splat(1.0) / t102;
            let t104 = t15 * t103;
            let t106 = ((t9).select(f64x8::splat(0.0), (t13).select(f64x8::splat(0.0), t6 - t104)));
            let t109 = ((t19).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(2.0) * t22 * t106));
            let t110 = t4 * t109;
            let t113 = f64x8::splat(1.0) / t27;
            let t114 = t26 * t113;
            let t115 = t114 * t56;
            let t117 = t25 * t115 / f64x8::splat(3.0);
            let t119 = f64x8::splat(1.0) / t41 / t40;
            let t120 = f64x8::splat(1.0) / t35;
            let t121 = v_sigma0 * t120;
            let t124 = f64x8::splat(1.0) / t35 / t30;
            let t127 = -f64x8::splat(1.237588837482578) * t121 - f64x8::splat(0.003781792915213529) * t34 * t124;
            let t133 = -f64x8::splat(0.08381554031628043) * t121 + f64x8::splat(0.01228676160669432) * v_tau0 * t31;
            let t137 = f64x8::splat(1.0) / t52 / t40;
            let t138 = t51 * t137;
            let t141 = -t119 * t127 / f64x8::splat(15.0) + f64x8::splat(2.0) / f64x8::splat(5.0) * t133 * t53 - f64x8::splat(2.0) / f64x8::splat(25.0) * t138 * t127;
            let t142 = t28 * t141;
            let t146 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t110 * t57 - t117 - f64x8::splat(2.0) / f64x8::splat(3.0) * t25 * t142));
            let t147 = t62 * t103;
            let t149 = ((t13).select(f64x8::splat(0.0), (t9).select(f64x8::splat(0.0), -t6 - t147)));
            let t152 = ((t66).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(2.0) * t67 * t149));
            let t153 = t4 * t152;
            let t156 = t114 * t97;
            let t158 = t70 * t156 / f64x8::splat(3.0);
            let t160 = ((t61).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t153 * t98 - t158));
            let tvrho0 = t60 + t101 + t5 * (t146 + t160);
            acc_vrho_0 = tvrho0;
            let t164 = ((t9).select(f64x8::splat(0.0), (t13).select(f64x8::splat(0.0), -t6 - t104)));
            let t167 = ((t19).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(2.0) * t22 * t164));
            let t168 = t4 * t167;
            let t172 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t168 * t57 - t117));
            let t174 = ((t13).select(f64x8::splat(0.0), (t9).select(f64x8::splat(0.0), t6 - t147)));
            let t177 = ((t66).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(2.0) * t67 * t174));
            let t178 = t4 * t177;
            let t182 = f64x8::splat(1.0) / t83 / t82;
            let t183 = f64x8::splat(1.0) / t77;
            let t184 = v_sigma2 * t183;
            let t187 = f64x8::splat(1.0) / t77 / t72;
            let t190 = -f64x8::splat(1.237588837482578) * t184 - f64x8::splat(0.003781792915213529) * t76 * t187;
            let t196 = -f64x8::splat(0.08381554031628043) * t184 + f64x8::splat(0.01228676160669432) * v_tau1 * t73;
            let t200 = f64x8::splat(1.0) / t93 / t82;
            let t201 = t92 * t200;
            let t204 = -t182 * t190 / f64x8::splat(15.0) + f64x8::splat(2.0) / f64x8::splat(5.0) * t196 * t94 - f64x8::splat(2.0) / f64x8::splat(25.0) * t201 * t190;
            let t205 = t28 * t204;
            let t209 = ((t61).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t178 * t98 - t158 - f64x8::splat(2.0) / f64x8::splat(3.0) * t70 * t205));
            let tvrho1 = t60 + t101 + t5 * (t172 + t209);
            acc_vrho_1 = tvrho1;
            let t213 = v_sigma0 * t37;
            let t215 = f64x8::splat(0.41252961249419273) * t31 + f64x8::splat(0.0012605976384045096) * t213;
            let t222 = -t119 * t215 / f64x8::splat(15.0) + f64x8::splat(0.011175405375504056) * t31 * t53 - f64x8::splat(2.0) / f64x8::splat(25.0) * t138 * t215;
            let t223 = t28 * t222;
            let t226 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t25 * t223));
            let tvsigma0 = t5 * t226;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t228 = v_sigma2 * t79;
            let t230 = f64x8::splat(0.41252961249419273) * t73 + f64x8::splat(0.0012605976384045096) * t228;
            let t237 = -t182 * t230 / f64x8::splat(15.0) + f64x8::splat(0.011175405375504056) * t73 * t94 - f64x8::splat(2.0) / f64x8::splat(25.0) * t201 * t230;
            let t238 = t28 * t237;
            let t241 = ((t61).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(3.0) * t70 * t238));
            let tvsigma2 = t5 * t241;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t242 = t24 * t26;
            let t244 = t27 * t44 * t53;
            let t247 = ((t2).select(f64x8::splat(0.0), f64x8::splat(0.0009242750552041906) * t242 * t244));
            let tvtau0 = t5 * t247;
            acc_vtau_0 = tvtau0;
            let t248 = t69 * t26;
            let t250 = t27 * t86 * t94;
            let t253 = ((t61).select(f64x8::splat(0.0), f64x8::splat(0.0009242750552041906) * t248 * t250));
            let tvtau1 = t5 * t253;
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
