//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1336/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1336(t20838: f64, t4143: f64, t2189: f64, t2234: f64, t4140: f64, t10703: f64, t2311: f64, t3352: f64, t2188: f64, t810: f64, t25146: f64, t8618: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29068 = 0.16081979498692535067e2_f64 * t20838 * t4143;
    let t29071 = 6.0_f64 * t2234 * t4140 * t2189;
    let t29072 = t2311 * t10703;
    let t29076 = t3352 * t3352;
    let t29079 = 4.0_f64 * t2188 * t29076 * t810;
    let t29081 = 0.38596750796862084161e3_f64 * t25146 * t8618;
    (t29068, t29071, t29072, t29076, t29079, t29081)
}
