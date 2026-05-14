//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1021/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1021<F: Float>(t193: F, t39009: F, t4752: F, t16288: F, t1924: F, t16247: F, t603: F, t75: F, t16579: F, t9412: F, t9416: F, t3546: F, t4744: F, t1256: F, t4595: F, t172: F, t4599: F) -> (F, F, F, F, F, F, F, F) {
    let t47896 = t193 * t39009 * t4752;
    let t47906 = t193 * t1924 * t16288;
    let t47938 = t16247 * t75 * t603;
    let t47955 = t9412 * t16579;
    let t47957 = t9416 * t16579;
    let t47989 = t3546 * t4744;
    let t48000 = t4595 * t1256;
    let t48009 = t4599 * t172;
    (t47896, t47906, t47938, t47955, t47957, t47989, t48000, t48009)
}
