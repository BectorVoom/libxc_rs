//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 962/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk962<F: Float>(t23499: F, t4919: F, t1011: F, t1063: F, t11774: F, t11972: F, t15862: F, t1675: F, t19901: F, t19908: F, t19913: F, t19921: F, t19968: F, t19977: F, t23931: F, t23936: F, t23939: F, t23945: F, t23961: F, t23966: F, t23976: F, t23980: F, t375: F, t4834: F, t4837: F, t4892: F, t4899: F, t6323: F, t6327: F) -> F {
    let t23984 = t4919 * t23499;
    let t23988 = -F::cast_from(0.14291339372689912324e-3_f64) * t15862 + F::cast_from(0.12862205435420921092e-2_f64) * t4892 * t23931 - F::cast_from(0.64311027177104605458e-3_f64) * t4899 * t23936 - F::cast_from(0.85748036236139473944e-3_f64) * t11774 * t23939 - t19901 / F::cast_from(144.0_f64) + t19908 / F::cast_from(288.0_f64) + t19913 / F::cast_from(216.0_f64) - t1011 * t23945 / F::cast_from(48.0_f64) + F::cast_from(0.21437009059034868486e-3_f64) * t23961 * t375 + F::cast_from(0.12862205435420921092e-2_f64) * t4837 * t23966 + F::cast_from(0.42874018118069736972e-3_f64) * t19968 * t1675 + F::cast_from(0.42874018118069736972e-3_f64) * t4834 * t6323 + F::cast_from(0.7145669686344956162e-3_f64) * t4834 * t6327 + F::cast_from(0.14291339372689912324e-3_f64) * t1063 * t23976 + F::cast_from(0.63517063878621832552e-3_f64) * t1063 * t23980 - F::cast_from(0.57165357490759649295e-3_f64) * t19921 + t1011 * t23984 / F::cast_from(72.0_f64) - F::cast_from(0.85748036236139473944e-3_f64) * t19977 + t11972;
    t23988
}
