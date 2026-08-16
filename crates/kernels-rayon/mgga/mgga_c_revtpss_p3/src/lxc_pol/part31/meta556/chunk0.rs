//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1963/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1963(t25082: f64, t30123: f64, t7732: f64, t7742: f64, t1936: f64, t6765: f64, t651: f64, t18245: f64, t1501: f64, t1518: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30125 = 6.0_f64 * t25082 * t30123;
    let t30127 = 4.0_f64 * t7732 * t7742;
    let t30128 = t6765 * t1936;
    let t30130 = 2.0_f64 * t651 * t30128;
    let t30137 = 2.0_f64 * t18245 * t1936;
    let t30138 = t1501 * t1518;
    (t30125, t30127, t30128, t30130, t30137, t30138)
}
