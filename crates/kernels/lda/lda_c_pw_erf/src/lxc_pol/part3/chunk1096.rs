//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1096/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1096<F: Float>(t12814: F, t3863: F, t5306: F, t571: F, t3859: F, t4628: F, t519: F, t11777: F, t1326: F, t12788: F, t12793: F, t12796: F, t12799: F, t12801: F, t12803: F, t12807: F, t12810: F, t12812: F) -> (F, F, F, F, F) {
    let t12815 = F::new(8.0) / F::new(135.0) * t12814;
    let t12817 = t571 * t3863 * t5306;
    let t12818 = F::new(16.0) / F::new(45.0) * t12817;
    let t12820 = t519 * t3859 * t4628;
    let t12821 = F::new(16.0) / F::new(15.0) * t12820;
    let t12824 = F::new(32.0) / F::new(15.0) * t519 * t1326 * t11777;
    let t12825 = t12788 + t12793 + t12796 + t12799 - t12801 + t12803 - t12807 - t12810 - t12812 + t12815 + t12818 + t12821 - t12824;
    (t12815, t12818, t12821, t12824, t12825)
}
