//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1267/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1267<F: Float>(t684: F, t7067: F, t1738: F, t2375: F, t281: F, t285: F, t477: F, t6039: F, t1128: F, t2363: F, t11507: F, t11548: F, t11551: F, t11557: F, t11561: F, t11563: F, t159: F, t17095: F, t2208: F, t2645: F, t2779: F, t8827: F, t8831: F, t8834: F, t8838: F, t8842: F, t8845: F) -> (F,) {
    let t18876 = t684 * t7067;
    let t18880 = t1738 * t2375;
    let t18888 = t281 * t6039 * t477 * t285;
    let t18892 = t281 * t2363 * t1128 * t285;
    let t18899 = 0.008135887625008338 * t8827 + t8831 - 0.013430671634934398 * t8834 - t8838 + t8842 + 0.001355981270834723 * t8845 - 0.013430671634934398 * t11548 + 0.008135887625008338 * t11551 + 0.039914113367515366 * t18876 + 6.0 * t11507 * t2208 - 0.05321881782335382 * t18880 - 0.01197423401025461 * t281 * t17095 * t159 * t285 - 0.02394846802050922 * t18888 - 0.01197423401025461 * t18892 + 0.3902713307045947 * t11557 + 0.0003279343847708718 * t11561 - 0.31931290694012293 * t11563 + 2.0 * t2645 * t2779;
    (t18899,)
}
