//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1595/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1595(t43994: f64, t44007: f64, t448: f64, t300: f64, t1126: f64, t12226: f64, t12231: f64, t3382: f64, t3431: f64, t408: f64, t3385: f64, t12230: f64) -> (f64, f64, f64, f64, f64) {
    let t44009 = (t43994 + t44007) * t448;
    let t44011 = 0.19751673498613801407e-1_f64 * t300 * t44009;
    let t44012 = t1126 * t12226;
    let t44014 = 0.2069040516770936012e4_f64 * t44012 * t12231;
    let t44017 = t408 / t3431 / t3382;
    let t44018 = t3385 * t3385;
    let t44021 = 0.62071215503128080361e4_f64 * t44017 * t44018 * t12230;
    (t44009, t44011, t44014, t44018, t44021)
}
