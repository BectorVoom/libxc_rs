//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 613/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk613<F: Float>(t277: F, t364: F, t4033: F, t4783: F, t4785: F, t4817: F, t4821: F, t4851: F, t4858: F, t4900: F, t4927: F, t5053: F, t5079: F, t95: F, t962: F) -> F {
    let t5080 = t4783 + t4785 + t4817 + t4821 + t4033 / F::cast_from(3.0_f64) + t4851 * t364 / F::cast_from(2.0_f64) + t4858 + t4927 + F::cast_from(0.25844881434903430496e-2_f64) * t95 * t277 * t5053 * t962 - t4900 + t5079;
    t5080
}
