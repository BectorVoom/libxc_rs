//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2166/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2166(t1032: f64, t4469: f64, t867: f64, t786: f64, t7060: f64, t1559: f64, t2771: f64, t7760: f64, t2467: f64, t1579: f64, t231: f64, t2645: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99270 = t4469 * t1032;
    let t99271 = t99270 * t867;
    let t99272 = t786 * t99271;
    let t99274 = 0.14456046980341999104e-1_f64 * t99272 * t7060;
    let t99277 = t1559 * t2771;
    let t99285 = t786 * t7760 * t867;
    let t99287 = 0.19514881078765566038e-1_f64 * t99285 * t2467;
    let t99289 = t1579 * t2645 * t231;
    (t99270, t99271, t99274, t99277, t99287, t99289)
}
