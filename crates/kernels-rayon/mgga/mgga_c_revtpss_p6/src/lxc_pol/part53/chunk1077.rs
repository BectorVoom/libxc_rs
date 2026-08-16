//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1077/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1077(t34381: f64, t34422: f64, t34438: f64, t34466: f64, t3: f64, t2042: f64, t8245: f64, t2170: f64, t7950: f64, t7953: f64, t1918: f64, t33996: f64, t33998: f64, t34000: f64, t34003: f64, t34006: f64, t34009: f64, t34011: f64, t34014: f64, t573: f64, t8616: f64, t8771: f64, param_d: f64) -> (f64, f64, f64, f64) {
    let t34468 = t34381 + t34422 + t34438 + t34466;
    let t34469 = t3 * t34468;
    let t34477 = param_d * t34468;
    let t34481 = t8245 * t2042;
    let t34483 = t2170 * t7950;
    let t34485 = t2170 * t7953;
    let t34490 = 3.0_f64 * t1918 * t8771 + t34477 * t573 + 3.0_f64 * t33996 + 6.0_f64 * t33998 + 3.0_f64 * t34000 + t34003 + t34006 + t34009 + t34011 + t34014 + 3.0_f64 * t34481 + 6.0_f64 * t34483 + 3.0_f64 * t34485 + t8616;
    (t34468, t34469, t34477, t34490)
}
