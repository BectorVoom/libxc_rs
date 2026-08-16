//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1841/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1841(t26028: f64, t7032: f64, t26016: f64, t84173: f64, t26959: f64, t6486: f64, t1860: f64, t26024: f64, t7031: f64, t2031: f64, t90090: f64, t26012: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92008 = 16.0_f64 / 9.0_f64 * t26028 * t7032;
    let t92012 = 160.0_f64 / 9.0_f64 * t26016 * t84173;
    let t92031 = 16.0_f64 / 9.0_f64 * t6486 * t26959;
    let t92034 = 16.0_f64 / 9.0_f64 * t1860 * t7031 * t26024;
    let t92040 = t2031 * t90090;
    let t92047 = t7031 * t26012;
    (t92008, t92012, t92031, t92034, t92040, t92047)
}
