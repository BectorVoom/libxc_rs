//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta537<F: Float>(t29494: F, t7237: F, t2014: F, t1450: F, t6836: F, t25864: F, t1843: F, t7741: F, t651: F, t196: F, t197: F, t6773: F) -> (F, F, F, F, F, F, F, F) {
        let (t29495, t29497, t29498, t29499, t29501, t29502, t29504, t29506) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1922::<F>(t29494, t7237, t2014, t1450, t6836, t25864, t1843, t7741, t651, t196, t197, t6773);
    (t29495, t29497, t29498, t29499, t29501, t29502, t29504, t29506)
}
