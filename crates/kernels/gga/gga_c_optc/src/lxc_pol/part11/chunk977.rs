//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 977/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk977<F: Float>(t1388: F, t3843: F, t893: F, t1384: F, t7894: F, t874: F, t1382: F, t24447: F, t25217: F, t1397: F, t3902: F, t913: F, t953: F, t1378: F, t930: F, t1405: F, t940: F) -> (F, F, F, F, F, F, F, F) {
    let t31718 = t3843 * t1388;
    let t31719 = t893 * t31718;
    let t31765 = t874 * t7894 * t1384;
    let t32008 = t24447 * t1382;
    let t32131 = t25217 * t1382;
    let t32252 = t913 * t3902 * t1397;
    let t32576 = t953 * t31718;
    let t32722 = t930 * t3902 * t1378;
    let t33398 = t940 * t3843 * t1405;
    (t31719, t31765, t32008, t32131, t32252, t32576, t32722, t33398)
}
