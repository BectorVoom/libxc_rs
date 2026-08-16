//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1419;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1420;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta453<F: Float>(t4398: F, t9419: F, t14362: F, t9572: F, t1549: F, t40861: F, t14779: F, t40721: F, t14819: F, t40517: F, t4372: F, t9789: F, t40424: F, t4430: F, t1561: F, t40360: F, t9784: F, t10504: F, t15002: F, t9285: F, t11015: F, t4325: F, t4477: F, t9292: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t50893, t50901, t50941, t50943, t51042, t51083) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1419::<F>(t4398, t9419, t14362, t9572, t1549, t40861, t14779, t40721, t14819, t40517, t4372, t9789);
        let (t51100, t51104, t51170, t51203, t51211, t51213) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1420::<F>(t40424, t4430, t1561, t40360, t4372, t9784, t10504, t15002, t9285, t11015, t4325, t4477, t9292);
    (t50893, t50901, t50941, t50943, t51042, t51083, t51100, t51104, t51170, t51203, t51211, t51213)
}
