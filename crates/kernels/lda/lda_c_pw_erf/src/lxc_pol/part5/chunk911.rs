//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 911/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk911<F: Float>(t4763: F, t5282: F, t10654: F, t1318: F, t2384: F, t3787: F, t519: F, t7003: F, t581: F, t6865: F, t1476: F, t6205: F, t7007: F, t3899: F, t6964: F, t1529: F, t2402: F) -> (F, F, F, F, F, F, F, F) {
    let t17715 = t4763 * t5282;
    let t17718 = t1318 * t10654 * t2384;
    let t17753 = t519 * t3787 * t7003;
    let t17759 = t581 * t6865;
    let t17768 = t6205 * t1476;
    let t17785 = t7007 * t1476;
    let t17788 = t1318 * t3899 * t6964;
    let t17794 = t2402 * t1529;
    (t17715, t17718, t17753, t17759, t17768, t17785, t17788, t17794)
}
