//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 284/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk284(t949: f64, t969: f64, t935: f64, t942: f64) -> (f64, f64, f64) {
    let t971 = 1.0_f64 * t949 * t969;
    let t972 = 0.17123333333333333333e-1_f64 * t935;
    let t974 = -t972 + 0.5137e-1_f64 * t942;
    (t971, t972, t974)
}
