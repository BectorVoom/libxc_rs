//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1319/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1319(t831: f64, t9674: f64, t2639: f64, t2681: f64, t116: f64, t126: f64, t136: f64, t16: f64, t2386: f64, t625: f64, t2385: f64, t686: f64, t781: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9675 = t9674 * t831;
    let t9679 = t2639 * t2681;
    let t9688 = 1.0_f64 / t126 / t136 * t116 / 4.0_f64;
    let t9689 = t9688 * t16;
    let t9691 = t2386 * t625;
    let t9692 = t2385 * t9691;
    let t9694 = t686 * t781;
    (t9675, t9679, t9689, t9691, t9692, t9694)
}
