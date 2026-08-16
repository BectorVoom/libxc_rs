//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 866/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk866(t4041: f64, t5859: f64, t6885: f64, t6889: f64, t6892: f64, t7730: f64, t7736: f64, t7740: f64, t7744: f64, t7748: f64, t7751: f64, t7754: f64, t7755: f64, t7757: f64, t7796: f64, t7801: f64, t7805: f64, t7807: f64) -> f64 {
    let t8044 = t7730 + t7736 - t7740 + t7744 + t7748 + 0.09973633333333333_f64 * t5859 - t7751 - t7754 + t4041 + t7755 + t6885 + 0.18233333333333332_f64 * t6889 + t6892 - t7757 - t7796 + t7801 - t7805 + t7807;
    t8044
}
