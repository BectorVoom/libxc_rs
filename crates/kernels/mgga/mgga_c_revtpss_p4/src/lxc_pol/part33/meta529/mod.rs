//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1880;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta529<F: Float>(t5627: F, t8996: F, t28167: F, t531: F, t7933: F, t7238: F, t2014: F, t1450: F, t5591: F, t7237: F, t13648: F, t2034: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28168, t28170, t28172, t28173, t28175, t28176, t28177, t28179, t28182) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1880::<F>(t5627, t8996, t28167, t531, t7933, t7238, t2014, t1450, t5591, t7237, t13648, t2034);
    (t28168, t28170, t28172, t28173, t28175, t28176, t28177, t28179, t28182)
}
