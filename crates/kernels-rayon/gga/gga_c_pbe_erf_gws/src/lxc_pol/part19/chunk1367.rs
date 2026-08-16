//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1367/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1367(t1161: f64, t353: f64, t55151: f64, t859: f64, t11363: f64, t1206: f64, t14911: f64, t2498: f64, t52353: f64, t55279: f64, t55290: f64, t55311: f64, t56647: f64, t56651: f64, t56657: f64, t56667: f64, t56674: f64, t56678: f64, t56686: f64, t56697: f64, t56701: f64, t6793: f64, t9241: f64, t9283: f64) -> f64 {
    let t58292 = t859 * t353 * t55151 * t1161;
    let t58302 = -t2498 * t14911 / 48.0_f64 + t56647 / 192.0_f64 - t56651 / 384.0_f64 + t56657 / 384.0_f64 + t55279 + t56667 / 192.0_f64 - t56674 / 24.0_f64 - t56678 / 192.0_f64 - t56686 / 768.0_f64 + t6793 * t58292 / 24.0_f64 + t55290 - 35.0_f64 / 432.0_f64 * t52353 - t56697 / 768.0_f64 - t56701 / 1536.0_f64 + t9241 * t9283 * t1206 * t11363 / 4.0_f64 - t55311;
    t58302
}
