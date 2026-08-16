//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 917/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk917(t76538: f64, t1550: f64, t7778: f64, t8975: f64, t15081: f64, t68613: f64, t2416: f64, t7349: f64, t28317: f64, t3157: f64, t73688: f64, t73701: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t76539 = 0.15965655602485078085e0_f64 * t76538;
    let t76541 = t1550 * t7778 * t8975;
    let t76542 = 0.15965655602485078085e0_f64 * t76541;
    let t76545 = t68613 * t15081;
    let t76547 = t7349 * t2416;
    let t76550 = t28317 * t3157;
    let t76604 = 0.5959043985061697516e-4_f64 * t73688;
    let t76607 = 0.2627895913935205078e-5_f64 * t73701;
    (t76539, t76542, t76545, t76547, t76550, t76604, t76607)
}
