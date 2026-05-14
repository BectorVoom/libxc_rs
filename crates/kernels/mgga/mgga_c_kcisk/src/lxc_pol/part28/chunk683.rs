//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 683/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk683<F: Float>(t5182: F, t8486: F, t682: F, t7715: F, t2487: F, t6734: F, t7718: F) -> (F, F, F, F, F) {
    let t8487 = t5182 * t8486;
    let t8491 = t682 * t7715;
    let t8494 = t6734 * t2487;
    let t8497 = t682 * t7718;
    let t8500 = t2487 * t2487;
    (t8487, t8491, t8494, t8497, t8500)
}
