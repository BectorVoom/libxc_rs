//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1929/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1929(t23168: f64, t28330: f64, t1888: f64, t232: f64, t5631: f64, t6646: f64, t828: f64, t25319: f64, t4119: f64, t6552: f64, t6637: f64, t234: f64) -> (f64, f64, f64, f64) {
    let t98564 = t23168 * t28330;
    let t98571 = t1888 * t6646 * t5631 * t828 * t232;
    let t98575 = t6552 * t6637 * t25319 * t4119;
    let t98598 = t234 * t5631;
    (t98564, t98571, t98575, t98598)
}
