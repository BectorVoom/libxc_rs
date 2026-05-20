//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta450 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1414;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta450<F: Float>(t10199: F, t1514: F, t4398: F, t9372: F, t9387: F, t14362: F, t9575: F, t9318: F, t10565: F, t1469: F, t706: F, t1531: F, t36: F) -> (F, F, F, F, F, F, F) {
        let (t49698, t49866, t49897, t49926, t49940, t50084, t50089) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1414::<F>(t10199, t1514, t4398, t9372, t9387, t14362, t9575, t9318, t10565, t1469, t706, t1531, t36);
    (t49698, t49866, t49897, t49926, t49940, t50084, t50089)
}
