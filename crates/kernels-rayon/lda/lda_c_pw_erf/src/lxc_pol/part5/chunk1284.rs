//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1284/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1284(t15149: f64, t10145: f64, t11709: f64, t13846: f64, t15777: f64, t15779: f64, t15788: f64, t15798: f64, t15800: f64, t15820: f64, t15836: f64, t15838: f64, t22655: f64, t22665: f64, t22667: f64, t22669: f64, t22671: f64, t22681: f64, t22684: f64, t22687: f64, t9847: f64) -> (f64, f64) {
    let t22983 = 1.2e-20_f64 * t15149;
    let t23004 = -0.005037777777777778_f64 * t11709 + t13846 + 0.002518888888888889_f64 * t15777 - 0.005037777777777778_f64 * t15779 - 0.0018891666666666666_f64 * t15788 - 0.002099074074074074_f64 * t15798 - 0.005037777777777778_f64 * t15800 - t10145 - 0.011335_f64 * t15820 + 0.0018891666666666666_f64 * t22655 - 0.0006297222222222223_f64 * t22665 - 0.0006996913580246914_f64 * t22667 - 0.003778333333333333_f64 * t22669 + 0.002518888888888889_f64 * t22671 + 0.034005_f64 * t22681 - 0.02267_f64 * t22684 + 0.006297222222222222_f64 * t22687 - 0.0019591358024691357_f64 * t9847 + 0.007556666666666666_f64 * t15836 + 0.015113333333333333_f64 * t15838;
    (t22983, t23004)
}
