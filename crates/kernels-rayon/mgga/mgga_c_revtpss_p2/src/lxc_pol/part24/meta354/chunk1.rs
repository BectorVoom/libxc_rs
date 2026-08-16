//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1220/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1220(t23964: f64, t247: f64, t3116: f64, t1066: f64, t23474: f64, t11853: f64, t23470: f64, t23499: f64, t4919: f64, t1011: f64, t1063: f64, t11774: f64, t11972: f64, t15862: f64, t1675: f64, t19901: f64, t19908: f64, t19913: f64, t19921: f64, t19968: f64, t19977: f64, t23931: f64, t23936: f64, t23939: f64, t23945: f64, t23961: f64, t375: f64, t4834: f64, t4837: f64, t4892: f64, t4899: f64, t6323: f64, t6327: f64) -> (f64, f64, f64, f64) {
    let t23966 = t247 * t3116 * t23964;
    let t23976 = t247 * t1066 * t23474;
    let t23980 = t247 * t11853 * t23470;
    let t23984 = t4919 * t23499;
    let t23988 = -0.14291339372689912324e-3_f64 * t15862 + 0.12862205435420921092e-2_f64 * t4892 * t23931 - 0.64311027177104605458e-3_f64 * t4899 * t23936 - 0.85748036236139473944e-3_f64 * t11774 * t23939 - t19901 / 144.0_f64 + t19908 / 288.0_f64 + t19913 / 216.0_f64 - t1011 * t23945 / 48.0_f64 + 0.21437009059034868486e-3_f64 * t23961 * t375 + 0.12862205435420921092e-2_f64 * t4837 * t23966 + 0.42874018118069736972e-3_f64 * t19968 * t1675 + 0.42874018118069736972e-3_f64 * t4834 * t6323 + 0.7145669686344956162e-3_f64 * t4834 * t6327 + 0.14291339372689912324e-3_f64 * t1063 * t23976 + 0.63517063878621832552e-3_f64 * t1063 * t23980 - 0.57165357490759649295e-3_f64 * t19921 + t1011 * t23984 / 72.0_f64 - 0.85748036236139473944e-3_f64 * t19977 + t11972;
    (t23966, t23976, t23980, t23988)
}
