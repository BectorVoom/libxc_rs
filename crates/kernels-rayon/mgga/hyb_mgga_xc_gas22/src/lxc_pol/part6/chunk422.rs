//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 422/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk422(t550: f64, t641: f64, t669: f64, t584: f64, t25: f64, t459: f64) -> (f64, f64, f64, f64) {
    let t1874 = t550 * t641;
    let t1877 = t550 * t669;
    let t1880 = t584 * t584;
    let t1884 = 1.0_f64 / t25 / t459;
    (t1874, t1877, t1880, t1884)
}
