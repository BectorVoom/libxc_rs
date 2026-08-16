//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 616/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk616(t678: f64, t7921: f64, t1550: f64, t7810: f64, t2084: f64, t271: f64) -> (f64, f64, f64) {
    let t7922 = t7921 * t678;
    let t7924 = t1550 * t7810;
    let t7925 = 0.2993560425465952141e-1_f64 * t7924;
    let t7926 = t2084 * t271;
    (t7922, t7925, t7926)
}
