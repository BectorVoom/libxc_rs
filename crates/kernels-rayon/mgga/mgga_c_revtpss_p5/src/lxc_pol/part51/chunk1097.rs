//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1097/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1097(t125442: f64, t125444: f64, t125456: f64, t125459: f64, t125467: f64, t125470: f64, t125472: f64, t125474: f64, t125475: f64, t125479: f64, t1518: f64, t2322: f64, t25805: f64, t27145: f64, t28025: f64, t28050: f64, t32095: f64, t33584: f64, t4246: f64, t4254: f64, t651: f64, t6985: f64, t7746: f64, t8557: f64) -> f64 {
    let t125481 = -2.0_f64 * t1518 * t32095 * t651 - 2.0_f64 * t2322 * t33584 - 4.0_f64 * t25805 * t7746 - 4.0_f64 * t27145 * t6985 - 4.0_f64 * t28025 * t7746 - 4.0_f64 * t28050 * t6985 - 2.0_f64 * t33584 * t4254 - t4246 * t8557 - 4.0_f64 * t125442 - 4.0_f64 * t125444 - t125456 - 4.0_f64 * t125459 - 2.0_f64 * t125467 - t125470 + t125472 - t125474 + 2.0_f64 * t125475 + 4.0_f64 * t125479;
    t125481
}
