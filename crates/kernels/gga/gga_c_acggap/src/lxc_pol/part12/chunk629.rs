//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 629/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk629<F: Float>(t3196: F, t599: F, t1181: F, t7337: F, t3176: F, t604: F, t2068: F, t1160: F, t2067: F) -> (F, F, F, F, F, F, F) {
    let t7338 = t599 * t3196;
    let t7339 = t1181 * t7338;
    let t7340 = t7337 * t7339;
    let t7342 = t604 * t3176;
    let t7343 = t1181 * t7342;
    let t7344 = t2068 * t7343;
    let t7346 = t1160 * t2067;
    (t7338, t7339, t7340, t7342, t7343, t7344, t7346)
}
