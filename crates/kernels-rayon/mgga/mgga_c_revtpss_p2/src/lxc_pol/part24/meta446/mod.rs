//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta446 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta446(t5571: f64, t9323: f64, t5635: f64, t9586: f64, t9425: f64, t9318: f64, t1857: f64, t9342: f64, t9855: f64, t9410: f64, t9413: f64, t9372: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48269, t48280, t48282, t48285, t48287, t48290, t48292, t48294, t48297) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1407(t5571, t9323, t5635, t9586, t9425, t9318, t1857, t9342, t9855, t9410, t9413, t9372);
    (t48269, t48280, t48282, t48285, t48287, t48290, t48292, t48294, t48297)
}
