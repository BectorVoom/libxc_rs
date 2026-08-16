//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1529/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1529(t1214: f64, t6702: f64, t3737: f64, t17974: f64, t5422: f64, t6573: f64, t1211: f64, t487: f64, t6564: f64, t1770: f64, t1811: f64, t1294: f64, t6744: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20740 = t6702 * t1214;
    let t20741 = t3737 * t20740;
    let t20744 = t17974 * t5422;
    let t20747 = t6573 * t1214;
    let t20748 = t1211 * t20747;
    let t20753 = t6564 * t487;
    let t20756 = t1770 * t1811;
    let t20759 = t6744 * t1294;
    (t20741, t20744, t20747, t20748, t20753, t20756, t20759)
}
