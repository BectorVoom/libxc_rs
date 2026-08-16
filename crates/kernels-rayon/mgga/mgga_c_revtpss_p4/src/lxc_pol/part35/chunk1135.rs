//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1135/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1135(t1468: f64, t2411: f64, t1568: f64, t7063: f64, t25410: f64, t25304: f64, t27212: f64, t27253: f64, t9775: f64, t25240: f64, t2710: f64, t4371: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98658 = t2411 * t1468;
    let t98848 = t7063 * t1568;
    let t98849 = t98848 * t25410;
    let t98867 = t25304 * t27212;
    let t98964 = t9775 * t27253;
    let t98976 = t2710 * t25240 * t4371;
    (t98658, t98848, t98849, t98867, t98964, t98976)
}
