//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1234/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1234(t127349: f64, t127357: f64, t127359: f64, t127361: f64, t127363: f64, t127366: f64, t127369: f64, t127371: f64, t127373: f64, t127375: f64, t127378: f64, t129491: f64, t129502: f64, t569: f64, t5787: f64, t8761: f64) -> f64 {
    let t129507 = t127349 - t127357 - t127359 + t127361 + (2.0_f64 * t129491 + t129502) * t569 + t8761 * t5787 - 2.0_f64 * t127363 - t127366 - t127369 - t127371 - t127373 - t127375 - t127378;
    t129507
}
