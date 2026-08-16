//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 820/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk820<F: Float>(t237: F, t5880: F, t5909: F, t5488: F, t5504: F, t5580: F, t5765: F, t5768: F, t5770: F, t5773: F, t5779: F, t5799: F, t5807: F, t5811: F) -> (F, F) {
    let t5911 = t237 * (t5880 + t5909);
    let t5912 = -t5580 + t5765 + t5768 + t5770 + t5773 - t5779 + t5799 + t5807 - t5488 + t5811 + t5504 + t5911;
    (t5911, t5912)
}
