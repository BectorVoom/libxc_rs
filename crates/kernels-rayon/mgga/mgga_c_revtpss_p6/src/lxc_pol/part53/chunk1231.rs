//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1231/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1231(t129467: f64, t1936: f64, t129470: f64, t34446: f64, t7002: f64, t27060: f64, t7741: f64, t29432: f64, t28042: f64, t7586: f64, t104115: f64, t111734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t129478 = t129467 * t1936;
    let t129479 = t129470 * t1936;
    let t129480 = t34446 * t7002;
    let t129481 = t27060 * t7741;
    let t129482 = t29432 * t7741;
    let t129483 = t7586 * t28042;
    let t129488 = t104115 * t1936;
    let t129489 = t111734 * t1936;
    (t129478, t129479, t129480, t129481, t129482, t129483, t129488, t129489)
}
