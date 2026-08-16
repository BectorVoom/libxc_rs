//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1427/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1427(t1063: f64, t1592: f64, t247: f64, t42778: f64, t3298: f64, t4746: f64, t4891: f64, t225: f64, t53014: f64, t366: f64, t1011: f64, t1655: f64, t2438: f64) -> (f64, f64, f64, f64, f64) {
    let t53762 = t1063 * t247 * t42778 * t1592;
    let t53800 = t4746 * t3298 * t4891;
    let t53877 = t53014 * t225;
    let t53878 = t53877 * t366;
    let t54118 = t1011 * t2438 * t1655;
    (t53762, t53800, t53877, t53878, t54118)
}
