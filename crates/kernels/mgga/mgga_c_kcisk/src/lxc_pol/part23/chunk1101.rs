//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1101/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1101<F: Float>(t1620: F, t6638: F, t15084: F, t1611: F, t20918: F, t20921: F, t20924: F, t20925: F, t20926: F, t22052: F, t22056: F, t22151: F, t2347: F, t4535: F, t4565: F, t6604: F) -> (F, F) {
    let t22153 = t6638 * t1620;
    let t22157 = -t15084 * t2347 - t1611 * t22151 - 2.0 * t1620 * t22056 + 2.0 * t22052 * t4535 + 4.0 * t22153 * t4535 - t4565 * t6604 - t20918 + t20921 - t20924 + t20925 + t20926;
    (t22153, t22157)
}
