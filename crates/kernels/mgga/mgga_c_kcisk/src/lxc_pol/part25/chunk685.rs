//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 685/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk685<F: Float>(t1957: F, t7293: F, t2594: F, t5213: F, t5218: F, t5290: F, t6689: F, t5289: F, t718: F, t733: F, t1755: F, t41: F) -> (F, F, F, F, F, F, F, F) {
    let t7294 = t7293 * t1957;
    let t7295 = t5213 * t2594;
    let t7296 = t2594 * t1957;
    let t7298 = 2.0 * t5218 * t7296;
    let t7299 = t5290 * t6689;
    let t7300 = t5289 * t7299;
    let t7302 = t733 * t718;
    let t7303 = t41 * t1755;
    (t7294, t7295, t7296, t7298, t7299, t7300, t7302, t7303)
}
