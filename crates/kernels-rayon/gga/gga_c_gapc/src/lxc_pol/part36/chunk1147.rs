//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1147/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1147(t11966: f64, t28346: f64, t189: f64, t1899: f64, t15508: f64, t90: f64, t18680: f64, t277: f64, t327: f64, t2394: f64, t3750: f64, t9624: f64) -> (f64, f64, f64, f64, f64) {
    let t34193 = t11966 * t28346;
    let t34195 = t189 * t1899;
    let t34197 = t15508 * t90;
    let t34200 = t277 * t34195 * t34197 * t327 * t18680;
    let t34205 = t2394 * t3750 * t9624;
    (t34193, t34195, t34197, t34200, t34205)
}
