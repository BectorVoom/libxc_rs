//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1248/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1248(t1433: f64, t2599: f64, t7109: f64, t6993: f64, t1409: f64, t2521: f64, t7148: f64, t1056: f64, t3622: f64, t3630: f64, t2707: f64, t9321: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25813 = t2599 * t1433;
    let t25816 = t7109 * t1433;
    let t25819 = t6993 * t1433;
    let t25823 = t2521 * t1409;
    let t25826 = t7148 * t1409;
    let t25907 = 32.0_f64 * t3622 * t1056;
    let t25930 = 32.0_f64 * t3630 * t1056;
    let t25937 = t9321 * t2707;
    (t25813, t25816, t25819, t25823, t25826, t25907, t25930, t25937)
}
