//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta737 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2588;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2589;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta737(t123: f64, t2434: f64, t4077: f64, t9680: f64, t125: f64, t1358: f64, t555: f64, t8779: f64, t9645: f64, t1445: f64, t689: f64, t9634: f64, t2435: f64, t9667: f64, t268: f64, t39644: f64, t556: f64, t561: f64, t786: f64, t9656: f64, t686: f64, t72: f64, t9658: f64, t10150: f64, t9651: f64, t2439: f64, t4066: f64, t785: f64, t9303: f64, t9641: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47580, t47591, t47593) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2588(t123, t2434, t4077, t9680, t125, t1358, t555, t8779, t9645, t1445, t689, t9634);
        let (t47595, t47601, t47603, t47606) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2589(t2435, t9667, t268, t39644, t556, t561, t8779, t786, t9656, t686, t72, t9658);
        let (t47608, t47612, t47616, t47618) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2590(t10150, t2435, t686, t72, t9651, t9680, t1358, t2439, t4066, t785, t9303, t9641);
    (t47580, t47591, t47593, t47595, t47601, t47603, t47606, t47608, t47612, t47616, t47618)
}
