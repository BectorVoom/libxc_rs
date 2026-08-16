//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1136/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1136<F: Float>(t11902: F, t225: F, t366: F, t1053: F, t3196: F, t11151: F, t247: F, t3182: F, t3163: F, t3172: F, t3161: F, t1017: F, t1063: F, t11855: F, t11859: F, t11862: F, t11866: F, t11871: F, t11875: F, t11877: F, t11881: F, t11883: F, t11886: F, t11888: F, t3101: F, t3115: F, t3120: F, t3188: F, t375: F) -> (F, F, F, F, F, F) {
    let t11903 = t11902 * t225;
    let t11904 = t11903 * t366;
    let t11907 = t3196 * t1053;
    let t11913 = t247 * t3182 * t11151;
    let t11916 = t3172 * t3163;
    let t11917 = t3161 * t11916;
    let t11919 = F::cast_from(0.63517063878621832552e-3_f64) * t1063 * t11855 - F::cast_from(0.12862205435420921092e-2_f64) * t11859 * t11862 - F::cast_from(0.12862205435420921092e-2_f64) * t11866 * t3120 - F::cast_from(0.64311027177104605458e-3_f64) * t3115 * t11871 + F::cast_from(0.64311027177104605458e-3_f64) * t11875 * t11877 - t11881 / F::cast_from(432.0_f64) + F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t11883 * t1017 - t11886 / F::cast_from(54.0_f64) + F::cast_from(0.42874018118069736972e-3_f64) * t11888 + F::cast_from(0.21437009059034868486e-3_f64) * t11904 * t375 - F::cast_from(0.34299214494455789577e-2_f64) * t11907 * t375 - F::cast_from(0.85748036236139473944e-3_f64) * t3188 * t3101 - F::cast_from(0.14291339372689912324e-2_f64) * t1063 * t11913 - F::cast_from(0.42874018118069736972e-3_f64) * t11917;
    (t11903, t11904, t11907, t11913, t11916, t11919)
}
