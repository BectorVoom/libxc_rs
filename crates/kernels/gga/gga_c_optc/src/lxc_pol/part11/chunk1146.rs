//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1146/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1146<F: Float>(t16988: F, t2641: F, t17060: F, t24: F, t862: F, t17064: F, t17031: F, t24583: F, t16975: F, t2640: F, t32131: F, t17045: F, t2669: F) -> (F, F, F, F, F, F) {
    let t49850 = t2641 * t16988;
    let t49860 = t862 * t24 * t17060;
    let t49865 = t862 * t24 * t17064;
    let t49869 = t24583 * t17031;
    let t49882 = t2640 * t32131 * t16975;
    let t49896 = t2669 * t17045;
    (t49850, t49860, t49865, t49869, t49882, t49896)
}
