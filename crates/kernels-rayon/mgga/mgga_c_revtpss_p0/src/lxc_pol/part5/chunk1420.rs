//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1420/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1420(t22564: f64, t670: f64, t117: f64, t21881: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t22544: f64, t22556: f64, t22559: f64, t572: f64, t573: f64, t5795: f64, t5802: f64, t5805: f64, t6941: f64, t6945: f64, t6948: f64) -> f64 {
    let t22565 = t22564 * t670;
    let t22568 = t117 * t21881;
    let t22571 = 6.0_f64 * t1459 * t6945 + 3.0_f64 * t1459 * t6948 + 3.0_f64 * t1461 * t6941 + 12.0_f64 * t1916 * t5802 + 6.0_f64 * t1916 * t5805 + 6.0_f64 * t1918 * t5795 + t22544 * t573 + 6.0_f64 * t22556 * t572 + 12.0_f64 * t22559 * t572 + 6.0_f64 * t22565 * t572 + 3.0_f64 * t22568 * t572;
    t22571
}
