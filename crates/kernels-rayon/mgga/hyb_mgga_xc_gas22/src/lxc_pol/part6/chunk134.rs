//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 134/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk134(t345: f64, t348: f64, t351: f64, t355: f64) -> (f64, f64, f64) {
    let t383 = 0.51785e1_f64 * t348 + 0.905775e0_f64 * t345 + 0.1100325e0_f64 * t351 + 0.1241775e0_f64 * t355;
    let t386 = 1.0_f64 + 0.29608749977793437516e2_f64 / t383;
    let t387 = f64::ln(t386);
    (t383, t386, t387)
}
