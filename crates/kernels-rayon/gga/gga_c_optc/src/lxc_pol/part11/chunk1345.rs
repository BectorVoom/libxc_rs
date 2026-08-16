//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1345/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1345(t1027: f64, t55927: f64, t3018: f64, t5171: f64, t5186: f64, t15562: f64, t5308: f64, t5202: f64) -> (f64, f64, f64, f64) {
    let t58295 = t1027 * t55927;
    let t58308 = 36.0_f64 * t3018 * t5171 * t5186;
    let t58310 = 0.1038945353962551798e3_f64 * t15562 * t5308;
    let t58311 = t5202 * t5202;
    (t58295, t58308, t58310, t58311)
}
