//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1219/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1219(t13856: f64, t13861: f64, t13863: f64, t13865: f64, t13867: f64, t13869: f64, t13872: f64, t13875: f64, t13878: f64, t13882: f64, t13884: f64, t13886: f64, t13888: f64, t13892: f64, t13894: f64, t13896: f64, t13899: f64, t13904: f64, t13906: f64, t13908: f64, t13910: f64, t13912: f64, t13914: f64) -> (f64, f64) {
    let t14458 = -t13856 + t13861 - t13863 - t13865 - t13867 + t13869 - t13872 - t13875 + t13878 - t13882 - t13884;
    let t14459 = -t13886 - t13888 - t13892 - t13894 - t13896 + t13899 + t13904 - t13906 - t13908 + t13910 - t13912 - t13914;
    (t14458, t14459)
}
