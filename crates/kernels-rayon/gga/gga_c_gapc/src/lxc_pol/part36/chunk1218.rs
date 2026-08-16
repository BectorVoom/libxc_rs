//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1218/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1218(t11387: f64, t21053: f64, t21054: f64, t11365: f64, t5285: f64, t5703: f64, t1386: f64, t3663: f64, t3665: f64, t2981: f64, t34754: f64, t458: f64) -> (f64, f64, f64, f64) {
    let t35231 = t21053 * t11387 * t21054;
    let t35234 = t5285 * t11365 * t5703;
    let t35240 = t1386 * t3663 * t3665;
    let t35243 = t34754 * t2981 * t458;
    (t35231, t35234, t35240, t35243)
}
