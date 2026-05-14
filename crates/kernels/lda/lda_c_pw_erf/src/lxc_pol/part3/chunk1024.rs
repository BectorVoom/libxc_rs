//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1024/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1024<F: Float>(t11699: F, t11703: F, t11707: F, t11709: F, t11711: F, t11713: F, t11715: F, t11718: F, t11721: F, t11726: F, t11731: F, t11748: F, t11751: F, t11755: F, t11770: F, t11773: F, t11775: F, t11779: F, t13846: F) -> (F,) {
    let t13852 = 0.04534 * t11699 - 0.003778333333333333 * t11703 - 0.02267 * t11707 - 0.005037777777777778 * t11709 - 0.0012594444444444445 * t11711 - 0.002099074074074074 * t11713 - 0.02770777777777778 * t11715 + 0.0012594444444444445 * t11718 + 0.007556666666666666 * t11721 + 0.005597530864197531 * t11726 + 0.012594444444444445 * t11731 - 0.04534 * t11748 + 0.003778333333333333 * t11751 + t13846 + 0.007556666666666666 * t11755 + 0.08312333333333333 * t11770 + 0.06801 * t11773 - 0.011335 * t11775 - 0.04534 * t11779;
    (t13852,)
}
