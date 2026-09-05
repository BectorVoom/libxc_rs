//! GGA_X_LG93 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lg93.c`
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
pub fn gga_x_lg93_vxc_pol(
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
            let t26 = t5 * t25;
            let t27 = (simd::cbrt(t6));
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t28 * t32;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t40 = t33 * v_sigma0 * t38;
            let t42 = t28 * t28;
            let t44 = f64x8::splat(1.0) / t30 / t29;
            let t45 = t42 * t44;
            let t46 = v_sigma0 * v_sigma0;
            let t47 = t34 * t34;
            let t48 = t47 * v_rho0;
            let t50 = f64x8::splat(1.0) / t35 / t48;
            let t54 = t46 * v_sigma0;
            let t55 = t47 * t47;
            let t56 = f64x8::splat(1.0) / t55;
            let t59 = t29 * t29;
            let t62 = t28 / t31 / t59;
            let t63 = t46 * t46;
            let t64 = t55 * t34;
            let t66 = f64x8::splat(1.0) / t36 / t64;
            let t73 = t42 / t30 / t59 / t29;
            let t74 = t63 * v_sigma0;
            let t75 = t55 * t48;
            let t77 = f64x8::splat(1.0) / t35 / t75;
            let t81 = t63 * t46;
            let t82 = t55 * t55;
            let t83 = f64x8::splat(1.0) / t82;
            let t86 = f64x8::splat(1.0) + f64x8::splat(0.2058807993646726) * t40 + f64x8::splat(0.05171875) * t45 * t46 * t50 + f64x8::splat(9.988390807433105e-05) * t54 * t56 + f64x8::splat(0.00021916594328703703) * t62 * t63 * t66 + f64x8::splat(0.0011831024546682099) * t73 * t74 * t77 + f64x8::splat(1.1106816177675317e-09) * t81 * t83;
            let t87 = (simd::pow(t86, f64x8::splat(0.024974)));
            let t88 = t27 * t87;
            let t90 = f64x8::splat(1.0) + f64x8::splat(4.166666666666667e-10) * t40;
            let t91 = f64x8::splat(1.0) / t90;
            let t92 = t88 * t91;
            let t95 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t92));
            let t96 = (v_rho1).simd_le(dens_threshold);
            let t97 = -t16;
            let t99 = ((t14).select(t11, (t10).select(t15, t97 * t7)));
            let t100 = f64x8::splat(1.0) + t99;
            let t101 = (t100).simd_le(zeta_threshold);
            let t102 = (simd::cbrt(t100));
            let t104 = ((t101).select(t22, t102 * t100));
            let t105 = t5 * t104;
            let t106 = v_rho1 * v_rho1;
            let t107 = (simd::cbrt(v_rho1));
            let t108 = t107 * t107;
            let t110 = f64x8::splat(1.0) / t108 / t106;
            let t112 = t33 * v_sigma2 * t110;
            let t114 = v_sigma2 * v_sigma2;
            let t115 = t106 * t106;
            let t116 = t115 * v_rho1;
            let t118 = f64x8::splat(1.0) / t107 / t116;
            let t122 = t114 * v_sigma2;
            let t123 = t115 * t115;
            let t124 = f64x8::splat(1.0) / t123;
            let t127 = t114 * t114;
            let t128 = t123 * t106;
            let t130 = f64x8::splat(1.0) / t108 / t128;
            let t134 = t127 * v_sigma2;
            let t135 = t123 * t116;
            let t137 = f64x8::splat(1.0) / t107 / t135;
            let t141 = t127 * t114;
            let t142 = t123 * t123;
            let t143 = f64x8::splat(1.0) / t142;
            let t146 = f64x8::splat(1.0) + f64x8::splat(0.2058807993646726) * t112 + f64x8::splat(0.05171875) * t45 * t114 * t118 + f64x8::splat(9.988390807433105e-05) * t122 * t124 + f64x8::splat(0.00021916594328703703) * t62 * t127 * t130 + f64x8::splat(0.0011831024546682099) * t73 * t134 * t137 + f64x8::splat(1.1106816177675317e-09) * t141 * t143;
            let t147 = (simd::pow(t146, f64x8::splat(0.024974)));
            let t148 = t27 * t147;
            let t150 = f64x8::splat(1.0) + f64x8::splat(4.166666666666667e-10) * t112;
            let t151 = f64x8::splat(1.0) / t150;
            let t152 = t148 * t151;
            let t155 = ((t96).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t105 * t152));
            let tzk0 = t95 + t155;
            acc_zk = tzk0;
            let t156 = t6 * t6;
            let t157 = f64x8::splat(1.0) / t156;
            let t158 = t16 * t157;
            let t160 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t158)));
            let t163 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t160));
            let t164 = t5 * t163;
            let t167 = t27 * t27;
            let t168 = f64x8::splat(1.0) / t167;
            let t169 = t168 * t87;
            let t170 = t169 * t91;
            let t172 = t26 * t170 / f64x8::splat(8.0);
            let t173 = (simd::pow(t86, -f64x8::splat(0.975026)));
            let t174 = t27 * t173;
            let t175 = t34 * v_rho0;
            let t177 = f64x8::splat(1.0) / t36 / t175;
            let t178 = v_sigma0 * t177;
            let t181 = t47 * t34;
            let t183 = f64x8::splat(1.0) / t35 / t181;
            let t187 = t55 * v_rho0;
            let t188 = f64x8::splat(1.0) / t187;
            let t191 = t55 * t175;
            let t193 = f64x8::splat(1.0) / t36 / t191;
            let t197 = t55 * t181;
            let t199 = f64x8::splat(1.0) / t35 / t197;
            let t203 = t82 * v_rho0;
            let t204 = f64x8::splat(1.0) / t203;
            let t207 = -f64x8::splat(0.5490154649724602) * t33 * t178 - f64x8::splat(0.2758333333333333) * t45 * t46 * t183 - f64x8::splat(0.0007990712645946484) * t54 * t188 - f64x8::splat(0.0023377700617283953) * t62 * t63 * t193 - f64x8::splat(0.01577469939557613) * t73 * t74 * t199 - f64x8::splat(1.7770905884280507e-08) * t81 * t204;
            let t208 = t91 * t207;
            let t209 = t174 * t208;
            let t212 = t2 * t25;
            let t213 = t212 * t88;
            let t214 = t90 * t90;
            let t215 = f64x8::splat(1.0) / t214;
            let t216 = t215 * t28;
            let t217 = t32 * v_sigma0;
            let t218 = t217 * t177;
            let t219 = t216 * t218;
            let t223 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t164 * t92 - t172 - f64x8::splat(0.00936525) * t26 * t209 - f64x8::splat(2.8449335968970655e-10) * t213 * t219));
            let t224 = t97 * t157;
            let t226 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t224)));
            let t229 = ((t101).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t102 * t226));
            let t230 = t5 * t229;
            let t233 = t168 * t147;
            let t234 = t233 * t151;
            let t236 = t105 * t234 / f64x8::splat(8.0);
            let t238 = ((t96).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t230 * t152 - t236));
            let tvrho0 = t95 + t155 + t6 * (t223 + t238);
            acc_vrho_0 = tvrho0;
            let t242 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t158)));
            let t245 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t242));
            let t246 = t5 * t245;
            let t250 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t246 * t92 - t172));
            let t252 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t224)));
            let t255 = ((t101).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t102 * t252));
            let t256 = t5 * t255;
            let t259 = (simd::pow(t146, -f64x8::splat(0.975026)));
            let t260 = t27 * t259;
            let t261 = t106 * v_rho1;
            let t263 = f64x8::splat(1.0) / t108 / t261;
            let t264 = v_sigma2 * t263;
            let t267 = t115 * t106;
            let t269 = f64x8::splat(1.0) / t107 / t267;
            let t273 = t123 * v_rho1;
            let t274 = f64x8::splat(1.0) / t273;
            let t277 = t123 * t261;
            let t279 = f64x8::splat(1.0) / t108 / t277;
            let t283 = t123 * t267;
            let t285 = f64x8::splat(1.0) / t107 / t283;
            let t289 = t142 * v_rho1;
            let t290 = f64x8::splat(1.0) / t289;
            let t293 = -f64x8::splat(0.5490154649724602) * t33 * t264 - f64x8::splat(0.2758333333333333) * t45 * t114 * t269 - f64x8::splat(0.0007990712645946484) * t122 * t274 - f64x8::splat(0.0023377700617283953) * t62 * t127 * t279 - f64x8::splat(0.01577469939557613) * t73 * t134 * t285 - f64x8::splat(1.7770905884280507e-08) * t141 * t290;
            let t294 = t151 * t293;
            let t295 = t260 * t294;
            let t298 = t2 * t104;
            let t299 = t298 * t148;
            let t300 = t150 * t150;
            let t301 = f64x8::splat(1.0) / t300;
            let t302 = t301 * t28;
            let t303 = t32 * v_sigma2;
            let t304 = t303 * t263;
            let t305 = t302 * t304;
            let t309 = ((t96).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t256 * t152 - t236 - f64x8::splat(0.00936525) * t105 * t295 - f64x8::splat(2.8449335968970655e-10) * t299 * t305));
            let tvrho1 = t95 + t155 + t6 * (t250 + t309);
            acc_vrho_1 = tvrho1;
            let t312 = t33 * t38;
            let t327 = f64x8::splat(0.2058807993646726) * t312 + f64x8::splat(0.1034375) * t45 * v_sigma0 * t50 + f64x8::splat(0.00029965172422299316) * t46 * t56 + f64x8::splat(0.0008766637731481481) * t62 * t54 * t66 + f64x8::splat(0.005915512273341049) * t73 * t63 * t77 + f64x8::splat(6.66408970660519e-09) * t74 * t83;
            let t328 = t91 * t327;
            let t329 = t174 * t328;
            let t332 = t32 * t38;
            let t333 = t216 * t332;
            let t337 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(0.00936525) * t26 * t329 + f64x8::splat(1.0668500988363994e-10) * t213 * t333));
            let tvsigma0 = t6 * t337;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t338 = t33 * t110;
            let t353 = f64x8::splat(0.2058807993646726) * t338 + f64x8::splat(0.1034375) * t45 * v_sigma2 * t118 + f64x8::splat(0.00029965172422299316) * t114 * t124 + f64x8::splat(0.0008766637731481481) * t62 * t122 * t130 + f64x8::splat(0.005915512273341049) * t73 * t127 * t137 + f64x8::splat(6.66408970660519e-09) * t134 * t143;
            let t354 = t151 * t353;
            let t355 = t260 * t354;
            let t358 = t32 * t110;
            let t359 = t302 * t358;
            let t363 = ((t96).select(f64x8::splat(0.0), -f64x8::splat(0.00936525) * t105 * t355 + f64x8::splat(1.0668500988363994e-10) * t299 * t359));
            let tvsigma2 = t6 * t363;
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
