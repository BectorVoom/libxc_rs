//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 604/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk604<F: Float>(t8865: F, t8963: F, t752: F, t2594: F, t7293: F, t5218: F, t747: F, t8939: F, t746: F, t1948: F, t196: F, t8616: F, t5248: F, t5249: F, t7715: F, t1919: F, t2063: F, t7389: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8964 = t8865 + t8963;
    let t8965 = t8964 * t752;
    let t8967 = 2.0 * t7293 * t2594;
    let t8968 = t2594 * t2594;
    let t8970 = 2.0 * t5218 * t8968;
    let t8971 = t747 * t8939;
    let t8972 = t746 * t8971;
    let t8973 = t1948 * t8972;
    let t8975 = t8616 * t196;
    let t8995 = t5248 * t5249 * t7715;
    let t8999 = t1919 * t7389 * t2063;
    (t8964, t8965, t8967, t8968, t8970, t8971, t8972, t8973, t8975, t8995, t8999)
}
