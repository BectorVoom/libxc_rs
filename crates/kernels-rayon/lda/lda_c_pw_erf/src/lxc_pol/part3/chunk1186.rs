//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1186/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1186(t13975: f64, t1284: f64, t5175: f64, t10454: f64, t10456: f64, t10465: f64, t13952: f64, t13956: f64, t13958: f64, t13961: f64, t13965: f64, t13969: f64, t13972: f64, t13974: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13976 = 4.0_f64 / 3.0_f64 * t13975;
    let t13977 = t1284 * t5175;
    let t13978 = 4.0_f64 / 3.0_f64 * t13977;
    let t13979 = 8.0_f64 / 45.0_f64 * t10454;
    let t13980 = 16.0_f64 / 45.0_f64 * t10456;
    let t13981 = 32.0_f64 / 135.0_f64 * t10465;
    let t13982 = -t13952 + t13956 - t13958 + t13961 + t13965 - t13969 + t13972 - t13974 + t13976 + t13978 - t13979 - t13980 - t13981;
    (t13976, t13978, t13979, t13980, t13981, t13982)
}
