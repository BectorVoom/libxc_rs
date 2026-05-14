//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 562/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk562<F: Float>(t1336: F, t140: F, t4594: F, t1801: F, t4640: F, t1800: F, t1870: F, t715: F) -> (F, F, F, F, F) {
    let t5054 = t140 * t1336 * t4594;
    let t5055 = t1801 * t4640;
    let t5056 = t1800 * t5055;
    let t5057 = t5054 * t5056;
    let t5060 = 1.0 / t1870 / t715;
    (t5054, t5055, t5056, t5057, t5060)
}
