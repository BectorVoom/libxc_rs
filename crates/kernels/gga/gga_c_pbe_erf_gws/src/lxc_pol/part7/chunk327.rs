//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 327/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk327<F: Float>(t1257: F, t62: F, t70: F, t1231: F, t31: F, t4: F, t542: F, t155: F, t388: F, t174: F, t405: F, t27: F, t387: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1258 = F::new(1.0) / t1257;
    let t1259 = t62 * t1258;
    let t1260 = t70 * t70;
    let t1261 = F::new(1.0) / t1260;
    let t1262 = t1231 * t1261;
    let t1266 = t4 * t542 * t31;
    let t1267 = F::cast_from(0.14764770444444444444e-2_f64) * t1266;
    let t1268 = t155 * t388;
    let t1270 = t174 * t1268 * t405;
    let t1271 = F::cast_from(0.35616666666666666667e-1_f64) * t1270;
    let t1272 = t387 * t27;
    (t1258, t1259, t1260, t1261, t1262, t1267, t1268, t1271, t1272)
}
