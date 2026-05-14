//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 853/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk853<F: Float>(t21535: F, t2250: F, t2132: F, t6472: F, t20133: F, t326: F, t339: F, t6094: F, t16463: F, t333: F, t56: F, t338: F, t348: F, t15651: F, t191: F, t22: F, t364: F, t369: F, t371: F) -> (F, F, F, F, F, F, F) {
    let t21536 = t2250 * t21535;
    let t21597 = t6472 * t2132;
    let t21621 = t326 * t20133;
    let t21623 = t6094 * t339;
    let t21637 = t16463 * t56 * t333;
    let t21640 = 455.0 / 243.0 * t348 * t21637 * t338;
    let t21647 = 13685.0 / 31104.0 * t364 / t22 / t15651 * t191 * t369 * t371;
    (t21536, t21597, t21621, t21623, t21637, t21640, t21647)
}
