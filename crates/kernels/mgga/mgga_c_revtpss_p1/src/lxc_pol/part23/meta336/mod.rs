//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1636;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta336<F: Float>(t2435: F, t5760: F, t1892: F, t3999: F, t545: F, t5710: F, t869: F, t689: F, t225: F, t9990: F, t213: F) -> (F, F, F, F, F, F, F) {
        let (t14166, t14171, t14188, t14189, t14191, t14192, t14193) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1636::<F>(t2435, t5760, t1892, t3999, t545, t5710, t869, t689, t225, t9990, t213);
    (t14166, t14171, t14188, t14189, t14191, t14192, t14193)
}
