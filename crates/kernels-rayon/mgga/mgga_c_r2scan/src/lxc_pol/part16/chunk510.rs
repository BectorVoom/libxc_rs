//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 510/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk510(t2719: f64, t552: f64, t551: f64, t1632: f64, t938: f64, t549: f64, t910: f64, t566: f64, t378: f64, t5: f64, t966: f64, t750: f64, t963: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2720 = t552 * t2719;
    let t2721 = t551 * t2720;
    let t2726 = t1632 * t938;
    let t2727 = t551 * t2726;
    let t2728 = t549 * t2727;
    let t2730 = t1632 * t910;
    let t2731 = t551 * t2730;
    let t2732 = t566 * t2731;
    let t2736 = t5 * t378 * t966;
    let t2738 = t963 * t750;
    (t2720, t2721, t2726, t2727, t2728, t2730, t2731, t2732, t2736, t2738)
}
