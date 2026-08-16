//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 905/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk905(t2215: f64, t3747: f64, t836: f64, t841: f64, t9798: f64, t218: f64, t3757: f64, t675: f64) -> (f64, f64, f64) {
    let t9811 = t2215 * t3747;
    let t9812 = t9811 * t836;
    let t9814 = t841 * t9798;
    let t9819 = t218 * t675 * t3757;
    (t9812, t9814, t9819)
}
