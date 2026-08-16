//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 823/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk823(t580: f64, t6012: f64, t1890: f64, t1900: f64, t1905: f64, t1909: f64, t573: f64, t577: f64, t17: f64, t1896: f64, t576: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6013 = t6012 * t580;
    let t6015 = t1890 * t1900;
    let t6017 = t1890 * t1905;
    let t6019 = t1890 * t1909;
    let t6022 = 1.0_f64 / t573 / t577;
    let t6023 = t17 * t6022;
    let t6025 = 1.0_f64 / t1896 / t576;
    (t6013, t6015, t6017, t6019, t6022, t6023, t6025)
}
