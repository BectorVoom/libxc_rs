//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1037/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1037(t11822: f64, t22688: f64, t1012: f64, t11827: f64, t23481: f64, t247: f64, t3182: f64, t1592: f64, t19675: f64, t1042: f64, t11660: f64, t1469: f64) -> (f64, f64, f64, f64, f64) {
    let t23873 = t11822 * t22688;
    let t23874 = t1012 * t23873;
    let t23877 = t11827 * t22688;
    let t23878 = t1012 * t23877;
    let t23886 = t247 * t3182 * t23481;
    let t23891 = t19675 * t1592;
    let t23892 = t1042 * t23891;
    let t23898 = t11660 * t1469;
    (t23874, t23878, t23886, t23892, t23898)
}
