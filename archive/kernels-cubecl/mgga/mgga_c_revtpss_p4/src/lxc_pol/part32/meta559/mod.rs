//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta559<F: Float>(t25240: F, t3964: F, t5617: F, t786: F, t97961: F, t25898: F, t98040: F, t25081: F, t7897: F, t2: F, t2411: F, t892: F) -> (F, F, F, F, F, F) {
        let (t98285, t98308, t98380, t98450, t98631, t98646) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1878::<F>(t25240, t3964, t5617, t786, t97961, t25898, t98040, t25081, t7897, t2, t2411, t892);
    (t98285, t98308, t98380, t98450, t98631, t98646)
}
