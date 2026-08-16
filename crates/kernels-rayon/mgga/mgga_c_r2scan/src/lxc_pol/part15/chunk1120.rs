//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1120/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1120(t39421: f64, t39422: f64, t39424: f64, t39426: f64, t39429: f64, t39431: f64, t39434: f64, t39438: f64, t39440: f64, t39444: f64, t39446: f64, t39448: f64) -> f64 {
    let t39450 = -t39421 + 0.54878743191129263322e-1_f64 * t39422 + 0.10975748638225852664e0_f64 * t39424 - 0.27439371595564631661e-1_f64 * t39426 + 0.15573871527278325618e-1_f64 * t39429 + 0.54878743191129263322e-1_f64 * t39431 + 0.86682217400542685632e-1_f64 * t39434 + t39438 - 0.95219938395347901943e-2_f64 * t39440 - t39444 + t39446 - 0.10401866088065122276e1_f64 * t39448;
    t39450
}
