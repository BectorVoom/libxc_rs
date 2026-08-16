//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 802/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk802(t1971: f64, t3351: f64, t4617: f64, t8941: f64, t1986: f64, t2400: f64, t7720: f64, t14125: f64, t14131: f64, t8446: f64, t3352: f64, t70423: f64, t9146: f64) -> (f64, f64, f64, f64) {
    let t74456 = t3351 * t1971 * t4617 * t8941;
    let t74458 = t1986 * t2400;
    let t74459 = t7720 * t74458;
    let t74462 = t14131 * t14125 * t8446;
    let t74465 = t70423 * t3352 * t9146;
    (t74456, t74459, t74462, t74465)
}
