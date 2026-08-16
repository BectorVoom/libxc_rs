//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1719/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1719(t4930: f64, t994: f64, t1678: f64, t3046: f64, t3057: f64, t379: f64, t1078: f64, t1651: f64, t342: f64, t1071: f64, t1647: f64, t378: f64, t4743: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16302 = t994 * t4930;
    let t16305 = t3046 * t1678;
    let t16312 = t3057 * t379;
    let t16313 = t1078 * t1651;
    let t16333 = t342 * t4930;
    let t16340 = t1647 * t1071;
    let t16362 = t4743 * t378;
    (t16302, t16305, t16312, t16313, t16333, t16340, t16362)
}
