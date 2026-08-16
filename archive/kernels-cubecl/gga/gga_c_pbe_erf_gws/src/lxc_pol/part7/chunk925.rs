//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 925/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk925<F: Float>(t1648: F, t5395: F, t1627: F, t5403: F, t1642: F, t212: F, t22: F, t16972: F, t219: F, t16973: F, t639: F, t1656: F, t5406: F) -> (F, F, F, F) {
    let t17316 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t1648 * t5395;
    let t17318 = F::cast_from(128.0_f64) / F::cast_from(81.0_f64) * t1627 * t5403;
    let t17321 = t22 / t212 / t1642;
    let t17322 = t219 * t16972;
    let t17326 = F::cast_from(352.0_f64) / F::cast_from(243.0_f64) * t639 * t17321 * t17322 * t16973;
    let t17328 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t5406 * t1656;
    (t17316, t17318, t17326, t17328)
}
