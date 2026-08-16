//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 981/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk981(t11902: f64, t225: f64, t366: f64, t1053: f64, t3196: f64, t11151: f64, t247: f64, t3182: f64, t3163: f64, t3172: f64, t3161: f64, t1017: f64, t1063: f64, t11855: f64, t11859: f64, t11862: f64, t11866: f64, t11871: f64, t11875: f64, t11877: f64, t11881: f64, t11883: f64, t11886: f64, t11888: f64, t3101: f64, t3115: f64, t3120: f64, t3188: f64, t375: f64) -> f64 {
    let t11903 = t11902 * t225;
    let t11904 = t11903 * t366;
    let t11907 = t3196 * t1053;
    let t11913 = t247 * t3182 * t11151;
    let t11916 = t3172 * t3163;
    let t11917 = t3161 * t11916;
    let t11919 = 0.63517063878621832552e-3_f64 * t1063 * t11855 - 0.12862205435420921092e-2_f64 * t11859 * t11862 - 0.12862205435420921092e-2_f64 * t11866 * t3120 - 0.64311027177104605458e-3_f64 * t3115 * t11871 + 0.64311027177104605458e-3_f64 * t11875 * t11877 - t11881 / 432.0_f64 + 11.0_f64 / 108.0_f64 * t11883 * t1017 - t11886 / 54.0_f64 + 0.42874018118069736972e-3_f64 * t11888 + 0.21437009059034868486e-3_f64 * t11904 * t375 - 0.34299214494455789577e-2_f64 * t11907 * t375 - 0.85748036236139473944e-3_f64 * t3188 * t3101 - 0.14291339372689912324e-2_f64 * t1063 * t11913 - 0.42874018118069736972e-3_f64 * t11917;
    t11919
}
