//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1993/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1993<F: Float>(t22674: F, t22892: F, t22916: F, t22716: F, t6908: F, t22751: F, t22930: F, t22917: F, t22723: F, t22891: F, t22920: F, t117: F, t5247: F, t6559: F) -> (F, F, F, F, F, F, F) {
    let t80659 = t22892 * t22674 * t22916;
    let t80663 = t22716 * t6908;
    let t80665 = t22751 * t22930;
    let t80667 = t22751 * t22917;
    let t80670 = t22723 * t22891;
    let t80671 = t80670 * t22920;
    let t80681 = t6559 * t5247 * t117;
    (t80659, t80663, t80665, t80667, t80670, t80671, t80681)
}
