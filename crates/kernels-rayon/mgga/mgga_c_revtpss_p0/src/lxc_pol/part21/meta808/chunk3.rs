//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2950/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2950(t1042: f64, t1045: f64, t1063: f64, t11202: f64, t11252: f64, t11259: f64, t11933: f64, t1469: f64, t15716: f64, t16045: f64, t3115: f64, t3117: f64, t3130: f64, t42421: f64, t42439: f64, t4872: f64, t51963: f64, t53474: f64, t53683: f64, t53690: f64, t53692: f64, t53704: f64, t53707: f64, t53710: f64) -> f64 {
    let t53716 = -0.21437009059034868486e-3_f64 * t3115 * t3117 * t53683 * t1045 - 0.30488190661738479624e-2_f64 * t42421 + 0.57165357490759649295e-3_f64 * t53690 - 0.85748036236139473944e-3_f64 * t53692 * t3130 - 0.85748036236139473944e-3_f64 * t15716 * t1042 * t4872 * t1469 * t11202 - 0.57165357490759649295e-3_f64 * t42439 + 0.34299214494455789577e-2_f64 * t11933 * t16045 - 0.12862205435420921092e-2_f64 * t53704 * t11252 + 0.21437009059034868486e-3_f64 * t53707 * t11259 - 0.57165357490759649295e-3_f64 * t53710 + 0.85748036236139473944e-2_f64 * t1063 * t1042 * t51963 * t53474;
    t53716
}
