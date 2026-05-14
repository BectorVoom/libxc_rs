//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1103/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1103<F: Float>(t11093: F, t11097: F, t11098: F, t11101: F, t11104: F, t11105: F, t11107: F, t12719: F, t12720: F, t12721: F, t12722: F, t12724: F, t12726: F, t12729: F, t12731: F, t12732: F, t12733: F, t12734: F, t12735: F, t12740: F, t12741: F, t12742: F, t12743: F, t12745: F, t12746: F, t12748: F) -> (F, F) {
    let t15043 = t12719 + t12720 - t12721 - t12722 + t12724 + t12726 - 8.0 / 135.0 * t11093 + t11097 + 8.0 / 27.0 * t11098 + t11101 - t11104 + 2.0 / 45.0 * t11105 - 2.0 / 45.0 * t11107;
    let t15044 = -t12729 + t12731 + t12732 - t12733 - t12734 + t12735 + t12740 - t12741 + t12742 - t12743 - t12745 - t12746 - t12748;
    (t15043, t15044)
}
