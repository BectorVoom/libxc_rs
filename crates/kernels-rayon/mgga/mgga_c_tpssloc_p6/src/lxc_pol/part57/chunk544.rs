//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 544/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk544(t1937: f64, t6726: f64, t1948: f64, t363: f64, t3034: f64, t334: f64, t1930: f64, t1934: f64, t344: f64, t1009: f64, t1014: f64, t1018: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6728 = 0.10093189023535097714e-3_f64 * t6726 * t1937;
    let t6734 = t1948 * t363;
    let t6739 = 1.0_f64 / t3034 / t334;
    let t6740 = t1930 * t6739;
    let t6741 = t1934 * t344;
    let t6742 = t6740 * t6741;
    let t6743 = t1009 * t1014;
    let t6744 = t6743 * t363;
    let t6753 = t1014 * sigma0;
    let t6754 = t6753 * t1018;
    (t6728, t6734, t6740, t6741, t6742, t6743, t6744, t6753, t6754)
}
