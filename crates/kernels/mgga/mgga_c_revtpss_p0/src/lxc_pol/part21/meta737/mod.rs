//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta737 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2588;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2589;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta737<F: Float>(t123: F, t2434: F, t4077: F, t9680: F, t125: F, t1358: F, t555: F, t8779: F, t9645: F, t1445: F, t689: F, t9634: F, t2435: F, t9667: F, t268: F, t39644: F, t556: F, t561: F, t786: F, t9656: F, t686: F, t72: F, t9658: F, t10150: F, t9651: F, t2439: F, t4066: F, t785: F, t9303: F, t9641: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t47580, t47591, t47593) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2588::<F>(t123, t2434, t4077, t9680, t125, t1358, t555, t8779, t9645, t1445, t689, t9634);
        let (t47595, t47601, t47603, t47606) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2589::<F>(t2435, t9667, t268, t39644, t556, t561, t8779, t786, t9656, t686, t72, t9658);
        let (t47608, t47612, t47616, t47618) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2590::<F>(t10150, t2435, t686, t72, t9651, t9680, t1358, t2439, t4066, t785, t9303, t9641);
    (t47580, t47591, t47593, t47595, t47601, t47603, t47606, t47608, t47612, t47616, t47618)
}
