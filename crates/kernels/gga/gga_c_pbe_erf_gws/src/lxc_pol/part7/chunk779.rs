//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 779/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk779<F: Float>(t100: F, t1504: F, t2182: F, t6868: F, t810: F, t3205: F, t858: F, t893: F, t3065: F, t2416: F, t891: F, t2081: F, t326: F, t6469: F, t329: F, t838: F) -> (F, F, F, F, F, F, F, F) {
    let t8335 = t1504 * t100;
    let t8524 = param_gamma * t2182;
    let t8556 = t6868 * t810;
    let t8599 = t3205 * t858;
    let t8605 = t858 * t893;
    let t8606 = t3065 * t8605;
    let t8734 = t891 * t2416;
    let t8782 = t326 * t6469 * t2081;
    let t8801 = t329 * t838 * t3205;
    (t8335, t8524, t8556, t8599, t8606, t8734, t8782, t8801)
}
