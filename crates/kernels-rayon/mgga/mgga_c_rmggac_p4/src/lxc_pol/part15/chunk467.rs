//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 467/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk467(t5803: f64, t814: f64, t1714: f64, t809: f64, t312: f64, t3878: f64, t90: f64, t1726: f64, t3885: f64, t316: f64, t50: f64, t547: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5804 = t5803 * t814;
    let t5809 = t809 * t1714;
    let t5810 = t5809 * t312;
    let t5814 = -t814 - 3.0_f64 * t3878;
    let t5815 = t90 * t5814;
    let t5824 = t3885 * t1726;
    let t5825 = t5824 * t316;
    let t5828 = t547 * t50;
    (t5804, t5810, t5814, t5815, t5825, t5828)
}
