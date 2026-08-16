//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2130/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2130(t23097: f64, t2628: f64, t2632: f64, t47012: f64, t23033: f64, t25155: f64, t6546: f64, t13191: f64, t221: f64, t25154: f64, t13196: f64, t13171: f64, t6605: f64, t815: f64) -> (f64, f64, f64, f64, f64) {
    let t87458 = t23097 * t2628 * t47012 * t2632;
    let t87463 = t6546 * t23033 * t25155;
    let t87464 = 7.0_f64 / 24.0_f64 * t87463;
    let t87466 = t25154 * t221 * t13191;
    let t87469 = t25154 * t221 * t13196;
    let t87472 = t6605 * t815 * t13171;
    (t87458, t87464, t87466, t87469, t87472)
}
