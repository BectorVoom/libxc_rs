//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1272/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1272(t2974: f64, t4571: f64, t6094: f64, t6098: f64, t6102: f64) -> f64 {
    let t6184 = t2974 + 0.61805555555555555556e-2_f64 * t4571 - 0.61805555555555555555e-2_f64 * t6094 + 0.18541666666666666667e-1_f64 * t6098 - 0.92708333333333333333e-2_f64 * t6102;
    t6184
}
