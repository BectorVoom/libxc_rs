//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 920/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk920(t2047: f64, t5611: f64, t5584: f64, t193: f64, t7859: f64, t111: f64, t28942: f64, t12020: f64, t7936: f64, t1824: f64, t7918: f64, t2085: f64, t6414: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t101708 = t2047 * t5611;
    let t101715 = t2047 * t5584;
    let t101840 = t193 * t7859;
    let t102386 = t28942 * t111;
    let t102466 = t12020 * t7936;
    let t102562 = t7918 * t1824;
    let t102587 = t2085 * t6414;
    (t101708, t101715, t101840, t102386, t102466, t102562, t102587)
}
