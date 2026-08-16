//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1051/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1051(t113: f64, t2526: f64, t808: f64, t153: f64, t160: f64, t2150: f64, t2605: f64, t2484: f64, t7624: f64, t7627: f64, t815: f64, t2491: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26533 = t113 * t2526;
    let t26534 = t808 * t26533;
    let t26536 = t153 * t160;
    let t26538 = t2605 * t2150;
    let t26540 = t2484 * t7624;
    let t26542 = t815 * t7627;
    let t26544 = t2150 * t2491;
    (t26533, t26534, t26536, t26538, t26540, t26542, t26544)
}
