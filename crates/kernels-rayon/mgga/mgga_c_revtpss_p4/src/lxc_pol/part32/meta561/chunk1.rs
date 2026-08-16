//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1881/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1881(t14833: f64, t240: f64, t2661: f64, t7043: f64, t14857: f64, t25234: f64, t25240: f64, t2710: f64, t4371: f64, t10744: f64, t4353: f64, t7028: f64) -> (f64, f64, f64, f64) {
    let t98968 = t2661 * t7043 * t240 * t14833;
    let t98972 = t25234 * t14857;
    let t98976 = t2710 * t25240 * t4371;
    let t98979 = t10744 * t7028 * t4353;
    (t98968, t98972, t98976, t98979)
}
