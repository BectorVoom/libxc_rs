//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 643/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk643<F: Float>(t1957: F, t7293: F, t2594: F, t5213: F, t5218: F, t5290: F, t6689: F) -> (F, F, F, F, F) {
    let t7294 = t7293 * t1957;
    let t7295 = t5213 * t2594;
    let t7296 = t2594 * t1957;
    let t7298 = 2.0 * t5218 * t7296;
    let t7299 = t5290 * t6689;
    (t7294, t7295, t7296, t7298, t7299)
}
