//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 825/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk825(t2055: f64, t7586: f64, t8564: f64, t8689: f64, t8694: f64, t8886: f64, t2052: f64, t2056: f64, t2089: f64, t2108: f64, t2127: f64, t2163: f64, t508: f64, t569: f64, t651: f64, t8463: f64, t8630: f64, t8636: f64, t8643: f64, t8687: f64, t8699: f64, t8716: f64, t8719: f64, t8764: f64, t8892: f64) -> (f64, f64) {
    let t8897 = 2.0_f64 * t2055 * t7586 + t8564 + t8689 + t8694 + t8886;
    let t8900 = -t2052 * t2163 - 2.0_f64 * t2056 * t7586 - t2089 * t2127 + t2108 * t8764 - t508 * t8886 + t569 * t8897 - 2.0_f64 * t651 * t8892 - t8463 - t8630 - t8636 - t8643 - t8687 + t8699 + t8716 - t8719;
    (t8897, t8900)
}
