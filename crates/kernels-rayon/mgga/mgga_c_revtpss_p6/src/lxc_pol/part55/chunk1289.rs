//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1289/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1289(t122820: f64, t128539: f64, t128543: f64, t128552: f64, t128554: f64, t128557: f64, t128560: f64, t128562: f64, t128572: f64, t128574: f64, t128577: f64, t128867: f64, t1453: f64, t28588: f64, t32822: f64, t34788: f64, t8111: f64) -> f64 {
    let t131005 = -3.0_f64 * t122820 * t28588 + t1453 * t34788 - t32822 * t8111 - t128539 - t128543 - t128552 - t128554 - t128557 - t128560 - t128562 + t128572 - t128574 + t128577 + t128867;
    t131005
}
