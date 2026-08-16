//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 962/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk962(t23499: f64, t4919: f64, t1011: f64, t1063: f64, t11774: f64, t11972: f64, t15862: f64, t1675: f64, t19901: f64, t19908: f64, t19913: f64, t19921: f64, t19968: f64, t19977: f64, t23931: f64, t23936: f64, t23939: f64, t23945: f64, t23961: f64, t23966: f64, t23976: f64, t23980: f64, t375: f64, t4834: f64, t4837: f64, t4892: f64, t4899: f64, t6323: f64, t6327: f64) -> f64 {
    let t23984 = t4919 * t23499;
    let t23988 = -0.14291339372689912324e-3_f64 * t15862 + 0.12862205435420921092e-2_f64 * t4892 * t23931 - 0.64311027177104605458e-3_f64 * t4899 * t23936 - 0.85748036236139473944e-3_f64 * t11774 * t23939 - t19901 / 144.0_f64 + t19908 / 288.0_f64 + t19913 / 216.0_f64 - t1011 * t23945 / 48.0_f64 + 0.21437009059034868486e-3_f64 * t23961 * t375 + 0.12862205435420921092e-2_f64 * t4837 * t23966 + 0.42874018118069736972e-3_f64 * t19968 * t1675 + 0.42874018118069736972e-3_f64 * t4834 * t6323 + 0.7145669686344956162e-3_f64 * t4834 * t6327 + 0.14291339372689912324e-3_f64 * t1063 * t23976 + 0.63517063878621832552e-3_f64 * t1063 * t23980 - 0.57165357490759649295e-3_f64 * t19921 + t1011 * t23984 / 72.0_f64 - 0.85748036236139473944e-3_f64 * t19977 + t11972;
    t23988
}
