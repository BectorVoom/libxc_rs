//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 478/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk478<F: Float>(t1450: F, t3742: F, t1415: F, t1411: F, t1337: F, t140: F, t3737: F) -> (F, F, F, F) {
    let t3743 = t1450 * t3742;
    let t3744 = t1415 * t3743;
    let t3745 = t1411 * t3744;
    let t3748 = t140 * t3737 * t1337;
    (t3743, t3744, t3745, t3748)
}
