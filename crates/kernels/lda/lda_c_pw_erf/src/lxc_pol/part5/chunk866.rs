//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 866/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk866<F: Float>(t4041: F, t5859: F, t6885: F, t6889: F, t6892: F, t7730: F, t7736: F, t7740: F, t7744: F, t7748: F, t7751: F, t7754: F, t7755: F, t7757: F, t7796: F, t7801: F, t7805: F, t7807: F) -> F {
    let t8044 = t7730 + t7736 - t7740 + t7744 + t7748 + F::new(0.09973633333333333) * t5859 - t7751 - t7754 + t4041 + t7755 + t6885 + F::new(0.18233333333333332) * t6889 + t6892 - t7757 - t7796 + t7801 - t7805 + t7807;
    t8044
}
