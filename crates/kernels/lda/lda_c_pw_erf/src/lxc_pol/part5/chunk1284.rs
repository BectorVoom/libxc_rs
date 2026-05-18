//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1284/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1284<F: Float>(t15149: F, t10145: F, t11709: F, t13846: F, t15777: F, t15779: F, t15788: F, t15798: F, t15800: F, t15820: F, t15836: F, t15838: F, t22655: F, t22665: F, t22667: F, t22669: F, t22671: F, t22681: F, t22684: F, t22687: F, t9847: F) -> (F, F) {
    let t22983 = F::new(1.2e-20) * t15149;
    let t23004 = -F::new(0.005037777777777778) * t11709 + t13846 + F::new(0.002518888888888889) * t15777 - F::new(0.005037777777777778) * t15779 - F::new(0.0018891666666666666) * t15788 - F::new(0.002099074074074074) * t15798 - F::new(0.005037777777777778) * t15800 - t10145 - F::new(0.011335) * t15820 + F::new(0.0018891666666666666) * t22655 - F::new(0.0006297222222222223) * t22665 - F::new(0.0006996913580246914) * t22667 - F::new(0.003778333333333333) * t22669 + F::new(0.002518888888888889) * t22671 + F::new(0.034005) * t22681 - F::new(0.02267) * t22684 + F::new(0.006297222222222222) * t22687 - F::new(0.0019591358024691357) * t9847 + F::new(0.007556666666666666) * t15836 + F::new(0.015113333333333333) * t15838;
    (t22983, t23004)
}
