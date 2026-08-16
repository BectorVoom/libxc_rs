//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 446/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk446(t4709: f64, t4746: f64, t1311: f64, t1314: f64, t1320: f64, t20: f64, t253: f64, t1327: f64, t28: f64, t1330: f64, t2044: f64, t1318: f64, t40: f64) -> (f64, f64, f64, f64, f64) {
    let t4747 = t4746 * t4709;
    let t4750 = t1311 * t1314;
    let t4753 = t20 * t1320;
    let t4754 = t253 * t4753;
    let t4755 = t28 * t1327;
    let t4757 = t2044 * t4755 * t1330;
    let t4762 = t1318 * t40;
    (t4747, t4750, t4754, t4757, t4762)
}
