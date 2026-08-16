//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1507/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1507(t16087: f64, t16088: f64, t4757: f64, t906: f64, t3092: f64, t380: f64, t994: f64, t606: f64, t999: f64, t4578: f64, t905: f64, t1045: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16089 = t16087 * t16088;
    let t16090 = t4757 * t906;
    let t16091 = t3092 * t16090;
    let t16094 = t994 * t380;
    let t16095 = t16094 * t16088;
    let t16096 = t606 * t999;
    let t16097 = t4578 * t16096;
    let t16098 = t3092 * t16097;
    let t16101 = t999 * t905;
    let t16102 = t16101 * t606;
    let t16103 = t1045 * t16102;
    (t16089, t16091, t16095, t16096, t16098, t16103)
}
