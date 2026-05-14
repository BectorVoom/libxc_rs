//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 872/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk872<F: Float>(t23958: F, t341: F, t225: F, t366: F, t1651: F, t6258: F, t247: F, t3116: F, t1066: F, t23474: F, t11853: F, t23470: F, t23499: F, t4919: F, t1011: F, t1063: F, t11774: F, t11972: F, t15862: F, t1675: F, t19901: F, t19908: F, t19913: F, t19921: F, t19968: F, t19977: F, t23931: F, t23936: F, t23939: F, t23945: F, t375: F, t4834: F, t4837: F, t4892: F, t4899: F, t6323: F, t6327: F) -> (F, F, F) {
    let t23959 = t23958 * t341;
    let t23960 = t23959 * t225;
    let t23961 = t23960 * t366;
    let t23964 = t1651 * t6258;
    let t23966 = t247 * t3116 * t23964;
    let t23976 = t247 * t1066 * t23474;
    let t23980 = t247 * t11853 * t23470;
    let t23984 = t4919 * t23499;
    let t23988 = -0.14291339372689912324e-3 * t15862 + 0.12862205435420921092e-2 * t4892 * t23931 - 0.64311027177104605458e-3 * t4899 * t23936 - 0.85748036236139473944e-3 * t11774 * t23939 - t19901 / 144.0 + t19908 / 288.0 + t19913 / 216.0 - t1011 * t23945 / 48.0 + 0.21437009059034868486e-3 * t23961 * t375 + 0.12862205435420921092e-2 * t4837 * t23966 + 0.42874018118069736972e-3 * t19968 * t1675 + 0.42874018118069736972e-3 * t4834 * t6323 + 0.7145669686344956162e-3 * t4834 * t6327 + 0.14291339372689912324e-3 * t1063 * t23976 + 0.63517063878621832552e-3 * t1063 * t23980 - 0.57165357490759649295e-3 * t19921 + t1011 * t23984 / 72.0 - 0.85748036236139473944e-3 * t19977 + t11972;
    (t23959, t23964, t23988)
}
