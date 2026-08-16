//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 739/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk739(t6635: f64, t6644: f64, t2047: f64, t814: f64, t829: f64, t235: f64, t7084: f64, t2051: f64, t226: f64, t6641: f64, t6650: f64, t6654: f64, t808: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7095 = 0.38381794893125283518e-1_f64 * t6635;
    let t7097 = 0.82246703342411321825e-2_f64 * t6644;
    let t7101 = t814 * t2047;
    let t7102 = t7101 * t829;
    let t7104 = t235 * t7084;
    let t7106 = -t7095 - 0.3289868133696452873e-1_f64 * t6641 - t7097 - 0.16449340668482264365e-1_f64 * t6650 + 0.16449340668482264365e-1_f64 * t6654 + t808 * t2051 - t812 * t7102 + t226 * t7104;
    (t7095, t7097, t7101, t7102, t7104, t7106)
}
