//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta557(t26024: f64, t5661: f64, t14054: f64, t25986: f64, t2661: f64, t14046: f64, t14050: f64, t13850: f64, t2482: f64, t25981: f64, t814: f64, t13829: f64, t94550: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t98226, t98229, t98235, t98238, t98243, t98258) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1876(t26024, t5661, t14054, t25986, t2661, t14046, t14050, t13850, t2482, t25981, t814, t13829, t94550);
    (t98226, t98229, t98235, t98238, t98243, t98258)
}
