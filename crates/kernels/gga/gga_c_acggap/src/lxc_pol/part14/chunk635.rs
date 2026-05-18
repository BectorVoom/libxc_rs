//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 635/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk635<F: Float>(t1008: F, t1901: F, t435: F, t5674: F, t386: F, t387: F, t1579: F, t2325: F, t436: F, t5679: F, t1896: F, t1574: F, t1894: F) -> (F, F, F, F, F, F) {
    let t6098 = t1008 * t1901;
    let t6100 = t435 * t5674;
    let t6102 = t386 * t387 * t6100;
    let t6106 = t386 * t2325 * t1579;
    let t6110 = t386 * t5679 * t436;
    let t6113 = t1008 * t1896;
    let t6116 = t386 * t1574 * t1894;
    (t6098, t6102, t6106, t6110, t6113, t6116)
}
