//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1247;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta366(t24232: f64, t3417: f64, t141: f64, t1145: f64, t24240: f64, t24248: f64, t24236: f64, t12296: f64, t16706: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24288, t24289, t24291, t24292, t24294, t24295, t24297, t24298, t24312) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1247(t24232, t3417, t141, t1145, t24240, t24248, t24236, t12296, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250);
    (t24288, t24289, t24291, t24292, t24294, t24295, t24297, t24298, t24312)
}
