//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1153/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1153<F: Float>(t1163: F, t1165: F, t4162: F, t5862: F, t3372: F, t6351: F, t1101: F, t1879: F, t3361: F, t495: F, t955: F, t1016: F, t1173: F, t1181: F, t1298: F, t1439: F, t1454: F, t1460: F, t1531: F, t1532: F, t15748: F, t15750: F, t15754: F, t15761: F, t15947: F, t3396: F, t4463: F, t945: F) -> F {
    let t20794 = t1163 * t1165 * t5862 * t4162;
    let t20806 = t3372 * t6351;
    let t20810 = t3361 * t1165 * t1879 * t1101;
    let t20817 = t955 * t495;
    let t20822 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t15748 - F::cast_from(0.34299214494455789578e-1_f64) * t4463 * t1165 * t15947 * t1439 + F::cast_from(0.12862205435420921092e-2_f64) * t15750 + F::cast_from(0.21437009059034868486e-3_f64) * t20794 + F::cast_from(0.85748036236139473944e-3_f64) * t15754 + F::cast_from(0.13719685797782315831e-1_f64) * t3396 * t1181 * t15947 * t1454 - F::cast_from(0.20579528696673473746e-1_f64) * t3396 * t1165 * t1016 * t1298 * t1460 - F::cast_from(0.20007875121765877254e-2_f64) * t20806 + F::cast_from(0.10289764348336736874e-1_f64) * t20810 + F::cast_from(0.25724410870841842184e-2_f64) * t1531 * t1165 * t1879 * t945 + F::cast_from(0.68598428988911579156e-2_f64) * t15761 + F::cast_from(0.17149607247227894789e-2_f64) * t1173 * t1165 * t1532 * t20817;
    t20822
}
