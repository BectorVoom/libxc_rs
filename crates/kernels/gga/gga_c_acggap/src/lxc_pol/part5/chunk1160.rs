//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1160/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1160<F: Float>(t1165: F, t1173: F, t1180: F, t15906: F, t15914: F, t15916: F, t15918: F, t15920: F, t15922: F, t20897: F, t20904: F, t20906: F, t3403: F, t407: F, t4289: F, t5735: F, t5862: F, t6258: F, t930: F) -> F {
    let t20924 = F::cast_from(0.90702367218671976884e-1_f64) * t20897 + F::cast_from(0.12004725073059526352e-1_f64) * t15906 + F::cast_from(0.34299214494455789578e-2_f64) * t1173 * t1165 * t4289 * t6258 - F::cast_from(0.42874018118069736972e-3_f64) * t20904 - F::cast_from(0.42874018118069736972e-3_f64) * t1180 * t1165 * t20906 * t407 - F::cast_from(0.21437009059034868486e-3_f64) * t1180 * t1165 * t5862 * t930 + F::cast_from(0.80031500487063509016e-2_f64) * t15914 + F::cast_from(0.32012600194825403606e-1_f64) * t15916 - F::cast_from(0.17149607247227894789e-1_f64) * t3403 * t1165 * t4289 * t5735 + F::cast_from(0.80031500487063509016e-1_f64) * t15918 - F::cast_from(0.32012600194825403606e-1_f64) * t15920 + F::cast_from(0.32012600194825403606e-1_f64) * t15922;
    t20924
}
