//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1042/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1042<F: Float>(t10838: F, t16636: F, t862: F, t17035: F, t24503: F, t17056: F, t24: F, t16988: F, t2641: F, t17060: F, t17064: F, t17031: F, t24583: F, t16975: F, t2640: F, t32131: F) -> (F, F, F, F, F, F, F, F) {
    let t49816 = t862 * t10838 * t16636;
    let t49822 = t24503 * t17035;
    let t49833 = t862 * t24 * t17056;
    let t49850 = t2641 * t16988;
    let t49860 = t862 * t24 * t17060;
    let t49865 = t862 * t24 * t17064;
    let t49869 = t24583 * t17031;
    let t49882 = t2640 * t32131 * t16975;
    (t49816, t49822, t49833, t49850, t49860, t49865, t49869, t49882)
}
