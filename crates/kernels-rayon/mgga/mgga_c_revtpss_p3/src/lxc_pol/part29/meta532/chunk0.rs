//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1862/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1862(t25386: f64, t95536: f64, t92840: f64, t26518: f64, t9285: f64, t25299: f64, t7407: f64, t92890: f64, t2061: f64, t22: f64, t25402: f64, t93140: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95537 = t25386 * t95536;
    let t95538 = t95537 * t92840;
    let t95540 = t26518 * t9285;
    let t95542 = 0.68540937416128198417e-2_f64 * t25299 * t95540;
    let t95543 = t92890 * t7407;
    let t95546 = t25402 * t2061 * t22;
    let t95548 = 0.51727911450665971904e-3_f64 * t93140 * t95546;
    (t95537, t95538, t95540, t95542, t95543, t95546, t95548)
}
