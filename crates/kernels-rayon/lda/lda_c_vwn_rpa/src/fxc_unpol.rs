//! LDA_C_VWN_RPA fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_rpa.c`
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
pub fn lda_c_vwn_rpa_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t6 * t8;
            let t10 = t4 * t9;
            let t11 = t10 / f64x8::splat(4.0);
            let t12 = ((t10).sqrt());
            let t14 = t11 + f64x8::splat(6.536) * t12 + f64x8::splat(42.7198);
            let t15 = f64x8::splat(1.0) / t14;
            let t19 = (simd::ln(t4 * t9 * t15 / f64x8::splat(4.0)));
            let t21 = t12 + f64x8::splat(13.072);
            let t24 = (simd::atan(f64x8::splat(0.0448998886412873) / t21));
            let t26 = t12 / f64x8::splat(2.0);
            let t27 = t26 + f64x8::splat(0.409286);
            let t28 = t27 * t27;
            let t30 = (simd::ln(t28 * t15));
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t38 = f64x8::splat(2.0) * t36 - f64x8::splat(2.0);
            let t39 = f64x8::splat(M_CBRT2);
            let t42 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t44 = -t38 * t42 + f64x8::splat(1.0);
            let t45 = (f64x8::splat(0.0310907) * t19 + f64x8::splat(20.521972937837504) * t24 + f64x8::splat(0.004431373767749538) * t30) * t44;
            let t47 = t11 + f64x8::splat(10.06155) * t12 + f64x8::splat(101.578);
            let t48 = f64x8::splat(1.0) / t47;
            let t52 = (simd::ln(t4 * t9 * t48 / f64x8::splat(4.0)));
            let t54 = t12 + f64x8::splat(20.1231);
            let t57 = (simd::atan(f64x8::splat(1.171685277708993) / t54));
            let t59 = t26 + f64x8::splat(0.743294);
            let t60 = t59 * t59;
            let t62 = (simd::ln(t60 * t48));
            let t66 = (f64x8::splat(0.01554535) * t52 + f64x8::splat(0.6188180297906063) * t57 + f64x8::splat(0.002667310007273315) * t62) * t38 * t42;
            let tzk0 = t45 + t66;
            acc_zk = tzk0;
            let t68 = f64x8::splat(1.0) / t7 / v_rho;
            let t69 = t6 * t68;
            let t73 = t4 * t6;
            let t74 = t14 * t14;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t8 * t75;
            let t77 = t4 * t69;
            let t78 = t77 / f64x8::splat(12.0);
            let t79 = f64x8::splat(1.0) / t12;
            let t80 = t79 * t1;
            let t81 = t3 * t6;
            let t83 = t80 * t81 * t68;
            let t85 = -t78 - f64x8::splat(1.0893333333333333) * t83;
            let t90 = t1 * t1;
            let t92 = f64x8::splat(1.0) / t3;
            let t93 = (-t4 * t69 * t15 / f64x8::splat(12.0) - t73 * t76 * t85 / f64x8::splat(4.0)) * t90 * t92;
            let t94 = t5 * t7;
            let t95 = t94 * t14;
            let t98 = t21 * t21;
            let t99 = f64x8::splat(1.0) / t98;
            let t101 = t99 * t79 * t1;
            let t103 = f64x8::splat(0.002016) * t99 + f64x8::splat(1.0);
            let t104 = f64x8::splat(1.0) / t103;
            let t109 = t27 * t15;
            let t110 = t109 * t79;
            let t113 = t28 * t75;
            let t115 = -t110 * t77 / f64x8::splat(6.0) - t113 * t85;
            let t116 = f64x8::splat(1.0) / t28;
            let t117 = t115 * t116;
            let t121 = (f64x8::splat(0.010363566666666667) * t93 * t95 + f64x8::splat(0.15357238326806924) * t101 * t81 * t68 * t104 + f64x8::splat(0.004431373767749538) * t117 * t14) * t44;
            let t125 = t47 * t47;
            let t126 = f64x8::splat(1.0) / t125;
            let t127 = t8 * t126;
            let t129 = -t78 - f64x8::splat(1.676925) * t83;
            let t135 = (-t4 * t69 * t48 / f64x8::splat(12.0) - t73 * t127 * t129 / f64x8::splat(4.0)) * t90 * t92;
            let t136 = t94 * t47;
            let t139 = t54 * t54;
            let t140 = f64x8::splat(1.0) / t139;
            let t142 = t140 * t79 * t1;
            let t144 = f64x8::splat(1.37284639) * t140 + f64x8::splat(1.0);
            let t145 = f64x8::splat(1.0) / t144;
            let t150 = t59 * t48;
            let t151 = t150 * t79;
            let t154 = t60 * t126;
            let t156 = -t151 * t77 / f64x8::splat(6.0) - t154 * t129;
            let t157 = f64x8::splat(1.0) / t60;
            let t158 = t156 * t157;
            let t163 = (f64x8::splat(0.005181783333333334) * t135 * t136 + f64x8::splat(0.12084332918108974) * t142 * t81 * t68 * t145 + f64x8::splat(0.002667310007273315) * t158 * t47) * t38 * t42;
            let tvrho0 = t45 + t66 + v_rho * (t121 + t163);
            acc_vrho = tvrho0;
            let t168 = v_rho * v_rho;
            let t170 = f64x8::splat(1.0) / t7 / t168;
            let t171 = t6 * t170;
            let t173 = t4 * t171 * t15;
            let t175 = t68 * t75;
            let t180 = f64x8::splat(1.0) / t74 / t14;
            let t181 = t8 * t180;
            let t182 = t85 * t85;
            let t186 = t4 * t171;
            let t187 = t186 / f64x8::splat(9.0);
            let t189 = f64x8::splat(1.0) / t12 / t10;
            let t190 = t189 * t90;
            let t191 = t3 * t3;
            let t192 = t191 * t5;
            let t193 = t7 * t7;
            let t195 = f64x8::splat(1.0) / t193 / t168;
            let t197 = t190 * t192 * t195;
            let t200 = t80 * t81 * t170;
            let t202 = t187 - f64x8::splat(0.7262222222222222) * t197 + f64x8::splat(1.4524444444444444) * t200;
            let t208 = (t173 / f64x8::splat(9.0) + t73 * t175 * t85 / f64x8::splat(6.0) + t73 * t181 * t182 / f64x8::splat(2.0) - t73 * t76 * t202 / f64x8::splat(4.0)) * t90 * t92;
            let t212 = t5 / t193;
            let t213 = t212 * t14;
            let t216 = t94 * t85;
            let t219 = t98 * t21;
            let t221 = f64x8::splat(1.0) / t219 * t1;
            let t222 = t221 * t3;
            let t227 = t99 * t189 * t90;
            let t236 = t98 * t98;
            let t238 = f64x8::splat(1.0) / t236 / t21;
            let t239 = t238 * t1;
            let t240 = t239 * t3;
            let t241 = t103 * t103;
            let t242 = f64x8::splat(1.0) / t241;
            let t247 = t27 * t75;
            let t248 = t247 * t80;
            let t249 = t68 * t85;
            let t253 = t109 * t189;
            let t254 = t90 * t191;
            let t255 = t5 * t195;
            let t256 = t254 * t255;
            let t261 = t28 * t180;
            let t265 = t173 / f64x8::splat(72.0) + t248 * t81 * t249 / f64x8::splat(3.0) - t253 * t256 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t110 * t186 + f64x8::splat(2.0) * t261 * t182 - t113 * t202;
            let t266 = t265 * t116;
            let t270 = f64x8::splat(1.0) / t28 / t27;
            let t271 = t115 * t270;
            let t272 = t14 * t79;
            let t273 = t271 * t272;
            let t279 = (f64x8::splat(0.010363566666666667) * t208 * t95 + f64x8::splat(0.003454522222222222) * t93 * t213 + f64x8::splat(0.010363566666666667) * t93 * t216 + f64x8::splat(0.05119079442268974) * t222 * t171 * t104 + f64x8::splat(0.10238158884537948) * t227 * t192 * t195 * t104 - f64x8::splat(0.20476317769075897) * t101 * t81 * t170 * t104 - f64x8::splat(0.00010320064155614252) * t240 * t171 * t242 + f64x8::splat(0.004431373767749538) * t266 * t14 + f64x8::splat(0.0007385622946249231) * t273 * t77 + f64x8::splat(0.004431373767749538) * t117 * t85) * t44;
            let t281 = t4 * t171 * t48;
            let t283 = t68 * t126;
            let t288 = f64x8::splat(1.0) / t125 / t47;
            let t289 = t8 * t288;
            let t290 = t129 * t129;
            let t296 = t187 - f64x8::splat(1.11795) * t197 + f64x8::splat(2.2359) * t200;
            let t302 = (t281 / f64x8::splat(9.0) + t73 * t283 * t129 / f64x8::splat(6.0) + t73 * t289 * t290 / f64x8::splat(2.0) - t73 * t127 * t296 / f64x8::splat(4.0)) * t90 * t92;
            let t305 = t212 * t47;
            let t308 = t94 * t129;
            let t311 = t139 * t54;
            let t313 = f64x8::splat(1.0) / t311 * t1;
            let t314 = t313 * t3;
            let t319 = t140 * t189 * t90;
            let t328 = t139 * t139;
            let t330 = f64x8::splat(1.0) / t328 / t54;
            let t331 = t330 * t1;
            let t332 = t331 * t3;
            let t333 = t144 * t144;
            let t334 = f64x8::splat(1.0) / t333;
            let t339 = t59 * t126;
            let t340 = t339 * t80;
            let t341 = t68 * t129;
            let t345 = t150 * t189;
            let t350 = t60 * t288;
            let t354 = t281 / f64x8::splat(72.0) + t340 * t81 * t341 / f64x8::splat(3.0) - t345 * t256 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t151 * t186 + f64x8::splat(2.0) * t350 * t290 - t154 * t296;
            let t355 = t354 * t157;
            let t359 = f64x8::splat(1.0) / t60 / t59;
            let t360 = t156 * t359;
            let t361 = t47 * t79;
            let t362 = t360 * t361;
            let t369 = (f64x8::splat(0.005181783333333334) * t302 * t136 + f64x8::splat(0.001727261111111111) * t135 * t305 + f64x8::splat(0.005181783333333334) * t135 * t308 + f64x8::splat(0.04028110972702991) * t314 * t171 * t145 + f64x8::splat(0.08056221945405982) * t319 * t192 * t195 * t145 - f64x8::splat(0.16112443890811964) * t142 * t81 * t170 * t145 - f64x8::splat(0.055299776073946906) * t332 * t171 * t334 + f64x8::splat(0.002667310007273315) * t355 * t47 + f64x8::splat(0.0004445516678788859) * t362 * t77 + f64x8::splat(0.002667310007273315) * t158 * t129) * t38 * t42;
            let tv2rho20 = f64x8::splat(2.0) * t121 + f64x8::splat(2.0) * t163 + v_rho * (t279 + t369);
            acc_v2rho2 = tv2rho20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
