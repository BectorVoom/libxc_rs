//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1912;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta566(t4292: f64, t648: f64, t1907: f64, t4144: f64, t3829: f64, t13514: f64, t94: f64, t4135: f64, t13716: f64, t1450: f64, t28166: f64, t7234: f64, t8995: f64, t14468: f64, t30: f64, t2: f64, t2411: f64, t580: f64, t890: f64, t892: f64, t775: f64, t1583: f64, t2430: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98487, t98496, t98519, t98535, t98550, t98564, t98579) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1912(t4292, t648, t1907, t4144, t3829, t13514, t94, t4135, t13716, t1450, t28166, t7234);
        let (t98588, t98627, t98633, t98648, t98651) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1913(t7234, t8995, t14468, t30, t2, t2411, t580, t890, t892, t775, t1583, t2430);
    (t98487, t98496, t98519, t98535, t98550, t98564, t98579, t98588, t98627, t98633, t98648, t98651)
}
