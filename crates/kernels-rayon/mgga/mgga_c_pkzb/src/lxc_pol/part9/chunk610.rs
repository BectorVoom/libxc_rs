//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 610/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk610(t1034: f64, t600: f64, t164: f64, t179: f64, t1020: f64, t1041: f64, t1769: f64, t177: f64, t1774: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2646 = t1034 * t600;
    let t2647 = t2646 * t164;
    let t2648 = t179 * t2647;
    let t2653 = t1020 * t600;
    let t2654 = t2653 * t164;
    let t2655 = t179 * t2654;
    let t2658 = t1769 * t1041;
    let t2660 = t1774 * t177;
    (t2646, t2647, t2648, t2653, t2655, t2658, t2660)
}
