//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 833/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk833<F: Float>(t495: F, t8778: F, t360: F, t277: F, t2892: F, t571: F, t7983: F, t2573: F, t2551: F, t2562: F, t2654: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8779 = t8778 * t495;
    let t8780 = t360 * t8779;
    let t8783 = t277 * t2892;
    let t8784 = t8783 * t495;
    let t8785 = t360 * t8784;
    let t8792 = t571 * t7983;
    let t8795 = t8778 * t2573;
    let t8796 = t360 * t8795;
    let t8799 = t8778 * t2551;
    let t8800 = t360 * t8799;
    let t8803 = t2562 * t2654;
    let t8804 = t360 * t8803;
    (t8779, t8780, t8783, t8784, t8785, t8792, t8795, t8796, t8799, t8800, t8803, t8804)
}
