//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 747/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk747(t9698: f64, t9742: f64, t9747: f64, t9755: f64, t9759: f64, t9763: f64, t9765: f64, t9768: f64, t9773: f64, t9777: f64, t9883: f64, t9893: f64, t9970: f64) -> f64 {
    let t10119 = 28.0_f64 / 27.0_f64 * t9698;
    let t10120 = -2.0_f64 / 3.0_f64 * t9768 - 2.0_f64 / 3.0_f64 * t9755 + t9759 + t9763 - 2.0_f64 / 3.0_f64 * t9765 - 2.0_f64 * t9773 - 2.0_f64 * t9777 + 2.0_f64 * t9742 + 2.0_f64 / 3.0_f64 * t9747 - 3.0_f64 / 4.0_f64 * t9883 + 3.0_f64 / 8.0_f64 * t9893 + t9970 / 2.0_f64 - t10119;
    t10120
}
