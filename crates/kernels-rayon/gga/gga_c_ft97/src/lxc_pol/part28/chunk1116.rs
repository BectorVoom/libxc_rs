//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1116/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1116(t1349: f64, t138551: f64, t138557: f64, t138560: f64, t138652: f64, t138655: f64, t147590: f64, t147602: f64, t147604: f64, t165: f64, t23413: f64, t26515: f64, t26581: f64, t27422: f64, t28: f64, t35007: f64, t35234: f64, t378: f64, t525: f64, t5772: f64, t5845: f64, t7309: f64, t7313: f64, t7315: f64) -> f64 {
    let t147614 = -t138551 / 18.0_f64 + t35007 * t5845 / 6.0_f64 + t1349 * t28 * t525 * t147590 * t165 / 6.0_f64 + t138557 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t5772 * t378 * t7313 * t27422 - t138560 / 18.0_f64 + t147602 / 54.0_f64 + t147604 / 9.0_f64 - t23413 * t35234 / 9.0_f64 + t7309 * t26515 / 6.0_f64 - t138652 / 18.0_f64 - t138655 / 9.0_f64 - t26581 * t7315 / 3.0_f64;
    t147614
}
