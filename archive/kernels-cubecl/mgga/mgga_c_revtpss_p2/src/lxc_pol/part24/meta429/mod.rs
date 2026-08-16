//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta429 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1379;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta429<F: Float>(t3566: F, t5462: F, t5477: F, t10: F, t22: F, t576: F, t588: F, t15: F, t27: F, t11: F, t10276: F, t2224: F) -> (F, F, F, F, F, F, F, F) {
        let (t45859, t45863, t45927, t45929, t45931, t45933, t45935, t45936) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1379::<F>(t3566, t5462, t5477, t10, t22, t576, t588, t15, t27, t11, t10276, t2224);
    (t45859, t45863, t45927, t45929, t45931, t45933, t45935, t45936)
}
