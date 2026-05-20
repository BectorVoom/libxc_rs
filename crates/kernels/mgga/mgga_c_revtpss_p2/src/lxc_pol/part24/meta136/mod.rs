//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta136 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk718;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta136<F: Float>(t5571: F, t762: F, t1468: F, t3874: F, t1711: F, t3881: F, t1892: F, t212: F, t1358: F, t689: F, t1893: F, t786: F) -> (F, F, F, F, F, F, F) {
        let (t5572, t5574, t5582, t5599, t5600, t5601, t5603) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk718::<F>(t5571, t762, t1468, t3874, t1711, t3881, t1892, t212, t1358, t689, t1893, t786);
    (t5572, t5574, t5582, t5599, t5600, t5601, t5603)
}
