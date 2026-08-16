//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1954/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1954(t1955: f64, t6888: f64, t225: f64, t30055: f64, t2022: f64, t6861: f64, t4003: f64, t26079: f64, t543: f64, t7301: f64, t6843: f64, t1882: f64, t7910: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30071 = t1955 * t6888;
    let t30074 = t30055 * t225;
    let t30080 = t2022 * t6861;
    let t30081 = t30080 * t4003;
    let t30082 = t26079 * t30081;
    let t30088 = t30080 * t543;
    let t30089 = t7301 * t30088;
    let t30095 = t2022 * t6843 * t543;
    let t30096 = t7301 * t30095;
    let t30100 = t7910 * t1882 * t543;
    (t30071, t30074, t30081, t30082, t30088, t30089, t30095, t30096, t30100)
}
