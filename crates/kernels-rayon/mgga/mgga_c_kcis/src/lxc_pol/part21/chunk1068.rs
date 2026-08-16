//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1068/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1068(t233: f64, t26664: f64, t7673: f64, t7676: f64, t7679: f64, t380: f64, t982: f64) -> (f64, f64, f64, f64) {
    let t26665 = t233 * t26664;
    let t26666 = t26665 / 8.0_f64;
    let t26667 = t7673 * t7676;
    let t26668 = t26667 / 8.0_f64;
    let t26669 = t7673 * t7679;
    let t26670 = t26669 / 8.0_f64;
    let t26671 = t380 * t982;
    (t26666, t26668, t26670, t26671)
}
