//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1026/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1026<F: Float>(t14698: F, t3972: F, t13948: F, t13954: F, t13962: F, t13964: F, t14664: F, t14669: F, t14674: F, t14678: F, t14680: F, t14685: F, t14689: F, t14693: F, t3066: F, t13780: F, t3060: F, t3990: F) -> (F, F) {
    let t14699 = t3972 * t14698;
    let t14703 = t3066 * t14664 / 48.0 + t3066 * t14669 / 48.0 + t14674 / 96.0 + t14678 / 96.0 + t14680 / 96.0 + t14685 / 1536.0 - 7.0 / 288.0 * t14689 - t13948 - t14693 / 3072.0 + 7.0 / 288.0 * t13954 + t14699 / 768.0 + 7.0 / 288.0 * t13962 + 7.0 / 4608.0 * t13964;
    let t14705 = t3990 * t13780 * t3060;
    (t14703, t14705)
}
