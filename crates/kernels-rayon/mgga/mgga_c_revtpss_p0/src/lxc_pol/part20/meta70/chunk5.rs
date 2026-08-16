//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 461/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk461(t1319: f64, t1322: f64, t1332: f64, t1334: f64, t1336: f64, t1339: f64, t1342: f64, t1343: f64, t1353: f64, t1448: f64, t1450: f64, t198: f64, t532: f64, t679: f64, t704: f64) -> f64 {
    let t1453 = t1448 * t1450 * t198 * t532 + 3.0_f64 * t1343 * t1353 * t198 + t1319 - t1322 + t1332 + t1334 + t1336 - t1339 - t1342 + t679 + t704;
    t1453
}
