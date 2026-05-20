//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1165;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1166;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta294<F: Float>(t12247: F, t408: F, t12228: F, t3435: F, t3418: F, t698: F, t240: F, t3698: F, t3361: F, t635: F, t10356: F) -> (F, F, F, F, F, F, F) {
        let (t12248, t12249, t12251, t12252, t12254, t12256) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1165::<F>(t12247, t408, t12228, t3435, t3418, t698, t240, t3698, t3361, t635);
        let t12257 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1166::<F>(t10356, t12256);
    (t12248, t12249, t12251, t12252, t12254, t12256, t12257)
}
