//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1268/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1268(t12167: f64, t15905: f64, t3057: f64, t380: f64, t3088: f64, t370: f64, t994: f64, t905: f64, t999: f64, t606: f64, t1045: f64, t11150: f64, t3181: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16081 = t12167 * t15905;
    let t16087 = t3057 * t380;
    let t16088 = t3088 * t370;
    let t16089 = t16087 * t16088;
    let t16094 = t994 * t380;
    let t16095 = t16094 * t16088;
    let t16101 = t999 * t905;
    let t16102 = t16101 * t606;
    let t16103 = t1045 * t16102;
    let t16199 = t3181 * t11150;
    (t16081, t16089, t16095, t16101, t16102, t16103, t16199)
}
