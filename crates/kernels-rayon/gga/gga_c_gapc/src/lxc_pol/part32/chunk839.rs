//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 839/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk839(t314: f64, t9739: f64, t8957: f64, t9738: f64, t197: f64, t7764: f64, t1077: f64, t3171: f64, t820: f64, t3443: f64, t869: f64, t896: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9740 = t9739 * t314;
    let t9741 = t8957 * t9740;
    let t9742 = t9738 * t9741;
    let t9744 = t197 * t7764;
    let t9745 = t1077 * t9744;
    let t9747 = t3171 * t820;
    let t9748 = t3443 * t9747;
    let t9750 = t869 * t896;
    (t9740, t9741, t9742, t9745, t9748, t9750)
}
