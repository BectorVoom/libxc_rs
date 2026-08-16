//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 513/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk513(t1916: f64, t1918: f64, t572: f64, t573: f64, t198: f64, t207: f64, t159: f64, t215: f64, t10: f64, t17: f64, t576: f64, t580: f64) -> (f64, f64, f64, f64, f64) {
    let t1921 = t1916 * t573 + 3.0_f64 * t1918 * t572;
    let t1940 = t198 * t207;
    let t1941 = t215 * t159;
    let t2219 = 2.0_f64 * t10 * t17;
    let t2221 = 8.0_f64 * t576 * t580;
    (t1921, t1940, t1941, t2219, t2221)
}
