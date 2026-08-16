//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 338/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk338<F: Float>(t1216: F, t1322: F, t1224: F, t1230: F, t1232: F, t1254: F, t1259: F, t1262: F, t1267: F, t1271: F, t1278: F, t1288: F, t1296: F, t1300: F, t1304: F, t1305: F, t1315: F, t1320: F, t174: F, t4: F, t435: F, t442: F, t450: F, t457: F, t542: F, t71: F, t84: F) -> (F, F) {
    let t1323 = t1216 * t1322;
    let t1326 = -F::cast_from(0.70981924444444444442e-3_f64) * t4 * t542 * t71 - F::cast_from(0.34246666666666666666e-1_f64) * t174 * t1224 * t442 - F::cast_from(2.0_f64) * t1230 * t1232 + F::cast_from(1.0_f64) * t435 * t1254 + F::cast_from(0.32164683177870697974e2_f64) * t1259 * t1262 + t1267 + t1271 + t1278 - t1288 - t1296 - F::cast_from(0.24415406715670879921e-3_f64) * t4 * t542 * t84 - F::cast_from(0.10843580882781524214e-1_f64) * t174 * t1300 * t457 - F::cast_from(0.11696446794910408142e1_f64) * t1304 * t1305 + F::cast_from(0.58482233974552040708e0_f64) * t450 * t1315 + F::cast_from(0.17315755899375863299e2_f64) * t1320 * t1323;
    (t1323, t1326)
}
