//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1428;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1429;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta383<F: Float>(t15993: F, t4574: F, t1011: F, t1012: F, t11821: F, t11922: F, t4906: F, t3115: F, t4895: F, t4892: F, t140: F, t4886: F, t3241: F, t4924: F, t12047: F, t15905: F, t12167: F, t3057: F, t380: F, t3088: F, t370: F, t994: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15996, t16012, t16037, t16057, t16060) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1428::<F>(t15993, t4574, t1011, t1012, t11821, t11922, t4906, t3115, t4895, t4892, t140, t4886);
        let (t16062, t16064, t16067, t16081, t16088, t16089, t16094) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1429::<F>(t1011, t16060, t3241, t4924, t12047, t15905, t12167, t3057, t380, t3088, t370, t994);
    (t15996, t16012, t16037, t16057, t16062, t16064, t16067, t16081, t16088, t16089, t16094)
}
