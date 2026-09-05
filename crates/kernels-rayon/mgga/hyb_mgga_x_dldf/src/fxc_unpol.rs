//! HYB_MGGA_X_DLDF fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_dldf.c`
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
pub fn hyb_mgga_x_dldf_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t6 = zeta_threshold - f64x8::splat(1.0);
            let t8 = ((t5).select(t6, (t5).select(-t6, f64x8::splat(0.0))));
            let t9 = f64x8::splat(1.0) + t8;
            let t11 = (simd::cbrt(zeta_threshold));
            let t13 = (simd::cbrt(t9));
            let t15 = (((t9).simd_le(zeta_threshold)).select(t11 * zeta_threshold, t13 * t9));
            let t16 = t4 * t15;
            let t17 = (simd::cbrt(v_rho));
            let t18 = f64x8::splat(M_CBRT6);
            let t19 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t20 = (simd::cbrt(t19));
            let t21 = t20 * t20;
            let t23 = t18 / t21;
            let t24 = f64x8::splat(M_CBRT2);
            let t25 = t24 * t24;
            let t26 = v_sigma * t25;
            let t27 = v_rho * v_rho;
            let t28 = t17 * t17;
            let t30 = f64x8::splat(1.0) / t28 / t27;
            let t34 = f64x8::splat(4.8827323) + f64x8::splat(0.0146297) * t23 * t26 * t30;
            let t37 = f64x8::splat(5.8827323) - f64x8::splat(23.84107471346329) / t34;
            let t38 = t17 * t37;
            let t39 = t18 * t18;
            let t41 = f64x8::splat(3.0) / f64x8::splat(10.0) * t39 * t21;
            let t42 = v_tau * t25;
            let t44 = f64x8::splat(1.0) / t28 / v_rho;
            let t45 = t42 * t44;
            let t46 = t41 - t45;
            let t47 = t41 + t45;
            let t48 = f64x8::splat(1.0) / t47;
            let t51 = t46 * t46;
            let t52 = t47 * t47;
            let t53 = f64x8::splat(1.0) / t52;
            let t56 = t51 * t46;
            let t57 = t52 * t47;
            let t58 = f64x8::splat(1.0) / t57;
            let t61 = t51 * t51;
            let t62 = t52 * t52;
            let t63 = f64x8::splat(1.0) / t62;
            let t66 = f64x8::splat(1.0) - f64x8::splat(0.1637571) * t46 * t48 - f64x8::splat(0.1880028) * t51 * t53 - f64x8::splat(0.4490609) * t56 * t58 - f64x8::splat(0.0082359) * t61 * t63;
            let t70 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.09872727257880975) * t16 * t38 * t66));
            let tzk0 = f64x8::splat(2.0) * t70;
            acc_zk = tzk0;
            let t72 = f64x8::splat(1.0) / t28 * t37;
            let t76 = t27 * v_rho;
            let t78 = f64x8::splat(1.0) / t17 / t76;
            let t79 = t34 * t34;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = t16 * t78 * t80;
            let t84 = t23 * t26 * t66;
            let t90 = t46 * t53;
            let t91 = t42 * t30;
            let t94 = t51 * t58;
            let t97 = t56 * t63;
            let t101 = f64x8::splat(1.0) / t62 / t47;
            let t102 = t61 * t101;
            let t105 = -f64x8::splat(0.2729285) * t42 * t30 * t48 - f64x8::splat(0.8996045) * t90 * t91 - f64x8::splat(2.8719805) * t94 * t91 - f64x8::splat(2.3002105) * t97 * t91 - f64x8::splat(0.054906) * t102 * t91;
            let t110 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.03290909085960325) * t16 * t72 * t66 + f64x8::splat(0.09182630750283849) * t82 * t84 - f64x8::splat(0.09872727257880975) * t16 * t38 * t105));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t110 + f64x8::splat(2.0) * t70;
            acc_vrho = tvrho0;
            let t116 = t16 / t17 / t27 * t80;
            let t118 = t23 * t25 * t66;
            let t121 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.03443486531356443) * t116 * t118));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t121;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t123 = t25 * t44;
            let t134 = f64x8::splat(0.1637571) * t123 * t48 + f64x8::splat(0.5397627) * t90 * t123 + f64x8::splat(1.7231883) * t94 * t123 + f64x8::splat(1.3801263) * t97 * t123 + f64x8::splat(0.0329436) * t102 * t123;
            let t138 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.09872727257880975) * t16 * t38 * t134));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t138;
            acc_vtau = tvtau0;
            let t141 = t44 * t37;
            let t145 = t27 * t27;
            let t147 = f64x8::splat(1.0) / t17 / t145;
            let t149 = t16 * t147 * t80;
            let t155 = t145 * t76;
            let t156 = f64x8::splat(1.0) / t155;
            let t158 = f64x8::splat(1.0) / t79 / t34;
            let t160 = t16 * t156 * t158;
            let t163 = t39 / t20 / t19;
            let t164 = v_sigma * v_sigma;
            let t165 = t164 * t24;
            let t167 = t163 * t165 * t66;
            let t171 = t23 * t26 * t105;
            let t175 = f64x8::splat(1.0) / t28 / t76;
            let t179 = v_tau * v_tau;
            let t180 = t179 * t24;
            let t181 = t145 * v_rho;
            let t183 = f64x8::splat(1.0) / t17 / t181;
            let t187 = t46 * t58;
            let t188 = t180 * t183;
            let t191 = t42 * t175;
            let t194 = t51 * t63;
            let t199 = t56 * t101;
            let t205 = f64x8::splat(1.0) / t62 / t52;
            let t206 = t61 * t205;
            let t211 = f64x8::splat(0.7278093333333333) * t42 * t175 * t48 - f64x8::splat(3.9084433333333335) * t180 * t183 * t53 - f64x8::splat(25.1439) * t187 * t188 + f64x8::splat(2.3989453333333333) * t90 * t191 - f64x8::splat(51.72191) * t194 * t188 + f64x8::splat(7.658614666666667) * t94 * t191 - f64x8::splat(31.401553333333332) * t199 * t188 + f64x8::splat(6.1338946666666665) * t97 * t191 - f64x8::splat(0.9151) * t206 * t188 + f64x8::splat(0.146416) * t102 * t191;
            let t216 = ((t3).select(f64x8::splat(0.0), f64x8::splat(0.02193939390640217) * t16 * t141 * t66 - f64x8::splat(0.27547892250851547) * t149 * t84 - f64x8::splat(0.0658181817192065) * t16 * t72 * t105 + f64x8::splat(0.014329507529325615) * t160 * t167 + f64x8::splat(0.18365261500567698) * t82 * t171 - f64x8::splat(0.09872727257880975) * t16 * t38 * t211));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t216 + f64x8::splat(4.0) * t110;
            acc_v2rho2 = tv2rho20;
            let t221 = t145 * t27;
            let t222 = f64x8::splat(1.0) / t221;
            let t224 = t16 * t222 * t158;
            let t225 = t24 * t66;
            let t227 = t163 * t225 * v_sigma;
            let t231 = t23 * t25 * t105;
            let t235 = ((t3).select(f64x8::splat(0.0), f64x8::splat(0.08034801906498368) * t82 * t118 - f64x8::splat(0.005373565323497105) * t224 * t227 - f64x8::splat(0.03443486531356443) * t116 * t231));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t235 + f64x8::splat(2.0) * t121;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let t242 = t23 * t26 * t134;
            let t245 = t25 * t30;
            let t248 = t24 * t147;
            let t249 = t53 * v_tau;
            let t252 = t248 * v_tau;
            let t269 = -f64x8::splat(0.2729285) * t245 * t48 + f64x8::splat(2.345066) * t248 * t249 + f64x8::splat(15.08634) * t187 * t252 - f64x8::splat(0.8996045) * t90 * t245 + f64x8::splat(31.033146) * t194 * t252 - f64x8::splat(2.8719805) * t94 * t245 + f64x8::splat(18.840932) * t199 * t252 - f64x8::splat(2.3002105) * t97 * t245 + f64x8::splat(0.54906) * t206 * t252 - f64x8::splat(0.054906) * t102 * t245;
            let t274 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.03290909085960325) * t16 * t72 * t134 + f64x8::splat(0.09182630750283849) * t82 * t242 - f64x8::splat(0.09872727257880975) * t16 * t38 * t269));
            let tv2rhotau0 = f64x8::splat(2.0) * v_rho * t274 + f64x8::splat(2.0) * t138;
            acc_v2rhotau = tv2rhotau0;
            let t277 = f64x8::splat(1.0) / t181;
            let t279 = t16 * t277 * t158;
            let t280 = t163 * t225;
            let t283 = ((t3).select(f64x8::splat(0.0), f64x8::splat(0.0020150869963114146) * t279 * t280));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t283;
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let t286 = t23 * t25 * t134;
            let t289 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.03443486531356443) * t116 * t286));
            let tv2sigmatau0 = f64x8::splat(2.0) * v_rho * t289;
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let t291 = t24 * t78;
            let t302 = -f64x8::splat(1.4070396) * t291 * t53 - f64x8::splat(9.051804) * t187 * t291 - f64x8::splat(18.6198876) * t194 * t291 - f64x8::splat(11.3045592) * t199 * t291 - f64x8::splat(0.329436) * t206 * t291;
            let t306 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.09872727257880975) * t16 * t38 * t302));
            let tv2tau20 = f64x8::splat(2.0) * v_rho * t306;
            acc_v2tau2 = tv2tau20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2rholapl, ip, m, acc_v2rholapl);
        store_add(v2rhotau, ip, m, acc_v2rhotau);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v2sigmalapl, ip, m, acc_v2sigmalapl);
        store_add(v2sigmatau, ip, m, acc_v2sigmatau);
        store_add(v2lapl2, ip, m, acc_v2lapl2);
        store_add(v2lapltau, ip, m, acc_v2lapltau);
        store_add(v2tau2, ip, m, acc_v2tau2);
        ip += 8;
    }
}
