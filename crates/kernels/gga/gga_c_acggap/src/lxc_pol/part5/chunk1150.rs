//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1150/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1150<F: Float>(t3409: F, t6148: F, t16987: F, t5855: F, t4396: F, t5859: F, t4389: F, t6265: F, t1896: F, t3228: F, t1077: F, t1089: F, t1095: F, t1131: F, t1165: F, t1180: F, t12804: F, t1459: F, t15733: F, t16871: F, t1734: F, t1795: F, t368: F, t398: F, t418: F, t4267: F, t5265: F, t5852: F, t930: F) -> F {
    let t20720 = t3409 * t6148;
    let t20722 = t16987 * t5855;
    let t20724 = t4396 * t5859;
    let t20732 = t4389 * t6265;
    let t20734 = t3228 * t1896;
    let t20736 = F::new(0.12862205435420921092e-2) * t418 * t398 * t1459 * t1795 * t1131 - F::new(0.17149607247227894789e-2) * t418 * t1089 * t368 * t1734 * t1077 + F::new(0.17149607247227894789e-2) * t418 * t1089 * t1095 * t1734 * t1131 - F::new(0.21437009059034868486e-3) * t1180 * t1165 * t5852 * t930 + F::new(0.20007875121765877254e-2) * t20720 - F::new(0.25724410870841842184e-2) * t20722 + F::new(0.25724410870841842183e-2) * t20724 + F::new(0.10289764348336736873e0) * t16871 * t1165 * t4267 * t5265 + F::new(0.13719685797782315831e-1) * t15733 + F::new(0.85748036236139473944e-3) * t12804 - F::new(0.80031500487063509015e-2) * t20732 + F::new(0.42874018118069736972e-3) * t20734;
    t20736
}
