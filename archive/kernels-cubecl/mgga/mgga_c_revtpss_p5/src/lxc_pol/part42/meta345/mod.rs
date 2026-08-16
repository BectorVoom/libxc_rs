//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1151;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta345<F: Float>(t15191: F, t1058: F, t4794: F, t11243: F, t72: F, t3088: F, t12078: F, t1086: F, t4746: F, t3090: F, t1065: F, t2852: F, t3173: F, t4879: F, t4866: F, t73: F, t11710: F, t4782: F, t3091: F, t1014: F, t140: F, t4579: F, t1011: F, t3252: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15876, t15892, t15904, t15905, t15906, t15926, t15935) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1151::<F>(t15191, t1058, t4794, t11243, t72, t3088, t12078, t1086, t4746, t3090, t1065, t2852);
        let (t15942, t15957, t15986, t15990, t15993) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1152::<F>(t3173, t4879, t4866, t73, t11710, t4782, t3091, t1014, t140, t4579, t1011, t3252);
    (t15876, t15892, t15904, t15905, t15906, t15926, t15935, t15942, t15957, t15986, t15990, t15993)
}
