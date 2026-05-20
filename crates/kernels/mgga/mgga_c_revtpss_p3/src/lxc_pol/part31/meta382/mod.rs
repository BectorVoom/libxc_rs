//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta382 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta382<F: Float>(t15191: F, t1058: F, t4794: F, t11243: F, t72: F, t3088: F, t12078: F, t1086: F, t4746: F, t3090: F, t1065: F, t2852: F) -> (F, F, F, F, F, F, F, F) {
        let (t15876, t15892, t15904, t15905, t15906, t15925, t15926, t15935) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1419::<F>(t15191, t1058, t4794, t11243, t72, t3088, t12078, t1086, t4746, t3090, t1065, t2852);
    (t15876, t15892, t15904, t15905, t15906, t15925, t15926, t15935)
}
