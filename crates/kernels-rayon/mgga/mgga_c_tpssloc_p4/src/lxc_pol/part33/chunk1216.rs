//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1216/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1216(t22814: f64, t80782: f64, t22823: f64, t281: f64, t22690: f64, t3787: f64, t1336: f64, t6943: f64, t836: f64, t1995: f64, t1999: f64, t213: f64, t39041: f64) -> (f64, f64, f64, f64, f64) {
    let t80783 = t22814 * t80782;
    let t80791 = t22823 * t281;
    let t80798 = t22690 * t3787;
    let t80820 = t1336 * t6943 * t836;
    let t80825 = t39041 * t1995 * t213 * t1999;
    (t80783, t80791, t80798, t80820, t80825)
}
