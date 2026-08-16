//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1031/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1031(t3351: f64, t3352: f64, t511: f64, t6441: f64, t1971: f64, t6421: f64, t880: f64, t2144: f64, t45622: f64, t7720: f64, t9817: f64, t39277: f64, t9046: f64) -> (f64, f64, f64, f64, f64) {
    let t47520 = t3351 * t3352 * t511 * t6441;
    let t47524 = t3351 * t1971 * t880 * t6421;
    let t47528 = t3351 * t1971 * t2144 * t45622;
    let t47530 = t7720 * t9817;
    let t47532 = t39277 * t9046;
    (t47520, t47524, t47528, t47530, t47532)
}
