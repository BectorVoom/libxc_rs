//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1077/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1077(t10995: f64, t979: f64, t7696: f64, t7699: f64, t283: f64, t3049: f64, t990: f64) -> (f64, f64, f64) {
    let t26742 = t979 * t10995;
    let t26745 = t7696 * t7699;
    let t26748 = t3049 * t283 * t990;
    (t26742, t26745, t26748)
}
