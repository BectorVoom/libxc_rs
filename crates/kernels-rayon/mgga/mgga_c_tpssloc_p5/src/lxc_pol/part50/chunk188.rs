//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 188/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk188(t632: f64, t73: f64, t52: f64, t76: f64, t607: f64) -> (f64, f64, f64, f64) {
    let t634 = 1.0_f64 / t73 / t632;
    let t636 = t52 * t52;
    let t638 = 1.0_f64 / t76 / t636;
    let t641 = -4.0_f64 / 3.0_f64 * t634 * t607 + 4.0_f64 / 3.0_f64 * t638 * t607;
    (t634, t636, t638, t641)
}
