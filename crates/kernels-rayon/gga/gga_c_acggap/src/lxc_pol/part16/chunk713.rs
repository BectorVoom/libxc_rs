//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 713/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk713(t1994: f64, t7637: f64, t601: f64, t7630: f64, t1101: f64, t599: f64, t1181: f64, t7493: f64, t168: f64, t7559: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7638 = t7637 * t1994;
    let t7639 = 0.13976929906490734252e-2_f64 * t7638;
    let t7640 = t7630 * t601;
    let t7641 = 0.12862205435420921092e-2_f64 * t7640;
    let t7642 = t599 * t1101;
    let t7643 = t1181 * t7642;
    let t7644 = t7493 * t7643;
    let t7646 = t7559 * t168;
    (t7639, t7641, t7642, t7643, t7644, t7646)
}
