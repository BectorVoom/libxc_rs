//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1055/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1055(t31964: f64, t370: f64, t8499: f64, t32009: f64, t93982: f64, t120334: f64, t7150: f64, t11922: f64, t31975: f64, t31977: f64, t25638: f64, t31902: f64) -> (f64, f64, f64, f64, f64) {
    let t120555 = t8499 * t31964 * t370;
    let t120558 = t32009 * t93982;
    let t120569 = t7150 * t120334;
    let t120578 = t31975 * t11922 * t31977;
    let t120584 = t31902 * t25638;
    (t120555, t120558, t120569, t120578, t120584)
}
