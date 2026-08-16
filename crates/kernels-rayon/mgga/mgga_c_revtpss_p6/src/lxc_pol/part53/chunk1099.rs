//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1099/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1099(t120068: f64, t31746: f64, t786: f64, t7063: f64, t31809: f64, t31837: f64, t2439: f64, t785: f64, t8471: f64, t8473: f64, t25410: f64, t2801: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120070 = t786 * t120068 * t31746;
    let t120071 = 0.20077843028252776532e-3_f64 * t120070;
    let t120073 = t7063 * t120068 * t31746;
    let t120074 = 0.35698404904233436678e-3_f64 * t120073;
    let t120082 = t31809 * t31837;
    let t120088 = 0.4818682326780666368e-3_f64 * t2439 * t785 * t8471 * t8473;
    let t120090 = t786 * t8471 * t25410;
    let t120091 = t120090 * t2801;
    (t120071, t120074, t120082, t120088, t120090, t120091)
}
