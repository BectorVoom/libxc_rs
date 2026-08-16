//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 986/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk986(t2484: f64, t3490: f64, t952: f64, t1399: f64, t7009: f64, t2485: f64, t7025: f64, t1405: f64, t2213: f64, t238: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9148 = t2484 * t3490;
    let t9149 = t9148 * t952;
    let t9151 = t7009 * t1399;
    let t9152 = t9151 * t2485;
    let t9154 = t7025 * t1399;
    let t9155 = t9154 * t2485;
    let t9159 = t238 * t2213 * t1405;
    (t9149, t9151, t9152, t9154, t9155, t9159)
}
