//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1131/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1131(t6646: f64, t7524: f64, t1888: f64, t1519: f64, t1894: f64, t214: f64, t1880: f64, t1530: f64, t25: f64, t1484: f64, t28: f64, t1458: f64, t88: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7525 = t6646 * t7524;
    let t7526 = t1888 * t7525;
    let t7528 = t1894 * t1519;
    let t7529 = t214 * t7528;
    let t7530 = t1880 * t7529;
    let t7545 = t25 * t1530;
    let t7649 = t28 * t1484;
    let t7656 = t28 * t1530;
    let t7676 = t88 * t1458;
    (t7525, t7526, t7528, t7529, t7530, t7545, t7649, t7656, t7676)
}
