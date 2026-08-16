//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1347/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1347(t235: f64, t4503: f64, t2453: f64, t123: f64, t125: f64, t2452: f64, t40633: f64, t810: f64, t10759: f64, t2735: f64, t10293: f64, t240: f64) -> (f64, f64, f64, f64) {
    let t40798 = t4503 * t235;
    let t40799 = t2453 * t40798;
    let t40810 = 0.30119321664969771194e-5_f64 * t123 * t125 * t40633 * t2452 * t810;
    let t40834 = t2735 * t10759;
    let t40846 = t10293 * t240;
    (t40799, t40810, t40834, t40846)
}
