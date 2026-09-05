//! LDA_C_PW_ERF exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pw_erf.c`
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
pub fn lda_c_pw_erf_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
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
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t1 * t1;
            let t20 = t3 * t3;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t23 = f64x8::splat(1.0) / t22;
            let t24 = t5 * t23;
            let t25 = t21 * t24;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.062182) * t13 * t31;
            let t34 = v_rho0 - v_rho1;
            let t35 = t34 * t34;
            let t36 = t35 * t35;
            let t37 = t7 * t7;
            let t38 = t37 * t37;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t36 * t39;
            let t41 = f64x8::splat(1.0) / t7;
            let t42 = t34 * t41;
            let t43 = f64x8::splat(1.0) + t42;
            let t44 = (t43).simd_le(zeta_threshold);
            let t45 = (simd::cbrt(zeta_threshold));
            let t46 = t45 * zeta_threshold;
            let t47 = (simd::cbrt(t43));
            let t48 = t47 * t43;
            let t49 = ((t44).select(t46, t48));
            let t50 = f64x8::splat(1.0) - t42;
            let t51 = (t50).simd_le(zeta_threshold);
            let t52 = (simd::cbrt(t50));
            let t53 = t52 * t50;
            let t54 = ((t51).select(t46, t53));
            let t55 = t49 + t54 - f64x8::splat(2.0);
            let t56 = f64x8::splat(M_CBRT2);
            let t59 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t56 - f64x8::splat(2.0));
            let t60 = t55 * t59;
            let t62 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t11;
            let t67 = f64x8::splat(7.05945) * t14 + f64x8::splat(1.549425) * t11 + f64x8::splat(0.420775) * t17 + f64x8::splat(0.1562925) * t25;
            let t70 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t67;
            let t71 = (simd::ln(t70));
            let t75 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t80 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t83 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t80;
            let t84 = (simd::ln(t83));
            let t85 = t75 * t84;
            let t87 = -f64x8::splat(0.03109) * t62 * t71 + t33 - f64x8::splat(0.019751789702565206) * t85;
            let t88 = t60 * t87;
            let t89 = t40 * t88;
            let t91 = f64x8::splat(0.019751789702565206) * t60 * t85;
            let t92 = t47 * t47;
            let t93 = t52 * t52;
            let t95 = t92 / f64x8::splat(2.0) + t93 / f64x8::splat(2.0);
            let t96 = t95 * t95;
            let t97 = t96 * t95;
            let t98 = (simd::ln(f64x8::splat(2.0)));
            let t99 = t98 - f64x8::splat(1.0);
            let t100 = f64x8::splat(2.0) * t99;
            let t101 = t97 * t100;
            let t102 = param_hyb_omega_0 * t14;
            let t103 = f64x8::splat(1.0) / t95;
            let t105 = f64x8::splat(2.923025) * t102 * t103;
            let t107 = (simd::cbrt(f64x8::splat(9.0)));
            let t108 = t107 * t107;
            let t116 = param_hyb_omega_0 * param_hyb_omega_0;
            let t117 = (f64x8::splat(3.44851) - f64x8::splat(M_PI) * t5 * t108 * t3 / t99 / f64x8::splat(12.0)) * t116;
            let t118 = t117 * t1;
            let t119 = t3 * t6;
            let t120 = f64x8::splat(1.0) / t96;
            let t125 = t116 * param_hyb_omega_0;
            let t126 = t14 * t11;
            let t127 = t125 * t126;
            let t128 = f64x8::splat(1.0) / t97;
            let t131 = f64x8::splat(1.0) + t105 + t118 * t119 * t9 * t120 / f64x8::splat(4.0) + f64x8::splat(0.48968) * t127 * t128;
            let t132 = t116 * t1;
            let t133 = t132 * t3;
            let t137 = f64x8::splat(1.0) + t105 + f64x8::splat(0.8621275) * t133 * t10 * t120;
            let t138 = f64x8::splat(1.0) / t137;
            let t140 = (simd::ln(t131 * t138));
            let t141 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t142 = f64x8::splat(1.0) / t141;
            let t143 = t140 * t142;
            let t145 = f64x8::splat(1.0) / t37;
            let t147 = -t35 * t145 + f64x8::splat(1.0);
            let t148 = t41 * t147;
            let t152 = t3 * t2;
            let t153 = t1 * t152;
            let t155 = f64x8::splat(1.0) / t8 / t7;
            let t156 = t6 * t155;
            let t159 = f64x8::splat(1.0) + f64x8::splat(0.005175) * t11 + f64x8::splat(0.0204825) * t25 - f64x8::splat(0.0030486129349252553) * t41 + f64x8::splat(0.0003485625) * t153 * t156;
            let t161 = (simd::exp(-f64x8::splat(0.1881) * t11));
            let t162 = t159 * t161;
            let t163 = f64x8::splat(M_SQRT2);
            let t164 = t162 * t163;
            let t168 = t19 * t20 * t142;
            let t169 = t168 * t5;
            let t171 = f64x8::splat(1.0) / t22 / t7;
            let t172 = t43 / f64x8::splat(2.0);
            let t173 = t172 * t172;
            let t174 = t4 * t6;
            let t175 = t9 * t56;
            let t176 = f64x8::splat(1.0) / t43;
            let t177 = (simd::cbrt(t176));
            let t179 = t174 * t175 * t177;
            let t181 = f64x8::splat(1.0) - f64x8::splat(0.0056675) * t179;
            let t182 = t173 * t181;
            let t183 = f64x8::splat(1.0) / t152;
            let t184 = t108 * t183;
            let t185 = t182 * t184;
            let t186 = t1 * t22;
            let t187 = t177 * t177;
            let t188 = f64x8::splat(1.0) / t187;
            let t190 = t21 * t5;
            let t191 = t56 * t56;
            let t192 = t23 * t191;
            let t196 = f64x8::splat(1.0) + f64x8::splat(0.107975) * t179 + f64x8::splat(0.01) * t190 * t192 * t187;
            let t197 = f64x8::splat(1.0) / t196;
            let t198 = t188 * t197;
            let t199 = t186 * t198;
            let t201 = f64x8::splat(2.0) / f64x8::splat(15.0) * t185 * t199;
            let t202 = t50 / f64x8::splat(2.0);
            let t203 = t202 * t202;
            let t204 = f64x8::splat(1.0) / t50;
            let t205 = (simd::cbrt(t204));
            let t207 = t174 * t175 * t205;
            let t209 = f64x8::splat(1.0) - f64x8::splat(0.0056675) * t207;
            let t210 = t203 * t209;
            let t211 = t210 * t184;
            let t212 = t205 * t205;
            let t213 = f64x8::splat(1.0) / t212;
            let t218 = f64x8::splat(1.0) + f64x8::splat(0.107975) * t207 + f64x8::splat(0.01) * t190 * t192 * t212;
            let t219 = f64x8::splat(1.0) / t218;
            let t220 = t213 * t219;
            let t221 = t186 * t220;
            let t223 = f64x8::splat(2.0) / f64x8::splat(15.0) * t211 * t221;
            let t225 = (simd::exp(-f64x8::splat(0.0775) * t11));
            let t226 = t147 * t225;
            let t229 = -f64x8::splat(1.2375) * t11 + t25 / f64x8::splat(4.0);
            let t230 = t229 * f64x8::splat(M_PI);
            let t231 = t230 * t7;
            let t234 = t201 + t223 + f64x8::splat(4.0) / f64x8::splat(3.0) * t226 * t231;
            let t242 = t162 / f64x8::splat(2.0) - f64x8::splat(1.0) / f64x8::splat(2.0);
            let t245 = t5 * t171;
            let t247 = (simd::exp(-f64x8::splat(0.13675) * t11));
            let t248 = t147 * t247;
            let t251 = -f64x8::splat(0.097) * t11 + f64x8::splat(0.169) * t25;
            let t252 = t248 * t251;
            let t254 = t1 / t20;
            let t256 = t254 * t6 * t22;
            let t259 = t43 * t43;
            let t260 = t92 * t259;
            let t261 = t50 * t50;
            let t262 = t93 * t261;
            let t265 = (t260 / f64x8::splat(2.0) + t262 / f64x8::splat(2.0)) * t108;
            let t266 = t183 * t1;
            let t267 = t266 * t22;
            let t270 = t201 + t223 + t252 * t256 / f64x8::splat(3.0) - t265 * t267 / f64x8::splat(15.0);
            let t274 = -t33 + t89 + t91;
            let t279 = t116 * t116;
            let t281 = t168 * t245;
            let t283 = t161 * t163;
            let t285 = t283 * t279 * param_hyb_omega_0;
            let t286 = t147 * t159 * t285;
            let t289 = t171 * t147;
            let t296 = t279 * t116;
            let t299 = f64x8::splat(1.0) / t22 / t37;
            let t301 = t279 * t279;
            let t305 = t101 * t143 + (-f64x8::splat(0.031505407223141116) * t148 * t164 - f64x8::splat(0.005388405304614574) * t169 * t171 * t234 * t163) * t125 + (-f64x8::splat(0.0837628205355044) * t148 * t242 - f64x8::splat(0.011938374665504766) * t168 * t245 * t270 + f64x8::splat(0.42708890021612717) * t153 * t156 * t274) * t279 - f64x8::splat(0.01197423401025461) * t281 * t286 + (-f64x8::splat(0.031835665774679375) * t169 * t289 * t242 + f64x8::splat(0.05332506774217938) * t145 * t274) * t296 + f64x8::splat(0.020267214298646783) * t169 * t299 * t274 * t301;
            let t309 = f64x8::splat(1.0) + f64x8::splat(0.15403623315025) * t21 * t24 * t116;
            let t310 = t309 * t309;
            let t311 = t310 * t310;
            let t312 = f64x8::splat(1.0) / t311;
            let t313 = t305 * t312;
            let tzk0 = -t33 + t89 + t91 - t313;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
