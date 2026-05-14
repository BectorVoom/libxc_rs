//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1168/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1168<F: Float>(t57181: F, t828: F, t837: F, t845: F, t39411: F, t49385: F, t49387: F, t56966: F, t56978: F, t56981: F, t56984: F, t57024: F, t57057: F, t57060: F, t57063: F, t24321: F, t30270: F, t39413: F, t39418: F, t49240: F, t49242: F, t49393: F, t49395: F, t56969: F, t57027: F, t57037: F, t57041: F) -> (F, F, F) {
    let t57185 = 0.58482233974552040708e0 * t845 * t828 * t57181 * t837;
    let t57197 = -0.92708333333333333333e-2 * t57057 + 0.2225e0 * t57060 - 0.33375e0 * t56978 + 0.55625000000000000001e-1 * t57063 - 0.49444444444444444444e-1 * t49385 + 0.74166666666666666668e-1 * t49387 + 0.74166666666666666668e-1 * t56981 - 0.24722222222222222222e-1 * t56984 - 0.24722222222222222222e-1 * t39411 - 0.22249999999999999999e0 * t57024 + 0.22249999999999999999e0 * t56966;
    let t57209 = -0.18541666666666666666e-1 * t57027 - 0.61805555555555555555e-1 * t56969 - 0.16481481481481481482e-1 * t39413 + 0.49444444444444444445e-1 * t39418 + t24321 + 0.24722222222222222222e-1 * t49240 - 0.74166666666666666668e-1 * t49242 + 0.12361111111111111111e-1 * t49393 + 0.13734567901234567901e-1 * t49395 + 0.38456790123456790123e-1 * t30270 + 0.12361111111111111111e0 * t57037 - 0.27469135802469135803e-1 * t57041;
    (t57185, t57197, t57209)
}
