//! GGA_X_WPBEH exc unpol kernel — explicit SIMD (bit-exact).
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
pub fn gga_x_wpbeh_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    // Loop-invariant bindings (constants, parameters, thresholds):
    // the same statements maple2c emits per point, evaluated once.
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
    let t22 = t4 * t4;
    let t23 = param_hyb_omega_0 * t22;
    let t24 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
    let t25 = (simd::cbrt(t24));
    let t26 = f64x8::splat(1.0) / t25;
    let t27 = ((t14).select(t15, t17));
    let t29 = t26 / t27;
    let t35 = f64x8::splat(M_CBRT6);
    let t36 = t35 * t35;
    let t37 = t36 * t26;
    let t39 = f64x8::splat(M_CBRT2);
    let t77 = param_hyb_omega_0 * param_hyb_omega_0;
    let t80 = t25 * t25;
    let t82 = t27 * t27;
    let t84 = f64x8::splat(1.0) / t80 / t82;
    let t235 = ((f64x8::splat(M_PI)).sqrt());
    let t258 = f64x8::splat(1.0) / t235;
    let t274 = t23 * t29;
    let t275 = t77 * t4;
    let t299 = t77 * param_hyb_omega_0 / t24;
    let t301 = f64x8::splat(1.0) / t82 / t27;
    let t302 = t299 * t301;
    let t314 = t77 * t77;
    let t316 = t314 * param_hyb_omega_0 * t4;
    let t319 = t82 * t82;
    let t322 = f64x8::splat(1.0) / t80 / t24 / t319 / t27;
    let t323 = t316 * t322;
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (v_rho * f64x8::splat(0.5)).simd_le(dens_threshold);
            let t20 = (simd::cbrt(v_rho));
            let t21 = t19 * t20;
            let t30 = f64x8::splat(1.0) / t20;
            let t32 = t23 * t29 * t30;
            let t33 = t32 / f64x8::splat(3.0);
            let t34 = (f64x8::splat(14.0)).simd_lt(t33);
            let t38 = ((v_sigma).sqrt());
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
            let t79 = t76 * t77 * t4;
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
            let t220 = t112 * t119 * f64x8::splat(0.5) - t128 * t129 * f64x8::splat(0.5) - f64x8::splat(1.0159746228068032) * t134 * t101 + f64x8::splat(0.738073119521991) * t137 * t103 + t147 * t104 + t154 * t106 - f64x8::splat(0.09302717396924197) * t166 * t168 + t180 * t124 + f64x8::splat(0.0026165591067112575) * t193 * t195 + f64x8::splat(0.007566670425467926) * t212 * t214 + f64x8::splat(0.5080572) * t218;
            let t221 = ((t34).select(f64x8::splat(0.5080572) * t91 - f64x8::splat(0.5080572) * t95 + f64x8::splat(0.5080572) * t98, t220));
            let t223 = f64x8::splat(0.57786348) + t73;
            let t224 = t223 * t223;
            let t226 = f64x8::splat(0.077215461) * t73;
            let t227 = t223 * t60;
            let t230 = f64x8::splat(6.4753871) * t64 * t72 + f64x8::splat(0.4796583);
            let t233 = (f64x8::splat(0.08)).simd_lt(t59);
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
            let t259 = (f64x8::splat(3.0) * f64x8::splat(0.25) * f64x8::splat(M_PI) + t244 * t247 * f64x8::splat(0.0625) - f64x8::splat(2.3751029502456897) * t255) * t258;
            let t260 = f64x8::splat(1.0) / t60;
            let t261 = t260 * t246;
            let t267 = ((t233).select(-f64x8::splat(16.0) / f64x8::splat(15.0) * t259 * t261, -f64x8::splat(0.0262841788) - f64x8::splat(0.07117647788) * t60 + f64x8::splat(0.08534541323) * t62));
            let t268 = t60 * t267;
            let t270 = -f64x8::splat(0.37170836) * t224 - f64x8::splat(0.14853145700326428) - t226 - f64x8::splat(0.077215461) * t227 * t230 + f64x8::splat(2.0) * t268;
            let t271 = f64x8::splat(1.0) / t241;
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
            let t303 = f64x8::splat(1.0) / v_rho;
            let t307 = -f64x8::splat(0.30439865000326427) - t226 - f64x8::splat(0.025738487) * t276 - f64x8::splat(0.077215461) * t284 + f64x8::splat(5.0) * t268;
            let t309 = f64x8::splat(1.0) / t224;
            let t310 = t309 * t292;
            let t325 = f64x8::splat(1.0) / t85 / v_rho;
            let t326 = -f64x8::splat(0.051955731) + t268;
            let t327 = t325 * t326;
            let t328 = t271 * t292;
            let t332 = -f64x8::splat(8.0) / f64x8::splat(9.0) * t221 - f64x8::splat(4.0) / f64x8::splat(9.0) * t270 * t271 + t274 * t288 * t293 / f64x8::splat(27.0) + f64x8::splat(4.0) / f64x8::splat(27.0) * t302 * t303 * t307 * t310 + f64x8::splat(8.0) / f64x8::splat(81.0) * t323 * t327 * t328;
            let t336 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) * f64x8::splat(0.125) * t7 * t21 * t332));
            let tzk0 = f64x8::splat(2.0) * t336;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
