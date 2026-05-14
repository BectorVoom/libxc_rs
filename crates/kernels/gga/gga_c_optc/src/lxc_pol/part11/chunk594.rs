//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 594/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk594<F: Float>(t106: F, t1411: F, t2694: F, t335: F, t3853: F, t4983: F, t4990: F, t5049: F, t908: F, t1415: F, t1000: F, t4776: F, t914: F, t2549: F, t4768: F, t4772: F) -> (F, F, F, F, F, F, F) {
    let t5053 = 0.27818116767324025134e1 * t106 * t4983 * t335 - 0.55636233534648050268e1 * t106 * t3853 * t1411 + 0.55636233534648050268e1 * t106 * t2694 * t4990 - 0.27818116767324025134e1 * t106 * t908 * t5049;
    let t5059 = t1415 * t1415;
    let t5064 = t1000 * t4776;
    let t5065 = t914 * t5064;
    let t5068 = t2549 * t4768;
    let t5069 = t914 * t5068;
    let t5075 = t1000 * t4772;
    (t5053, t5059, t5064, t5065, t5068, t5069, t5075)
}
