//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2105/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2105(t14046: f64, t25986: f64, t2661: f64, t14050: f64, t13850: f64, t2482: f64, t25981: f64, t814: f64, t13962: f64, t26028: f64, t14020: f64, t7252: f64) -> (f64, f64, f64, f64, f64) {
    let t98235 = t2661 * t25986 * t14046;
    let t98236 = 0.11433071498151929859e-3_f64 * t98235;
    let t98238 = t2661 * t25986 * t14050;
    let t98239 = 0.28582678745379824648e-4_f64 * t98238;
    let t98243 = t2482 * t25981 * t814 * t13850;
    let t98244 = 0.10164000561857065645e-3_f64 * t98243;
    let t98245 = t26028 * t13962;
    let t98253 = t7252 * t14020;
    (t98236, t98239, t98244, t98245, t98253)
}
