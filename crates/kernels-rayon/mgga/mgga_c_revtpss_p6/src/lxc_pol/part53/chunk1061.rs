//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1061/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1061(t2035: f64, t33913: f64, t7898: f64, t8600: f64, t8596: f64, t1883: f64, t32195: f64, t5673: f64, t32194: f64, t1868: f64, t3936: f64, t32206: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33914 = t33913 * t2035;
    let t33916 = t7898 * t8600;
    let t33920 = t7898 * t8596;
    let t33922 = t5673 * t32195 * t1883;
    let t33923 = t32194 * t33922;
    let t33926 = t3936 * t32195 * t1868;
    let t33927 = t32206 * t33926;
    (t33914, t33916, t33920, t33922, t33923, t33926, t33927)
}
