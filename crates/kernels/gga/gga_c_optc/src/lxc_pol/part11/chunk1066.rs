//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1066/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1066<F: Float>(t1382: F, t24407: F, t1388: F, t3843: F, t893: F, t1384: F, t7894: F, t874: F, t24447: F, t25217: F, t1397: F, t3902: F, t913: F) -> (F, F, F, F, F, F, F) {
    let t31579 = t24407 * t1382;
    let t31718 = t3843 * t1388;
    let t31719 = t893 * t31718;
    let t31765 = t874 * t7894 * t1384;
    let t32008 = t24447 * t1382;
    let t32131 = t25217 * t1382;
    let t32252 = t913 * t3902 * t1397;
    (t31579, t31718, t31719, t31765, t32008, t32131, t32252)
}
