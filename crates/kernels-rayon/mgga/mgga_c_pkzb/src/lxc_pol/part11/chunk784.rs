//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 784/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk784(t276: f64, t7620: f64, t154: f64, t2048: f64, t2739: f64, t2932: f64, t5974: f64, t2104: f64, t2029: f64, t2916: f64, t178: f64, t5723: f64) -> (f64, f64, f64, f64, f64) {
    let t7621 = t276 * t7620;
    let t7628 = t154 * t2048 * t2739;
    let t7630 = t276 * t7628 / 144.0_f64;
    let t7637 = t5974 * t2932;
    let t7639 = 0.57165357490759649296e-3_f64 * t2104 * t7637;
    let t7653 = t2916 * t2029;
    let t7663 = t5723 * t178;
    (t7621, t7630, t7639, t7653, t7663)
}
