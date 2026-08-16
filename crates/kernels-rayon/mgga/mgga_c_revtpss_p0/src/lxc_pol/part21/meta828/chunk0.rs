//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3084/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3084(t12627: f64, t1811: f64, t12657: f64, t1208: f64, t17330: f64, t487: f64, t1269: f64, t17306: f64, t1209: f64, t1270: f64, t3566: f64, t56183: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t56393 = t12627 * t1811;
    let t56396 = t12657 * t1811;
    let t56412 = t17330 * t1208;
    let t56413 = t56412 * t487;
    let t56416 = t17306 * t1269;
    let t56419 = t1209 * t1270;
    let t56432 = t3566 * t1270;
    let t56447 = 0.22222222222222222222e-1_f64 * t56183;
    (t56393, t56396, t56412, t56413, t56416, t56419, t56432, t56447)
}
