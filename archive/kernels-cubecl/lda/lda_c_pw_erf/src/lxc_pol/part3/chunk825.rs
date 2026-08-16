//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 825/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk825<F: Float>(t1880: F, t405: F, t455: F, t5495: F, t1556: F, t1733: F, t1735: F, t1881: F, t2822: F, t2828: F, t2831: F, t2835: F, t2836: F, t2838: F, t2841: F, t2842: F, t2847: F, t2860: F, t2864: F, t2876: F) -> (F, F, F) {
    let t5735 = t405 * t1880;
    let t5740 = t455 * t5495;
    let t5743 = -t2822 + t2828 - F::cast_from(3.64371538634302e-05_f64) * t2831 - t2835 + F::cast_from(0.019957056683757683_f64) * t2836 + F::cast_from(0.07982822673503073_f64) * t2838 - t2841 - F::cast_from(0.10643763564670763_f64) * t2842 + t2847 - F::cast_from(0.01197423401025461_f64) * t2860 - F::cast_from(0.02394846802050922_f64) * t2864 - t2876 + F::cast_from(6.0_f64) * t5735 * t1735 - F::cast_from(2.0_f64) * t1881 * t1556 + F::cast_from(6.0_f64) * t1733 * t5740;
    (t5735, t5740, t5743)
}
