//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 573/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk573(t14970: f64, t14119: f64, t14128: f64, t14133: f64, t3282: f64, t333: f64, t884: f64, t3281: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14971 = 0.59871208509319042821e-1_f64 * t14970;
    let t14973 = 0.17519306092901367186e-5_f64 * t14119;
    let t14974 = 0.35038612185802734374e-6_f64 * t14128;
    let t14975 = 0.35038612185802734374e-6_f64 * t14133;
    let t14977 = t3282 * t333;
    let t14978 = t884 * t14977;
    let t14979 = 0.59871208509319042821e-1_f64 * t14978;
    let t14980 = t874 * t3281;
    (t14971, t14973, t14974, t14975, t14977, t14979, t14980)
}
