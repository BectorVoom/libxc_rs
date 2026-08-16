//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 806/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk806(t15670: f64, t366: f64, t372: f64, t4823: f64, t1062: f64, t4857: f64, t11986: f64, t1592: f64, t247: f64, t1063: f64, t11262: f64, t1670: f64) -> (f64, f64, f64, f64, f64) {
    let t15671 = t15670 * t366;
    let t15696 = t372 * t4823;
    let t15707 = t4857 * t1062;
    let t15711 = t247 * t11986 * t1592;
    let t15712 = t1063 * t15711;
    let t15731 = t11262 * t1670;
    (t15671, t15696, t15707, t15712, t15731)
}
