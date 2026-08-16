//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1283/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1283(t1230: f64, t7834: f64, t10111: f64, t1804: f64, t6214: f64, t1055: f64, t239: f64, t24: f64, t20127: f64, t3815: f64, t23048: f64, t2970: f64, t9846: f64) -> (f64, f64, f64, f64, f64) {
    let t27759 = t7834 * t1230;
    let t27766 = t1804 * t6214 * t10111;
    let t27770 = t24 / t239 / t1055;
    let t27777 = t1804 * t20127 * t3815;
    let t27789 = t2970 * t23048 * t9846;
    (t27759, t27766, t27770, t27777, t27789)
}
