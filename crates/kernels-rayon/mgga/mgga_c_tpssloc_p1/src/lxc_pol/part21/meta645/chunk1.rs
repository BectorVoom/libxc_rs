//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2438/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2438(t10890: f64, t10948: f64, t10508: f64, t248: f64, t3130: f64, t3132: f64, t1015: f64, t3033: f64, t42520: f64, t3142: f64, t698: f64, t973: f64) -> (f64, f64, f64, f64) {
    let t42573 = t10948 * t10890;
    let t42586 = t3130 * t248 * t10508 * t3132;
    let t42600 = t3033 * t1015 * t42520;
    let t42610 = t973 * t698 * t3142;
    (t42573, t42586, t42600, t42610)
}
