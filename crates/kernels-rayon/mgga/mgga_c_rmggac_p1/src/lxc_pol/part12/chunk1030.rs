//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1030/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1030(t874: f64, t8794: f64, t352: f64, t25820: f64, t38977: f64, t27101: f64, t38980: f64, t25854: f64, t38983: f64, t36058: f64, t6444: f64, t9005: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41483 = t874 * t8794;
    let t41484 = t41483 * t352;
    let t41488 = t25820 * t38977;
    let t41490 = t27101 * t38980;
    let t41492 = t25854 * t38983;
    let t41500 = 0.2927036860455597649e0_f64 * t36058;
    let t41501 = t6444 * t9005;
    (t41484, t41488, t41490, t41492, t41500, t41501)
}
