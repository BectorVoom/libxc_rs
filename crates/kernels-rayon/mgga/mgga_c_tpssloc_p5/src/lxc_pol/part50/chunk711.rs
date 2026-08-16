//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 711/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk711(t1000: f64, t1025: f64, t1046: f64, t1935: f64, t1937: f64, t350: f64, t378: f64, t6712: f64, t6716: f64, t6717: f64, t6723: f64, t6728: f64, t6730: f64, t6735: f64, t6742: f64, t6747: f64, t6750: f64, t6755: f64, t6759: f64, t6763: f64, t6765: f64) -> f64 {
    let t6768 = -t6712 * t350 / 36.0_f64 + t6716 + t6717 * t1000 / 288.0_f64 - 0.80745512188280781712e-3_f64 * t6723 * t1937 + t6728 + 0.10093189023535097714e-3_f64 * t6730 * t1937 - 0.10093189023535097714e-3_f64 * t1935 * t6735 + 0.10093189023535097714e-3_f64 * t6742 * t6747 + t6750 * t378 / 1536.0_f64 + t6755 * t1025 / 1536.0_f64 - t6759 * t378 / 288.0_f64 + t6763 + t6765 * t1046 / 2304.0_f64;
    t6768
}
