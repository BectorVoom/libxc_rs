//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2267/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2267(t870: f64, t99042: f64, t16596: f64, t86721: f64, t1484: f64, t584: f64, t86753: f64, t22959: f64, t16949: f64, t25014: f64, t1408: f64, t4255: f64) -> (f64, f64, f64, f64, f64) {
    let t99043 = t99042 * t870;
    let t99049 = t86721 * t16596;
    let t99053 = t86753 * t584 * t1484;
    let t99055 = 6.0_f64 * t22959 * t99053;
    let t99056 = t25014 * t16949;
    let t99060 = t870 * t1408 * t4255;
    (t99043, t99049, t99055, t99056, t99060)
}
