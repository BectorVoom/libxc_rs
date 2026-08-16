//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1937/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1937(t1497: f64, t2311: f64, t77: f64, t4241: f64, t640: f64, t13420: f64, t84: f64, t10298: f64, t1470: f64, t2242: f64, t4181: f64, t4187: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t101172 = t77 * t2311 * t1497;
    let t101176 = t77 * t640 * t4241;
    let t101182 = t77 * t84 * t13420;
    let t101187 = t10298 * t1470;
    let t101190 = t2242 * t4181;
    let t101193 = t2242 * t4187;
    (t101172, t101176, t101182, t101187, t101190, t101193)
}
