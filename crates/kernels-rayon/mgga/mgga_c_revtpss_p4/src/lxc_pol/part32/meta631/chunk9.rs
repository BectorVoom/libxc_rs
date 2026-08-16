//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2052/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2052(t670: f64, t7983: f64, t101705: f64, t1459: f64, t1461: f64, t1518: f64, t1916: f64, t1918: f64, t2113: f64, t21881: f64, t22556: f64, t22568: f64, t26733: f64, t28956: f64, t28974: f64, t28978: f64, t28986: f64, t30637: f64, t30651: f64, t30660: f64, t4292: f64, t572: f64, t5795: f64, t5805: f64, t5883: f64, t5920: f64, t6941: f64, t6948: f64, t7373: f64, t7547: f64, t7553: f64, t7557: f64, t8118: f64, t8127: f64) -> f64 {
    let t111371 = t670 * t7983;
    let t111390 = 3.0_f64 * t2113 * t22568 + 6.0_f64 * t8118 * t5805 + 6.0_f64 * t572 * t5883 * t7373 + 3.0_f64 * t30637 * t1461 + 3.0_f64 * t7547 * t6948 + 3.0_f64 * t6941 * t7557 + 12.0_f64 * t572 * t101705 * t1518 + 12.0_f64 * t572 * t28986 * t4292 + 6.0_f64 * t28956 * t1918 + 3.0_f64 * t1459 * t30660 + 6.0_f64 * t2113 * t22556 + 12.0_f64 * t572 * t111371 * t1518 + 12.0_f64 * t1916 * t28978 + 6.0_f64 * t572 * t28974 * t5920 + 6.0_f64 * t572 * t26733 * t5920 + 6.0_f64 * t572 * t7553 * t21881 + 6.0_f64 * t1459 * t30651 + 6.0_f64 * t5795 * t8127;
    t111390
}
