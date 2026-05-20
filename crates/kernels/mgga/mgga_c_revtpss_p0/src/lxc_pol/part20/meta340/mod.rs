//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1266;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta340<F: Float>(t12078: F, t15905: F, t12160: F, t4891: F, t1065: F, t2852: F, t2857: F, t357: F, t2251: F, t1014: F, t140: F, t3252: F) -> (F, F, F, F, F, F) {
        let (t15906, t15917, t15935, t15963, t15987, t15993) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1266::<F>(t12078, t15905, t12160, t4891, t1065, t2852, t2857, t357, t2251, t1014, t140, t3252);
    (t15906, t15917, t15935, t15963, t15987, t15993)
}
