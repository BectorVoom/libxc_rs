//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 499/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk499(t5980: f64, t6280: f64, t6293: f64, t6301: f64, t109: f64, t1368: f64, t1652: f64, t1602: f64, t558: f64, t1614: f64, t552: f64, t559: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6303 = t5980 + t6280 + t6293 + t6301;
    let t6304 = t6303 * t109;
    let t6308 = t1368 * t1652;
    let t6311 = t1602 * t558;
    let t6315 = t552 * t1614;
    let t6327 = t559 * t1614;
    (t6303, t6304, t6308, t6311, t6315, t6327)
}
