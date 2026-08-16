//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1121/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1121(t27544: f64, t5913: f64, t2034: f64, t491: f64, t7953: f64, t28614: f64, t28616: f64, t28618: f64, t28620: f64, t28622: f64, t28625: f64, t28627: f64, t28630: f64, t28632: f64, t28634: f64, t28636: f64) -> (f64, f64, f64, f64) {
    let t28638 = t27544 * t5913;
    let t28640 = t2034 * t491;
    let t28641 = t28640 * t7953;
    let t28643 = t28614 / 16.0_f64 + t28616 / 24.0_f64 + t28618 / 128.0_f64 + t28620 / 24.0_f64 - t28622 / 72.0_f64 - t28625 / 64.0_f64 + t28627 / 96.0_f64 - t28630 / 288.0_f64 - t28632 / 6.0_f64 - t28634 / 16.0_f64 - t28636 / 24.0_f64 + t28638 / 96.0_f64 + t28641 / 24.0_f64;
    (t28638, t28640, t28641, t28643)
}
