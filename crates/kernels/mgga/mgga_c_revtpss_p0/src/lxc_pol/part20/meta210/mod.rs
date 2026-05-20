//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta210 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk987;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta210<F: Float>(t10355: F, t10356: F, t2275: F, t606: F, t2258: F, t10326: F, t48: F, t58: F, t59: F, t2282: F, t60: F, t10199: F, t10345: F, t2270: F, t2276: F, t2279: F, t44: F, t49: F, t56: F, t614: F, t617: F) -> (F, F, F, F, F, F, F) {
        let (t10357, t10360, t10361, t10364, t10368, t10369, t10372, t10373, t10376, t10379) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk987::<F>(t10355, t10356, t2275, t606, t2258, t10326, t48, t58, t59, t2282, t60, t10199);
        let t10380 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk988::<F>(t10345, t10357, t10361, t10364, t10369, t10373, t10376, t10379, t2270, t2276, t2279, t44, t49, t56, t614, t617);
    (t10357, t10360, t10361, t10364, t10368, t10372, t10380)
}
