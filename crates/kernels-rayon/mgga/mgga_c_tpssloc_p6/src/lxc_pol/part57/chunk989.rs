//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 989/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk989(t121349: f64, t1527: f64, t1888: f64, t23270: f64, t1880: f64, t214: f64, t225: f64, t258: f64, t29040: f64, t118578: f64, t118580: f64, t123566: f64, t123571: f64, t123572: f64, t126294: f64, t126298: f64, t126302: f64, t126306: f64, t126309: f64, t126312: f64, t126316: f64, t126320: f64) -> (f64, f64, f64) {
    let t127889 = t1888 * t23270 * t121349 * t1527;
    let t127896 = t1880 * t214 * t29040 * t225 * t258;
    let t127908 = -t126294 / 384.0_f64 - t126298 / 768.0_f64 + t126302 / 384.0_f64 - t126306 / 768.0_f64 - 0.16149102437656156341e-2_f64 * t126309 + t123566 + t126312 / 96.0_f64 + 0.22608743412718618878e-1_f64 * t118578 + 0.13565246047631171327e0_f64 * t118580 + t126316 / 768.0_f64 - 0.96894614625936938046e-2_f64 * t126320 + t123571 + t123572;
    (t127889, t127896, t127908)
}
