//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 878/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk878<F: Float>(t2119: F, t7931: F, t13632: F, t13618: F, t20292: F, t26138: F, t26150: F, t26159: F, t30288: F, t30292: F, t30296: F, t30300: F, t30303: F, t30306: F, t1235: F, t20373: F, t26176: F, t26179: F) -> (F, F, F, F, F) {
    let t30326 = t7931 * t2119;
    let t30327 = t13632 * t30326;
    let t30339 = -t13618 - 4.0 / 9.0 * t20292 + 2.0 / 9.0 * t26138 - 2.0 / 3.0 * t26150 + t26159 / 3.0 - 10.0 / 27.0 * t30288 + 4.0 / 3.0 * t30292 - 2.0 / 3.0 * t30296 - 2.0 * t30300 + 2.0 * t30303 - t30306 / 3.0;
    let t30340 = t1235 * t30339;
    let t30350 = -0.59793333333333333333e0 * t30296 + 0.17938e1 * t30303 - 0.5477111111111111111e0 * t20373 - 0.39862222222222222223e0 * t20292 - 0.76790625e-1 * t30327 + 0.1898925e1 * t30340 + 0.10954222222222222222e0 * t26176 - 0.65725333333333333332e0 * t26179 - 0.59793333333333333333e0 * t26150 + 0.29896666666666666667e0 * t26159 + 0.19931111111111111111e0 * t26138 - 0.33218518518518518518e0 * t30288 + 0.11958666666666666667e1 * t30292 - 0.17938e1 * t30300;
    (t30326, t30327, t30339, t30340, t30350)
}
