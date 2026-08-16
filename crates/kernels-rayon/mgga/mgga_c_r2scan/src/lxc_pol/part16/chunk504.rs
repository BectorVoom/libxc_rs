//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 504/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk504(t560: f64, t910: f64, t551: f64, t552: f64, t2526: f64, t133: f64, t978: f64, t255: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2654 = t910 * t560;
    let t2656 = t551 * t552 * t2654;
    let t2661 = t552 * t2526;
    let t2662 = t551 * t2661;
    let t2665 = t133 * t978;
    let t2666 = t2665 * t255;
    (t2654, t2656, t2661, t2662, t2665, t2666)
}
