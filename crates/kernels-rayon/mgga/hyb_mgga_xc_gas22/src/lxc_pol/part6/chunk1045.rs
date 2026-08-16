//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1045/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1045(t9463: f64, t9511: f64, t9565: f64, t9617: f64, t9677: f64, t9723: f64, t9764: f64, t9810: f64, t500: f64, t3918: f64, t550: f64, t19: f64) -> (f64, f64, f64, f64) {
    let t9813 = t9463 + t9511 + t9565 + t9617 + t9677 + t9723 + t9764 + t9810;
    let t9814 = t500 * t9813;
    let t9824 = t550 * t3918;
    let t9825 = t19 * t9824;
    (t9813, t9814, t9824, t9825)
}
