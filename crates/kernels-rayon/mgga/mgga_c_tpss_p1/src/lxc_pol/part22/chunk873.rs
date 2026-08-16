//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 873/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk873(t509: f64, t6435: f64, t1270: f64, t1845: f64, t4525: f64, t118: f64, t1322: f64, t1339: f64, t1600: f64, t1663: f64, t1760: f64, t1796: f64, t1800: f64, t1830: f64, t1834: f64, t1846: f64, t3493: f64, t485: f64, t544: f64, t5801: f64, t6103: f64, t6243: f64, t626: f64, t6309: f64, t6318: f64, t6324: f64, t6328: f64, t6399: f64, t6409: f64, t6413: f64) -> (f64, f64, f64, f64) {
    let t6436 = t509 * t6435;
    let t6437 = t6436 * t1270;
    let t6439 = t1845 * t4525;
    let t6441 = -t118 * t6399 - t1322 * t1830 - 2.0_f64 * t1339 * t5801 - t1600 * t1796 + t1663 * t1834 + 3.0_f64 * t1760 * t6413 + t1760 * t6437 - t1760 * t6439 - 2.0_f64 * t1800 * t3493 - 2.0_f64 * t1800 * t6103 + t1846 * t6243 - t485 * t6309 + t544 * t6409 - 2.0_f64 * t626 * t6318 - 2.0_f64 * t626 * t6324 - 2.0_f64 * t626 * t6328;
    (t6436, t6437, t6439, t6441)
}
