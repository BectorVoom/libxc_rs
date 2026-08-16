//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1229/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1229(t25082: f64, t32737: f64, t34495: f64, t125939: f64, t28196: f64, t28286: f64, t127317: f64, t128204: f64, t128211: f64, t128219: f64, t128223: f64, t128225: f64, t2108: f64, t25805: f64, t28025: f64, t28704: f64, t28709: f64, t32322: f64, t33913: f64, t6985: f64, t7537: f64, t7984: f64, t8079: f64, t8568: f64) -> f64 {
    let t128228 = 3.0_f64 * t25082 * t34495 * t32737;
    let t128231 = 2.0_f64 * t28196 * t28286 * t125939;
    let t128232 = t127317 * t2108 - 2.0_f64 * t25805 * t7984 - 2.0_f64 * t28025 * t7984 - 2.0_f64 * t28704 * t6985 - t28709 * t8568 + 3.0_f64 * t32322 * t8079 + t33913 * t7537 - t128204 - t128211 - t128219 + t128223 + t128225 - t128228 + t128231;
    t128232
}
