//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1144/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1144(t11880: f64, t263: f64, t2938: f64, t826: f64, t31689: f64, t3363: f64, t3358: f64, t9673: f64, t37031: f64, t9650: f64, t11036: f64, t9653: f64) -> (f64, f64, f64, f64, f64) {
    let t42524 = t11880 * t263 * t2938 * t826;
    let t42526 = t31689 * t3363;
    let t42528 = t3358 * t9673;
    let t42530 = t37031 * t9650;
    let t42532 = t11036 * t9653;
    (t42524, t42526, t42528, t42530, t42532)
}
