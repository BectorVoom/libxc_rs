//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1074/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1074(t1453: f64, t32107: f64, t32109: f64, t32619: f64, t32620: f64, t32627: f64, t32628: f64, t32632: f64, t32634: f64, t32635: f64, t32637: f64, t32663: f64, t7539: f64, t8463: f64, t8764: f64, t8897: f64) -> f64 {
    let t33261 = t1453 * t8897 - t7539 * t8764 - t32107 - t32109 - t32619 - t32620 + t32627 + t32628 + t32632 - t32634 - t32635 - t32637 - t32663 - t8463;
    t33261
}
