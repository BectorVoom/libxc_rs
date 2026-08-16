//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 363/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk363(t1319: f64, t1322: f64, t1334: f64, t1339: f64, t1342: f64, t1858: f64, t1860: f64, t225: f64, t679: f64, t704: f64, t1394: f64, t1868: f64) -> (f64, f64) {
    let t1877 = (t679 + t704 - t1319 - t1322 + t1858 + t1334 + t1860 - t1339 - t1342) * t225;
    let t1879 = t1394 * t1868;
    (t1877, t1879)
}
