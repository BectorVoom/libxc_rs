//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta207 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk981;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta207<F: Float>(t10259: F, t508: F, t3813: F, t670: F, t10: F, t580: F, t22: F, t576: F, t15: F, t588: F, t11: F, t2: F) -> (F, F, F, F, F, F) {
        let (t10260, t10263, t10271, t10273, t10275, t10276) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk981::<F>(t10259, t508, t3813, t670, t10, t580, t22, t576, t15, t588, t11, t2);
    (t10260, t10263, t10271, t10273, t10275, t10276)
}
