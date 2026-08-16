//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1070/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1070(t198: f64, t2075: f64, t26179: f64, t7706: f64, t7349: f64, t7709: f64, t13272: f64, t7342: f64, t2047: f64, t28150: f64, t7702: f64, t7348: f64, t7719: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28472 = t198 * t2075;
    let t28598 = t26179 * t7706;
    let t28600 = t7709 * t7349;
    let t28602 = t13272 * t7342;
    let t28628 = t2047 * t28150;
    let t28638 = t7702 * t7349;
    let t28640 = t7348 * t7719;
    (t28472, t28598, t28600, t28602, t28628, t28638, t28640)
}
