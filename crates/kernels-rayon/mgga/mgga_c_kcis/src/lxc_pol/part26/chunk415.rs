//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 415/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk415(t154: f64, t2491: f64, t2593: f64, t774: f64, t812: f64, t808: f64, t2526: f64, t153: f64, t2150: f64, t137: f64, t2479: f64, t161: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2594 = t154 * t2491;
    let t2595 = t2593 * t2594;
    let t2597 = t812 * t774;
    let t2598 = t808 * t2597;
    let t2600 = t154 * t2526;
    let t2601 = t808 * t2600;
    let t2603 = t153 * t2150;
    let t2605 = t2479 * t137;
    let t2606 = t2605 * t161;
    (t2594, t2595, t2597, t2598, t2600, t2601, t2603, t2605, t2606)
}
