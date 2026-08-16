//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1647;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta335<F: Float>(t11151: F, t2908: F, t141: F, t11160: F, t930: F, t11132: F, t240: F, t624: F, t281: F, t283: F, t2909: F, t698: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11328, t11329, t11331, t11332, t11334, t11335, t11337, t11338, t11339) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1647::<F>(t11151, t2908, t141, t11160, t930, t11132, t240, t624, t281, t283, t2909, t698);
    (t11328, t11329, t11331, t11332, t11334, t11335, t11337, t11338, t11339)
}
