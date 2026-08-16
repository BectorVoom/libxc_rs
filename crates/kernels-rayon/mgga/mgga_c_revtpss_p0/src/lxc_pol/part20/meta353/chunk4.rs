//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1288/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1288(t39461: f64, t39474: f64, t2576: f64, t2565: f64, t701: f64) -> (f64, f64, f64) {
    let t39476 = t39461 / 2.0_f64 + t39474 / 2.0_f64;
    let t39480 = t2576 * t2576;
    let t39483 = 6.0_f64 * t2565 * t39480 * t701;
    (t39476, t39480, t39483)
}
