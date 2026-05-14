//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 870/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk870<F: Float>(t13917: F, t3954: F, t1309: F, t3943: F, t3973: F, t10471: F, t1337: F, t140: F, t1343: F, t3480: F, t3737: F) -> (F, F, F, F, F) {
    let t13918 = t13917 * t3954;
    let t13919 = t1309 * t13918;
    let t13923 = t3973 * t3943;
    let t13924 = t1309 * t13923;
    let t13955 = t140 * t10471 * t1337;
    let t13956 = t13955 * t1343;
    let t13959 = t140 * t3737 * t3480;
    (t13919, t13924, t13955, t13956, t13959)
}
