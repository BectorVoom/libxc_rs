//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1213/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1213(t110177: f64, t113111: f64, t113420: f64, t113424: f64, t113444: f64, t113454: f64, t113461: f64, t113465: f64, t113484: f64, t1468: f64, t1940: f64, t2071: f64, t22670: f64, t2403: f64, t26590: f64, t28460: f64, t29716: f64, t29719: f64, t30420: f64, t4541: f64, t5824: f64, t7432: f64, t7749: f64, t7787: f64, t8020: f64, t95964: f64) -> f64 {
    let t115462 = 9.0_f64 / 2.0_f64 * t2403 * t2071 * t113461 + 9.0_f64 / 2.0_f64 * t2403 * t2071 * t113420 + 9.0_f64 * t4541 * t2071 * t113454 + 3.0_f64 * t1940 * t26590 * t113465 - t1940 * t7432 * t113424 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t1940 * t110177 * t7787 - 3.0_f64 / 2.0_f64 * t1940 * t28460 * t29719 - 3.0_f64 * t1940 * t95964 * t113444 + 9.0_f64 / 2.0_f64 * t2403 * t30420 * t7749 - 3.0_f64 / 2.0_f64 * t1940 * t7432 * t113484 - 3.0_f64 * t1940 * t28460 * t29716 + 3.0_f64 / 2.0_f64 * t1940 * t8020 * t5824 + t1940 * t2071 * t22670 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t1940 * t7432 * t113111 + 3.0_f64 / 2.0_f64 * t1940 * t30420 * t1468;
    t115462
}
