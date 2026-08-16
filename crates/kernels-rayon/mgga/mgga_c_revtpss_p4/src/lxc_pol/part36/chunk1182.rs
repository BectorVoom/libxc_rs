//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1182/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1182(t6884: f64, t7252: f64, t25983: f64, t6864: f64, t26003: f64, t26011: f64, t26013: f64, t26022: f64, t27921: f64, t27953: f64, t28873: f64, t28874: f64, t28885: f64) -> f64 {
    let t30048 = t7252 * t6884;
    let t30050 = t25983 * t6864;
    let t30054 = t26003 - t26011 - t30048 / 48.0_f64 + t28885 + 0.85748036236139473944e-3_f64 * t30050 + t26013 + t26022 - 0.50820002809285328226e-4_f64 * t27953 + t28873 + t28874 + 0.40015750243531754508e-2_f64 * t27921;
    t30054
}
