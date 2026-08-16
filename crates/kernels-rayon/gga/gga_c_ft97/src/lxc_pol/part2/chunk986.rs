//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 986/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk986(t10478: f64, t319: f64, t14686: f64, t2766: f64, t871: f64, t2883: f64, t3690: f64, t10491: f64, t14678: f64, t4167: f64, t684: f64, t10703: f64) -> (f64, f64, f64, f64) {
    let t15290 = t10478 * t319;
    let t15291 = t15290 * t14686;
    let t15294 = t2766 * t871;
    let t15295 = t3690 * t2883;
    let t15296 = t15294 * t15295;
    let t15299 = t10491 * t319;
    let t15300 = t15299 * t14678;
    let t15303 = t4167 * t684;
    let t15304 = t10703 * t15303;
    (t15291, t15296, t15300, t15304)
}
