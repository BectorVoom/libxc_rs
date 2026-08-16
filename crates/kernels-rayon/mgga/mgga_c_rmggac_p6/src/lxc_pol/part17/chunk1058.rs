//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1058/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1058(t46005: f64, t739: f64, t7577: f64, t2305: f64, t39284: f64, t1550: f64, t2060: f64, t30453: f64, t30311: f64, t903: f64, t2604: f64, t9957: f64) -> (f64, f64, f64, f64, f64) {
    let t47371 = t739 * t7577 * t46005;
    let t47375 = t39284 * t2305;
    let t47378 = t1550 * t2060 * t30453;
    let t47381 = t903 * t2060 * t30311;
    let t47385 = t2604 * t9957;
    (t47371, t47375, t47378, t47381, t47385)
}
