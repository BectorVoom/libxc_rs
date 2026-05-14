//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1259/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1259<F: Float>(t10902: F, t10934: F, t10937: F, t10940: F, t10943: F, t10946: F, t14696: F, t14699: F, t14702: F, t14705: F, t18979: F, t123: F, t4429: F, t868: F, t199: F, t315: F, t6716: F) -> (F, F, F) {
    let t18985 = -0.14149184788746388 * t10934 - 0.28298369577492777 * t10937 - 0.14149184788746388 * t10940 + 1.0376068845080684 * t10943 + 1.0376068845080684 * t10946 - 0.2133002709687175 * t18979 - 0.5659673915498555 * t14696 - 0.5659673915498555 * t14699 - 0.5659673915498555 * t14702 - 0.5659673915498555 * t14705 - t10902;
    let t18988 = t123 * t4429 * t868;
    let t18995 = t123 * t315 * t6716 * t199;
    (t18985, t18988, t18995)
}
