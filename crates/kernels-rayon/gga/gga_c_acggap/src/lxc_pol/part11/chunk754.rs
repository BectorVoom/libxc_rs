//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 754/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk754(t2133: f64, t322: f64, t2132: f64, t7896: f64, t1960: f64, t872: f64, t157: f64, t7891: f64, t2152: f64, t2139: f64, t463: f64, t2147: f64) -> (f64, f64, f64, f64, f64) {
    let t7897 = t2133 * t322;
    let t7898 = t2132 * t7897;
    let t7900 = 0.34694512752820797848e1_f64 * t7896 * t7898;
    let t7901 = t1960 * t872;
    let t7903 = t7891 * t157;
    let t7904 = t2152 * t7903;
    let t7907 = t2139 * t463;
    let t7908 = t2147 * t7907;
    (t7898, t7900, t7901, t7904, t7908)
}
