//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 552/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk552(t2148: f64, t2608: f64, t2147: f64, t481: f64, t938: f64, t551: f64, t552: f64, t1600: f64, t928: f64, t1632: f64, t921: f64, t574: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2609 = t2148 * t2608;
    let t2610 = t2147 * t2609;
    let t2612 = t938 * t481;
    let t2614 = t551 * t552 * t2612;
    let t2617 = t1600 * t928;
    let t2620 = t551 * t1632 * t921;
    let t2621 = t574 * t2620;
    (t2609, t2610, t2612, t2614, t2617, t2620, t2621)
}
