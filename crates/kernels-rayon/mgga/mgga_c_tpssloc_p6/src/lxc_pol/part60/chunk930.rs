//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 930/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk930(t114814: f64, t23012: f64, t8548: f64, t23030: f64, t31319: f64, t2047: f64, t212: f64, t23171: f64, t6554: f64, t23228: f64, t8547: f64, t193: f64, t201: f64, t8565: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114815 = 0.26044789391763585244e-1_f64 * t114814;
    let t114864 = t23012 * t8548;
    let t114865 = 0.63969658155208805863e-1_f64 * t114864;
    let t114891 = t23030 * t31319;
    let t114892 = 0.26044789391763585244e-1_f64 * t114891;
    let t114932 = t23171 * t212 * t2047 * t6554;
    let t114933 = 0.82246703342411321824e-2_f64 * t114932;
    let t114943 = t23171 * t23228 * t8547;
    let t114944 = 0.82246703342411321824e-2_f64 * t114943;
    let t115009 = t193 * t201 * t8565;
    (t114815, t114865, t114892, t114933, t114944, t115009)
}
