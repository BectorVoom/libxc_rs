//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1125/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1125(t1111: f64, t12445: f64, t1571: f64, t3087: f64, t3074: f64, t4231: f64, t3931: f64, t3081: f64, t4245: f64, t461: f64, t1114: f64, t11453: f64, t4252: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12446 = t1111 * t12445;
    let t12448 = t1571 * t3087;
    let t12450 = t4231 * t3074;
    let t12451 = t3931 * t12450;
    let t12454 = t4231 * t3081;
    let t12455 = t3931 * t12454;
    let t12458 = t461 * t4245;
    let t12459 = t12458 * t1114;
    let t12460 = t3931 * t12459;
    let t12463 = t11453 * t4252;
    (t12446, t12448, t12451, t12455, t12458, t12460, t12463)
}
