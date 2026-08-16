//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 964/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk964(t13048: f64, t10654: f64, t1949: f64, t571: f64, t1401: f64, t2151: f64, t219: f64, t4900: f64, t3704: f64, t3973: f64, t1333: f64, t4507: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13049 = 8.0_f64 / 135.0_f64 * t13048;
    let t13051 = t571 * t10654 * t1949;
    let t13052 = 16.0_f64 / 135.0_f64 * t13051;
    let t13060 = t2151 * t1401;
    let t13080 = t4900 * t219;
    let t13115 = t3973 * t3704;
    let t13122 = t4507 * t1333;
    (t13049, t13052, t13060, t13080, t13115, t13122)
}
