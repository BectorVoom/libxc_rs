//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1791/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1791(t10179: f64, t1450: f64, t1353: f64, t3889: f64, t39799: f64, t39807: f64, t39813: f64, t4139: f64, t47020: f64, t47057: f64, t47059: f64, t47061: f64, t47064: f64, t47067: f64, t47070: f64, t47072: f64, t47074: f64, t5536: f64, t566: f64, t9547: f64, t9628: f64) -> f64 {
    let t47651 = t10179 * t1450;
    let t47662 = 24.0_f64 * t1353 * t5536 * t566 * t9628 + 12.0_f64 * t1353 * t4139 * t47651 + 18.0_f64 * t3889 * t4139 * t9547 + t39799 + t39807 - t39813 - t47020 + t47057 + t47059 + t47061 + t47064 + t47067 + t47070 - t47072 + t47074;
    t47662
}
