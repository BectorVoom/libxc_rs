//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2019/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2019(t1016: f64, t3034: f64, t1081: f64, t2752: f64, t608: f64, t9239: f64, t835: f64, t531: f64, t6995: f64, t22573: f64, t6875: f64, t111: f64, t7415: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t82985 = 1.0_f64 / t3034 / t1016;
    let t83555 = t2752 * t1081;
    let t83717 = t9239 * t608;
    let t83803 = 1232.0_f64 / 27.0_f64 * t835;
    let t83859 = t531 * t6995;
    let t83886 = t6875 * t22573;
    let t85416 = t7415 * t111;
    (t82985, t83555, t83717, t83803, t83859, t83886, t85416)
}
