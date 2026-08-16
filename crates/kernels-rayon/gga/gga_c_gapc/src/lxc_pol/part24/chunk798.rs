//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 798/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk798(t1074: f64, t9532: f64, t1018: f64, t876: f64, t3272: f64, t1045: f64, t7442: f64, t1092: f64, t2542: f64, t3281: f64, t7208: f64, t906: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9533 = t9532 * t1074;
    let t9535 = t1018 * t876;
    let t9536 = t3272 * t9535;
    let t9538 = t1045 * t7442;
    let t9539 = t1092 * t9538;
    let t9541 = t2542 * t3281;
    let t9543 = t7208 * t906;
    (t9533, t9536, t9538, t9539, t9541, t9543)
}
