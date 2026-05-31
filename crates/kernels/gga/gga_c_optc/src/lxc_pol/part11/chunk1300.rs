//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1300/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1300<F: Float>(t57135: F, t57148: F, t57164: F, t57179: F, t828: F, t837: F, t845: F, t39411: F, t49385: F, t49387: F, t56966: F, t56978: F, t56981: F, t56984: F, t57024: F, t57057: F, t57060: F, t57063: F) -> (F, F, F) {
    let t57181 = t57135 + t57148 + t57164 + t57179;
    let t57185 = F::cast_from(0.58482233974552040708e0_f64) * t845 * t828 * t57181 * t837;
    let t57197 = -F::cast_from(0.92708333333333333333e-2_f64) * t57057 + F::cast_from(0.2225e0_f64) * t57060 - F::cast_from(0.33375e0_f64) * t56978 + F::cast_from(0.55625000000000000001e-1_f64) * t57063 - F::cast_from(0.49444444444444444444e-1_f64) * t49385 + F::cast_from(0.74166666666666666668e-1_f64) * t49387 + F::cast_from(0.74166666666666666668e-1_f64) * t56981 - F::cast_from(0.24722222222222222222e-1_f64) * t56984 - F::cast_from(0.24722222222222222222e-1_f64) * t39411 - F::cast_from(0.22249999999999999999e0_f64) * t57024 + F::cast_from(0.22249999999999999999e0_f64) * t56966;
    (t57181, t57185, t57197)
}
