//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 709/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk709<F: Float>(t159: F, t285: F, t6039: F, t2363: F, t477: F, t281: F, t2675: F, t2805: F, t2375: F, t684: F, t1881: F, t2676: F, t2822: F, t2828: F, t2831: F, t2835: F, t2838: F, t2841: F, t2842: F, t2847: F, t2864: F, t2876: F, t777: F) -> (F, F, F, F, F, F) {
    let t7067 = t6039 * t159 * t285;
    let t7071 = t2363 * t477 * t285;
    let t7072 = t281 * t7071;
    let t7075 = t2805 * t2675;
    let t7077 = t684 * t2375;
    let t7079 = -t2822 + t2828 - 1.82185769317151e-05 * t2831 - t2835 + 0.039914113367515366 * t2838 - t2841 - 0.05321881782335382 * t2842 + t2847 - 0.01197423401025461 * t2864 - t2876 - 0.01197423401025461 * t281 * t7067 - 0.01197423401025461 * t7072 - t1881 * t2676 - t777 * t7075 + 0.019957056683757683 * t7077;
    (t7067, t7071, t7072, t7075, t7077, t7079)
}
