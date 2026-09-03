//! MGGA_C_RMGGAC vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rmggac.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_rmggac_vxc_unpol(
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = (simd::cbrt(v_rho));
            let t11 = t5 * t7 / t8;
            let t12 = ((t11).sqrt());
            let t15 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t12 + f64x8::splat(0.03138525) * t11;
            let t16 = f64x8::splat(1.0) / t15;
            let t19 = (simd::exp(f64x8::splat(1.0) * t16));
            let t20 = t19 - f64x8::splat(1.0);
            let t21 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t21 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = t28 * v_sigma;
            let t30 = v_rho * v_rho;
            let t31 = t8 * t8;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t35 = t26 * t29 * t33;
            let t37 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t35;
            let t38 = ((t37).sqrt().sqrt());
            let t40 = f64x8::splat(1.0) - f64x8::splat(1.0) / t38;
            let t42 = t20 * t40 + f64x8::splat(1.0);
            let t43 = (simd::ln(t42));
            let t46 = t27 - f64x8::splat(1.0);
            let t47 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t48 = (simd::cbrt(zeta_threshold));
            let t50 = ((t47).select(t48 * zeta_threshold, f64x8::splat(1.0)));
            let t52 = f64x8::splat(2.0) * t50 - f64x8::splat(2.0);
            let t55 = f64x8::splat(1.0) / t46 / f64x8::splat(2.0);
            let t58 = f64x8::splat(1.0) - f64x8::splat(2.363) * t46 * t52 * t55;
            let t59 = (-f64x8::splat(0.0285764) * t16 + f64x8::splat(0.0285764) * t43) * t58;
            let t61 = f64x8::splat(1.0) / t31 / v_rho;
            let t66 = f64x8::splat(2.0) * v_tau * t61 - v_sigma * t33 / f64x8::splat(4.0);
            let t67 = t66 * t66;
            let t68 = t67 * t66;
            let t73 = f64x8::splat(0.08) + f64x8::splat(5.0) / f64x8::splat(18.0) * t66 * t28 * t26 + f64x8::splat(0.0125) * t35;
            let t74 = t73 * t73;
            let t75 = t74 * t73;
            let t76 = f64x8::splat(1.0) / t75;
            let t77 = t68 * t76;
            let t79 = t67 * t67;
            let t80 = t79 * t67;
            let t81 = t74 * t74;
            let t83 = f64x8::splat(1.0) / t81 / t74;
            let t86 = f64x8::splat(1.0) + f64x8::splat(0.006652356501035449) * t77 + f64x8::splat(4.42538470168686e-05) * t80 * t83;
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = t77 * t87;
            let t90 = f64x8::splat(1.0) - f64x8::splat(0.01995706950310635) * t88;
            let t91 = t59 * t90;
            let t93 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t96 = ((t11) * (t11).sqrt());
            let t98 = t2 * t2;
            let t99 = t4 * t4;
            let t100 = t98 * t99;
            let t103 = t100 * t6 / t31;
            let t105 = f64x8::splat(3.79785) * t12 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t96 + f64x8::splat(0.123235) * t103;
            let t108 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t105;
            let t109 = (simd::ln(t108));
            let t111 = f64x8::splat(0.0621814) * t93 * t109;
            let t112 = t52 * t55;
            let t114 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t119 = f64x8::splat(5.1785) * t12 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t96 + f64x8::splat(0.1241775) * t103;
            let t122 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t119;
            let t123 = (simd::ln(t122));
            let t126 = f64x8::splat(0.0197516734986138) * t112 * t114 * t123;
            let t127 = t48 * t48;
            let t128 = ((t47).select(t127, f64x8::splat(1.0)));
            let t129 = t128 * t128;
            let t130 = t129 * t128;
            let t131 = -t111 + t126;
            let t132 = f64x8::splat(1.0) / t130;
            let t135 = (simd::exp(-f64x8::splat(32.16364864430221) * t131 * t132));
            let t136 = t135 - f64x8::splat(1.0);
            let t137 = (simd::ln(f64x8::splat(2.0)));
            let t138 = f64x8::splat(1.0) - t137;
            let t139 = f64x8::splat(1.0) / t138;
            let t143 = (simd::exp(-t131 * t139 * t22 * t132));
            let t144 = t143 - f64x8::splat(1.0);
            let t145 = f64x8::splat(1.0) / t144;
            let t146 = t139 * t145;
            let t148 = f64x8::splat(1.0) / t8 / t30;
            let t151 = f64x8::splat(1.0) / t129;
            let t153 = f64x8::splat(1.0) / t4;
            let t154 = t98 * t153;
            let t155 = t154 * t6;
            let t156 = t27 * t151 * t155;
            let t159 = f64x8::splat(1.0) + f64x8::splat(0.02743955640261198) * t146 * v_sigma * t148 * t156;
            let t160 = ((t159).sqrt().sqrt());
            let t162 = f64x8::splat(1.0) - f64x8::splat(1.0) / t160;
            let t164 = t136 * t162 + f64x8::splat(1.0);
            let t165 = (simd::ln(t164));
            let t168 = -t111 + t126 + f64x8::splat(0.031091) * t130 * t165;
            let t169 = t168 * t68;
            let t170 = t76 * t87;
            let t172 = f64x8::splat(0.01995706950310635) * t169 * t170;
            let tzk0 = t91 + t172;
            acc_zk = tzk0;
            let t173 = t15 * t15;
            let t174 = f64x8::splat(1.0) / t173;
            let t176 = f64x8::splat(1.0) / t12 * t2;
            let t177 = t4 * t7;
            let t179 = f64x8::splat(1.0) / t8 / v_rho;
            let t180 = t177 * t179;
            let t181 = t176 * t180;
            let t183 = t7 * t179;
            let t184 = t5 * t183;
            let t186 = -f64x8::splat(0.007408333333333334) * t181 - f64x8::splat(0.01046175) * t184;
            let t187 = t174 * t186;
            let t189 = t19 * t40;
            let t193 = f64x8::splat(1.0) / t38 / t37;
            let t194 = t20 * t193;
            let t195 = t194 * t21;
            let t196 = t25 * t28;
            let t197 = t30 * v_rho;
            let t199 = f64x8::splat(1.0) / t31 / t197;
            let t200 = v_sigma * t199;
            let t204 = -f64x8::splat(1.0) * t187 * t189 - f64x8::splat(0.014225094736250906) * t195 * t196 * t200;
            let t205 = f64x8::splat(1.0) / t42;
            let t209 = (f64x8::splat(0.0285764) * t187 + f64x8::splat(0.0285764) * t204 * t205) * t58;
            let t210 = t209 * t90;
            let t211 = t67 * t76;
            let t215 = -f64x8::splat(10.0) / f64x8::splat(3.0) * v_tau * t33 + f64x8::splat(2.0) / f64x8::splat(3.0) * t200;
            let t216 = t87 * t215;
            let t217 = t211 * t216;
            let t219 = f64x8::splat(1.0) / t81;
            let t220 = t68 * t219;
            let t222 = t215 * t28 * t26;
            let t225 = t26 * t29 * t199;
            let t227 = f64x8::splat(5.0) / f64x8::splat(18.0) * t222 - f64x8::splat(0.03333333333333333) * t225;
            let t228 = t87 * t227;
            let t229 = t220 * t228;
            let t231 = t86 * t86;
            let t232 = f64x8::splat(1.0) / t231;
            let t237 = t79 * t66;
            let t238 = t237 * t83;
            let t242 = f64x8::splat(1.0) / t81 / t75;
            let t243 = t80 * t242;
            let t246 = f64x8::splat(0.01995706950310635) * t211 * t215 - f64x8::splat(0.01995706950310635) * t220 * t227 + f64x8::splat(0.0002655230821012116) * t238 * t215 - f64x8::splat(0.0002655230821012116) * t243 * t227;
            let t247 = t232 * t246;
            let t248 = t77 * t247;
            let t250 = -f64x8::splat(0.05987120850931904) * t217 + f64x8::splat(0.05987120850931904) * t229 + f64x8::splat(0.01995706950310635) * t248;
            let t251 = t59 * t250;
            let t254 = f64x8::splat(0.0011073470983333333) * t5 * t183 * t109;
            let t255 = t105 * t105;
            let t256 = f64x8::splat(1.0) / t255;
            let t257 = t93 * t256;
            let t260 = ((t11).sqrt());
            let t261 = t260 * t2;
            let t262 = t261 * t180;
            let t265 = t100 * t6 * t61;
            let t267 = -f64x8::splat(0.632975) * t181 - f64x8::splat(0.29896666666666666) * t184 - f64x8::splat(0.1023875) * t262 - f64x8::splat(0.08215666666666667) * t265;
            let t268 = f64x8::splat(1.0) / t108;
            let t269 = t267 * t268;
            let t271 = f64x8::splat(1.0) * t257 * t269;
            let t272 = t112 * t2;
            let t276 = f64x8::splat(0.00018311447306006544) * t272 * t177 * t179 * t123;
            let t277 = t112 * t114;
            let t278 = t119 * t119;
            let t279 = f64x8::splat(1.0) / t278;
            let t284 = -f64x8::splat(0.8630833333333333) * t181 - f64x8::splat(0.301925) * t184 - f64x8::splat(0.05501625) * t262 - f64x8::splat(0.082785) * t265;
            let t286 = f64x8::splat(1.0) / t122;
            let t287 = t279 * t284 * t286;
            let t289 = f64x8::splat(0.5848223622634646) * t277 * t287;
            let t290 = t254 + t271 - t276 - t289;
            let t291 = t290 * t132;
            let t292 = t135 * t162;
            let t296 = f64x8::splat(1.0) / t160 / t159;
            let t297 = t136 * t296;
            let t298 = t138 * t138;
            let t299 = f64x8::splat(1.0) / t298;
            let t300 = t144 * t144;
            let t301 = f64x8::splat(1.0) / t300;
            let t302 = t299 * t301;
            let t303 = t302 * v_sigma;
            let t304 = t148 * t27;
            let t305 = t129 * t129;
            let t307 = f64x8::splat(1.0) / t305 / t128;
            let t309 = t303 * t304 * t307;
            let t311 = t290 * t22 * t143;
            let t312 = t155 * t311;
            let t316 = f64x8::splat(1.0) / t8 / t197;
            let t321 = f64x8::splat(0.02743955640261198) * t309 * t312 - f64x8::splat(0.06402563160609462) * t146 * v_sigma * t316 * t156;
            let t324 = -f64x8::splat(32.16364864430221) * t291 * t292 + t297 * t321 / f64x8::splat(4.0);
            let t326 = f64x8::splat(1.0) / t164;
            let t329 = t254 + t271 - t276 - t289 + f64x8::splat(0.031091) * t130 * t324 * t326;
            let t330 = t329 * t68;
            let t331 = t330 * t170;
            let t333 = t168 * t67;
            let t334 = t170 * t215;
            let t335 = t333 * t334;
            let t337 = t219 * t87;
            let t338 = t337 * t227;
            let t339 = t169 * t338;
            let t341 = t76 * t232;
            let t342 = t341 * t246;
            let t343 = t169 * t342;
            let tvrho0 = t91 + t172 + v_rho * (t210 + t251 + f64x8::splat(0.01995706950310635) * t331 + f64x8::splat(0.05987120850931904) * t335 - f64x8::splat(0.05987120850931904) * t339 - f64x8::splat(0.01995706950310635) * t343);
            acc_vrho = tvrho0;
            let t347 = t194 * t26;
            let t348 = t28 * t33;
            let t349 = t205 * t58;
            let t350 = t349 * t90;
            let t353 = f64x8::splat(0.00015243824895787514) * t347 * t348 * t350;
            let t354 = t87 * t33;
            let t355 = t211 * t354;
            let t357 = t220 * t87;
            let t358 = t26 * t348;
            let t359 = t357 * t358;
            let t361 = t211 * t33;
            let t363 = t220 * t21;
            let t364 = t196 * t33;
            let t365 = t363 * t364;
            let t367 = t238 * t33;
            let t369 = t243 * t21;
            let t370 = t369 * t364;
            let t372 = -f64x8::splat(0.004989267375776587) * t361 + f64x8::splat(0.0011364442355935559) * t365 - f64x8::splat(6.63807705253029e-05) * t367 + f64x8::splat(1.5120064397430106e-05) * t370;
            let t373 = t232 * t372;
            let t374 = t77 * t373;
            let t376 = f64x8::splat(0.01496780212732976) * t355 - f64x8::splat(0.0034093327067806676) * t359 + f64x8::splat(0.01995706950310635) * t374;
            let t377 = t59 * t376;
            let t378 = t128 * t136;
            let t379 = t378 * t296;
            let t380 = t146 * t304;
            let t381 = t379 * t380;
            let t382 = t326 * t68;
            let t383 = t382 * t170;
            let t384 = t155 * t383;
            let t386 = f64x8::splat(4.256459989329784e-06) * t381 * t384;
            let t387 = t170 * t33;
            let t388 = t333 * t387;
            let t389 = f64x8::splat(0.01496780212732976) * t388;
            let t390 = t169 * t337;
            let t391 = t390 * t358;
            let t392 = f64x8::splat(0.0034093327067806676) * t391;
            let t393 = t341 * t372;
            let t395 = f64x8::splat(0.01995706950310635) * t169 * t393;
            let tvsigma0 = v_rho * (t353 + t377 + t386 - t389 + t392 - t395);
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t397 = t87 * t61;
            let t400 = t61 * t28;
            let t401 = t400 * t26;
            let t407 = t28 * t21;
            let t408 = t407 * t25;
            let t416 = f64x8::splat(0.0399141390062127) * t211 * t61 - f64x8::splat(0.011087260835059082) * t220 * t61 * t408 + f64x8::splat(0.0005310461642024232) * t238 * t61 - f64x8::splat(0.00014751282338956202) * t243 * t61 * t408;
            let t417 = t232 * t416;
            let t418 = t77 * t417;
            let t420 = -f64x8::splat(0.11974241701863808) * t211 * t397 + f64x8::splat(0.033261782505177244) * t357 * t401 + f64x8::splat(0.01995706950310635) * t418;
            let t421 = t59 * t420;
            let t422 = t170 * t61;
            let t424 = f64x8::splat(0.11974241701863808) * t333 * t422;
            let t426 = f64x8::splat(0.033261782505177244) * t390 * t401;
            let t427 = t341 * t416;
            let t429 = f64x8::splat(0.01995706950310635) * t169 * t427;
            let tvtau0 = v_rho * (t421 + t424 - t426 - t429);
            acc_vtau = tvtau0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
