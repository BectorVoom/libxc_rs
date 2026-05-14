//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 846/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk846<F: Float>(t1648: F, t5395: F, t1627: F, t5403: F, t1642: F, t212: F, t22: F, t16972: F, t219: F, t16973: F, t639: F, t1656: F, t5406: F, t1666: F, t5400: F, t649: F) -> (F, F, F, F, F, F) {
    let t17316 = 16.0 / 5.0 * t1648 * t5395;
    let t17318 = 128.0 / 81.0 * t1627 * t5403;
    let t17321 = t22 / t212 / t1642;
    let t17322 = t219 * t16972;
    let t17326 = 352.0 / 243.0 * t639 * t17321 * t17322 * t16973;
    let t17328 = 8.0 / 15.0 * t5406 * t1656;
    let t17330 = 8.0 / 9.0 * t5406 * t1666;
    let t17331 = t5400 * t649;
    (t17316, t17318, t17326, t17328, t17330, t17331)
}
