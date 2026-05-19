//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 317/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk317<F: Float>(t1002: F, t1010: F, t1015: F, t277: F, t355: F, t364: F, t776: F, t802: F, t842: F, t844: F, t849: F, t95: F, t960: F, t962: F, t984: F, t989: F, t995: F, t999: F) -> F {
    let t1018 = -t776 + t802 + t842 + t844 - t849 + F::cast_from(0.25844881434903430496e-2_f64) * t95 * t277 * t960 * t962 + t984 * t364 / F::new(2.0) - F::new(4.0) / F::new(3.0) * t355 * t989 + t995 + t999 * t1002 / F::new(6.0) + F::new(50.0) / F::new(27.0) * t1010 * t1015;
    t1018
}
