//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 812/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk812(t2201: f64, t785: f64, t8266: f64, t1616: f64, t910: f64, t2207: f64, t2837: f64, t783: f64, t2842: f64, t5100: f64, t2832: f64, t784: f64) -> (f64, f64, f64, f64, f64) {
    let t8268 = t2201 * t785 * t8266;
    let t8270 = t1616 * t910;
    let t8272 = t2207 * t785 * t8270;
    let t8275 = t783 * t2837 * t1616;
    let t8277 = t5100 * t2842;
    let t8279 = t2832 * t784;
    (t8268, t8272, t8275, t8277, t8279)
}
