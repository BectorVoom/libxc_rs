//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2983/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2983(t54387: f64, t1028: f64, t1042: f64, t1043: f64, t11875: f64, t11927: f64, t15604: f64, t15691: f64, t15700: f64, t15780: f64, t16222: f64, t16223: f64, t16226: f64, t2858: f64, t3059: f64, t3117: f64, t3155: f64, t4186: f64, t43044: f64, t43050: f64, t4781: f64, t4837: f64, t4872: f64, t4893: f64, t54166: f64, t54267: f64, t54348: f64, t54360: f64, t54365: f64, t54370: f64, t54384: f64) -> f64 {
    let t54388 = 0.14291339372689912324e-3_f64 * t54387;
    let t54389 = 0.11433071498151929859e-2_f64 * t54348 - 0.17149607247227894789e-2_f64 * t16226 * t15691 * t3155 * t2858 * t1043 + 0.14291339372689912324e-2_f64 * t54166 * t16223 + 0.71456696863449561621e-3_f64 * t15700 * t16222 * t54267 + 0.12862205435420921092e-2_f64 * t11927 * t3117 * t4781 * t54360 + 0.25724410870841842183e-2_f64 * t43050 * t3117 * t4893 * t54365 - 0.12862205435420921092e-2_f64 * t43044 * t3117 * t4893 * t54370 + 0.12862205435420921092e-2_f64 * t11875 * t3117 * t15780 * t15604 + 0.85748036236139473944e-3_f64 * t4837 * t1042 * t4872 * t4186 * t3059 - 0.21722835846488666732e-1_f64 * t54384 * t1028 + t54388;
    t54389
}
