//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta807 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2909;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta807<F: Float>(t9572: F, t9860: F, t3869: F, t39742: F, t39440: F, t9866: F, t9863: F, t39532: F, t123: F, t2630: F, t3850: F, t9575: F) -> (F, F, F, F, F, F, F, F) {
        let (t47119, t47122, t47124, t47125, t47127, t47131, t47133, t47135) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2909::<F>(t9572, t9860, t3869, t39742, t39440, t9866, t9863, t39532, t123, t2630, t3850, t9575);
    (t47119, t47122, t47124, t47125, t47127, t47131, t47133, t47135)
}
