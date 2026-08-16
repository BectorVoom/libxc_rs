//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 860/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk860(t13510: f64, t905: f64, t11975: f64, t11944: f64, t13485: f64, t13486: f64, t13488: f64, t13493: f64, t13498: f64, t13500: f64, t13503: f64, t13505: f64, t13507: f64, t6592: f64, t6597: f64, t902: f64, t929: f64) -> (f64, f64, f64) {
    let t13511 = t905 * t13510;
    let t13514 = 7.0_f64 / 96.0_f64 * t11975;
    let t13515 = t13485 - t13486 - t13488 - t13493 - 35.0_f64 / 384.0_f64 * t11944 + t13498 - 5.0_f64 / 128.0_f64 * t929 * t13500 + t13503 - t6592 - t6597 - t13505 - t929 * t13507 / 768.0_f64 + t902 * t13511 / 384.0_f64 - t13514;
    (t13511, t13514, t13515)
}
