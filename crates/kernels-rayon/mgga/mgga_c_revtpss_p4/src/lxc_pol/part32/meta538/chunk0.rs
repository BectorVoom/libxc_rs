//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1849/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1849(t25386: f64, t95536: f64, t26518: f64, t9285: f64, t25299: f64, t2061: f64, t22: f64, t25402: f64, t93140: f64, t25310: f64, t26506: f64, t2439: f64, t7398: f64, t780: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95537 = t25386 * t95536;
    let t95540 = t26518 * t9285;
    let t95542 = 0.68540937416128198417e-2_f64 * t25299 * t95540;
    let t95546 = t25402 * t2061 * t22;
    let t95548 = 0.51727911450665971904e-3_f64 * t93140 * t95546;
    let t95551 = t25310 * t26506;
    let t95562 = t2439 * t785 * t7398 * t780;
    (t95537, t95540, t95542, t95546, t95548, t95551, t95562)
}
