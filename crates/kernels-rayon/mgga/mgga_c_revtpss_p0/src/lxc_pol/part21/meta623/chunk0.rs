//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2382/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2382(t10752: f64, t10905: f64, t2783: f64, t9801: f64, t10745: f64, t2735: f64, t4503: f64, t10728: f64, t808: f64, t10680: f64, t2710: f64, t2713: f64) -> (f64, f64, f64, f64, f64) {
    let t40511 = t10905 * t10752;
    let t40517 = t9801 * t2783;
    let t40518 = t40517 * t10745;
    let t40521 = t2735 * t4503;
    let t40523 = t40521 * t808 * t10728;
    let t40526 = t2710 * t2713 * t10680;
    (t40511, t40517, t40518, t40523, t40526)
}
