//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta486(t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64, t12296: f64, t12297: f64, t16706: f64, t16915: f64, t16916: f64, t16917: f64) -> (f64, f64) {
        let (t20322, t20337) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1956(t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320, t12296, t12297, t16706, t16915, t16916, t16917);
    (t20322, t20337)
}
