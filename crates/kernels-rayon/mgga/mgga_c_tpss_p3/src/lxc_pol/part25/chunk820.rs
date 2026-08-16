//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 820/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk820(t1830: f64, t645: f64, t5545: f64, t5555: f64, t5548: f64, t5553: f64, t5560: f64) -> (f64, f64, f64, f64) {
    let t5820 = t1830 * t645;
    let t5826 = 7.0_f64 / 144.0_f64 * t5545;
    let t5829 = 7.0_f64 / 1152.0_f64 * t5555;
    let t5831 = -t5826 - t5548 / 24.0_f64 - t5553 / 768.0_f64 - t5829 - t5560 / 192.0_f64;
    (t5820, t5826, t5829, t5831)
}
