//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta455 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1422;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1423;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta455<F: Float>(t4522: F, t874: F, t9288: F, t1573: F, t40317: F, t10867: F, t1568: F, t4503: F, t786: F, t40270: F, t4496: F, t10115: F, t1576: F, t10535: F, t14523: F, t9285: F, t14946: F, t2710: F, t10111: F, t22: F, t4518: F, t231: F, t39698: F, t4494: F, t40921: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t51445, t51452, t51498, t51549, t51553, t51578) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1422::<F>(t4522, t874, t9288, t1573, t40317, t10867, t1568, t4503, t786, t40270, t4496, t10115, t1576);
        let (t51635, t51646, t51660, t51676, t51686) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1423::<F>(t10535, t14523, t9285, t14946, t2710, t10111, t22, t4518, t231, t39698, t4494, t40921, t4496);
    (t51445, t51452, t51498, t51549, t51553, t51578, t51635, t51646, t51660, t51676, t51686)
}
