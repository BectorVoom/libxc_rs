//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 517/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk517(t1361: f64, t1366: f64, t1424: f64, t1894: f64, t1904: f64, t213: f64, t1319: f64, t1322: f64, t1334: f64, t1339: f64, t1342: f64, t1343: f64, t1450: f64, t1858: f64, t1860: f64, t1868: f64, t198: f64, t532: f64, t679: f64, t704: f64) -> (f64, f64) {
    let t1907 = -t1361 + t1366 + 0.65854491829355115987e0_f64 * t213 * t1894 - 0.65854491829355115987e0_f64 * t1424 * t1904;
    let t1911 = t1450 * t1907 * t198 * t532 + 3.0_f64 * t1343 * t1868 * t198 - t1319 - t1322 + t1334 - t1339 - t1342 + t1858 + t1860 + t679 + t704;
    (t1907, t1911)
}
