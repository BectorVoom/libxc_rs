//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1613/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1613(t372: f64, t6163: f64, t479: f64, t471: f64, t248: f64, t3521: f64, t5979: f64, t1227: f64, t1009: f64, t6150: f64, t1011: f64, t1212: f64) -> (f64, f64, f64, f64) {
    let t19031 = t6163 * t372;
    let t19032 = t479 * t19031;
    let t19033 = t471 * t19032;
    let t19040 = t248 * t3521 * t5979;
    let t19041 = t1227 * t19040;
    let t19045 = t6150 * t1009;
    let t19046 = t19045 * t1011;
    let t19047 = t19046 * t1212;
    (t19033, t19041, t19045, t19047)
}
