//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 750/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk750(t5012: f64, t1482: f64, t2776: f64, t366: f64, t4977: f64, t1464: f64, t1474: f64, t4988: f64, t220: f64, t2782: f64, t2786: f64, t2798: f64, t2799: f64, t368: f64, t983: f64, t985: f64, param_beta: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5013 = param_beta * t5012;
    let t5017 = t1482 * t1482;
    let t5018 = t2776 * t5017;
    let t5021 = t366 * t4977;
    let t5025 = t1474 * t1464;
    let t5029 = t366 * t4988;
    let t5036 = t220 * t368 * t5012 + 2.0_f64 * t2782 * t2786 * t5021 - t2798 * t2799 * t5021 + 2.0_f64 * t5025 * t983 * t985 + t5029 * t983 * t985;
    (t5013, t5017, t5018, t5021, t5025, t5029, t5036)
}
