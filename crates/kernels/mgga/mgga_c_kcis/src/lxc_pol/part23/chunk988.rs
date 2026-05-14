//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 988/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk988<F: Float>(t4142: F, t8165: F, t1598: F, t17287: F, t5737: F, t7899: F, t6176: F, t5633: F, t7931: F, t303: F, t553: F, t5757: F, t1459: F, t2012: F, t1014: F, t8179: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28395 = t4142 * t8165;
    let t28397 = t17287 * t1598;
    let t28402 = t7899 * t5737;
    let t28403 = t6176 * t28402;
    let t28406 = t7931 * t5633;
    let t28407 = t303 * t28406;
    let t28409 = t553 * t5757;
    let t28410 = t303 * t28409;
    let t28412 = t1459 * t2012;
    let t28413 = t303 * t28412;
    let t28415 = t1014 * t8179;
    (t28395, t28397, t28402, t28403, t28406, t28407, t28409, t28410, t28412, t28413, t28415)
}
