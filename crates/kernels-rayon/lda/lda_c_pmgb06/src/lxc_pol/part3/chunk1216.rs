//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1216/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1216(t13718: f64, t13720: f64, t13722: f64, t13725: f64, t13728: f64, t13730: f64, t13732: f64, t13734: f64, t13739: f64, t13741: f64, t13743: f64, t13745: f64, t13747: f64, t13749: f64, t13751: f64, t13753: f64, t13755: f64, t13757: f64, t13759: f64, t13762: f64, t13764: f64, t13767: f64, t13769: f64) -> (f64, f64) {
    let t14439 = t13718 - t13720 + t13722 + t13725 + t13728 + t13730 + t13732 + t13734 + t13739 + t13741 + t13743;
    let t14440 = t13745 + t13747 + t13749 + t13751 + t13753 + t13755 - t13757 - t13759 + t13762 + t13764 + t13767 - t13769;
    (t14439, t14440)
}
