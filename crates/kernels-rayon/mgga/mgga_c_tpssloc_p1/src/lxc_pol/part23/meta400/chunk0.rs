//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1209/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1209(t2791: f64, t5689: f64, t10704: f64, t5726: f64, t2885: f64, t5737: f64, t2904: f64, t5769: f64, t10632: f64, t5790: f64, t11094: f64, t5946: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60357 = t5689 * t2791;
    let t60378 = t5726 * t10704;
    let t60407 = t5737 * t2885;
    let t60424 = t5769 * t2904;
    let t60722 = t5790 * t10632;
    let t60874 = t5946 * t11094;
    (t60357, t60378, t60407, t60424, t60722, t60874)
}
