//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta553<F: Float>(t545: F, t6888: F, t869: F, t689: F, t22005: F, t4003: F, t5744: F, t2782: F, t21981: F, t4086: F, t543: F, t22009: F) -> (F, F, F, F, F, F, F, F) {
        let (t22351, t22352, t22353, t22361, t22362, t22365, t22366, t22369) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2110::<F>(t545, t6888, t869, t689, t22005, t4003, t5744, t2782, t21981, t4086, t543, t22009);
    (t22351, t22352, t22353, t22361, t22362, t22365, t22366, t22369)
}
