//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 423/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk423(t1319: f64, t1322: f64, t1332: f64, t1334: f64, t1336: f64, t1339: f64, t1342: f64, t225: f64, t679: f64, t704: f64, t550: f64, t73: f64) -> (f64, f64) {
    let t1392 = (t679 + t704 + t1319 - t1322 + t1332 + t1334 + t1336 - t1339 - t1342) * t225;
    let t1394 = t73 * t550;
    (t1392, t1394)
}
