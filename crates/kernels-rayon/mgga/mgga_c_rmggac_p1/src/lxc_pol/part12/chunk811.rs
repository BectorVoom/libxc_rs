//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 811/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk811(t2046: f64, t7297: f64, t8482: f64, t1341: f64, t535: f64, t638: f64, t7310: f64, t5016: f64, t9005: f64, t1276: f64, t2338: f64, t639: f64) -> (f64, f64, f64, f64) {
    let t38322 = t2046 * t7297 * t8482;
    let t38326 = t638 * t7310 * t535 * t1341;
    let t38328 = t5016 * t9005;
    let t38336 = t638 * t639 * t2338 * t1276;
    (t38322, t38326, t38328, t38336)
}
