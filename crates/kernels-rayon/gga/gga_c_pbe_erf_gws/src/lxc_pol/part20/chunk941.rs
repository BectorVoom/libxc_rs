//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 941/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk941(t10559: f64, t10591: f64, t650: f64, t186: f64, t211: f64, t7421: f64, t1033: f64, t2724: f64, t7460: f64, t10474: f64, t10476: f64, t10478: f64, t10480: f64, t10484: f64, t10487: f64, t10491: f64, t10495: f64, t10497: f64, t10499: f64, t10504: f64, t10509: f64, t10512: f64) -> (f64, f64, f64, f64, f64) {
    let t10592 = t10559 + t10591;
    let t10593 = t650 * t10592;
    let t10594 = t186 * t10593;
    let t10596 = 2.0_f64 / 15.0_f64 * t211 * t10594;
    let t10597 = 8.0_f64 / 135.0_f64 * t7421;
    let t10599 = 4.0_f64 / 15.0_f64 * t1033 * t2724;
    let t10600 = 16.0_f64 / 405.0_f64 * t7460;
    let t10601 = t10474 - t10476 - t10478 - t10480 - t10484 + t10487 - t10491 - t10495 - t10497 + t10499 + t10504 - t10509 + t10512 - t10596 - t10597 - t10599 - t10600;
    (t10596, t10597, t10599, t10600, t10601)
}
