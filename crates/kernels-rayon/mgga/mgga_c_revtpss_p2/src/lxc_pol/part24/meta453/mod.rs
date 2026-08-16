//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1419;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1420;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta453(t4398: f64, t9419: f64, t14362: f64, t9572: f64, t1549: f64, t40861: f64, t14779: f64, t40721: f64, t14819: f64, t40517: f64, t4372: f64, t9789: f64, t40424: f64, t4430: f64, t1561: f64, t40360: f64, t9784: f64, t10504: f64, t15002: f64, t9285: f64, t11015: f64, t4325: f64, t4477: f64, t9292: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50893, t50901, t50941, t50943, t51042, t51083) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1419(t4398, t9419, t14362, t9572, t1549, t40861, t14779, t40721, t14819, t40517, t4372, t9789);
        let (t51100, t51104, t51170, t51203, t51211, t51213) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1420(t40424, t4430, t1561, t40360, t4372, t9784, t10504, t15002, t9285, t11015, t4325, t4477, t9292);
    (t50893, t50901, t50941, t50943, t51042, t51083, t51100, t51104, t51170, t51203, t51211, t51213)
}
