//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1269/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1269(t1268: f64, t2061: f64, t21137: f64, t21186: f64, t21190: f64, t21196: f64, t21240: f64, t21246: f64, t22277: f64, t22281: f64, t22285: f64, t22747: f64, t22759: f64, t22764: f64, t22786: f64, t22788: f64, t22790: f64, t22792: f64, t22795: f64, t22798: f64, t22801: f64, t22804: f64, t25: f64, t3516: f64, t538: f64) -> f64 {
    let t22806 = 0.8638_f64 * t22747 - 0.24_f64 * t2061 * t538 * t22285 + 0.04_f64 * t25 * t538 * t21186 + 0.08_f64 * t2061 * t538 * t21190 + 0.013333333333333334_f64 * t25 * t538 * t22759 - 0.0022222222222222222_f64 * t25 * t1268 * t22764 - 0.08_f64 * t25 * t1268 * t22281 + 0.035555555555555556_f64 * t25 * t3516 * t21240 + 0.08_f64 * t2061 * t1268 * t21246 - 0.006666666666666667_f64 * t25 * t1268 * t21196 - 0.013333333333333334_f64 * t2061 * t1268 * t21137 + 0.16_f64 * t25 * t538 * t22277 - 0.07198333333333333_f64 * t22786 + 0.023994444444444443_f64 * t22788 - 0.02666666666666667_f64 * t22790 + 0.0044444444444444444_f64 * t22792 + 0.07198333333333333_f64 * t22795 - 0.023994444444444443_f64 * t22798 - 0.10664197530864197_f64 * t22801 - 0.23994444444444443_f64 * t22804;
    t22806
}
