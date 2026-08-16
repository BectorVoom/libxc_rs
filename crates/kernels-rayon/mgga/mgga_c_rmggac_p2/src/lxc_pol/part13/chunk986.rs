//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 986/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk986(t2085: f64, t8339: f64, t1162: f64, t1979: f64, t1982: f64, t201: f64, t589: f64, t1692: f64, t2046: f64, t2050: f64, t31: f64, t2604: f64, t8413: f64) -> (f64, f64, f64, f64) {
    let t41656 = t8339 * t2085;
    let t41663 = t589 * t1162 * t201 * t1979 * t1982;
    let t41667 = t2046 * t2050 * t1692 * t31;
    let t41669 = t2604 * t8413;
    (t41656, t41663, t41667, t41669)
}
