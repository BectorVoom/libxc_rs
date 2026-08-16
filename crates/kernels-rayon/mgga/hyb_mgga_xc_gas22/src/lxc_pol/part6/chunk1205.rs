//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1205/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1205(t2856: f64, t528: f64, t530: f64, t22703: f64, t2867: f64, t7805: f64, t1143: f64, t9557: f64, t1166: f64, t9526: f64, t509: f64, t515: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22714 = 1.0_f64 / t530 / t2856 / t528 / 2.0_f64;
    let t22723 = 1.0_f64 / t22703;
    let t22746 = t2867 * t7805;
    let t22750 = t1143 * t9557;
    let t22754 = t1166 * t9526;
    let t22809 = t515 * t509;
    (t22714, t22723, t22746, t22750, t22754, t22809)
}
