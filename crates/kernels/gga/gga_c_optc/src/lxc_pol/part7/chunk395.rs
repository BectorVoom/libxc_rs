//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 395/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk395<F: Float>(t1906: F, t85: F, t755: F, t201: F, t5: F, t743: F) -> (F, F, F, F) {
    let t1908 = F::new(0.19751789702565206229e-1) * t1906 * t85;
    let t1909 = t755 * t755;
    let t1911 = t5 * t1909 * t201;
    let t1912 = t743 * t1911;
    (t1908, t1909, t1911, t1912)
}
