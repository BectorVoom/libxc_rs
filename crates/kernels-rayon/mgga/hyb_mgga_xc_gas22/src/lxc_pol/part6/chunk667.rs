//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 667/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk667(t222: f64, t3309: f64, t37: f64, t2165: f64, t2167: f64, t3300: f64, t251: f64, t1333: f64, t787: f64) -> (f64, f64, f64, f64) {
    let t3311 = t222 * t37 * t3309;
    let t3313 = t2165 - 0.17808333333333333333e-1_f64 * t2167 - 0.17808333333333333333e-1_f64 * t3300 + 0.53425e-1_f64 * t3311;
    let t3315 = 0.621814e-1_f64 * t3313 * t251;
    let t3316 = t1333 * t787;
    (t3311, t3313, t3315, t3316)
}
