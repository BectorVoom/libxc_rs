//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 200/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk200(t241: f64, t711: f64, t374: f64, t46: f64, t379: f64, t381: f64, t231: f64, t242: f64, t344: f64, t366: f64, t4: f64, t55: f64, t706: f64, t79: f64) -> (f64, f64, f64, f64, f64) {
    let t712 = 1.0_f64 / t241;
    let t713 = t711 * t712;
    let t719 = t46 * t374;
    let t720 = t379 * t381;
    let t724 = t231 * (0.53236443333333333332e-3_f64 * t4 * t79 * t242 + 1.0_f64 * t706 * t713 - t344 - t366 + 0.18311555036753159941e-3_f64 * t4 * t79 * t55 + 0.58482233974552040708e0_f64 * t719 * t720);
    (t712, t713, t719, t720, t724)
}
