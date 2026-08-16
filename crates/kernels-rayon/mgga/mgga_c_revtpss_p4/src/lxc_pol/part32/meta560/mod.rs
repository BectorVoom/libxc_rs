//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1879;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta560(t1468: f64, t2411: f64, t30: f64, t41154: f64, t14495: f64, t689: f64, t14587: f64, t27312: f64, t1568: f64, t7063: f64, t25410: f64, t25304: f64, t27212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98658, t98785, t98801, t98809, t98815, t98848, t98849, t98867) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1879(t1468, t2411, t30, t41154, t14495, t689, t14587, t27312, t1568, t7063, t25410, t25304, t27212);
    (t98658, t98785, t98801, t98809, t98815, t98848, t98849, t98867)
}
