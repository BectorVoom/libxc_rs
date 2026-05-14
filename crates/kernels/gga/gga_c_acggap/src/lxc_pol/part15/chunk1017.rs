//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1017/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1017<F: Float>(t2297: F, t8406: F, t13299: F, t31115: F, t1788: F, t31110: F, t2041: F, t5632: F, t1805: F, t7329: F, t2001: F, t5539: F, t31346: F, t6328: F, t6140: F, t5891: F, t7561: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40116 = t2297 * t8406;
    let t40118 = t31115 * t13299 * t40116;
    let t40121 = t31110 * t1788;
    let t40123 = t2041 * t5632;
    let t40126 = t7329 * t1805;
    let t40131 = t2001 * t5539;
    let t40134 = t31346 * t6328;
    let t40136 = t31346 * t6140;
    let t40145 = t7561 * t5891;
    (t40116, t40118, t40121, t40123, t40126, t40131, t40134, t40136, t40145)
}
