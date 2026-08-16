//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2014/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2014(t103247: f64, t103254: f64, t105985: f64, t105987: f64, t105989: f64, t105991: f64, t105993: f64, t105995: f64, t105997: f64, t105999: f64, t106001: f64, t106003: f64) -> f64 {
    let t110378 = -t103247 + 0.34299214494455789578e-2_f64 * t105985 - t103254 - 0.17149607247227894789e-2_f64 * t105987 + 0.68598428988911579156e-2_f64 * t105989 - 0.51448821741683684367e-2_f64 * t105991 - 0.17149607247227894789e-1_f64 * t105993 + 0.34299214494455789578e-2_f64 * t105995 + 0.34299214494455789578e-2_f64 * t105997 - 0.68598428988911579156e-2_f64 * t105999 - 0.85748036236139473944e-3_f64 * t106001 + 0.68598428988911579156e-2_f64 * t106003;
    t110378
}
