//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1347/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1347(t63973: f64, t63977: f64, t63990: f64, t61081: f64, t61087: f64, t61089: f64, t63971: f64, t63975: f64, t63979: f64, t63981: f64, t63984: f64, t63987: f64, t63995: f64) -> f64 {
    let t66427 = 7.0_f64 / 576.0_f64 * t63973;
    let t66429 = 35.0_f64 / 144.0_f64 * t63977;
    let t66434 = 7.0_f64 / 12.0_f64 * t63990;
    let t66439 = -5.0_f64 / 32.0_f64 * t63971 + t66427 - t63975 / 768.0_f64 - t66429 + 5.0_f64 / 96.0_f64 * t63979 + 5.0_f64 / 192.0_f64 * t63981 + t63984 / 4.0_f64 + t63987 / 8.0_f64 - t66434 - t63995 / 2.0_f64 + 7.0_f64 / 288.0_f64 * t61081 - 119.0_f64 / 432.0_f64 * t61087 - 35.0_f64 / 288.0_f64 * t61089;
    t66439
}
