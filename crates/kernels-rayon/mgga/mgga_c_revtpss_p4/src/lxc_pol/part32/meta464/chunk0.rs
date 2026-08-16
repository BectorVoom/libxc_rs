//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1690/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1690(t26179: f64, t6960: f64, t2047: f64, t25163: f64, t6963: f64, t7349: f64, t10301: f64, t7342: f64, t6954: f64, t239: f64, t72: f64, t1927: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26180 = t26179 * t6960;
    let t26182 = t2047 * t25163;
    let t26185 = t6963 * t7349;
    let t26187 = t10301 * t7342;
    let t26190 = t6954 * t7349;
    let t26204 = t239 * t72;
    let t26205 = t26204 * t1927;
    (t26180, t26182, t26185, t26187, t26190, t26204, t26205)
}
