//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 802/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk802(t1986: f64, t326: f64, t495: f64, t559: f64, t2001: f64, t305: f64, t498: f64, t552: f64, t1596: f64, t1594: f64, t2410: f64, t7228: f64) -> (f64, f64, f64, f64, f64) {
    let t39141 = t1986 * t326 * t559 * t495;
    let t39171 = t2001 * t305 * t552 * t498;
    let t39183 = t1986 * t1596;
    let t39199 = t1986 * t1594;
    let t39207 = t2410 * t7228;
    (t39141, t39171, t39183, t39199, t39207)
}
