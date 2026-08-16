//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3216/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3216(t49864: f64, t10605: f64, t18539: f64, t49866: f64, t39423: f64, t39425: f64, t39433: f64, t39436: f64, t14365: f64, t18865: f64, t2403: f64, t39419: f64, t39422: f64, t39429: f64, t39432: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t61019 = 2.0_f64 * t49864;
    let t61020 = t10605 * t18539;
    let t61021 = 24.0_f64 * t61020;
    let t61022 = 0.20508037716432813315e4_f64 * t49866;
    let t61026 = 0.43374325201206959368e-1_f64 * t39423;
    let t61027 = 0.65061487801810439052e-1_f64 * t39425;
    let t61028 = 0.96319466275353142156e0_f64 * t39433;
    let t61029 = 0.10843581300301739842e-1_f64 * t39436;
    let t61030 = -6.0_f64 * t14365 * t18865 * t2403 - t39419 - t39422 - t39429 - t39432 + t61019 + t61021 - t61022 - t61026 - t61027 + t61028 + t61029;
    (t61019, t61021, t61022, t61026, t61027, t61028, t61029, t61030)
}
