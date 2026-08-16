//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2251/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2251(t10298: f64, t1470: f64, t2242: f64, t4181: f64, t4187: f64, t28108: f64, t644: f64, t77: f64, t2315: f64, t7705: f64, t28150: f64, t6973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t101187 = t10298 * t1470;
    let t101190 = t2242 * t4181;
    let t101193 = t2242 * t4187;
    let t101200 = t77 * t28108 * t644;
    let t101204 = t77 * t7705 * t2315;
    let t101211 = t6973 * t28150;
    (t101187, t101190, t101193, t101200, t101204, t101211)
}
