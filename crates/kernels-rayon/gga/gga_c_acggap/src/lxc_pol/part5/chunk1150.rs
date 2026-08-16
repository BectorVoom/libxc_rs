//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1150/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1150(t3409: f64, t6148: f64, t16987: f64, t5855: f64, t4396: f64, t5859: f64, t4389: f64, t6265: f64, t1896: f64, t3228: f64, t1077: f64, t1089: f64, t1095: f64, t1131: f64, t1165: f64, t1180: f64, t12804: f64, t1459: f64, t15733: f64, t16871: f64, t1734: f64, t1795: f64, t368: f64, t398: f64, t418: f64, t4267: f64, t5265: f64, t5852: f64, t930: f64) -> f64 {
    let t20720 = t3409 * t6148;
    let t20722 = t16987 * t5855;
    let t20724 = t4396 * t5859;
    let t20732 = t4389 * t6265;
    let t20734 = t3228 * t1896;
    let t20736 = 0.12862205435420921092e-2_f64 * t418 * t398 * t1459 * t1795 * t1131 - 0.17149607247227894789e-2_f64 * t418 * t1089 * t368 * t1734 * t1077 + 0.17149607247227894789e-2_f64 * t418 * t1089 * t1095 * t1734 * t1131 - 0.21437009059034868486e-3_f64 * t1180 * t1165 * t5852 * t930 + 0.20007875121765877254e-2_f64 * t20720 - 0.25724410870841842184e-2_f64 * t20722 + 0.25724410870841842183e-2_f64 * t20724 + 0.10289764348336736873e0_f64 * t16871 * t1165 * t4267 * t5265 + 0.13719685797782315831e-1_f64 * t15733 + 0.85748036236139473944e-3_f64 * t12804 - 0.80031500487063509015e-2_f64 * t20732 + 0.42874018118069736972e-3_f64 * t20734;
    t20736
}
