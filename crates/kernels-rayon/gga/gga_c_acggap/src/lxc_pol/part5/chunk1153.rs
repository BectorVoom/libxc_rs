//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1153/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1153(t1163: f64, t1165: f64, t4162: f64, t5862: f64, t3372: f64, t6351: f64, t1101: f64, t1879: f64, t3361: f64, t495: f64, t955: f64, t1016: f64, t1173: f64, t1181: f64, t1298: f64, t1439: f64, t1454: f64, t1460: f64, t1531: f64, t1532: f64, t15748: f64, t15750: f64, t15754: f64, t15761: f64, t15947: f64, t3396: f64, t4463: f64, t945: f64) -> f64 {
    let t20794 = t1163 * t1165 * t5862 * t4162;
    let t20806 = t3372 * t6351;
    let t20810 = t3361 * t1165 * t1879 * t1101;
    let t20817 = t955 * t495;
    let t20822 = 7.0_f64 / 72.0_f64 * t15748 - 0.34299214494455789578e-1_f64 * t4463 * t1165 * t15947 * t1439 + 0.12862205435420921092e-2_f64 * t15750 + 0.21437009059034868486e-3_f64 * t20794 + 0.85748036236139473944e-3_f64 * t15754 + 0.13719685797782315831e-1_f64 * t3396 * t1181 * t15947 * t1454 - 0.20579528696673473746e-1_f64 * t3396 * t1165 * t1016 * t1298 * t1460 - 0.20007875121765877254e-2_f64 * t20806 + 0.10289764348336736874e-1_f64 * t20810 + 0.25724410870841842184e-2_f64 * t1531 * t1165 * t1879 * t945 + 0.68598428988911579156e-2_f64 * t15761 + 0.17149607247227894789e-2_f64 * t1173 * t1165 * t1532 * t20817;
    t20822
}
