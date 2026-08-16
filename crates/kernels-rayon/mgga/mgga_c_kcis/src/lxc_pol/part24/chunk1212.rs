//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1212/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1212(t1657: f64, t18445: f64, t2169: f64, t2209: f64, t233: f64, t29235: f64, t4534: f64, t5398: f64, t6294: f64, t7827: f64, t8121: f64, t911: f64, t91885: f64, t91895: f64, t91901: f64, t92157: f64, t92379: f64, t97561: f64) -> f64 {
    let t99825 = -t91885 + t97561 - t2169 * t1657 * t5398 / 8.0_f64 + t91895 - t91901 + t92379 + t911 * t29235 / 8.0_f64 - t233 * t4534 * t8121 / 8.0_f64 - t233 * t18445 * t2209 / 16.0_f64 + t92157 - t233 * t6294 * t7827 / 16.0_f64;
    t99825
}
