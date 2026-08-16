//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1481/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1481(t1390: f64, t828: f64, t9891: f64, t3926: f64, t3930: f64, t1398: f64, t3923: f64) -> (f64, f64, f64) {
    let t9893 = t1390 * t828 * t9891;
    let t9896 = t3930 * t3926;
    let t9898 = t3923 * t1398;
    (t9893, t9896, t9898)
}
