//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 811/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk811(t7463: f64, t8577: f64, t7469: f64, t7484: f64, t7450: f64, t1609: f64, t1986: f64, t7720: f64, t1212: f64, t1970: f64, t1971: f64, t209: f64, t511: f64, t558: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38389 = t8577 * t7463;
    let t38391 = t8577 * t7469;
    let t38393 = t8577 * t7484;
    let t38395 = t8577 * t7450;
    let t38397 = t1986 * t1609;
    let t38398 = t7720 * t38397;
    let t38404 = t1970 * t1971 * t511 * t558 * t1212 * t209;
    (t38389, t38391, t38393, t38395, t38398, t38404)
}
