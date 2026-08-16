//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1495/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1495(t31027: f64, t31268: f64, t100: f64, t101460: f64, t101463: f64, t10199: f64, t116942: f64, t117482: f64, t117484: f64, t117497: f64, t117499: f64, t117500: f64, t117505: f64, t1504: f64, t2174: f64, t2256: f64, t2366: f64, t31035: f64, t31043: f64, t31058: f64, t31283: f64, t4269: f64, t8258: f64, t8259: f64, t8267: f64, t8268: f64) -> f64 {
    let t117510 = 20.0_f64 / 9.0_f64 * t31027 * t31268;
    let t117517 = -5.0_f64 / 24.0_f64 * t10199 * t2174 * t100 - t117482 + t117484 + 5.0_f64 / 12.0_f64 * t8258 * t8268 * t1504 * t2366 + 25.0_f64 / 54.0_f64 * t8267 * t116942 * t31283 - 5.0_f64 / 36.0_f64 * t8267 * t31058 * t1504 * t2256 + t117497 - 5.0_f64 / 2.0_f64 * t117499 * t117500 * t31043 + 5.0_f64 / 9.0_f64 * t117505 * t4269 * t31043 - t117510 - 3.0_f64 / 2.0_f64 * t31035 * t8259 * t101460 - 3.0_f64 / 4.0_f64 * t31035 * t8259 * t101463;
    t117517
}
