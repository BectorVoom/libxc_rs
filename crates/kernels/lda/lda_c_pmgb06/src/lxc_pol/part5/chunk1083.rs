//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1083/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1083<F: Float>(t20879: F, t20881: F, t20884: F, t20888: F, t20890: F, t20892: F, t20894: F, t20895: F, t20896: F, t20897: F, t20898: F, t20901: F, t10350: F, t10353: F, t10356: F, t10358: F, t10362: F, t18329: F, t18331: F, t20902: F, t20903: F, t20914: F, t20919: F) -> (F, F) {
    let t22028 = t20879 - t20881 - t20884 - t20888 - t20890 - t20892 + t20894 + t20895 + t20896 - t20897 + t20898 + t20901;
    let t22036 = t20902 + t20903 - 2.0 / 9.0 * t10350 - 0.013506172839506173 * t10353 - t10356 - t10358 + t10362 - 2.0 / 15.0 * t18329 + 2.0 / 45.0 * t18331 - t20914 - t20919;
    (t22028, t22036)
}
