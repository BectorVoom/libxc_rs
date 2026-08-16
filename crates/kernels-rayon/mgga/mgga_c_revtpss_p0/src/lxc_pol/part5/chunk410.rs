//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 410/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk410(t1357: f64, t1358: f64, t689: f64, t556: f64, t786: f64, t561: f64, t72: f64, t686: f64) -> (f64, f64, f64, f64, f64) {
    let t1359 = t1357 * t1358;
    let t1361 = 0.54878743191129263322e-2_f64 * t689 * t1359;
    let t1362 = t786 * t556;
    let t1363 = t561 * t72;
    let t1364 = t1363 * t686;
    (t1359, t1361, t1362, t1363, t1364)
}
