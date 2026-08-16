//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 940/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk940(t4343: f64, t828: f64, t855: f64, t1544: f64, t221: f64, t2675: f64, t2674: f64, t1558: f64, t243: f64) -> (f64, f64, f64, f64) {
    let t4345 = t855 * t828 * t4343;
    let t4349 = t2675 * t221 * t1544;
    let t4350 = t2674 * t4349;
    let t4352 = t243 * t1558;
    (t4345, t4349, t4350, t4352)
}
