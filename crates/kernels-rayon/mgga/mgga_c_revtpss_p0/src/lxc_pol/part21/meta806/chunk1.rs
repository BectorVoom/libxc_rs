//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2935/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2935(t11710: f64, t15964: f64, t3091: f64, t11683: f64, t11774: f64, t12131: f64, t15689: f64, t15691: f64, t15693: f64, t15696: f64, t15963: f64, t42170: f64, t42172: f64, t42176: f64, t42190: f64, t53402: f64, t53407: f64, t53413: f64, t53416: f64) -> f64 {
    let t53422 = t3091 * t11710 * t15964;
    let t53425 = 0.85748036236139473944e-3_f64 * t11774 * t15696 * t11683 + 0.45732285992607719436e-2_f64 * t53402 * t15693 - 0.57165357490759649295e-3_f64 * t53407 + 0.85748036236139473944e-3_f64 * t15689 * t15691 * t12131 * t15963 - 0.42874018118069736972e-3_f64 * t53413 + 0.85748036236139473944e-3_f64 * t53416 - 0.85748036236139473944e-3_f64 * t42170 - 0.45732285992607719436e-2_f64 * t42172 - 0.28582678745379824648e-3_f64 * t42176 - 0.57165357490759649295e-3_f64 * t53422 - 0.57165357490759649295e-3_f64 * t42190;
    t53425
}
