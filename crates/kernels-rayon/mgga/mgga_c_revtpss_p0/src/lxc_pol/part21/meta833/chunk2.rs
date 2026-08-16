//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3117/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3117(t17547: f64, t3704: f64, t17609: f64, t12901: f64, t17525: f64, t1261: f64, t17551: f64, t3172: f64, t3588: f64, t5333: f64, t3625: f64, t44250: f64, t5406: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57314 = t17547 * t3704;
    let t57316 = t17609 * t3704;
    let t57318 = t17525 * t12901;
    let t57321 = t1261 * t3172 * t17551;
    let t57325 = t5333 * t3588;
    let t57331 = t3625 * t44250 * t5406;
    (t57314, t57316, t57318, t57321, t57325, t57331)
}
