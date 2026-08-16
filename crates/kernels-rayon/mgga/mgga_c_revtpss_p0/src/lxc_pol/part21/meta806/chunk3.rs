//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2937/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2937(t10356: f64, t1042: f64, t1063: f64, t11704: f64, t11994: f64, t15938: f64, t15952: f64, t16199: f64, t1671: f64, t3091: f64, t3092: f64, t3106: f64, t42193: f64, t42204: f64, t42584: f64, t4781: f64, t53427: f64, t53432: f64, t53433: f64, t53437: f64, t53450: f64) -> f64 {
    let t53455 = -0.28582678745379824648e-3_f64 * t42193 + 0.14481890564325777821e-1_f64 * t53427 - t53432 + 0.57165357490759649295e-3_f64 * t53433 + 0.30488190661738479624e-2_f64 * t42204 - 0.95275595817932748825e-4_f64 * t53437 + 0.85748036236139473944e-3_f64 * t3091 * t3092 * t4781 * t11704 * t10356 - 0.13719685797782315831e-1_f64 * t3106 * t15938 - 0.53100265402527852012e-1_f64 * t42584 * t1671 - 0.85748036236139473944e-3_f64 * t11994 * t15952 - 0.42874018118069736973e-2_f64 * t1063 * t1042 * t16199 * t53450;
    t53455
}
