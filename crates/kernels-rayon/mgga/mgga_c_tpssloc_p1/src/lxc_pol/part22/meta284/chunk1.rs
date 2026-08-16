//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1434/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1434(t1471: f64, t32: f64, t4095: f64, t67: f64, t758: f64, t118: f64, t1474: f64) -> (f64, f64, f64, f64) {
    let t13115 = t32 * t1471;
    let t13119 = t4095 * t67;
    let t13121 = 0.36622894612013090108e-3_f64 * t13119 * t758;
    let t13123 = t1474 * t118;
    (t13115, t13119, t13121, t13123)
}
