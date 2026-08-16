//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 989/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk989<F: Float>(t4327: F, t5313: F, t17978: F, t3109: F, t155: F, t17648: F, t464: F, t17855: F, t438: F, t449: F, t894: F, t18023: F, t3151: F) -> (F, F, F, F, F, F) {
    let t18085 = t4327 * t5313;
    let t18088 = t17978 * t3109;
    let t18092 = t155 * t464 * t17648;
    let t18098 = t449 * t17855 * t438;
    let t18099 = t894 * t18098;
    let t18102 = t3151 * t18023;
    (t18085, t18088, t18092, t18098, t18099, t18102)
}
