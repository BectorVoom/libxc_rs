//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2185/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2185(t12823: f64, t7468: f64, t26003: f64, t4034: f64, t26351: f64, t6883: f64, t1992: f64, t26355: f64, t80650: f64, t22635: f64, t26354: f64, t3911: f64) -> (f64, f64, f64, f64, f64) {
    let t90454 = 2.0_f64 * t12823 * t7468;
    let t90456 = 4.0_f64 * t4034 * t26003;
    let t90459 = t6883 * t26351;
    let t90460 = 0.38381794893125283518e-1_f64 * t90459;
    let t90462 = t1992 * t80650 * t26355;
    let t90466 = t1992 * t22635 * t26354 * t3911;
    (t90454, t90456, t90460, t90462, t90466)
}
