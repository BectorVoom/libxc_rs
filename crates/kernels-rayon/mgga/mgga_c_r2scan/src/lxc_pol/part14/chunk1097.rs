//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1097/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1097(t1115: f64, t1234: f64, t3270: f64, t1543: f64, t11449: f64, t1561: f64, t14402: f64, t795: f64, t498: f64, t11002: f64, t2259: f64, t3493: f64, t792: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38697 = t3270 * t1115 * t1234;
    let t38715 = t3270 * t1115 * t1543;
    let t38718 = t1561 * t11449;
    let t38722 = t14402 * t795;
    let t38723 = t3270 * t38722;
    let t38739 = t498 * t11449;
    let t38749 = t11002 * t1115 * t2259;
    let t38770 = t3493 * t792;
    (t38697, t38715, t38718, t38723, t38739, t38749, t38770)
}
