//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 578/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk578(t5415: f64, t2274: f64, t5392: f64, t5398: f64, t55: f64, t1420: f64, t1423: f64, t2282: f64, t39: f64, t51: f64, t5408: f64, t5411: f64, t56: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t5416 = sigma2 * t5415;
    let t5421 = t2274 * t5392;
    let t5424 = t55 * t5398;
    let t5427 = 5.0_f64 / 18.0_f64 * t39 * t5408 + 5.0_f64 / 6.0_f64 * t39 * t5411 + 88.0_f64 / 9.0_f64 * t5416 * t56 + 40.0_f64 / 9.0_f64 * t1420 * t1423 + 5.0_f64 / 18.0_f64 * t51 * t5421 - 5.0_f64 / 6.0_f64 * t51 * t5424 - t2282;
    (t5416, t5421, t5424, t5427)
}
