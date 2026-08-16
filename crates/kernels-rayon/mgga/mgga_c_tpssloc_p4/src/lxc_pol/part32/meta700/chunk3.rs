//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2196/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2196(t24987: f64, t7688: f64, t1874: f64, t75560: f64, t19451: f64, t6525: f64, t25994: f64, t4028: f64, t55943: f64, t191: f64, t192: f64, t19537: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97794 = 6.0_f64 * t24987 * t7688;
    let t97796 = 2.0_f64 * t75560 * t1874;
    let t97798 = 2.0_f64 * t19451 * t6525;
    let t97800 = 4.0_f64 * t4028 * t25994;
    let t97802 = 2.0_f64 * t55943 * t1874;
    let t97804 = t19537 * t191 * t192;
    (t97794, t97796, t97798, t97800, t97802, t97804)
}
