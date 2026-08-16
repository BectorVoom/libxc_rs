//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1536/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1536(t23958: f64, t993: f64, t225: f64, t366: f64, t20020: f64, t4858: f64, t1011: f64, t140: f64, t23877: f64, t15823: f64, t20029: f64, t11710: f64, t23899: f64, t4892: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t79862 = t23958 * t993;
    let t79863 = t79862 * t225;
    let t79864 = t79863 * t366;
    let t79874 = t4858 * t20020;
    let t79881 = t1011 * t140 * t23877;
    let t79892 = t15823 * t20029;
    let t79938 = t4892 * t11710 * t23899;
    (t79862, t79863, t79864, t79874, t79881, t79892, t79938)
}
