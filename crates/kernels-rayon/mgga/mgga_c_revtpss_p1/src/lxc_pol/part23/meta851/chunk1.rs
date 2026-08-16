//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2736/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2736(t17241: f64, t5373: f64, t17654: f64, t20766: f64, t56756: f64, t17693: f64, t20937: f64, t1222: f64, t17240: f64, t20310: f64, t20306: f64, t12772: f64, t21156: f64, t3625: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t71320 = t5373 * t17241;
    let t71329 = t17654 * t56756 * t20766;
    let t71341 = t17693 * t56756 * t20937;
    let t71373 = t1222 * t17240 * t20310;
    let t71377 = t1222 * t17240 * t20306;
    let t71400 = t3625 * t12772 * t21156;
    (t71320, t71329, t71341, t71373, t71377, t71400)
}
