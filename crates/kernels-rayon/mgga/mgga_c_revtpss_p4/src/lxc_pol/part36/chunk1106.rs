//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1106/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1106(t25048: f64, t3: f64, t1518: f64, t5883: f64, t5801: f64, t5920: f64, t117: f64, t22633: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, t6941: f64, t6945: f64, t6948: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25049 = t3 * t25048;
    let t25055 = param_d * t25048;
    let t25063 = t5883 * t1518;
    let t25066 = t5801 * t5920;
    let t25069 = t117 * t22633;
    let t25072 = 18.0_f64 * t1916 * t6945 + 9.0_f64 * t1916 * t6948 + 9.0_f64 * t1918 * t6941 + t25055 * t573 + 6.0_f64 * t25063 * t572 + 18.0_f64 * t25066 * t572 + 3.0_f64 * t25069 * t572;
    (t25049, t25055, t25063, t25066, t25069, t25072)
}
