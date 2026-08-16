//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 898/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk898(t15035: f64, t2019: f64, t2020: f64, t2010: f64, t2012: f64, t8876: f64, t69626: f64, t8571: f64, t15361: f64, t352: f64, t14236: f64, t14237: f64, t2078: f64) -> (f64, f64, f64, f64) {
    let t76127 = t2019 * t2020 * t15035;
    let t76130 = t2010 * t2012 * t8876;
    let t76132 = t8571 * t69626;
    let t76134 = t15361 * t352;
    let t76137 = t14236 * t14237 * t2078 * t76134;
    (t76127, t76130, t76132, t76137)
}
