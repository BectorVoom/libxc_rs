//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1969;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1970;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta559(t211: f64, t9644: f64, t138: f64, t785: f64, t9302: f64, t2452: f64, t9720: f64, t11006: f64, t256: f64, t10115: f64, t251: f64, t2410: f64, t3335: f64, t11198: f64, t340: f64, t11119: f64, t384: f64, t11238: f64, t196: f64, t10308: f64, t599: f64, t90: f64, t29: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39643, t40270, t40688, t41077, t41117, t41153) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1969(t211, t9644, t138, t785, t9302, t2452, t9720, t11006, t256, t10115, t251, t2410);
        let (t41154, t41937, t42058, t42066, t42859, t45963, t45972) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1970(t41153, t3335, t11198, t340, t11119, t384, t11238, t196, t10308, t599, t90, t29);
    (t39643, t40270, t40688, t41077, t41117, t41154, t41937, t42058, t42066, t42859, t45963, t45972)
}
