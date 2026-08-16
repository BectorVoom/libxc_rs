//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1220/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1220(t114: f64, t19588: f64, t18396: f64, t18622: f64, t19591: f64, t19593: f64) -> f64 {
    let t115 = 1.0_f64 < t114;
    let t20315 = 2.0_f64 / 3.0_f64 * t19588;
    let t20319 = piecewise3(t115, 0.0_f64, t18622 + t18396 + t20315 + t19591 / 2.0_f64 - t19593 / 4.0_f64);
    t20319
}
