//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3007/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3007<F: Float>(t3133: F, t3155: F, t11173: F, t1651: F, t1042: F, t11675: F, t11845: F, t11855: F, t11866: F, t12004: F, t1469: F, t15584: F, t15615: F, t16040: F, t16222: F, t16226: F, t247: F, t3116: F, t3127: F, t43063: F, t43244: F, t4783: F, t4831: F, t4834: F, t4837: F, t4872: F, t53585: F, t54271: F) -> (F, F) {
    let t54950 = t3155 * t3133;
    let t54955 = t1651 * t11173;
    let t54977 = -F::cast_from(0.57165357490759649295e-3_f64) * t43063 + F::cast_from(0.14291339372689912324e-2_f64) * t16226 * t16222 * t54271 + F::cast_from(0.85748036236139473944e-3_f64) * t16226 * t15584 * t53585 * t54950 + F::cast_from(0.42874018118069736972e-3_f64) * t4837 * t247 * t3116 * t54955 + F::cast_from(0.42874018118069736972e-3_f64) * t43244 * t4783 - F::cast_from(0.14291339372689912324e-3_f64) * t3127 * t1042 * t4872 * t1469 * t11173 - F::cast_from(0.12862205435420921092e-2_f64) * t11866 * t16040 + F::cast_from(0.85748036236139473944e-3_f64) * t11675 * t15615 + F::cast_from(0.14291339372689912324e-3_f64) * t4834 * t11845 + F::cast_from(0.63517063878621832552e-3_f64) * t4834 * t11855 + F::cast_from(0.14481890564325777822e-1_f64) * t12004 * t4831;
    (t54955, t54977)
}
