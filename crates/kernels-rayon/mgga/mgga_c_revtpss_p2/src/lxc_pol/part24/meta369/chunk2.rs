//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1257/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1257(t12382: f64, t16706: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64, t422: f64) -> (f64, f64) {
    let t24466 = -t12382 + 0.23744444444444444444e-1_f64 * t16706 + 0.11872222222222222222e-1_f64 * t20283 - 0.35616666666666666666e-1_f64 * t20285 - 0.17808333333333333333e-1_f64 * t20287 + 0.19787037037037037037e-1_f64 * t24230 - 0.71233333333333333332e-1_f64 * t24234 - 0.35616666666666666666e-1_f64 * t24238 + 0.10685e0_f64 * t24242 + 0.10685e0_f64 * t24246 + 0.17808333333333333333e-1_f64 * t24250;
    let t24468 = 0.621814e-1_f64 * t24466 * t422;
    (t24466, t24468)
}
