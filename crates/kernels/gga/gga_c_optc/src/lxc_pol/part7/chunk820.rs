//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 820/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk820<F: Float>(t7523: F, t7525: F, t7527: F, t7531: F, t7535: F, t7550: F, t7571: F, t7573: F, t7576: F, t7580: F, t7583: F, t8320: F, t2294: F, t973: F, t2300: F, t970: F) -> (F, F, F) {
    let t8321 = 0.60319259259259259259e1 * t7523;
    let t8332 = -t8321 - 0.4105e-2 * t7571 + 0.2463e-2 * t7573 + 0.821e-3 * t7576 - 0.54733333333333333333e-3 * t7580 - 0.12315e-2 * t7583 - 0.2585111111111111111e1 * t7525 + 0.19388333333333333333e1 * t7531 + 0.12925555555555555555e1 * t7527 - 0.21542592592592592592e1 * t7535 - 0.19388333333333333333e1 * t7550;
    let t8333 = t8320 + t8332;
    let t8335 = t2294 * t973;
    let t8338 = t970 * t2300;
    (t8333, t8335, t8338)
}
