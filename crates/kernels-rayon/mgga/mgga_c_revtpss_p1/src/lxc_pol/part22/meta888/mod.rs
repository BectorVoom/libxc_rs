//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta888 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3076;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta888(t11506: f64, t1626: f64, t1609: f64, t2924: f64, t2942: f64, t4644: f64, t11408: f64, t1614: f64, t2967: f64, t11449: f64, t15373: f64, t945: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t52642, t52645, t52809, t52812, t52820, t52825, t52830) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3076(t11506, t1626, t1609, t2924, t2942, t4644, t11408, t1614, t2967, t11449, t15373, t945);
    (t52642, t52645, t52809, t52812, t52820, t52825, t52830)
}
