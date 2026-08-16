//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1394/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1394(t28025: f64, t7458: f64, t28864: f64, t4028: f64, t28002: f64, t7468: f64, t1874: f64, t67001: f64, t1799: f64, t6463: f64, t22574: f64, t8643: f64) -> (f64, f64, f64, f64, f64) {
    let t106891 = 6.0_f64 * t7458 * t28025;
    let t106895 = 6.0_f64 * t4028 * t28864;
    let t106899 = 12.0_f64 * t28002 * t7468;
    let t106901 = 2.0_f64 * t67001 * t1874;
    let t106902 = t1799 * t6463;
    let t106905 = 9.0_f64 * t22574 * t8643 * t106902;
    (t106891, t106895, t106899, t106901, t106905)
}
