//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 709/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk709(t1203: f64, t7740: f64, t2189: f64, t3325: f64, t3330: f64, t1165: f64, t377: f64, t1169: f64, t283: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7741 = t7740 * t1203;
    let t7742 = t3325 * t2189;
    let t7743 = t2189 * t1203;
    let t7745 = 2.0_f64 * t3330 * t7743;
    let t7746 = t1165 * t377;
    let t7748 = t1169 * t283;
    (t7741, t7742, t7743, t7745, t7746, t7748)
}
