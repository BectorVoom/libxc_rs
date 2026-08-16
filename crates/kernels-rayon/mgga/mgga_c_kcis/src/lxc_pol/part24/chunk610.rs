//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 610/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk610(t278: f64, t2910: f64, t6533: f64, t286: f64, t6432: f64) -> (f64, f64, f64) {
    let t288 = 0.0_f64 < t278;
    let t6534 = t2910 * t6533;
    let t6535 = t286 * t6534;
    let t6539 = piecewise3(t288, t6432, -t6432);
    (t6534, t6535, t6539)
}
