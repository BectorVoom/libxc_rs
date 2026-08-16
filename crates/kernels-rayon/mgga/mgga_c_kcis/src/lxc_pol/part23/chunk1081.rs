//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1081/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1081(t27716: f64, t449: f64, t446: f64, t448: f64, t4504: f64, t2233: f64, t2272: f64, t3708: f64, t1300: f64, t8014: f64, t2167: f64, t4527: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27717 = t449 * t27716;
    let t27718 = t446 * t27717;
    let t27719 = t27718 / 16.0_f64;
    let t27720 = t448 * t4504;
    let t27721 = t2233 * t27720;
    let t27722 = t27721 / 16.0_f64;
    let t27723 = t3708 * t2272;
    let t27724 = t446 * t27723;
    let t27725 = t27724 / 16.0_f64;
    let t27726 = t1300 * t8014;
    let t27727 = t446 * t27726;
    let t27728 = t27727 / 8.0_f64;
    let t27733 = t4527 * t2167;
    (t27717, t27719, t27720, t27722, t27723, t27725, t27726, t27728, t27733)
}
