//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1354;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta344<F: Float>(t9395: F, t2626: F, t5571: F, t1856: F, t2608: F, t512: F, t9408: F, t9411: F, t1317: F, t5567: F, t4147: F, t5778: F) -> (F, F, F, F, F, F, F) {
        let (t13623, t13630, t13633, t13634, t13635, t13643, t13648) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1354::<F>(t9395, t2626, t5571, t1856, t2608, t512, t9408, t9411, t1317, t5567, t4147, t5778);
    (t13623, t13630, t13633, t13634, t13635, t13643, t13648)
}
