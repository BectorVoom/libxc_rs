//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 659/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk659<F: Float>(t7429: F, t7431: F, t1931: F, t2572: F, t7069: F, t747: F, t746: F, t1948: F, t5322: F, t6702: F) -> (F, F, F, F, F, F) {
    let t7432 = t7429 * t7431;
    let t7434 = t1931 * t2572;
    let t7436 = t747 * t7069;
    let t7437 = t746 * t7436;
    let t7438 = t1948 * t7437;
    let t7440 = t5322 * t6702;
    (t7432, t7434, t7436, t7437, t7438, t7440)
}
