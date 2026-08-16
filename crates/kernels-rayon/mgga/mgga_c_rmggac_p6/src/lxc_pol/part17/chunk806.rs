//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 806/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk806(t39338: f64, t2338: f64, t7323: f64, t7324: f64, t1327: f64, t574: f64, t640: f64, t34750: f64, t34755: f64, t577: f64, t2339: f64, t638: f64, t7184: f64) -> (f64, f64, f64, f64, f64) {
    let t39339 = 0.30487649791575028314e-3_f64 * t39338;
    let t39341 = t7323 * t2338 * t7324;
    let t39345 = t7323 * t640 * t574 * t1327;
    let t39370 = t34755 * t577 * t34750;
    let t39388 = t638 * t7184 * t2339;
    (t39339, t39341, t39345, t39370, t39388)
}
