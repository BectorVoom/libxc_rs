//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta414<F: Float>(t177: F, t4392: F, t762: F, t10605: F, t162: F, t4403: F, t2626: F, t4398: F, t10439: F) -> (F, F, F, F, F, F, F) {
        let (t14322, t14324, t14325, t14327, t14328, t14329, t14330) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2019::<F>(t177, t4392, t762, t10605, t162, t4403, t2626, t4398, t10439);
    (t14322, t14324, t14325, t14327, t14328, t14329, t14330)
}
