//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 958/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk958<F: Float>(t17321: F, t17322: F, t47377: F, t639: F, t11136: F, t1621: F, t3553: F, t31168: F, t3535: F, t39931: F, t3342: F) -> (F, F, F, F, F) {
    let t47381 = 352.0 / 243.0 * t639 * t17321 * t17322 * t47377;
    let t47385 = 24.0 / 5.0 * t639 * t1621 * t11136 * t3553;
    let t47387 = 16.0 / 5.0 * t31168 * t3535;
    let t47389 = 32.0 / 45.0 * t39931;
    let t47391 = t3342 * t3342;
    (t47381, t47385, t47387, t47389, t47391)
}
