//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 951/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk951(t1597: f64, t2819: f64, t2853: f64, t473: f64, t483: f64, t485: f64, t2877: f64, t2916: f64, t2826: f64, t1112: f64, t1124: f64, t1131: f64, t4166: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10793 = 0.02267957317922317_f64 * t2819 * t1597;
    let t10796 = t473 * t2853 * t483 * t485;
    let t10800 = 0.013871971944573394_f64 * t2877 * t2916 * t1597;
    let t10802 = 0.12408369628826103_f64 * t2826 * t485;
    let t10805 = t1124 * t1112 * t483 * t485;
    let t10808 = t4166 * t1131 * t485;
    (t10793, t10796, t10800, t10802, t10805, t10808)
}
