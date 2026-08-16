//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1251/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1251(t25240: f64, t2710: f64, t4371: f64, t10744: f64, t4353: f64, t7028: f64, t4430: f64, t93034: f64, t1565: f64, t93066: f64, t4349: f64, t93072: f64) -> (f64, f64, f64, f64, f64) {
    let t98976 = t2710 * t25240 * t4371;
    let t98979 = t10744 * t7028 * t4353;
    let t99002 = t93034 * t4430;
    let t99009 = t93066 * t1565;
    let t99013 = t93072 * t4349;
    (t98976, t98979, t99002, t99009, t99013)
}
