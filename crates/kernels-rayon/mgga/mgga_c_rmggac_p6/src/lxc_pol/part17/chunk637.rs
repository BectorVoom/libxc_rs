//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 637/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk637(t262: f64, t8901: f64, t7788: f64, t2392: f64, t333: f64, t7782: f64, t7835: f64, t8622: f64, t2068: f64, t8709: f64, t2073: f64, t8713: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8902 = t262 * t8901;
    let t8903 = t7788 * t8902;
    let t8905 = t2392 * t333;
    let t8906 = t262 * t8905;
    let t8907 = t7782 * t8906;
    let t8909 = t7835 * t8622;
    let t8911 = t2068 * t8709;
    let t8913 = t2073 * t8713;
    (t8902, t8903, t8905, t8906, t8907, t8909, t8911, t8913)
}
