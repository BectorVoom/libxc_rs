//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1634/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1634(t16237: f64, t380: f64, t15780: f64, t4998: f64, t15893: f64, t3304: f64, t3318: f64, t1086: f64, t1678: f64, t994: f64, t12166: f64, t378: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16529 = t380 * t16237;
    let t16534 = t15780 * t4998;
    let t16537 = t15893 * t3304;
    let t16540 = t15893 * t3318;
    let t16543 = t1086 * t1678;
    let t16544 = t994 * t16543;
    let t16551 = t12166 * t378;
    (t16529, t16534, t16537, t16540, t16544, t16551)
}
