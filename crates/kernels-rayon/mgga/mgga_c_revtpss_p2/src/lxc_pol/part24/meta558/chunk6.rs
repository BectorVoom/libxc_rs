//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1675/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1675(t52128: f64, t63453: f64, t63459: f64, t63464: f64, t63533: f64, t63538: f64, t63545: f64, t77559: f64, t77561: f64, t77806: f64, t77858: f64, t88252: f64, t88257: f64, t88260: f64) -> f64 {
    let t88427 = 0.21908444444444444444e0_f64 * t77806 + 0.97370864197530864199e0_f64 * t52128 - 0.5314962962962962963e0_f64 * t63453 + 0.15944888888888888889e1_f64 * t63459 - 0.18257037037037037037e0_f64 * t63533 + 0.10954222222222222222e1_f64 * t63538 - 0.54771111111111111111e0_f64 * t63545 + 0.79724444444444444444e0_f64 * t77559 - 0.23917333333333333333e1_f64 * t77561 + 0.3071625e0_f64 * t88252 - 0.79724444444444444446e0_f64 * t63464 + 0.21908444444444444444e0_f64 * t77858 + 0.98587999999999999999e0_f64 * t88257 - 0.295764e1_f64 * t88260;
    t88427
}
