//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2955/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2955(t11862: f64, t11875: f64, t11991: f64, t12017: f64, t15926: f64, t3117: f64, t3157: f64, t3162: f64, t42391: f64, t42487: f64, t42496: f64, t4803: f64, t4875: f64, t53790: f64, t53792: f64, t53800: f64, t53805: f64, t53807: f64, t53810: f64) -> f64 {
    let t53816 = -0.57165357490759649295e-3_f64 * t53790 + 0.64311027177104605458e-3_f64 * t11875 * t3117 * t53792 * t3162 - 0.64311027177104605458e-3_f64 * t15926 * t12017 - 0.12862205435420921092e-2_f64 * t53800 * t11862 + 0.85748036236139473944e-3_f64 * t42487 + 0.95275595817932748827e-4_f64 * t42496 - 0.57165357490759649295e-3_f64 * t53805 - 0.68598428988911579154e-2_f64 * t53807 * t3157 + 0.85748036236139473944e-3_f64 * t53810 - 0.42874018118069736972e-3_f64 * t42391 * t4875 - 0.85748036236139473944e-3_f64 * t11991 * t4803;
    t53816
}
