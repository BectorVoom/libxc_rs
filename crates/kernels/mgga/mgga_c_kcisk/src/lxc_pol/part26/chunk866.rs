//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 866/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk866<F: Float>(t1336: F, t140: F, t454: F, t13959: F, t6230: F, t5598: F, t5631: F, t2076: F, t2869: F) -> (F, F, F, F, F) {
    let t19067 = t140 * t1336 * t454;
    let t19075 = t13959 * t6230;
    let t19076 = 0.14739506172839506172e-2 * t19075;
    let t19086 = t140 * t5598 * t5631;
    let t19100 = t2869 * t2076;
    (t19067, t19075, t19076, t19086, t19100)
}
