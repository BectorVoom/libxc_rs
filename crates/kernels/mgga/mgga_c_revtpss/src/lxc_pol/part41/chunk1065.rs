//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1065/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1065<F: Float>(t15127: F, t15125: F, t15191: F, t1058: F, t4794: F, t11243: F, t72: F, t3088: F, t12078: F, t1086: F, t4746: F, t3090: F, t1065: F, t2852: F, t3173: F, t4879: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15874 = 0.37037037037037037037e-2 * t15127;
    let t15875 = 0.11111111111111111111e-1 * t15125;
    let t15876 = 0.55555555555555555556e-2 * t15191;
    let t15892 = 0.15244095330869239812e-2 * t4794 * t1058;
    let t15904 = t11243 * t72;
    let t15905 = t3088 * t15904;
    let t15906 = t12078 * t15905;
    let t15925 = t4746 * t1086;
    let t15926 = t15925 * t3090;
    let t15935 = t1065 * t2852;
    let t15942 = 0.28582678745379824648e-3 * t4879 * t3173;
    (t15874, t15875, t15876, t15892, t15904, t15905, t15906, t15926, t15935, t15942)
}
