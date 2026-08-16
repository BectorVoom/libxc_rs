//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 813/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk813(t783: f64, t788: f64, t8279: f64, t2547: f64, t6118: f64, t1248: f64, t295: f64, t1256: f64, t305: f64, t2376: f64, t818: f64, t1004: f64, t1275: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8282 = 0.11643651550782197811e-1_f64 * t783 * t8279 * t788;
    let t8284 = 0.25610080155860322884e0_f64 * t6118 * t2547;
    let t8319 = t295 * t1248;
    let t8340 = t305 * t1256;
    let t8355 = t2376 * t818;
    let t8358 = t1004 * t1275;
    (t8282, t8284, t8319, t8340, t8355, t8358)
}
