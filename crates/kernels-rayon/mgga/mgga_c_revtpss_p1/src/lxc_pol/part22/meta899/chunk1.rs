//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3092/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3092(t11922: f64, t16044: f64, t3115: f64, t11994: f64, t15769: f64, t3298: f64, t4746: f64, t4891: f64, t11744: f64, t4834: f64, t12009: f64, t15823: f64) -> (f64, f64, f64, f64, f64) {
    let t53771 = t3115 * t11922 * t16044;
    let t53790 = t11994 * t15769;
    let t53800 = t4746 * t3298 * t4891;
    let t53805 = t4834 * t11744;
    let t53810 = t15823 * t12009;
    (t53771, t53790, t53800, t53805, t53810)
}
