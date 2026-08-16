//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1100/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1100(t143158: f64, t152722: f64, t33820: f64, t143101: f64, t143120: f64, t143123: f64, t152715: f64, t152719: f64, t152724: f64, t152727: f64, t152730: f64, t152734: f64, t152738: f64, t152742: f64, t152746: f64, t152750: f64, t152754: f64, t152758: f64) -> (f64, f64) {
    let t152760 = t33820 * t143158 * t152722;
    let t152765 = -t152715 / 3.0_f64 + t152719 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t152724 + 2.0_f64 / 9.0_f64 * t152727 - 2.0_f64 / 27.0_f64 * t152730 + 4.0_f64 * t152734 + 2.0_f64 / 9.0_f64 * t152738 + t152742 / 2.0_f64 + t152746 / 2.0_f64 - t152750 / 3.0_f64 + t152754 / 4.0_f64 - t152758 + 4.0_f64 / 9.0_f64 * t152760 - 4.0_f64 / 27.0_f64 * t143101 - 8.0_f64 / 9.0_f64 * t143120 + 4.0_f64 / 9.0_f64 * t143123;
    (t152760, t152765)
}
