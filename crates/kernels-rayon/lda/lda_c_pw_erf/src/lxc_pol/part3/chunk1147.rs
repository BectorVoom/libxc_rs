//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1147/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1147(t13426: f64, t1472: f64, t4791: f64, t4795: f64, t4906: f64, t529: f64, t4849: f64, t519: f64, t12695: f64, t4633: f64, t1124: f64, t1458: f64, t197: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13427 = 16.0_f64 / 45.0_f64 * t13426;
    let t13428 = t1472 * t4791;
    let t13429 = 32.0_f64 / 45.0_f64 * t13428;
    let t13430 = t1472 * t4795;
    let t13431 = 16.0_f64 / 27.0_f64 * t13430;
    let t13432 = t4906 * t529;
    let t13434 = t519 * t13432 * t4849;
    let t13435 = 8.0_f64 / 9.0_f64 * t13434;
    let t13437 = t519 * t12695 * t4633;
    let t13438 = 16.0_f64 / 9.0_f64 * t13437;
    let t13440 = t1124 * t1458 * t197;
    (t13427, t13429, t13431, t13435, t13438, t13440)
}
