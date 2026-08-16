//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 825/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk825(t1880: f64, t405: f64, t455: f64, t5495: f64, t1556: f64, t1733: f64, t1735: f64, t1881: f64, t2822: f64, t2828: f64, t2831: f64, t2835: f64, t2836: f64, t2838: f64, t2841: f64, t2842: f64, t2847: f64, t2860: f64, t2864: f64, t2876: f64) -> (f64, f64, f64) {
    let t5735 = t405 * t1880;
    let t5740 = t455 * t5495;
    let t5743 = -t2822 + t2828 - 3.64371538634302e-05_f64 * t2831 - t2835 + 0.019957056683757683_f64 * t2836 + 0.07982822673503073_f64 * t2838 - t2841 - 0.10643763564670763_f64 * t2842 + t2847 - 0.01197423401025461_f64 * t2860 - 0.02394846802050922_f64 * t2864 - t2876 + 6.0_f64 * t5735 * t1735 - 2.0_f64 * t1881 * t1556 + 6.0_f64 * t1733 * t5740;
    (t5735, t5740, t5743)
}
