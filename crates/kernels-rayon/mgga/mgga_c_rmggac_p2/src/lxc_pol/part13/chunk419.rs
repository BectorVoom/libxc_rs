//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 419/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk419(t1015: f64, t422: f64, t13: f64, t145: f64, t3: f64, t154: f64, t265: f64, t952: f64, t951: f64, t243: f64, t483: f64, t242: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4124 = t1015 * t422;
    let t4129 = 1.0_f64 / t13 / t145 * t3 / 4.0_f64;
    let t4130 = t4129 * t154;
    let t4132 = t952 * t265;
    let t4133 = t951 * t4132;
    let t4135 = t243 * t483;
    let t4136 = t242 * t4135;
    (t4124, t4130, t4132, t4133, t4135, t4136)
}
