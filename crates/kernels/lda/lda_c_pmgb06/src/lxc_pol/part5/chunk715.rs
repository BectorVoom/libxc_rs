//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 715/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk715<F: Float>(t3392: F, t3395: F, t6733: F, t6738: F, t6740: F, t6743: F, t6746: F, t6750: F, t6754: F, t6758: F, t6763: F, t6768: F, t6772: F, t6777: F, t6779: F, t6909: F, t7178: F, t7182: F, t7184: F, t7185: F, t7188: F, t7190: F, t7196: F, t7200: F, t7204: F, t7207: F, t7212: F, t7217: F, t7220: F, t7221: F) -> (F,) {
    let t7224 = 8.0 / 3.0 * t3392 + t3395 - t6733 - t6738 - t6740 - t6743 - t6746 - t6750 + t6754 - t6758 - t6763 + t6768 - t6772 - t6777 - t6779;
    let t7228 = t7178 + t7182 + t7184 + t7185 + t7188 + t7190 + t7196 + t7200 + t7204 + t7207 + t7212 + t7217 + t7220 + t7221 + t7224 + t6909;
    (t7228,)
}
