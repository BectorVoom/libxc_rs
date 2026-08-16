//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 587/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk587(t1484: f64, t6638: f64, t6637: f64, t6552: f64, t232: f64, t4282: f64, t6646: f64, t1888: f64, t1519: f64, t1894: f64, t214: f64, t1880: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7520 = t6638 * t1484;
    let t7521 = t6637 * t7520;
    let t7522 = t6552 * t7521;
    let t7524 = t4282 * t232;
    let t7525 = t6646 * t7524;
    let t7526 = t1888 * t7525;
    let t7528 = t1894 * t1519;
    let t7529 = t214 * t7528;
    let t7530 = t1880 * t7529;
    (t7520, t7521, t7522, t7524, t7525, t7526, t7528, t7529, t7530)
}
