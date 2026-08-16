//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1220/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1220(t13916: f64, t13918: f64, t13921: f64, t13923: f64, t13926: f64, t13929: f64, t13932: f64, t13936: f64, t13938: f64, t13941: f64, t13943: f64, t13947: f64, t13949: f64, t13951: f64, t13958: f64, t13960: f64, t13963: f64, t13968: f64, t13970: f64, t13972: f64, t13974: f64, t13978: f64, t13983: f64) -> (f64, f64) {
    let t14461 = t13916 + t13918 + t13921 - t13923 - t13926 - t13929 + t13932 - t13936 + t13938 + t13941 + t13943;
    let t14462 = -t13947 + t13949 + t13951 - t13958 - t13960 + t13963 + t13968 + t13970 - t13972 - t13974 + t13978 - t13983;
    (t14461, t14462)
}
