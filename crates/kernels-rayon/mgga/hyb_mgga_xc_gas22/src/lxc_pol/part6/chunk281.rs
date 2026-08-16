//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 281/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk281(t952: f64, t957: f64, t238: f64, t353: f64, t801: f64, t343: f64, t940: f64, t242: f64, t942: f64, t953: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t958 = t957 * t952;
    let t961 = t238 * t801 * t353;
    let t962 = 0.82156666666666666667e-1_f64 * t961;
    let t963 = t343 * t940;
    let t965 = t238 * t242 * t963;
    let t967 = 0.1898925e1_f64 * t953 - t955 + 0.8969e0_f64 * t942 + 0.3071625e0_f64 * t958 - t962 + 0.24647e0_f64 * t965;
    (t958, t961, t962, t963, t965, t967)
}
