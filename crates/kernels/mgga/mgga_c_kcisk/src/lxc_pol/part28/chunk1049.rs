//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1049/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1049<F: Float>(t24079: F, t752: F, t1907: F, t8964: F, t1957: F, t17772: F, t2594: F, t17775: F, t7296: F, t7293: F, t7444: F, t11694: F, t8968: F, t11701: F, t5218: F, t5213: F, t9094: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t24080 = t24079 * t752;
    let t24081 = t8964 * t1907;
    let t24082 = t24081 * t1957;
    let t24084 = 2.0 * t17772 * t2594;
    let t24086 = 4.0 * t17775 * t7296;
    let t24088 = 2.0 * t7293 * t7444;
    let t24090 = 2.0 * t11694 * t8968;
    let t24091 = t8968 * t1957;
    let t24093 = 6.0 * t11701 * t24091;
    let t24094 = t2594 * t7444;
    let t24096 = 4.0 * t5218 * t24094;
    let t24097 = t5213 * t9094;
    (t24080, t24081, t24082, t24084, t24086, t24088, t24090, t24091, t24093, t24094, t24096, t24097)
}
