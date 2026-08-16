//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 657/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk657(t3747: f64, t834: f64, t2215: f64, t3743: f64, t841: f64, t1167: f64) -> (f64, f64, f64, f64) {
    let t3748 = t834 * t3747;
    let t3752 = t2215 * t3743;
    let t3754 = t841 * t3747;
    let t3757 = t1167 * t1167;
    (t3748, t3752, t3754, t3757)
}
