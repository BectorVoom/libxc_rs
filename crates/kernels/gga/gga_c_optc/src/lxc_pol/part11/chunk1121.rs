//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1121/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1121<F: Float>(t16288: F, t1924: F, t193: F, t16247: F, t603: F, t75: F, t16579: F, t9412: F, t9416: F, t3546: F, t4744: F, t1256: F, t4595: F) -> (F, F, F, F, F, F) {
    let t47906 = t193 * t1924 * t16288;
    let t47938 = t16247 * t75 * t603;
    let t47955 = t9412 * t16579;
    let t47957 = t9416 * t16579;
    let t47989 = t3546 * t4744;
    let t48000 = t4595 * t1256;
    (t47906, t47938, t47955, t47957, t47989, t48000)
}
