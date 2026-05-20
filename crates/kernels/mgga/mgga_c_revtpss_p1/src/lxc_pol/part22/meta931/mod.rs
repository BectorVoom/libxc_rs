//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta931 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3159;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3160;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta931<F: Float>(t17654: F, t17657: F, t56756: F, t247: F, t44545: F, t5230: F, t5384: F, t12984: F, t5327: F, t12995: F, t17438: F, t17303: F, t3667: F, t12886: F, t5381: F, t12627: F, t489: F, t17728: F, t13011: F, t5373: F, t1222: F, t5368: F, t697: F, t17170: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t57227, t57241, t57250, t57252, t57256) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3159::<F>(t17654, t17657, t56756, t247, t44545, t5230, t5384, t12984, t5327, t12995, t17438, t17303, t3667);
        let (t57258, t57264, t57265, t57270, t57273, t57275) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3160::<F>(t12886, t5381, t12627, t489, t17728, t13011, t5373, t1222, t5368, t697, t17170, t73);
    (t57227, t57241, t57250, t57252, t57256, t57258, t57264, t57265, t57270, t57273, t57275)
}
