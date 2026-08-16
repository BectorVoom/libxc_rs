//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2263/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2263(t101439: f64, t101472: f64, t101476: f64, t101482: f64, t101485: f64, t101486: f64, t101546: f64, t101548: f64, t101550: f64, t101552: f64, t13521: f64, t13532: f64, t14310: f64, t1843: f64, t2165: f64, t26800: f64, t26804: f64, t3813: f64, t4151: f64, t5517: f64, t5787: f64, t7584: f64, t7586: f64, t7687: f64, t8152: f64, t8237: f64) -> f64 {
    let t105756 = -2.0_f64 * t13521 * t7586 - 4.0_f64 * t13532 * t7586 + t14310 * t2165 - t1843 * t26800 - 2.0_f64 * t1843 * t26804 - t3813 * t8152 + t4151 * t8237 - 2.0_f64 * t5517 * t7584 + 2.0_f64 * t5787 * t7687 + t101439 - t101472 + t101476 - t101482 - t101485 - t101486 + t101546 - t101548 - t101550 - t101552;
    t105756
}
