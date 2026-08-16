//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 383/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk383(t131: f64, t270: f64, t31: f64, t1179: f64, t214: f64, t132: f64, t1338: f64, t668: f64, t934: f64, t4179: f64, t6: f64, t211: f64, t483: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7352 = t131 * t270;
    let t7353 = t7352 * t31;
    let t7363 = t1179 * t214;
    let t7385 = t132 * t1338;
    let t7399 = t934 * t668;
    let t7417 = t6 * t4179;
    let t7427 = t211 * t483;
    (t7352, t7353, t7363, t7385, t7399, t7417, t7427)
}
