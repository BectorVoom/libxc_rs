//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1090/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1090(t34132: f64, t34166: f64, t118: f64, t13272: f64, t8619: f64, t1497: f64, t8621: f64, t8622: f64, t1469: f64, t32591: f64, t8442: f64, t1493: f64, t32600: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34167 = t34132 + t34166;
    let t34168 = t118 * t34167;
    let t34169 = t13272 * t8619;
    let t34173 = t8621 * t8622 * t1497;
    let t34176 = t32591 * t1469;
    let t34177 = t8442 * t34176;
    let t34181 = t8621 * t32600 * t1493;
    (t34167, t34168, t34169, t34173, t34177, t34181)
}
