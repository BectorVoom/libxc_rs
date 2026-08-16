//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1189/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1189(t2605: f64, t7627: f64, t2491: f64, t2593: f64, t740: f64, t2588: f64, t26533: f64, t2526: f64, t808: f64, t9053: f64, t2150: f64, t755: f64, t8750: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91801 = t2605 * t7627;
    let t91804 = t2593 * t740 * t2491;
    let t91806 = t2588 * t26533;
    let t91809 = t808 * t740 * t2526;
    let t91811 = t2593 * t9053;
    let t91814 = t755 * t2150 * t8750;
    (t91801, t91804, t91806, t91809, t91811, t91814)
}
