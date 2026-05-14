//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 502/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk502<F: Float>(t1361: F, t1366: F, t1424: F, t1894: F, t1904: F, t213: F, t1319: F, t1322: F, t1334: F, t1339: F, t1342: F, t1343: F, t1450: F, t1858: F, t1860: F, t1868: F, t198: F, t532: F, t679: F, t704: F) -> (F, F) {
    let t1907 = -t1361 + t1366 + 0.65854491829355115987e0 * t213 * t1894 - 0.65854491829355115987e0 * t1424 * t1904;
    let t1911 = t1450 * t1907 * t198 * t532 + 3.0 * t1343 * t1868 * t198 - t1319 - t1322 + t1334 - t1339 - t1342 + t1858 + t1860 + t679 + t704;
    (t1907, t1911)
}
