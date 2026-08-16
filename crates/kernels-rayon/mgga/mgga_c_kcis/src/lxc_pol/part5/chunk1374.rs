//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1374/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1374(t22509: f64, t22542: f64, t22581: f64, t22627: f64, t552: f64, t573: f64, t12565: f64, t7393: f64, t21791: f64, t577: f64, t585: f64, t20956: f64, t4293: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t22629 = t22509 + t22542 + t22581 + t22627;
    let t22630 = t22629 * t552;
    let t22631 = t22630 * sigma2;
    let t22632 = t22631 * t573;
    let t22634 = t12565 * t7393;
    let t22636 = t21791 * t552;
    let t22637 = t22636 * t577;
    let t22638 = t22637 * t585;
    let t22640 = t4293 * t20956;
    (t22632, t22634, t22638, t22640)
}
