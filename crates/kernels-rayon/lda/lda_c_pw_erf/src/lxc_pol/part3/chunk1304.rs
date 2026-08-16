//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1304/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1304(t13551: f64, t13556: f64, t13558: f64, t13559: f64, t13560: f64, t13561: f64, t13746: f64, t13748: f64, t13750: f64, t13752: f64, t13755: f64, t13764: f64, t13765: f64) -> f64 {
    let t15100 = t13551 - t13556 - t13558 + t13559 - t13560 - t13561 - t13746 - t13748 - t13750 - t13752 + t13755 - t13764 + t13765;
    t15100
}
