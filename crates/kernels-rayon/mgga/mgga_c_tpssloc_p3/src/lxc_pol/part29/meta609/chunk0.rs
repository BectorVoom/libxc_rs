//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2048/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2048(t1914: f64, t40772: f64, t3034: f64, t336: f64, t221: f64, t697: f64, t1016: f64, t1081: f64, t2752: f64, t1864: f64, t2241: f64, t608: f64, t9231: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t82312 = t1914 * t40772;
    let t82510 = 1.0_f64 / t3034 / t336;
    let t82631 = t221 * t697;
    let t82985 = 1.0_f64 / t3034 / t1016;
    let t83555 = t2752 * t1081;
    let t83718 = t1864 * t2241;
    let t83722 = t9231 * t608;
    (t82312, t82510, t82631, t82985, t83555, t83718, t83722)
}
