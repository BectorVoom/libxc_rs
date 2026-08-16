//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2214/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2214(t60221: f64, t7565: f64, t13272: f64, t26754: f64, t101139: f64, t101323: f64, t101357: f64, t2123: f64, t25110: f64, t25114: f64, t28141: f64, t29375: f64, t29388: f64, t6960: f64, t6963: f64, t7576: f64, t7579: f64, t7706: f64, t96773: f64, t96776: f64) -> f64 {
    let t104279 = t60221 * t7565;
    let t104282 = t13272 * t26754;
    let t104303 = 2.0_f64 / 3.0_f64 * t6963 * t29375 + 5.0_f64 / 3.0_f64 * t104279 * t6960 + 5.0_f64 / 3.0_f64 * t104282 * t6960 + 5.0_f64 / 3.0_f64 * t29388 * t25110 + 5.0_f64 / 6.0_f64 * t29388 * t25114 + 2.0_f64 / 3.0_f64 * t101323 * t2123 + t101357 * t2123 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t28141 * t7576 + 2.0_f64 / 3.0_f64 * t28141 * t7579 + 5.0_f64 / 6.0_f64 * t96773 * t7706 + 5.0_f64 / 3.0_f64 * t96776 * t7706 + t101139 * t2123 / 3.0_f64;
    t104303
}
