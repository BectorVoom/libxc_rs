//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1894/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1894(t1932: f64, t3120: f64, t360: f64, t1629: f64, t1625: f64, t3040: f64, t3201: f64, t6739: f64, t14526: f64, t383: f64, t1022: f64, t4657: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14622 = t1932 * t3120 * t360;
    let t14623 = t1629 * t14622;
    let t14626 = t1625 * t3040;
    let t14627 = t14626 * t3201;
    let t14630 = t6739 * t3040 * t360;
    let t14631 = t1629 * t14630;
    let t14640 = t383 * t14526;
    let t14644 = t4657 * t1022;
    (t14622, t14623, t14626, t14627, t14630, t14631, t14640, t14644)
}
