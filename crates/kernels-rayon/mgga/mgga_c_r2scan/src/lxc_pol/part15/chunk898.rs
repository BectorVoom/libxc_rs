//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 898/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk898(t2214: f64, t2698: f64, t514: f64, t1616: f64, t938: f64, t2201: f64, t785: f64, t910: f64, t2207: f64, t2837: f64, t783: f64, t2842: f64, t5100: f64) -> (f64, f64, f64, f64, f64) {
    let t8263 = t2214 * t2698;
    let t8265 = 0.19514881078765566037e-1_f64 * t514 * t8263;
    let t8266 = t1616 * t938;
    let t8268 = t2201 * t785 * t8266;
    let t8270 = t1616 * t910;
    let t8272 = t2207 * t785 * t8270;
    let t8275 = t783 * t2837 * t1616;
    let t8277 = t5100 * t2842;
    (t8265, t8268, t8272, t8275, t8277)
}
