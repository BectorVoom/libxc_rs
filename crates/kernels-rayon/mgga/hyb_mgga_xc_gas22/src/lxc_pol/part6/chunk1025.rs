//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1025/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1025(t2874: f64, t518: f64, t2867: f64, t3687: f64, t1535: f64, t531: f64, tau0: f64) -> (f64, f64, f64, f64) {
    let t9620 = t2874 * tau0;
    let t9621 = t518 * t9620;
    let t9624 = t2867 * t3687;
    let t9625 = t531 * t1535;
    (t9620, t9621, t9624, t9625)
}
