//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2708/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2708(t1160: f64, t20597: f64, t20447: f64, t3435: f64, t3565: f64, t6563: f64, t225: f64, t1261: f64, t12879: f64, t247: f64, t6429: f64, t11262: f64, t1247: f64, t6624: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69565 = t20597 * t1160;
    let t69591 = t20447 * t3435;
    let t69636 = t6563 * t3565;
    let t69637 = t69636 * t225;
    let t69661 = t1261 * t247 * t12879 * t6429;
    let t69668 = t1247 * t11262 * t6624;
    (t69565, t69591, t69636, t69637, t69661, t69668)
}
