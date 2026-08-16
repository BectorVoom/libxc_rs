//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3007/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3007(t3133: f64, t3155: f64, t11173: f64, t1651: f64, t1042: f64, t11675: f64, t11845: f64, t11855: f64, t11866: f64, t12004: f64, t1469: f64, t15584: f64, t15615: f64, t16040: f64, t16222: f64, t16226: f64, t247: f64, t3116: f64, t3127: f64, t43063: f64, t43244: f64, t4783: f64, t4831: f64, t4834: f64, t4837: f64, t4872: f64, t53585: f64, t54271: f64) -> (f64, f64) {
    let t54950 = t3155 * t3133;
    let t54955 = t1651 * t11173;
    let t54977 = -0.57165357490759649295e-3_f64 * t43063 + 0.14291339372689912324e-2_f64 * t16226 * t16222 * t54271 + 0.85748036236139473944e-3_f64 * t16226 * t15584 * t53585 * t54950 + 0.42874018118069736972e-3_f64 * t4837 * t247 * t3116 * t54955 + 0.42874018118069736972e-3_f64 * t43244 * t4783 - 0.14291339372689912324e-3_f64 * t3127 * t1042 * t4872 * t1469 * t11173 - 0.12862205435420921092e-2_f64 * t11866 * t16040 + 0.85748036236139473944e-3_f64 * t11675 * t15615 + 0.14291339372689912324e-3_f64 * t4834 * t11845 + 0.63517063878621832552e-3_f64 * t4834 * t11855 + 0.14481890564325777822e-1_f64 * t12004 * t4831;
    (t54955, t54977)
}
