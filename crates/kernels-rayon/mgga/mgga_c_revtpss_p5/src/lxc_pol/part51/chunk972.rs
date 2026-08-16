//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 972/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk972(t32243: f64, t32295: f64, t532: f64, t1450: f64, t2014: f64, t118: f64, t1453: f64, t32095: f64, t32101: f64, t32102: f64, t32104: f64, t32107: f64, t32109: f64, t32112: f64, t32116: f64, t32118: f64, t32123: f64, t32124: f64, t32126: f64, t32131: f64, t32179: f64, t32182: f64, t569: f64, t649: f64, t8463: f64, t8557: f64, t8565: f64) -> (f64, f64, f64, f64) {
    let t32296 = t32243 + t32295;
    let t32297 = t532 * t32296;
    let t32298 = t32297 * t1450;
    let t32299 = t2014 * t32298;
    let t32300 = -t118 * t32095 + t1453 * t8565 + t32179 * t569 - t649 * t8557 + t32101 - t32102 - 4.0_f64 * t32104 - t32107 - t32109 - t32112 - t32116 - t32118 - t32123 - 2.0_f64 * t32124 + 6.0_f64 * t32126 + t32131 + t32182 + t32299 - t8463;
    (t32296, t32297, t32298, t32300)
}
