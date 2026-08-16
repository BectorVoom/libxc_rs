//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta455 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1422;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1423;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta455(t4522: f64, t874: f64, t9288: f64, t1573: f64, t40317: f64, t10867: f64, t1568: f64, t4503: f64, t786: f64, t40270: f64, t4496: f64, t10115: f64, t1576: f64, t10535: f64, t14523: f64, t9285: f64, t14946: f64, t2710: f64, t10111: f64, t22: f64, t4518: f64, t231: f64, t39698: f64, t4494: f64, t40921: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51445, t51452, t51498, t51549, t51553, t51578) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1422(t4522, t874, t9288, t1573, t40317, t10867, t1568, t4503, t786, t40270, t4496, t10115, t1576);
        let (t51635, t51646, t51660, t51676, t51686) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1423(t10535, t14523, t9285, t14946, t2710, t10111, t22, t4518, t231, t39698, t4494, t40921, t4496);
    (t51445, t51452, t51498, t51549, t51553, t51578, t51635, t51646, t51660, t51676, t51686)
}
