//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 937/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk937<F: Float>(t29593: F, t735: F, t734: F, t28303: F, t7311: F, t5321: F, t2586: F, t8971: F, t1948: F, t2572: F, t9016: F, t28963: F, t719: F) -> (F, F, F, F, F) {
    let t29594 = t735 * t29593;
    let t29595 = t734 * t29594;
    let t29597 = t7311 * t28303;
    let t29598 = t5321 * t29597;
    let t29600 = t2586 * t8971;
    let t29601 = t1948 * t29600;
    let t29603 = t9016 * t2572;
    let t29605 = t719 * t28963;
    (t29595, t29598, t29601, t29603, t29605)
}
