//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 796/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk796<F: Float>(t2641: F, t4961: F, t2669: F, t24: F, t4933: F, t862: F, t4937: F, t4929: F, t4983: F, t907: F, t106: F, t1392: F) -> (F, F, F, F, F, F, F) {
    let t14360 = t2641 * t4961;
    let t14390 = t2669 * t4961;
    let t14420 = t24 * t4933;
    let t14421 = t862 * t14420;
    let t14425 = t24 * t4937;
    let t14426 = t862 * t14425;
    let t14430 = t24 * t4929;
    let t14431 = t862 * t14430;
    let t14472 = t4983 * t907;
    let t14479 = t106 * t1392;
    (t14360, t14390, t14421, t14426, t14431, t14472, t14479)
}
