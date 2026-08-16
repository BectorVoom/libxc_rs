//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 455/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk455(t34: f64, t575: f64, t2151: f64, t571: f64, t581: f64, t833: f64, t549: f64, t1466: f64, t1318: f64, t1401: f64, t593: f64, t529: f64, t784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2152 = t575 * t34;
    let t2153 = t2151 * t2152;
    let t2155 = 8.0_f64 / 45.0_f64 * t571 * t2153;
    let t2156 = t581 * t833;
    let t2157 = t2156 * t549;
    let t2158 = t1466 * t2157;
    let t2160 = 4.0_f64 / 15.0_f64 * t1318 * t2158;
    let t2161 = t1401 * t833;
    let t2162 = t2161 * t593;
    let t2163 = t1466 * t2162;
    let t2165 = 4.0_f64 / 15.0_f64 * t571 * t2163;
    let t2166 = t529 * t784;
    (t2152, t2153, t2155, t2157, t2158, t2160, t2161, t2162, t2163, t2165, t2166)
}
