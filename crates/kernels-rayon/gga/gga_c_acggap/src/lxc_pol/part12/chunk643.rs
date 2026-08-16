//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 643/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk643(t301: f64, t5069: f64, t1403: f64, t839: f64, t402: f64, t4099: f64, t1396: f64, t1402: f64, t1404: f64, t1407: f64, t153: f64, t155: f64, t400: f64, t403: f64, t5050: f64, t5060: f64, t5066: f64, t519: f64, t521: f64, t917: f64, t923: f64, t926: f64) -> f64 {
    let t5070 = t5069 * t301;
    let t5073 = t1403 * t839;
    let t5076 = t402 * t4099;
    let t5079 = 6.0_f64 * t1396 * t403 + 60.0_f64 * t1402 * t5066 - 24.0_f64 * t1402 * t5070 - 12.0_f64 * t1402 * t5073 - 24.0_f64 * t1404 * t5060 + 6.0_f64 * t1407 * t400 + 3.0_f64 * t153 * t5076 - t155 * t5050 - 12.0_f64 * t519 * t923 + 3.0_f64 * t519 * t926 + 3.0_f64 * t521 * t917;
    t5079
}
