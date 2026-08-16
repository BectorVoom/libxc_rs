//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1101/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1101(t2035: f64, t34270: f64, t7984: f64, t8634: f64, t7359: f64, t7742: f64, t1907: f64, t2033: f64, t28286: f64, t28196: f64, t1868: f64, t26405: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34271 = t34270 * t2035;
    let t34285 = 2.0_f64 * t8634 * t7984;
    let t34294 = 2.0_f64 * t7359 * t7742;
    let t34297 = t2033 * t1907;
    let t34298 = t28286 * t34297;
    let t34300 = 2.0_f64 * t28196 * t34298;
    let t34301 = t2033 * t1868;
    let t34302 = t26405 * t34301;
    (t34271, t34285, t34294, t34297, t34298, t34300, t34301, t34302)
}
