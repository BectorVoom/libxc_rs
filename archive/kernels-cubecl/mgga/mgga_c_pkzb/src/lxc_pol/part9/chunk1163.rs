//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1163/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1163<F: Float>(t16592: F, t16599: F, t16607: F, t16612: F, t19736: F, t19737: F, t19738: F, t19741: F, t19743: F, t19748: F, t19749: F, t19750: F, t19751: F, t19752: F, t19755: F, t19757: F, t19759: F) -> F {
    let t20321 = t19736 - t16592 - t19737 - t19738 + t16599 - t19741 + t19743 + t16607 - t16612 + t19748 + t19749 - t19750 - t19751 - t19752 + t19755 - t19757 + t19759;
    t20321
}
