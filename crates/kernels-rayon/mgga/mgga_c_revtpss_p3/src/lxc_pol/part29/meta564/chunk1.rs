//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1909/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1909(t14050: f64, t25986: f64, t2661: f64, t13850: f64, t2482: f64, t25981: f64, t814: f64, t13962: f64, t26028: f64, t14020: f64, t7252: f64, t13829: f64, t94550: f64) -> (f64, f64, f64, f64, f64) {
    let t98238 = t2661 * t25986 * t14050;
    let t98243 = t2482 * t25981 * t814 * t13850;
    let t98245 = t26028 * t13962;
    let t98253 = t7252 * t14020;
    let t98258 = t2661 * t94550 * t13829;
    (t98238, t98243, t98245, t98253, t98258)
}
