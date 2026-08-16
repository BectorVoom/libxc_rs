//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 769/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk769(t159: f64, t285: f64, t6039: f64, t2363: f64, t477: f64, t281: f64, t2675: f64, t2805: f64, t2375: f64, t684: f64, t1881: f64, t2676: f64, t2822: f64, t2828: f64, t2831: f64, t2835: f64, t2838: f64, t2841: f64, t2842: f64, t2847: f64, t2864: f64, t2876: f64, t777: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7067 = t6039 * t159 * t285;
    let t7071 = t2363 * t477 * t285;
    let t7072 = t281 * t7071;
    let t7075 = t2805 * t2675;
    let t7077 = t684 * t2375;
    let t7079 = -t2822 + t2828 - 1.82185769317151e-05_f64 * t2831 - t2835 + 0.039914113367515366_f64 * t2838 - t2841 - 0.05321881782335382_f64 * t2842 + t2847 - 0.01197423401025461_f64 * t2864 - t2876 - 0.01197423401025461_f64 * t281 * t7067 - 0.01197423401025461_f64 * t7072 - t1881 * t2676 - t777 * t7075 + 0.019957056683757683_f64 * t7077;
    (t7067, t7071, t7072, t7075, t7077, t7079)
}
