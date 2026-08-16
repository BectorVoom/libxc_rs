//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2049/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2049(t12019: f64, t566: f64, t68: f64, t3700: f64, t195: f64, t632: f64, t197: f64, t636: f64, t2531: f64, t9892: f64, t718: f64, t9862: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40590 = 1.0_f64 / t12019 / t566;
    let t40591 = t68 * t40590;
    let t40610 = t3700 * t3700;
    let t40611 = 1.0_f64 / t40610;
    let t40632 = 1.0_f64 / t195 / t632;
    let t40647 = 1.0_f64 / t197 / t636;
    let t40667 = t2531 * t9892;
    let t40673 = t718 * t9862;
    (t40591, t40611, t40632, t40647, t40667, t40673)
}
