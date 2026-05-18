//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1269/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1269<F: Float>(t1268: F, t2061: F, t21137: F, t21186: F, t21190: F, t21196: F, t21240: F, t21246: F, t22277: F, t22281: F, t22285: F, t22747: F, t22759: F, t22764: F, t22786: F, t22788: F, t22790: F, t22792: F, t22795: F, t22798: F, t22801: F, t22804: F, t25: F, t3516: F, t538: F) -> F {
    let t22806 = F::new(0.8638) * t22747 - F::new(0.24) * t2061 * t538 * t22285 + F::new(0.04) * t25 * t538 * t21186 + F::new(0.08) * t2061 * t538 * t21190 + F::new(0.013333333333333334) * t25 * t538 * t22759 - F::new(0.0022222222222222222) * t25 * t1268 * t22764 - F::new(0.08) * t25 * t1268 * t22281 + F::new(0.035555555555555556) * t25 * t3516 * t21240 + F::new(0.08) * t2061 * t1268 * t21246 - F::new(0.006666666666666667) * t25 * t1268 * t21196 - F::new(0.013333333333333334) * t2061 * t1268 * t21137 + F::new(0.16) * t25 * t538 * t22277 - F::new(0.07198333333333333) * t22786 + F::new(0.023994444444444443) * t22788 - F::new(0.02666666666666667) * t22790 + F::new(0.0044444444444444444) * t22792 + F::new(0.07198333333333333) * t22795 - F::new(0.023994444444444443) * t22798 - F::new(0.10664197530864197) * t22801 - F::new(0.23994444444444443) * t22804;
    t22806
}
