//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 967/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk967(t8887: f64, t8899: f64, t829: f64, t2229: f64, t3316: f64, t1333: f64, t2233: f64) -> (f64, f64, f64, f64) {
    let t8900 = t8887 + t8899;
    let t8901 = t8900 * t829;
    let t8905 = 1.0_f64 * t3316 * t2229;
    let t8906 = t1333 * t2233;
    (t8900, t8901, t8905, t8906)
}
