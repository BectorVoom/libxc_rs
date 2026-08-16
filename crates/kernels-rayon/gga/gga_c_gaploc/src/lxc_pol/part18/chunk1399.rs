//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1399/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1399(t1531: f64, t34567: f64, t34706: f64, t34709: f64, t34712: f64, t34714: f64, t34717: f64, t34720: f64, t34726: f64, t34730: f64, t34733: f64, t34737: f64, t34740: f64, t34743: f64, t34746: f64, t34749: f64, t34752: f64, t7025: f64) -> f64 {
    let t34756 = -t34706 - t34709 - t34712 + t34714 + t34717 + t34720 - t34726 + t34730 + t34733 - t34737 - t34740 + t34743 + t34746 - t34749 - t34752 + 0.21450293971110256002e1_f64 * t7025 * t1531 * t34567;
    t34756
}
