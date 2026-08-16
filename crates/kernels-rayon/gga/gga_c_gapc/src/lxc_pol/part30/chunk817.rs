//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 817/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk817(t4043: f64, t9736: f64, t311: f64, t134: f64, t959: f64, t314: f64, t8957: f64, t197: f64, t7764: f64, t1077: f64, t3171: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9737 = t9736 * t4043;
    let t9738 = t311 * t9737;
    let t9739 = t134 * t959;
    let t9740 = t9739 * t314;
    let t9741 = t8957 * t9740;
    let t9742 = t9738 * t9741;
    let t9744 = t197 * t7764;
    let t9745 = t1077 * t9744;
    let t9747 = t3171 * t820;
    (t9739, t9740, t9741, t9742, t9745, t9747)
}
