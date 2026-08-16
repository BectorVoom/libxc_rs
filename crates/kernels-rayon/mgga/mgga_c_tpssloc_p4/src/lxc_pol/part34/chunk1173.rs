//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1173/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1173(t20800: f64, t6552: f64, t6553: f64, t6554: f64, t1880: f64, t25224: f64, t28294: f64, t22986: f64, t23270: f64, t25191: f64, t5657: f64, t28267: f64, t86873: f64) -> (f64, f64, f64, f64) {
    let t105453 = t6552 * t6553 * t6554 * t20800;
    let t105462 = t1880 * t25224 * t28294;
    let t105474 = t22986 * t23270 * t25191 * t5657;
    let t105519 = t22986 * t86873 * t28267;
    (t105453, t105462, t105474, t105519)
}
