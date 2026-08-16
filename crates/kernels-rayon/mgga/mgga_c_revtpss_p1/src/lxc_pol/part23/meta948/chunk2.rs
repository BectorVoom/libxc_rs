//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3135/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3135(t1189: f64, t1196: f64, t24493: f64, t82060: f64, t82394: f64, t82396: f64, t82398: f64, t82400: f64, t82402: f64, t82404: f64, t82406: f64, t82410: f64, t82415: f64) -> (f64, f64) {
    let t82418 = 0.14035736694323150897e2_f64 * t1196 * t24493 * t1189;
    let t82419 = -t82394 - t82396 - t82398 - t82400 + t82060 - t82402 - t82404 - t82406 - t82410 - t82415 + t82418;
    (t82418, t82419)
}
