//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1343/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1343<F: Float>(t26261: F, t26309: F, t26311: F, t26314: F, t26319: F, t26324: F, t26326: F, t26328: F, t26330: F, t26332: F, t26339: F, t26343: F) -> F {
    let t26808 = F::cast_from(0.17757530864197530864e0_f64) * t26261;
    let t26818 = F::cast_from(0.4566222222222222222e-1_f64) * t26309 - F::cast_from(0.9132444444444444444e-1_f64) * t26311 + t26808 + F::cast_from(0.22831111111111111111e-1_f64) * t26314 + F::cast_from(0.13698666666666666667e0_f64) * t26319 - F::cast_from(0.4566222222222222222e-1_f64) * t26324 - F::cast_from(0.45662222222222222221e-1_f64) * t26326 - F::cast_from(0.3044148148148148148e-1_f64) * t26328 + F::cast_from(0.9132444444444444444e-1_f64) * t26330 + F::cast_from(0.71030123456790123454e-1_f64) * t26332 - F::cast_from(0.50735802469135802467e-1_f64) * t26339 - F::cast_from(0.17123333333333333333e-1_f64) * t26343;
    t26818
}
