//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 796/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk796(t1390: f64, t561: f64, t828: f64, t1955: f64, t239: f64, t8571: f64, t2022: f64) -> (f64, f64, f64) {
    let t8575 = t1390 * t828 * t561;
    let t8576 = t1955 * t8571 * t239 * t8575;
    let t8578 = t2022 * t2022;
    (t8575, t8576, t8578)
}
