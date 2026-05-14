//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 861/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk861<F: Float>(t123: F, t317: F, t6104: F, t740: F, t5980: F, t73: F, t1122: F, t2395: F, t30: F, t6037: F, t959: F, t968: F, t273: F, t6067: F, t698: F, t27: F, t693: F) -> (F, F, F, F, F, F, F) {
    let t14852 = t123 * t740 * t6104 * t317;
    let t14875 = t73 * t5980;
    let t14939 = t2395 * t30 * t1122;
    let t14942 = t6037 * t959;
    let t14944 = t6037 * t968;
    let t14947 = t6067 * t273 * t698;
    let t14971 = t6067 * t27 * t693;
    (t14852, t14875, t14939, t14942, t14944, t14947, t14971)
}
