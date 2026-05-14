//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 456/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk456<F: Float>(t582: F, t996: F, t561: F, t1006: F, t583: F, t1076: F, t153: F, t542: F, t75: F, t959: F) -> (F, F, F, F, F) {
    let t2796 = t582 * t996;
    let t2797 = t561 * t2796;
    let t2807 = t1006 * t583;
    let t2837 = t153 * t542 * t1076;
    let t2840 = t959 * t75;
    (t2796, t2797, t2807, t2837, t2840)
}
