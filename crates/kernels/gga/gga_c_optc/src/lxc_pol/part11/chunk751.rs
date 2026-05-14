//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 751/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk751<F: Float>(t13911: F, t998: F, t4786: F, t7512: F, t7557: F, t4895: F, t778: F, t2569: F, t5053: F, t2476: F, t4919: F, t4854: F, t7504: F, t4780: F, t828: F, t2520: F, t4884: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13912 = t13911 * t998;
    let t13939 = t7512 * t4786;
    let t13947 = t7557 * t4786;
    let t13998 = t4895 * t778;
    let t14029 = t5053 * t2569;
    let t14091 = t4919 * t2476;
    let t14098 = t4854 * t7504;
    let t14102 = t4780 * t828;
    let t14148 = t4884 * t2520;
    (t13912, t13939, t13947, t13998, t14029, t14091, t14098, t14102, t14148)
}
