//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2100/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2100(t21881: f64, t572: f64, t7330: f64, t1916: f64, t28271: f64, t28268: f64, t1459: f64, t30185: f64, t5883: f64, t6982: f64, t25082: f64, t86771: f64, t8717: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t105837 = 6.0_f64 * t572 * t7330 * t21881;
    let t105839 = 12.0_f64 * t1916 * t28271;
    let t105841 = 12.0_f64 * t1916 * t28268;
    let t105843 = 6.0_f64 * t1459 * t30185;
    let t105850 = t6982 * t5883;
    let t105859 = 3.0_f64 * t25082 * t8717 * t86771;
    (t105837, t105839, t105841, t105843, t105850, t105859)
}
