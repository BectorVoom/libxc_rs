//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 511/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk511<F: Float>(t209: F, t221: F, t5590: F, t1212: F, t589: F, t605: F, t1494: F) -> (F, F, F, F, F) {
    let t5592 = t221 * t5590 * t209;
    let t5595 = t589 * t1212;
    let t5597 = t221 * t5595 * t209;
    let t5600 = t605 * t1212;
    let t5601 = t5600 * t209;
    let t5602 = t221 * t5601;
    let t5605 = t1494 * t209;
    (t5592, t5597, t5601, t5602, t5605)
}
