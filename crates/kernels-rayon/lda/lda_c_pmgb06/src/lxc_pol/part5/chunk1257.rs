//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1257/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1257(t19125: f64, t199: f64, t6946: f64, t868: f64, t122: f64, t569: f64, t7988: f64, t10472: f64, t10479: f64, t10487: f64, t14236: f64, t14238: f64, t14246: f64, t18404: f64, t9066: f64, t9070: f64) -> f64 {
    let t22065 = t19125 * t199;
    let t22067 = t6946 * t868;
    let t22071 = t122 * t569 * t7988;
    let t22074 = -t14236 - t14238 - t14246 - 4.429070076315393_f64 * t9066 + t9070 - t10472 + 0.0837628205355044_f64 * t22065 + 0.2512884616065132_f64 * t22067 - t10479 + 0.19455129084526285_f64 * t10487 + 0.019897291109174608_f64 * t22071 + 0.05969187332752383_f64 * t18404;
    t22074
}
