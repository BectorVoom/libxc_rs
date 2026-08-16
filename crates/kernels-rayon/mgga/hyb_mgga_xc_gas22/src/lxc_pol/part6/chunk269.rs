//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 269/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk269(t883: f64, t319: f64, t324: f64, t313: f64, t314: f64, t312: f64, t645: f64, t99: f64, t298: f64, t321: f64, t322: f64, rho0: f64, sigma0: f64, tau0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t884 = 1.0_f64 / t883;
    let t885 = t884 * tau0;
    let t889 = t319 * rho0;
    let t890 = 1.0_f64 / t889;
    let t891 = t890 * t324;
    let t894 = t313 * sigma0;
    let t895 = t314 * t894;
    let t896 = t312 * t895;
    let t897 = t319 * t645;
    let t899 = 1.0_f64 / t99 / t897;
    let t900 = t321 * t298;
    let t902 = 1.0_f64 / t322 / t900;
    (t884, t885, t890, t891, t894, t895, t896, t899, t900, t902)
}
