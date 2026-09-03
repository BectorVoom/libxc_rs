//! MGGA_C_RSCAN exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rscan.c`
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_rscan_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        {
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = v_rho0 + v_rho1;
            let t9 = (simd::cbrt(t8));
            let t12 = t5 * t7 / t9;
            let t14 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t12;
            let t15 = ((t12).sqrt());
            let t18 = ((t12) * (t12).sqrt());
            let t20 = t2 * t2;
            let t21 = t4 * t4;
            let t22 = t20 * t21;
            let t23 = t9 * t9;
            let t24 = f64x8::splat(1.0) / t23;
            let t26 = t22 * t6 * t24;
            let t28 = f64x8::splat(3.79785) * t15 + f64x8::splat(0.8969) * t12 + f64x8::splat(0.204775) * t18 + f64x8::splat(0.123235) * t26;
            let t31 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t28;
            let t32 = (simd::ln(t31));
            let t34 = f64x8::splat(0.0621814) * t14 * t32;
            let t35 = v_rho0 - v_rho1;
            let t36 = t35 * t35;
            let t37 = t36 * t36;
            let t38 = t8 * t8;
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t37 * t40;
            let t42 = f64x8::splat(1.0) / t8;
            let t43 = t35 * t42;
            let t44 = f64x8::splat(1.0) + t43;
            let t45 = (t44).simd_le(zeta_threshold);
            let t46 = (simd::cbrt(zeta_threshold));
            let t47 = t46 * zeta_threshold;
            let t48 = (simd::cbrt(t44));
            let t49 = t48 * t44;
            let t50 = ((t45).select(t47, t49));
            let t51 = f64x8::splat(1.0) - t43;
            let t52 = (t51).simd_le(zeta_threshold);
            let t53 = (simd::cbrt(t51));
            let t54 = t53 * t51;
            let t55 = ((t52).select(t47, t54));
            let t56 = t50 + t55 - f64x8::splat(2.0);
            let t57 = f64x8::splat(M_CBRT2);
            let t58 = t57 - f64x8::splat(1.0);
            let t60 = f64x8::splat(1.0) / t58 / f64x8::splat(2.0);
            let t61 = t56 * t60;
            let t63 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t12;
            let t68 = f64x8::splat(7.05945) * t15 + f64x8::splat(1.549425) * t12 + f64x8::splat(0.420775) * t18 + f64x8::splat(0.1562925) * t26;
            let t71 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t68;
            let t72 = (simd::ln(t71));
            let t76 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t12;
            let t81 = f64x8::splat(5.1785) * t15 + f64x8::splat(0.905775) * t12 + f64x8::splat(0.1100325) * t18 + f64x8::splat(0.1241775) * t26;
            let t84 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t81;
            let t85 = (simd::ln(t84));
            let t86 = t76 * t85;
            let t88 = -f64x8::splat(0.0310907) * t63 * t72 + t34 - f64x8::splat(0.0197516734986138) * t86;
            let t89 = t61 * t88;
            let t90 = t41 * t89;
            let t92 = f64x8::splat(0.0197516734986138) * t61 * t86;
            let t93 = (simd::ln(f64x8::splat(2.0)));
            let t94 = f64x8::splat(1.0) - t93;
            let t95 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t97 = t94 / t95;
            let t98 = t46 * t46;
            let t99 = t48 * t48;
            let t100 = ((t45).select(t98, t99));
            let t101 = t53 * t53;
            let t102 = ((t52).select(t98, t101));
            let t104 = t100 / f64x8::splat(2.0) + t102 / f64x8::splat(2.0);
            let t105 = t104 * t104;
            let t106 = t105 * t104;
            let t108 = f64x8::splat(1.0) + f64x8::splat(0.025) * t12;
            let t110 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t12;
            let t111 = f64x8::splat(1.0) / t110;
            let t112 = t108 * t111;
            let t113 = f64x8::splat(1.0) / t94;
            let t115 = (-t34 + t90 + t92) * t113;
            let t116 = f64x8::splat(1.0) / t106;
            let t117 = t95 * t116;
            let t119 = (simd::exp(-t115 * t117));
            let t120 = t119 - f64x8::splat(1.0);
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t113 * t121;
            let t124 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t125 = t122 * t124;
            let t126 = t112 * t125;
            let t127 = t9 * t38;
            let t128 = f64x8::splat(1.0) / t127;
            let t129 = t128 * t57;
            let t130 = f64x8::splat(1.0) / t105;
            let t132 = f64x8::splat(1.0) / t4;
            let t133 = t20 * t132;
            let t134 = t133 * t6;
            let t138 = f64x8::splat(1.0) + f64x8::splat(0.027439371595564633) * t126 * t129 * t130 * t134;
            let t139 = ((t138).sqrt().sqrt());
            let t141 = f64x8::splat(1.0) - f64x8::splat(1.0) / t139;
            let t144 = f64x8::splat(1.0) + f64x8::splat(1.0) * t141 * t120;
            let t145 = (simd::ln(t144));
            let t147 = t97 * t106 * t145;
            let t148 = t39 * t8;
            let t149 = (simd::cbrt(v_rho0));
            let t150 = t149 * t149;
            let t152 = f64x8::splat(1.0) / t150 / v_rho0;
            let t153 = v_tau0 * t152;
            let t154 = t44 / f64x8::splat(2.0);
            let t155 = (simd::cbrt(t154));
            let t156 = t155 * t155;
            let t157 = t156 * t154;
            let t159 = (simd::cbrt(v_rho1));
            let t160 = t159 * t159;
            let t162 = f64x8::splat(1.0) / t160 / v_rho1;
            let t163 = v_tau1 * t162;
            let t164 = t51 / f64x8::splat(2.0);
            let t165 = (simd::cbrt(t164));
            let t166 = t165 * t165;
            let t167 = t166 * t164;
            let t169 = t23 * t38;
            let t170 = f64x8::splat(1.0) / t169;
            let t173 = t153 * t157 + t163 * t167 - t124 * t170 / f64x8::splat(8.0);
            let t174 = (f64x8::splat(0.0)).simd_lt(t173);
            let t175 = ((t174).select(t173, f64x8::splat(0.0)));
            let t176 = t175 * t175;
            let t177 = t176 * t175;
            let t178 = t148 * t177;
            let t179 = f64x8::splat(M_CBRT6);
            let t180 = t179 * t179;
            let t181 = (simd::cbrt(t95));
            let t182 = t181 * t181;
            let t183 = t180 * t182;
            let t184 = t23 * t8;
            let t187 = t57 * t57;
            let t189 = f64x8::splat(3.0) / f64x8::splat(10.0) * t183 * t184 + f64x8::splat(0.0001) * t187;
            let t190 = t189 * t189;
            let t191 = t190 * t189;
            let t192 = f64x8::splat(1.0) / t191;
            let t193 = t157 + t167;
            let t194 = t193 * t193;
            let t195 = t194 * t193;
            let t196 = f64x8::splat(1.0) / t195;
            let t197 = t192 * t196;
            let t198 = t38 * t8;
            let t199 = t9 * t198;
            let t200 = t199 * t176;
            let t201 = f64x8::splat(1.0) / t190;
            let t202 = f64x8::splat(1.0) / t194;
            let t203 = t201 * t202;
            let t205 = t200 * t203 + f64x8::splat(0.001);
            let t206 = f64x8::splat(1.0) / t205;
            let t207 = t197 * t206;
            let t208 = t178 * t207;
            let t209 = (t208).simd_le(f64x8::splat(2.5));
            let t210 = (f64x8::splat(2.5)).simd_lt(t208);
            let t211 = ((t210).select(f64x8::splat(2.5), t208));
            let t213 = t211 * t211;
            let t215 = t213 * t211;
            let t217 = t213 * t213;
            let t219 = t217 * t211;
            let t221 = t217 * t213;
            let t226 = ((t210).select(t208, f64x8::splat(2.5)));
            let t227 = f64x8::splat(1.0) - t226;
            let t230 = (simd::exp(f64x8::splat(1.5) / t227));
            let t232 = ((t209).select(f64x8::splat(1.0) - f64x8::splat(0.64) * t211 - f64x8::splat(0.4352) * t213 - f64x8::splat(1.535685604549) * t215 + f64x8::splat(3.061560252175) * t217 - f64x8::splat(1.915710236206) * t219 + f64x8::splat(0.516884468372) * t221 - f64x8::splat(0.051848879792) * t217 * t215, -f64x8::splat(0.7) * t230));
            let t235 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t15 + f64x8::splat(0.03138525) * t12;
            let t236 = f64x8::splat(1.0) / t235;
            let t239 = (simd::exp(f64x8::splat(1.0) * t236));
            let t240 = t239 - f64x8::splat(1.0);
            let t241 = f64x8::splat(1.0) / t182;
            let t242 = t179 * t241;
            let t243 = t187 * t124;
            let t247 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t242 * t243 * t170;
            let t248 = ((t247).sqrt().sqrt());
            let t250 = f64x8::splat(1.0) - f64x8::splat(1.0) / t248;
            let t252 = t240 * t250 + f64x8::splat(1.0);
            let t253 = (simd::ln(t252));
            let t255 = -f64x8::splat(0.0285764) * t236 + f64x8::splat(0.0285764) * t253;
            let t259 = f64x8::splat(1.0) - f64x8::splat(2.363) * t58 * t56 * t60;
            let t260 = t255 * t259;
            let t261 = t37 * t37;
            let t262 = t261 * t37;
            let t263 = t39 * t39;
            let t264 = t263 * t39;
            let t265 = f64x8::splat(1.0) / t264;
            let t267 = -t262 * t265 + f64x8::splat(1.0);
            let t269 = t260 * t267 - t147 + t34 - t90 - t92;
            let t270 = t232 * t269;
            let tzk0 = -t34 + t90 + t92 + t147 + t270;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
