//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 807/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk807(t7487: f64, t8346: f64, t2145: f64, t27: f64, t3118: f64, t570: f64, t2046: f64, t7297: f64, t8482: f64, t1341: f64, t535: f64, t638: f64, t7310: f64) -> (f64, f64, f64, f64) {
    let t38314 = t7487 * t8346;
    let t38318 = t2145 * t27 * t3118 * t570;
    let t38322 = t2046 * t7297 * t8482;
    let t38326 = t638 * t7310 * t535 * t1341;
    (t38314, t38318, t38322, t38326)
}
