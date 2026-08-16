//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 709/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk709(t26078: f64, t3046: f64, t3056: f64, t71: f64, t7311: f64, t14063: f64, t2190: f64, t3151: f64, t1327: f64, t640: f64, t668: f64, t7323: f64) -> (f64, f64, f64) {
    let t69819 = t3056 * t3046 * t26078 * t71 * t7311;
    let t69827 = t2190 * t14063 * t3151;
    let t69828 = 0.29085809927086856922e-4_f64 * t69827;
    let t69832 = t7323 * t640 * t668 * t1327;
    (t69819, t69828, t69832)
}
