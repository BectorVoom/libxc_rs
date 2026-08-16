//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2045/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2045(t102070: f64, t109096: f64, t110110: f64, t110853: f64, t111004: f64, t111039: f64, t111068: f64, t118: f64, t13648: f64, t2014: f64, t2089: f64, t21814: f64, t21891: f64, t22287: f64, t22496: f64, t2322: f64, t25082: f64, t26399: f64, t26405: f64, t26411: f64, t27833: f64, t28167: f64, t28196: f64, t28658: f64, t28711: f64, t28932: f64, t29494: f64, t30209: f64, t30315: f64, t34495: f64, t569: f64, t5877: f64, t5887: f64, t671: f64, t7235: f64, t7359: f64, t7474: f64, t7732: f64, t7898: f64, t8108: f64, t8111: f64, t86771: f64, t9069: f64) -> f64 {
    let t111089 = 6.0_f64 * t28167 * t9069 * t22287 + t7235 * t30315 - t21814 * t2089 - t5877 * t7474 - 2.0_f64 * t110110 * t671 - 6.0_f64 * t28196 * t102070 * t109096 + 3.0_f64 * t2014 * t26411 * t29494 - t118 * (t110853 + t111004) + 6.0_f64 * t7898 * t28932 - 6.0_f64 * t25082 * t34495 * t22496 + (t111039 + t111068) * t569 - 2.0_f64 * t2014 * t8108 * t13648 - 2.0_f64 * t27833 * t8111 - 3.0_f64 * t25082 * t26405 * t86771 - 4.0_f64 * t26399 * t5887 - 4.0_f64 * t28658 * t5887 - 4.0_f64 * t7359 * t21891 - 4.0_f64 * t2322 * t30209 - 4.0_f64 * t7732 * t28711;
    t111089
}
