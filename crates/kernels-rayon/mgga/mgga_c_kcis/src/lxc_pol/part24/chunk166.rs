//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 166/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk166(t209: f64, t698: f64, t700: f64, t63: f64, t691: f64, t696: f64, t75: f64) -> (f64, f64) {
    let t702 = t209 * t698 * t700;
    let t705 = -7.0_f64 / 288.0_f64 * t63 * t691 * t75 - t696 * t702 / 96.0_f64;
    (t702, t705)
}
