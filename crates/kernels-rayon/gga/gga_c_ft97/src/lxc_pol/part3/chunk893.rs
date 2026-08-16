//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 893/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk893(t17794: f64, t9744: f64, t446: f64, t17720: f64, t17724: f64, t17729: f64, t17734: f64, t17738: f64, t17742: f64, t17746: f64, t17751: f64, t17755: f64, t17759: f64, t17763: f64, t17768: f64, t17773: f64, t17778: f64, t17782: f64, t17787: f64, t17792: f64, t9701: f64, t9735: f64) -> (f64, f64) {
    let t17795 = t9744 * t17794;
    let t17796 = t446 * t17795;
    let t17799 = -t17720 / 27.0_f64 + t17724 / 18.0_f64 + t17729 / 9.0_f64 - t17734 / 27.0_f64 - 2.0_f64 / 9.0_f64 * t17738 - t17742 / 9.0_f64 - t17746 / 3.0_f64 - 5.0_f64 / 81.0_f64 * t17751 + 4.0_f64 / 27.0_f64 * t17755 + t17759 / 9.0_f64 + t17763 / 27.0_f64 + 2.0_f64 / 9.0_f64 * t17768 + t17773 / 18.0_f64 - t17778 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t17782 - 2.0_f64 / 81.0_f64 * t9735 - 2.0_f64 / 9.0_f64 * t17787 - 2.0_f64 / 9.0_f64 * t17792 + 2.0_f64 / 27.0_f64 * t17796 - 2.0_f64 / 27.0_f64 * t9701;
    (t17796, t17799)
}
