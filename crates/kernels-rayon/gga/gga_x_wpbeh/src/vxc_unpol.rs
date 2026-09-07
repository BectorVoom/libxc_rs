//! GGA_X_WPBEH vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_wpbeh.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_wpbeh_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t12 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t13 = f64x8::splat(1.0) + t12;
            let t14 = (t13).simd_le(zeta_threshold);
            let t15 = (simd::cbrt(zeta_threshold));
            let t17 = (simd::cbrt(t13));
            let t19 = ((t14).select(t15 * zeta_threshold, t17 * t13));
            let t20 = (simd::cbrt(v_rho));
            let t21 = t19 * t20;
            let t22 = t4 * t4;
            let t23 = param_hyb_omega_0 * t22;
            let t24 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t25 = (simd::cbrt(t24));
            let t26 = f64x8::splat(1.0) / t25;
            let t27 = ((t14).select(t15, t17));
            let t29 = t26 / t27;
            let t30 = f64x8::splat(1.0) / t20;
            let t32 = t23 * t29 * t30;
            let t33 = t32 / f64x8::splat(3.0);
            let t34 = (f64x8::splat(14.0)).simd_lt(t33);
            let t35 = f64x8::splat(M_CBRT6);
            let t36 = t35 * t35;
            let t37 = t36 * t26;
            let t38 = ((v_sigma).sqrt());
            let t39 = f64x8::splat(M_CBRT2);
            let t40 = t38 * t39;
            let t42 = f64x8::splat(1.0) / t20 / v_rho;
            let t45 = t37 * t40 * t42 / f64x8::splat(12.0);
            let t46 = (t45).simd_lt(f64x8::splat(1.0));
            let t47 = (f64x8::splat(15.0)).simd_lt(t45);
            let t48 = ((t47).select(f64x8::splat(15.0), t45));
            let t49 = (f64x8::splat(1.0)).simd_lt(t48);
            let t50 = ((t49).select(t48, f64x8::splat(1.0)));
            let t52 = (simd::exp(t50 - f64x8::splat(8.572844)));
            let t53 = f64x8::splat(1.0) + t52;
            let t54 = (simd::ln(t53));
            let t56 = ((t47).select(f64x8::splat(8.572844), t50 - t54));
            let t57 = ((t46).select(t45, t56));
            let t58 = (t57).simd_lt(f64x8::splat(1e-15));
            let t59 = ((t58).select(f64x8::splat(1e-15), t57));
            let t60 = t59 * t59;
            let t62 = t60 * t60;
            let t64 = f64x8::splat(0.00979681) * t60 + f64x8::splat(0.0410834) * t62;
            let t65 = t60 * t64;
            let t67 = t62 * t59;
            let t69 = t62 * t60;
            let t71 = f64x8::splat(1.0) + f64x8::splat(0.18744) * t62 + f64x8::splat(0.00120824) * t67 + f64x8::splat(0.0347188) * t69;
            let t72 = f64x8::splat(1.0) / t71;
            let t73 = t65 * t72;
            let t74 = f64x8::splat(2.214317600459161) * t73;
            let t75 = (t33).simd_lt(f64x8::splat(14.0));
            let t76 = ((t75).select(f64x8::splat(1.455915450052607), f64x8::splat(2.0)));
            let t77 = param_hyb_omega_0 * param_hyb_omega_0;
            let t79 = t76 * t77 * t4;
            let t80 = t25 * t25;
            let t82 = t27 * t27;
            let t84 = f64x8::splat(1.0) / t80 / t82;
            let t85 = t20 * t20;
            let t86 = f64x8::splat(1.0) / t85;
            let t87 = t84 * t86;
            let t88 = t79 * t87;
            let t90 = t74 + f64x8::splat(0.7381058668197203) * t88;
            let t91 = (simd::e1_scaled(t90));
            let t93 = t88 / f64x8::splat(3.0);
            let t94 = f64x8::splat(0.57786348) + t73 + t93;
            let t95 = (simd::ln(t94));
            let t97 = t73 + t93;
            let t98 = (simd::ln(t97));
            let t101 = ((t34).select(f64x8::splat(14.0), t33));
            let t103 = t101 * t101;
            let t104 = t103 * t101;
            let t106 = t103 * t103;
            let t107 = t106 * t101;
            let t109 = t106 * t104;
            let t112 = (f64x8::splat(1.7059169152930056) * t101 - f64x8::splat(4.162270540644039) * t104 + f64x8::splat(4.217437034869465) * t107 - f64x8::splat(1.0676080470633098) * t109) * f64x8::splat(M_PI);
            let t113 = (t101).simd_lt(f64x8::splat(14.0));
            let t114 = ((t113).select(f64x8::splat(1.455915450052607), f64x8::splat(2.0)));
            let t115 = t114 * t103;
            let t117 = t74 + f64x8::splat(2.214317600459161) * t115;
            let t118 = ((t117).sqrt());
            let t119 = (simd::erfcx(t118));
            let t124 = t106 * t103;
            let t126 = t106 * t106;
            let t128 = -f64x8::splat(1.0161144) + f64x8::splat(3.2686565979666846) * t103 - f64x8::splat(4.841839888141759) * t106 + f64x8::splat(2.723636568586566) * t124 - f64x8::splat(0.20524577845574896) * t126;
            let t129 = (simd::e1_scaled(t117));
            let t132 = f64x8::splat(0.57786348) + t73 + t115;
            let t133 = ((t132).sqrt());
            let t134 = f64x8::splat(1.0) / t133;
            let t137 = f64x8::splat(1.0) / t132;
            let t140 = t73 + t115;
            let t141 = ((t140).sqrt());
            let t142 = f64x8::splat(1.0) / t141;
            let t144 = t133 * t132;
            let t145 = f64x8::splat(1.0) / t144;
            let t147 = f64x8::splat(2.478878780461809) * t142 - f64x8::splat(0.5597387610403739) * t145;
            let t149 = f64x8::splat(1.0) / t140;
            let t151 = t132 * t132;
            let t152 = f64x8::splat(1.0) / t151;
            let t154 = -f64x8::splat(1.0933029406300512) * t149 + f64x8::splat(0.49374260512735113) * t152;
            let t156 = t133 * t151;
            let t159 = f64x8::splat(9.0) * t73 + f64x8::splat(9.0) * t115 - f64x8::splat(2.0322288);
            let t162 = t141 * t140;
            let t164 = f64x8::splat(3.0) * t156 * t159 + f64x8::splat(4.12995389554944) * t162;
            let t165 = f64x8::splat(1.0) / t156;
            let t166 = t164 * t165;
            let t167 = f64x8::splat(1.0) / t162;
            let t168 = t167 * t107;
            let t171 = t151 * t132;
            let t172 = f64x8::splat(1.0) / t171;
            let t175 = -f64x8::splat(36.0) + f64x8::splat(79.7154336165298) * t73;
            let t176 = t140 * t140;
            let t177 = f64x8::splat(1.0) / t176;
            let t180 = f64x8::splat(0.2508588461882105) * t172 + f64x8::splat(0.007715016088131) * t175 * t177;
            let t182 = t141 * t176;
            let t184 = t133 * t171;
            let t188 = f64x8::splat(27.0) * t176 - f64x8::splat(6.0966864) * t73 - f64x8::splat(6.0966864) * t115 + f64x8::splat(4.12995389554944);
            let t191 = -f64x8::splat(41.96505624603882) * t182 + f64x8::splat(9.0) * t184 * t188;
            let t192 = f64x8::splat(1.0) / t184;
            let t193 = t191 * t192;
            let t194 = f64x8::splat(1.0) / t182;
            let t195 = t194 * t109;
            let t198 = t151 * t151;
            let t199 = t114 * t198;
            let t202 = t176 * t140;
            let t207 = -f64x8::splat(729.0) * t176 + f64x8::splat(329.2210656) * t73 + f64x8::splat(329.2210656) * t115 - f64x8::splat(297.35668047955966);
            let t210 = f64x8::splat(81.27826616498021) * t199 * t140 + f64x8::splat(3.384784484376542) * t202 + f64x8::splat(0.008401793031216) * t198 * t207;
            let t211 = f64x8::splat(1.0) / t198;
            let t212 = t210 * t211;
            let t213 = f64x8::splat(1.0) / t202;
            let t214 = t213 * t126;
            let t218 = (simd::ln(t140 * t137));
            let t220 = t112 * t119 / f64x8::splat(2.0) - t128 * t129 / f64x8::splat(2.0) - f64x8::splat(1.0159746228068032) * t134 * t101 + f64x8::splat(0.738073119521991) * t137 * t103 + t147 * t104 + t154 * t106 - f64x8::splat(0.09302717396924197) * t166 * t168 + t180 * t124 + f64x8::splat(0.0026165591067112575) * t193 * t195 + f64x8::splat(0.007566670425467926) * t212 * t214 + f64x8::splat(0.5080572) * t218;
            let t221 = ((t34).select(f64x8::splat(0.5080572) * t91 - f64x8::splat(0.5080572) * t95 + f64x8::splat(0.5080572) * t98, t220));
            let t223 = f64x8::splat(0.57786348) + t73;
            let t224 = t223 * t223;
            let t226 = f64x8::splat(0.077215461) * t73;
            let t227 = t223 * t60;
            let t230 = f64x8::splat(6.4753871) * t64 * t72 + f64x8::splat(0.4796583);
            let t233 = (f64x8::splat(0.08)).simd_lt(t59);
            let t235 = ((f64x8::splat(M_PI)).sqrt());
            let t237 = t230 * t60 + f64x8::splat(1.0);
            let t241 = t224 * t223;
            let t244 = t235 * (-f64x8::splat(0.779335965) - f64x8::splat(0.463292766) * t237 * t223 - f64x8::splat(1.48683344) * t224 + f64x8::splat(8.1289152) * t241);
            let t245 = ((t223).sqrt());
            let t246 = t245 * t241;
            let t247 = f64x8::splat(1.0) / t246;
            let t250 = (simd::exp(t74));
            let t251 = ((t73).sqrt());
            let t253 = (simd::erf(f64x8::splat(1.4880583323442536) * t251));
            let t254 = f64x8::splat(1.0) - t253;
            let t255 = t250 * t254;
            let t258 = f64x8::splat(1.0) / t235;
            let t259 = (f64x8::splat(3.0) / f64x8::splat(4.0) * f64x8::splat(M_PI) + t244 * t247 / f64x8::splat(16.0) - f64x8::splat(2.3751029502456897) * t255) * t258;
            let t260 = f64x8::splat(1.0) / t60;
            let t261 = t260 * t246;
            let t267 = ((t233).select(-f64x8::splat(16.0) / f64x8::splat(15.0) * t259 * t261, -f64x8::splat(0.0262841788) - f64x8::splat(0.07117647788) * t60 + f64x8::splat(0.08534541323) * t62));
            let t268 = t60 * t267;
            let t270 = -f64x8::splat(0.37170836) * t224 - f64x8::splat(0.14853145700326428) - t226 - f64x8::splat(0.077215461) * t227 * t230 + f64x8::splat(2.0) * t268;
            let t271 = f64x8::splat(1.0) / t241;
            let t274 = t23 * t29;
            let t275 = t77 * t4;
            let t276 = t275 * t87;
            let t278 = f64x8::splat(0.57786348) + t73 + t276 / f64x8::splat(3.0);
            let t279 = t278 * t278;
            let t283 = t278 * t60;
            let t284 = t283 * t230;
            let t287 = -f64x8::splat(1.48683344) * t279 - f64x8::splat(1.0470559350195856) - f64x8::splat(0.463292766) * t73 - f64x8::splat(0.154430922) * t276 - f64x8::splat(0.463292766) * t284 + f64x8::splat(15.0) * t268;
            let t288 = t30 * t287;
            let t289 = f64x8::splat(1.0) / t223;
            let t290 = ((t278).sqrt());
            let t292 = f64x8::splat(1.0) / t290 / t279;
            let t293 = t289 * t292;
            let t299 = t77 * param_hyb_omega_0 / t24;
            let t301 = f64x8::splat(1.0) / t82 / t27;
            let t302 = t299 * t301;
            let t303 = f64x8::splat(1.0) / v_rho;
            let t307 = -f64x8::splat(0.30439865000326427) - t226 - f64x8::splat(0.025738487) * t276 - f64x8::splat(0.077215461) * t284 + f64x8::splat(5.0) * t268;
            let t309 = f64x8::splat(1.0) / t224;
            let t310 = t309 * t292;
            let t314 = t77 * t77;
            let t316 = t314 * param_hyb_omega_0 * t4;
            let t319 = t82 * t82;
            let t322 = f64x8::splat(1.0) / t80 / t24 / t319 / t27;
            let t323 = t316 * t322;
            let t325 = f64x8::splat(1.0) / t85 / v_rho;
            let t326 = -f64x8::splat(0.051955731) + t268;
            let t327 = t325 * t326;
            let t328 = t271 * t292;
            let t332 = -f64x8::splat(8.0) / f64x8::splat(9.0) * t221 - f64x8::splat(4.0) / f64x8::splat(9.0) * t270 * t271 + t274 * t288 * t293 / f64x8::splat(27.0) + f64x8::splat(4.0) / f64x8::splat(27.0) * t302 * t303 * t307 * t310 + f64x8::splat(8.0) / f64x8::splat(81.0) * t323 * t327 * t328;
            let t336 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t21 * t332));
            let tzk0 = f64x8::splat(2.0) * t336;
            acc_zk = tzk0;
            let t337 = t19 * t86;
            let t342 = t91 - f64x8::splat(1.0) / t90;
            let t343 = t59 * t64;
            let t344 = v_rho * v_rho;
            let t346 = f64x8::splat(1.0) / t20 / t344;
            let t349 = t37 * t40 * t346 / f64x8::splat(9.0);
            let t350 = ((t47).select(f64x8::splat(0.0), -t349));
            let t351 = ((t49).select(t350, f64x8::splat(0.0)));
            let t353 = f64x8::splat(1.0) / t53;
            let t354 = t351 * t52 * t353;
            let t356 = ((t47).select(f64x8::splat(0.0), t351 - t354));
            let t357 = ((t46).select(-t349, t356));
            let t358 = ((t58).select(f64x8::splat(0.0), t357));
            let t359 = t72 * t358;
            let t360 = t343 * t359;
            let t361 = f64x8::splat(4.428635200918322) * t360;
            let t362 = t59 * t358;
            let t364 = t60 * t59;
            let t365 = t364 * t358;
            let t367 = f64x8::splat(0.01959362) * t362 + f64x8::splat(0.1643336) * t365;
            let t368 = t60 * t367;
            let t369 = t368 * t72;
            let t370 = f64x8::splat(2.214317600459161) * t369;
            let t371 = t71 * t71;
            let t372 = f64x8::splat(1.0) / t371;
            let t374 = t62 * t358;
            let t378 = f64x8::splat(0.74976) * t365 + f64x8::splat(0.0060412) * t374 + f64x8::splat(0.2083128) * t67 * t358;
            let t379 = t372 * t378;
            let t380 = t65 * t379;
            let t381 = f64x8::splat(2.214317600459161) * t380;
            let t382 = ((t75).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t384 = t382 * t77 * t4;
            let t385 = t384 * t87;
            let t386 = f64x8::splat(0.7381058668197203) * t385;
            let t387 = t84 * t325;
            let t388 = t79 * t387;
            let t390 = t361 + t370 - t381 + t386 - f64x8::splat(0.4920705778798135) * t388;
            let t391 = t342 * t390;
            let t393 = f64x8::splat(2.0) * t360;
            let t394 = t385 / f64x8::splat(3.0);
            let t396 = t393 + t369 - t380 + t394 - f64x8::splat(2.0) / f64x8::splat(9.0) * t388;
            let t397 = f64x8::splat(1.0) / t94;
            let t400 = f64x8::splat(1.0) / t97;
            let t404 = ((t113).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t405 = t404 * t103;
            let t406 = t114 * t101;
            let t408 = t23 * t29 * t42;
            let t410 = ((t34).select(f64x8::splat(0.0), -t408 / f64x8::splat(9.0)));
            let t411 = t406 * t410;
            let t413 = t393 + t369 - t380 + t405 + f64x8::splat(2.0) * t411;
            let t414 = t211 * t413;
            let t419 = f64x8::splat(159.4308672330596) * t360 + f64x8::splat(79.7154336165298) * t369 - f64x8::splat(79.7154336165298) * t380;
            let t422 = t175 * t213;
            let t425 = -f64x8::splat(0.7525765385646315) * t414 + f64x8::splat(0.007715016088131) * t419 * t177 - f64x8::splat(0.015430032176262) * t422 * t413;
            let t428 = t140 * t152;
            let t430 = t413 * t137 - t428 * t413;
            let t431 = t430 * t149;
            let t434 = t152 * t103;
            let t437 = t137 * t101;
            let t440 = t147 * t103;
            let t443 = t154 * t104;
            let t446 = t180 * t107;
            let t450 = t103 * t410;
            let t452 = t106 * t410;
            let t454 = t124 * t410;
            let t457 = (f64x8::splat(1.7059169152930056) * t410 - f64x8::splat(12.48681162193212) * t450 + f64x8::splat(21.087185174347326) * t452 - f64x8::splat(7.473256329443169) * t454) * f64x8::splat(M_PI);
            let t460 = f64x8::splat(1.0) / t117;
            let t461 = t129 - t460;
            let t462 = t128 * t461;
            let t463 = f64x8::splat(2.214317600459161) * t405;
            let t465 = t361 + t370 - t381 + t463 + f64x8::splat(4.428635200918322) * t411;
            let t468 = t145 * t101;
            let t473 = f64x8::splat(2.0) * t118 * t119 - f64x8::splat(2.0) * t258;
            let t474 = f64x8::splat(1.0) / t118;
            let t475 = t473 * t474;
            let t476 = t475 * t465;
            let t479 = t176 * t176;
            let t480 = f64x8::splat(1.0) / t479;
            let t481 = t480 * t126;
            let t482 = t481 * t413;
            let t485 = t213 * t109;
            let t486 = t485 * t410;
            let t489 = t425 * t124 + f64x8::splat(0.5080572) * t431 * t132 - f64x8::splat(0.738073119521991) * t434 * t413 + f64x8::splat(1.476146239043982) * t437 * t410 + f64x8::splat(3.0) * t440 * t410 + f64x8::splat(4.0) * t443 * t410 + f64x8::splat(6.0) * t446 * t410 + t457 * t119 / f64x8::splat(2.0) - t462 * t465 / f64x8::splat(2.0) + f64x8::splat(0.5079873114034016) * t468 * t413 + t112 * t476 / f64x8::splat(4.0) - f64x8::splat(0.022700011276403777) * t212 * t482 + f64x8::splat(0.060533363403743407) * t212 * t486;
            let t490 = t198 * t132;
            let t491 = f64x8::splat(1.0) / t490;
            let t492 = t210 * t491;
            let t493 = t214 * t413;
            let t496 = t194 * t124;
            let t497 = t496 * t410;
            let t501 = f64x8::splat(1.0) / t133 / t198;
            let t502 = t191 * t501;
            let t503 = t195 * t413;
            let t507 = f64x8::splat(1.0) / t141 / t202;
            let t508 = t507 * t109;
            let t509 = t508 * t413;
            let t512 = t194 * t107;
            let t513 = t512 * t413;
            let t516 = t167 * t106;
            let t517 = t516 * t410;
            let t520 = t164 * t192;
            let t521 = t168 * t413;
            let t524 = t144 * t159;
            let t530 = f64x8::splat(9.0) * t405;
            let t532 = f64x8::splat(18.0) * t360 + f64x8::splat(9.0) * t369 - f64x8::splat(9.0) * t380 + t530 + f64x8::splat(18.0) * t411;
            let t535 = t141 * t413;
            let t537 = f64x8::splat(15.0) / f64x8::splat(2.0) * t524 * t413 + f64x8::splat(3.0) * t156 * t532 + f64x8::splat(6.19493084332416) * t535;
            let t538 = t537 * t165;
            let t541 = t404 * t198;
            let t543 = f64x8::splat(81.27826616498021) * t541 * t140;
            let t544 = t114 * t171;
            let t545 = t140 * t413;
            let t552 = t171 * t207;
            let t559 = f64x8::splat(329.2210656) * t405;
            let t561 = -f64x8::splat(1458.0) * t545 + f64x8::splat(658.4421312) * t360 + f64x8::splat(329.2210656) * t369 - f64x8::splat(329.2210656) * t380 + t559 + f64x8::splat(658.4421312) * t411;
            let t564 = t543 + f64x8::splat(325.11306465992084) * t544 * t545 + f64x8::splat(81.27826616498021) * t199 * t413 + f64x8::splat(10.154353453129625) * t176 * t413 + f64x8::splat(0.033607172124864) * t552 * t413 + f64x8::splat(0.008401793031216) * t198 * t561;
            let t565 = t564 * t211;
            let t570 = t156 * t188;
            let t577 = f64x8::splat(6.0966864) * t405;
            let t579 = f64x8::splat(54.0) * t545 - f64x8::splat(12.1933728) * t360 - f64x8::splat(6.0966864) * t369 + f64x8::splat(6.0966864) * t380 - t577 - f64x8::splat(12.1933728) * t411;
            let t582 = -f64x8::splat(104.91264061509705) * t162 * t413 + f64x8::splat(63.0) / f64x8::splat(2.0) * t570 * t413 + f64x8::splat(9.0) * t184 * t579;
            let t583 = t582 * t192;
            let t588 = t101 * t410;
            let t590 = t104 * t410;
            let t592 = t107 * t410;
            let t594 = t109 * t410;
            let t596 = f64x8::splat(6.537313195933369) * t588 - f64x8::splat(19.367359552567034) * t590 + f64x8::splat(16.341819411519396) * t592 - f64x8::splat(1.6419662276459916) * t594;
            let t603 = f64x8::splat(1.0933029406300512) * t177 * t413 - f64x8::splat(0.9874852102547023) * t172 * t413;
            let t609 = -f64x8::splat(1.2394393902309044) * t167 * t413 + f64x8::splat(0.8396081415605609) * t165 * t413;
            let t611 = -f64x8::splat(0.030266681701871703) * t492 * t493 + f64x8::splat(0.0183159137469788) * t193 * t497 - f64x8::splat(0.0091579568734894) * t502 * t503 - f64x8::splat(0.006541397766778144) * t193 * t509 + f64x8::splat(0.13954076095386297) * t166 * t513 - f64x8::splat(0.46513586984620986) * t166 * t517 + f64x8::splat(0.23256793492310493) * t520 * t521 - f64x8::splat(0.09302717396924197) * t538 * t168 + f64x8::splat(0.007566670425467926) * t565 * t214 + f64x8::splat(0.0026165591067112575) * t583 * t195 - f64x8::splat(1.0159746228068032) * t134 * t410 - t596 * t129 / f64x8::splat(2.0) + t603 * t106 + t609 * t104;
            let t613 = ((t34).select(f64x8::splat(0.5080572) * t391 - f64x8::splat(0.5080572) * t396 * t397 + f64x8::splat(0.5080572) * t396 * t400, t489 + t611));
            let t615 = t393 + t369 - t380;
            let t616 = t223 * t615;
            let t618 = f64x8::splat(0.154430922) * t360;
            let t619 = f64x8::splat(0.077215461) * t369;
            let t620 = f64x8::splat(0.077215461) * t380;
            let t621 = t615 * t60;
            let t624 = t223 * t59;
            let t625 = t230 * t358;
            let t630 = t64 * t372;
            let t633 = f64x8::splat(6.4753871) * t367 * t72 - f64x8::splat(6.4753871) * t630 * t378;
            let t636 = t59 * t267;
            let t637 = t636 * t358;
            let t640 = t230 * t59;
            let t643 = f64x8::splat(2.0) * t640 * t358 + t633 * t60;
            let t652 = t235 * (-f64x8::splat(0.463292766) * t643 * t223 - f64x8::splat(0.463292766) * t237 * t615 - f64x8::splat(2.97366688) * t616 + f64x8::splat(24.3867456) * t224 * t615);
            let t655 = t224 * t224;
            let t657 = f64x8::splat(1.0) / t245 / t655;
            let t658 = t657 * t615;
            let t661 = t361 + t370 - t381;
            let t662 = t661 * t250;
            let t663 = t662 * t254;
            let t665 = (simd::exp(-t74));
            let t666 = t250 * t665;
            let t667 = f64x8::splat(1.0) / t251;
            let t668 = t667 * t615;
            let t672 = (t652 * t247 / f64x8::splat(16.0) - f64x8::splat(7.0) / f64x8::splat(32.0) * t244 * t658 - f64x8::splat(2.3751029502456897) * t663 + f64x8::splat(1.9940105822687055) * t666 * t668) * t258;
            let t675 = f64x8::splat(1.0) / t364;
            let t676 = t675 * t246;
            let t677 = t676 * t358;
            let t680 = t245 * t224;
            let t681 = t260 * t680;
            let t682 = t681 * t615;
            let t689 = ((t233).select(-f64x8::splat(16.0) / f64x8::splat(15.0) * t672 * t261 + f64x8::splat(32.0) / f64x8::splat(15.0) * t259 * t677 - f64x8::splat(56.0) / f64x8::splat(15.0) * t259 * t682, -f64x8::splat(0.14235295576) * t362 + f64x8::splat(0.34138165292) * t365));
            let t690 = t60 * t689;
            let t692 = -f64x8::splat(0.74341672) * t616 - t618 - t619 + t620 - f64x8::splat(0.077215461) * t621 * t230 - f64x8::splat(0.154430922) * t624 * t625 - f64x8::splat(0.077215461) * t227 * t633 + f64x8::splat(4.0) * t637 + f64x8::splat(2.0) * t690;
            let t695 = f64x8::splat(1.0) / t655;
            let t696 = t270 * t695;
            let t699 = t42 * t287;
            let t703 = t275 * t387;
            let t705 = t393 + t369 - t380 - f64x8::splat(2.0) / f64x8::splat(9.0) * t703;
            let t712 = t705 * t60;
            let t713 = t712 * t230;
            let t715 = t278 * t59;
            let t716 = t715 * t625;
            let t718 = t283 * t633;
            let t722 = -f64x8::splat(2.97366688) * t278 * t705 - f64x8::splat(0.926585532) * t360 - f64x8::splat(0.463292766) * t369 + f64x8::splat(0.463292766) * t380 + f64x8::splat(0.102953948) * t703 - f64x8::splat(0.463292766) * t713 - f64x8::splat(0.926585532) * t716 - f64x8::splat(0.463292766) * t718 + f64x8::splat(30.0) * t637 + f64x8::splat(15.0) * t690;
            let t723 = t30 * t722;
            let t727 = t310 * t615;
            let t733 = f64x8::splat(1.0) / t290 / t279 / t278;
            let t734 = t289 * t733;
            let t735 = t734 * t705;
            let t739 = f64x8::splat(1.0) / t344;
            let t750 = -t618 - t619 + t620 + f64x8::splat(0.017158991333333335) * t703 - f64x8::splat(0.077215461) * t713 - f64x8::splat(0.154430922) * t716 - f64x8::splat(0.077215461) * t718 + f64x8::splat(10.0) * t637 + f64x8::splat(5.0) * t690;
            let t755 = t301 * t303;
            let t756 = t299 * t755;
            let t757 = t307 * t271;
            let t758 = t292 * t615;
            let t759 = t757 * t758;
            let t762 = t307 * t309;
            let t763 = t733 * t705;
            let t764 = t762 * t763;
            let t768 = f64x8::splat(1.0) / t85 / t344;
            let t769 = t768 * t326;
            let t774 = f64x8::splat(2.0) * t637 + t690;
            let t775 = t325 * t774;
            let t779 = t695 * t292;
            let t780 = t779 * t615;
            let t784 = t271 * t733;
            let t785 = t784 * t705;
            let t789 = -f64x8::splat(8.0) / f64x8::splat(9.0) * t613 - f64x8::splat(4.0) / f64x8::splat(9.0) * t692 * t271 + f64x8::splat(4.0) / f64x8::splat(3.0) * t696 * t615 - t274 * t699 * t293 / f64x8::splat(81.0) + t274 * t723 * t293 / f64x8::splat(27.0) - t274 * t288 * t727 / f64x8::splat(27.0) - f64x8::splat(5.0) / f64x8::splat(54.0) * t274 * t288 * t735 - f64x8::splat(4.0) / f64x8::splat(27.0) * t302 * t739 * t307 * t310 + f64x8::splat(4.0) / f64x8::splat(27.0) * t302 * t303 * t750 * t310 - f64x8::splat(8.0) / f64x8::splat(27.0) * t756 * t759 - f64x8::splat(10.0) / f64x8::splat(27.0) * t756 * t764 - f64x8::splat(40.0) / f64x8::splat(243.0) * t323 * t769 * t328 + f64x8::splat(8.0) / f64x8::splat(81.0) * t323 * t775 * t328 - f64x8::splat(8.0) / f64x8::splat(27.0) * t323 * t327 * t780 - f64x8::splat(20.0) / f64x8::splat(81.0) * t323 * t327 * t785;
            let t794 = ((t3).select(f64x8::splat(0.0), -t7 * t337 * t332 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t21 * t789));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t794 + f64x8::splat(2.0) * t336;
            acc_vrho = tvrho0;
            let t798 = f64x8::splat(1.0) / t38 * t39;
            let t801 = t37 * t798 * t42 / f64x8::splat(24.0);
            let t802 = ((t47).select(f64x8::splat(0.0), t801));
            let t803 = ((t49).select(t802, f64x8::splat(0.0)));
            let t804 = t803 * t52;
            let t807 = ((t47).select(f64x8::splat(0.0), -t804 * t353 + t803));
            let t808 = ((t46).select(t801, t807));
            let t809 = ((t58).select(f64x8::splat(0.0), t808));
            let t810 = t72 * t809;
            let t811 = t343 * t810;
            let t813 = t59 * t809;
            let t815 = t364 * t809;
            let t817 = f64x8::splat(0.01959362) * t813 + f64x8::splat(0.1643336) * t815;
            let t818 = t60 * t817;
            let t819 = t818 * t72;
            let t822 = t62 * t809;
            let t826 = f64x8::splat(0.74976) * t815 + f64x8::splat(0.0060412) * t822 + f64x8::splat(0.2083128) * t67 * t809;
            let t827 = t372 * t826;
            let t828 = t65 * t827;
            let t830 = f64x8::splat(4.428635200918322) * t811 + f64x8::splat(2.214317600459161) * t819 - f64x8::splat(2.214317600459161) * t828;
            let t831 = t342 * t830;
            let t834 = f64x8::splat(2.0) * t811 + t819 - t828;
            let t840 = t475 * t830;
            let t849 = t167 * t834;
            let t853 = -f64x8::splat(1.2394393902309044) * t849 + f64x8::splat(0.8396081415605609) * t165 * t834;
            let t857 = t172 * t834;
            let t859 = f64x8::splat(1.0933029406300512) * t177 * t834 - f64x8::splat(0.9874852102547023) * t857;
            let t866 = f64x8::splat(18.0) * t811 + f64x8::splat(9.0) * t819 - f64x8::splat(9.0) * t828;
            let t869 = t141 * t834;
            let t871 = f64x8::splat(15.0) / f64x8::splat(2.0) * t524 * t834 + f64x8::splat(3.0) * t156 * t866 + f64x8::splat(6.19493084332416) * t869;
            let t872 = t871 * t165;
            let t875 = t168 * t834;
            let t878 = t512 * t834;
            let t881 = t211 * t834;
            let t886 = f64x8::splat(159.4308672330596) * t811 + f64x8::splat(79.7154336165298) * t819 - f64x8::splat(79.7154336165298) * t828;
            let t891 = -f64x8::splat(0.7525765385646315) * t881 + f64x8::splat(0.007715016088131) * t886 * t177 - f64x8::splat(0.015430032176262) * t422 * t834;
            let t897 = t140 * t834;
            let t902 = f64x8::splat(54.0) * t897 - f64x8::splat(12.1933728) * t811 - f64x8::splat(6.0966864) * t819 + f64x8::splat(6.0966864) * t828;
            let t905 = -f64x8::splat(104.91264061509705) * t162 * t834 + f64x8::splat(63.0) / f64x8::splat(2.0) * t570 * t834 + f64x8::splat(9.0) * t184 * t902;
            let t906 = t905 * t192;
            let t909 = t195 * t834;
            let t912 = t508 * t834;
            let t927 = -f64x8::splat(1458.0) * t897 + f64x8::splat(658.4421312) * t811 + f64x8::splat(329.2210656) * t819 - f64x8::splat(329.2210656) * t828;
            let t930 = f64x8::splat(325.11306465992084) * t544 * t897 + f64x8::splat(81.27826616498021) * t199 * t834 + f64x8::splat(10.154353453129625) * t176 * t834 + f64x8::splat(0.033607172124864) * t552 * t834 + f64x8::splat(0.008401793031216) * t198 * t927;
            let t931 = t930 * t211;
            let t934 = t214 * t834;
            let t937 = t481 * t834;
            let t942 = t834 * t137 - t428 * t834;
            let t943 = t942 * t149;
            let t946 = t112 * t840 / f64x8::splat(4.0) - t462 * t830 / f64x8::splat(2.0) + f64x8::splat(0.5079873114034016) * t468 * t834 - f64x8::splat(0.738073119521991) * t434 * t834 + t853 * t104 + t859 * t106 - f64x8::splat(0.09302717396924197) * t872 * t168 + f64x8::splat(0.23256793492310493) * t520 * t875 + f64x8::splat(0.13954076095386297) * t166 * t878 + t891 * t124 + f64x8::splat(0.0026165591067112575) * t906 * t195 - f64x8::splat(0.0091579568734894) * t502 * t909 - f64x8::splat(0.006541397766778144) * t193 * t912 + f64x8::splat(0.007566670425467926) * t931 * t214 - f64x8::splat(0.030266681701871703) * t492 * t934 - f64x8::splat(0.022700011276403777) * t212 * t937 + f64x8::splat(0.5080572) * t943 * t132;
            let t947 = ((t34).select(f64x8::splat(0.5080572) * t831 - f64x8::splat(0.5080572) * t834 * t397 + f64x8::splat(0.5080572) * t834 * t400, t946));
            let t949 = t223 * t834;
            let t951 = f64x8::splat(0.154430922) * t811;
            let t952 = f64x8::splat(0.077215461) * t819;
            let t953 = f64x8::splat(0.077215461) * t828;
            let t954 = t834 * t60;
            let t955 = t954 * t230;
            let t956 = f64x8::splat(0.077215461) * t955;
            let t957 = t230 * t809;
            let t964 = f64x8::splat(6.4753871) * t817 * t72 - f64x8::splat(6.4753871) * t630 * t826;
            let t967 = t636 * t809;
            let t972 = t964 * t60 + f64x8::splat(2.0) * t640 * t809;
            let t981 = t235 * (-f64x8::splat(0.463292766) * t972 * t223 - f64x8::splat(0.463292766) * t237 * t834 - f64x8::splat(2.97366688) * t949 + f64x8::splat(24.3867456) * t224 * t834);
            let t984 = t657 * t834;
            let t987 = t830 * t250;
            let t990 = t667 * t834;
            let t994 = (t981 * t247 / f64x8::splat(16.0) - f64x8::splat(7.0) / f64x8::splat(32.0) * t244 * t984 - f64x8::splat(2.3751029502456897) * t987 * t254 + f64x8::splat(1.9940105822687055) * t666 * t990) * t258;
            let t997 = t676 * t809;
            let t1000 = t681 * t834;
            let t1007 = ((t233).select(-f64x8::splat(16.0) / f64x8::splat(15.0) * t994 * t261 + f64x8::splat(32.0) / f64x8::splat(15.0) * t259 * t997 - f64x8::splat(56.0) / f64x8::splat(15.0) * t259 * t1000, -f64x8::splat(0.14235295576) * t813 + f64x8::splat(0.34138165292) * t815));
            let t1008 = t60 * t1007;
            let t1010 = -f64x8::splat(0.74341672) * t949 - t951 - t952 + t953 - t956 - f64x8::splat(0.154430922) * t624 * t957 - f64x8::splat(0.077215461) * t227 * t964 + f64x8::splat(4.0) * t967 + f64x8::splat(2.0) * t1008;
            let t1021 = t715 * t957;
            let t1023 = t283 * t964;
            let t1027 = -f64x8::splat(2.97366688) * t278 * t834 - f64x8::splat(0.926585532) * t811 - f64x8::splat(0.463292766) * t819 + f64x8::splat(0.463292766) * t828 - f64x8::splat(0.463292766) * t955 - f64x8::splat(0.926585532) * t1021 - f64x8::splat(0.463292766) * t1023 + f64x8::splat(30.0) * t967 + f64x8::splat(15.0) * t1008;
            let t1028 = t30 * t1027;
            let t1032 = t310 * t834;
            let t1036 = t734 * t834;
            let t1044 = -t951 - t952 + t953 - t956 - f64x8::splat(0.154430922) * t1021 - f64x8::splat(0.077215461) * t1023 + f64x8::splat(10.0) * t967 + f64x8::splat(5.0) * t1008;
            let t1049 = t292 * t834;
            let t1050 = t757 * t1049;
            let t1053 = t733 * t834;
            let t1054 = t762 * t1053;
            let t1058 = f64x8::splat(2.0) * t967 + t1008;
            let t1059 = t325 * t1058;
            let t1063 = t779 * t834;
            let t1067 = t784 * t834;
            let t1071 = -f64x8::splat(8.0) / f64x8::splat(9.0) * t947 - f64x8::splat(4.0) / f64x8::splat(9.0) * t1010 * t271 + f64x8::splat(4.0) / f64x8::splat(3.0) * t696 * t834 + t274 * t1028 * t293 / f64x8::splat(27.0) - t274 * t288 * t1032 / f64x8::splat(27.0) - f64x8::splat(5.0) / f64x8::splat(54.0) * t274 * t288 * t1036 + f64x8::splat(4.0) / f64x8::splat(27.0) * t302 * t303 * t1044 * t310 - f64x8::splat(8.0) / f64x8::splat(27.0) * t756 * t1050 - f64x8::splat(10.0) / f64x8::splat(27.0) * t756 * t1054 + f64x8::splat(8.0) / f64x8::splat(81.0) * t323 * t1059 * t328 - f64x8::splat(8.0) / f64x8::splat(27.0) * t323 * t327 * t1063 - f64x8::splat(20.0) / f64x8::splat(81.0) * t323 * t327 * t1067;
            let t1075 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t21 * t1071));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t1075;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}
