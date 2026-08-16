//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1299/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1299(t1888: f64, t232: f64, t40909: f64, t6646: f64, t23177: f64, t6579: f64, t23143: f64, t6649: f64, t22999: f64, t22998: f64, t23185: f64, t81914: f64) -> (f64, f64, f64, f64, f64) {
    let t82003 = t1888 * t6646 * t40909 * t232;
    let t82005 = t6579 * t23177;
    let t82011 = t23143 * t6649;
    let t82013 = t6579 * t22999;
    let t82016 = t23185 * t81914 * t22998;
    (t82003, t82005, t82011, t82013, t82016)
}
