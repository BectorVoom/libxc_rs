//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2983/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2983<F: Float>(t54387: F, t1028: F, t1042: F, t1043: F, t11875: F, t11927: F, t15604: F, t15691: F, t15700: F, t15780: F, t16222: F, t16223: F, t16226: F, t2858: F, t3059: F, t3117: F, t3155: F, t4186: F, t43044: F, t43050: F, t4781: F, t4837: F, t4872: F, t4893: F, t54166: F, t54267: F, t54348: F, t54360: F, t54365: F, t54370: F, t54384: F) -> F {
    let t54388 = F::cast_from(0.14291339372689912324e-3_f64) * t54387;
    let t54389 = F::cast_from(0.11433071498151929859e-2_f64) * t54348 - F::cast_from(0.17149607247227894789e-2_f64) * t16226 * t15691 * t3155 * t2858 * t1043 + F::cast_from(0.14291339372689912324e-2_f64) * t54166 * t16223 + F::cast_from(0.71456696863449561621e-3_f64) * t15700 * t16222 * t54267 + F::cast_from(0.12862205435420921092e-2_f64) * t11927 * t3117 * t4781 * t54360 + F::cast_from(0.25724410870841842183e-2_f64) * t43050 * t3117 * t4893 * t54365 - F::cast_from(0.12862205435420921092e-2_f64) * t43044 * t3117 * t4893 * t54370 + F::cast_from(0.12862205435420921092e-2_f64) * t11875 * t3117 * t15780 * t15604 + F::cast_from(0.85748036236139473944e-3_f64) * t4837 * t1042 * t4872 * t4186 * t3059 - F::cast_from(0.21722835846488666732e-1_f64) * t54384 * t1028 + t54388;
    t54389
}
