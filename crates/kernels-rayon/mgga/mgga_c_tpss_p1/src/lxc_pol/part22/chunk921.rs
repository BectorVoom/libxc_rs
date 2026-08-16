//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 921/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk921(t2668: f64, t917: f64, t2473: f64, t845: f64, t2530: f64, t841: f64, t2529: f64, t281: f64, t269: f64, t2470: f64, t664: f64) -> (f64, f64, f64, f64, f64) {
    let t8588 = t917 * t2668;
    let t8590 = t2473 * t845;
    let t8595 = t841 * t2530;
    let t8599 = 1.0_f64 / t2529 / t281;
    let t8600 = t269 * t8599;
    let t8605 = t664 * t2470;
    (t8588, t8590, t8595, t8600, t8605)
}
