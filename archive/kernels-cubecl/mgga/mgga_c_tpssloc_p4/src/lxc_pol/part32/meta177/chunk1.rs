//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 883/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk883<F: Float>(t1100: F, t4756: F, t1661: F, t3287: F, t1102: F, t1107: F, t1667: F, t699: F) -> (F, F, F, F, F) {
    let t4757 = t1100 * t4756;
    let t4764 = t3287 * t1661;
    let t4765 = t4764 * t1102;
    let t4767 = t1107 * t4756;
    let t4770 = t699 * t1667;
    (t4757, t4764, t4765, t4767, t4770)
}
