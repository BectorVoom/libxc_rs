//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 855/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk855(t4296: f64, t4300: f64, t2994: f64, t2999: f64, t3008: f64, t3015: f64, t3155: f64, t14: f64, t2: f64, t41: f64, t174: f64, t2824: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8120 = 7.302458460456296_f64 * t4296;
    let t8121 = 12.654485932329694_f64 * t4300;
    let t8122 = 14.03573615389249_f64 * t2994;
    let t8123 = 207.78907079251036_f64 * t2999;
    let t8126 = 0.0022787712934626155_f64 * t3008;
    let t8130 = 0.013780452414814815_f64 * t3015;
    let t8134 = 4.0_f64 * t3155;
    let t8138 = 1.0_f64 / t14 / t2 / t41 / 48.0_f64;
    let t8141 = t8138 * t2 * t2824 * t174;
    (t8120, t8121, t8122, t8123, t8126, t8130, t8134, t8138, t8141)
}
