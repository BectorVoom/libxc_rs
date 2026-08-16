//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1581;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1582;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta537(t22841: f64, t2661: f64, t74026: f64, t9934: f64, t14100: f64, t22399: f64, t5722: f64, t74835: f64, t1357: f64, t23043: f64, t689: f64, t1364: f64, t22965: f64, t786: f64, t22975: f64, t5599: f64, t6896: f64, t6919: f64, t5741: f64, t74892: f64, t22315: f64, t48084: f64, t22858: f64, t47372: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86274, t86285, t86296, t86300, t86311) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1581(t22841, t2661, t74026, t9934, t14100, t22399, t5722, t74835, t1357, t23043, t689, t1364, t22965, t786);
        let (t86314, t86317, t86346, t86350, t86354, t86358) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1582(t1357, t22975, t689, t5599, t6896, t6919, t5741, t74892, t22315, t48084, t22858, t47372, t686, t72);
    (t86274, t86285, t86296, t86300, t86311, t86314, t86317, t86346, t86350, t86354, t86358)
}
