//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1062/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1062(t581: f64, t949: f64, t3753: f64, t2741: f64, t3748: f64, t8523: f64, t3950: f64, t837: f64, t2703: f64, t2724: f64, t3932: f64, t3931: f64) -> (f64, f64, f64, f64) {
    let t11592 = t949 * t581;
    let t11593 = t3753 * t11592;
    let t11594 = t2741 * t11593;
    let t11597 = t3748 * t11592;
    let t11598 = t8523 * t11597;
    let t11601 = t3950 * t837;
    let t11602 = t2741 * t11601;
    let t11607 = t2724 * t2703;
    let t11608 = t3932 * t11607;
    let t11609 = t3931 * t11608;
    (t11594, t11598, t11602, t11609)
}
