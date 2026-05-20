//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta591<F: Float>(t94385: F, t9675: F, t94589: F, t7289: F, t94377: F, t7285: F, t9288: F, t7284: F, t7243: F, t9292: F, t2453: F, t3908: F, t7275: F) -> (F, F, F, F, F, F, F) {
        let (t94590, t94591, t94593, t94600, t94602, t94608, t94616) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2016::<F>(t94385, t9675, t94589, t7289, t94377, t7285, t9288, t7284, t7243, t9292, t2453, t3908, t7275);
    (t94590, t94591, t94593, t94600, t94602, t94608, t94616)
}
