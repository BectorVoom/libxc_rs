//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2203/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2203(t27833: f64, t7901: f64, t2014: f64, t28020: f64, t5542: f64, t1450: f64, t21969: f64, t7237: f64, t28167: f64, t35669: f64, t5627: f64, t29996: f64, t7235: f64) -> (f64, f64, f64, f64, f64) {
    let t109112 = 6.0_f64 * t27833 * t7901;
    let t109117 = 2.0_f64 * t2014 * t28020 * t5542;
    let t109118 = t1450 * t21969;
    let t109121 = 3.0_f64 * t2014 * t7237 * t109118;
    let t109124 = 12.0_f64 * t28167 * t35669 * t5627;
    let t109126 = 2.0_f64 * t7235 * t29996;
    (t109112, t109117, t109121, t109124, t109126)
}
