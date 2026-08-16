//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1030/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1030(t10654: f64, t1318: f64, t2384: f64, t3787: f64, t519: f64, t7003: f64, t581: f64, t6865: f64, t1476: f64, t6205: f64, t7007: f64, t3899: f64, t6964: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17718 = t1318 * t10654 * t2384;
    let t17753 = t519 * t3787 * t7003;
    let t17759 = t581 * t6865;
    let t17768 = t6205 * t1476;
    let t17785 = t7007 * t1476;
    let t17788 = t1318 * t3899 * t6964;
    (t17718, t17753, t17759, t17768, t17785, t17788)
}
