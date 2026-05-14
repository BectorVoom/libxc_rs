//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1087/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1087<F: Float>(t19125: F, t199: F, t6946: F, t868: F, t122: F, t569: F, t7988: F, t10472: F, t10479: F, t10487: F, t14236: F, t14238: F, t14246: F, t18404: F, t9066: F, t9070: F) -> (F,) {
    let t22065 = t19125 * t199;
    let t22067 = t6946 * t868;
    let t22071 = t122 * t569 * t7988;
    let t22074 = -t14236 - t14238 - t14246 - 4.429070076315393 * t9066 + t9070 - t10472 + 0.0837628205355044 * t22065 + 0.2512884616065132 * t22067 - t10479 + 0.19455129084526285 * t10487 + 0.019897291109174608 * t22071 + 0.05969187332752383 * t18404;
    (t22074,)
}
