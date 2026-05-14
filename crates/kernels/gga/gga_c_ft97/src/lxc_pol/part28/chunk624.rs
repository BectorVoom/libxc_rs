//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 624/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk624<F: Float>(t1060: F, t2185: F, t5860: F, t558: F, t6718: F, t574: F, t605: F, t144: F, t26521: F, t616: F, t6630: F, t1017: F, t5842: F) -> (F, F, F, F, F, F) {
    let t26894 = t2185 * t1060 * t5860;
    let t26897 = t6718 * t558;
    let t26899 = t574 * t605 * t26897;
    let t26902 = t144 * t26521;
    let t26906 = t2185 * t616 * t6630;
    let t26909 = t5842 * t1017;
    (t26894, t26897, t26899, t26902, t26906, t26909)
}
