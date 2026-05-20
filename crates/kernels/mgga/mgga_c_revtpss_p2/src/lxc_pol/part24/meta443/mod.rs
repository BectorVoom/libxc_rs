//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1401;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1402;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta443<F: Float>(t39494: F, t3964: F, t4096: F, t2453: F, t9679: F, t3906: F, t3907: F, t1359: F, t39501: F, t10115: F, t555: F, t123: F, t125: F, t1358: F, t8779: F, t9645: F, t268: F, t39644: F, t556: F, t561: F, t786: F, t9656: F, t4146: F, t1892: F, t9646: F, t9648: F, t1904: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47454, t47480, t47504, t47561, t47567, t47591) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1401::<F>(t39494, t3964, t4096, t2453, t9679, t3906, t3907, t1359, t39501, t10115, t555, t123, t125, t1358, t8779, t9645);
        let (t47601, t47603, t47672, t47764, t47772) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1402::<F>(t268, t39644, t556, t561, t8779, t786, t9656, t4146, t1892, t9646, t9648, t1904, t47567);
    (t47454, t47480, t47504, t47561, t47591, t47601, t47603, t47672, t47764, t47772)
}
