//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 563/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk563(t1856: f64, t72: f64, t757: f64, t539: f64, t73: f64, t1412: f64, t1868: f64, t1883: f64, t221: f64, t4019: f64, t4018: f64, t241: f64, t4000: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5635 = t1856 * t72;
    let t5636 = t5635 * t757;
    let t5650 = t539 * t73;
    let t5651 = t1412 * t1868;
    let t5665 = t4019 * t221 * t1883;
    let t5666 = t4018 * t5665;
    let t5671 = t820 * t4000 * t241;
    (t5635, t5636, t5650, t5651, t5665, t5666, t5671)
}
