//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1876/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1876(t26024: f64, t5661: f64, t14054: f64, t25986: f64, t2661: f64, t14046: f64, t14050: f64, t13850: f64, t2482: f64, t25981: f64, t814: f64, t13829: f64, t94550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98226 = t26024 * t5661;
    let t98229 = t2661 * t25986 * t14054;
    let t98235 = t2661 * t25986 * t14046;
    let t98238 = t2661 * t25986 * t14050;
    let t98243 = t2482 * t25981 * t814 * t13850;
    let t98258 = t2661 * t94550 * t13829;
    (t98226, t98229, t98235, t98238, t98243, t98258)
}
