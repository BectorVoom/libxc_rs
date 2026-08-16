//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 421/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk421(t148: f64, t1590: f64, t1131: f64, t482: f64, t485: f64, t283: f64, t732: f64) -> (f64, f64, f64) {
    let t1592 = 0.031505407223141116_f64 * t148 * t1590;
    let t1595 = 0.003950778065781896_f64 * t482 * t1131 * t485;
    let t1597 = t732 * t283;
    (t1592, t1595, t1597)
}
