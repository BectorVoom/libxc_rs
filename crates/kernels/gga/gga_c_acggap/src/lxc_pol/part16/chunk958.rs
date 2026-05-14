//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 958/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk958<F: Float>(t1849: F, t30148: F, t30154: F, t7842: F, t30937: F, t9608: F, t1181: F, t5527: F, t7564: F, t8600: F, t24196: F, t336: F, t570: F, t38766: F, t604: F, t7413: F) -> (F, F, F, F, F) {
    let t38976 = t30154 * t7842 * t30148 * t1849;
    let t38978 = t30937 * t9608;
    let t38982 = t7564 * t1181 * t8600 * t5527;
    let t38986 = t570 * t336 * t24196;
    let t38990 = t7413 * t1181 * t604 * t38766;
    (t38976, t38978, t38982, t38986, t38990)
}
