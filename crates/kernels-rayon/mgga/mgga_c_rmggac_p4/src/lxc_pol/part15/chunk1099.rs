//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1099/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1099(t38530: f64, t8432: f64, t8437: f64, t26287: f64, t46441: f64, t26283: f64, t46444: f64, t26291: f64, t46397: f64, t10112: f64, t2157: f64, t2868: f64, t8997: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47966 = t38530 * t8432;
    let t47968 = t38530 * t8437;
    let t47970 = t26287 * t46441;
    let t47972 = t26283 * t46444;
    let t47974 = t26291 * t46397;
    let t47976 = t10112 * t2157;
    let t47980 = t2868 * t8997;
    (t47966, t47968, t47970, t47972, t47974, t47976, t47980)
}
