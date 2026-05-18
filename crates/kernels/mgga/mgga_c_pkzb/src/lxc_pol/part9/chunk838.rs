//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 838/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk838<F: Float>(t5736: F, t5740: F, t5744: F, t5751: F, t5765: F, t5768: F, t5770: F, t5773: F, t5779: F, t5799: F, t5807: F, t5811: F, t5911: F) -> F {
    let t6070 = -t5744 - t5751 + t5770 + t5773 - t5779 + t5799 + t5807 + t5765 + t5768 - t5736 + t5740 + t5911 + t5811;
    t6070
}
