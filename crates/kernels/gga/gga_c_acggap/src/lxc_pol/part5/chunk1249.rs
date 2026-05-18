//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1249/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1249<F: Float>(t1111: F, t1165: F, t21532: F, t4282: F, t1096: F, t1181: F, t16871: F, t17592: F, t17605: F, t17607: F, t17613: F, t17615: F, t17617: F, t18751: F, t20206: F, t3396: F, t4199: F, t4267: F, t4450: F, t4463: F, t4526: F, t5258: F, t530: F, t5852: F) -> F {
    let t22962 = t4282 * t1165 * t21532 * t1111;
    let t22985 = F::new(0.17149607247227894789e-2) * t17592 + F::new(0.34299214494455789578e-1) * t4463 * t1181 * t530 * t5258 + F::new(0.17149607247227894789e-1) * t22962 + F::new(0.85748036236139473944e-3) * t17605 + F::new(0.68598428988911579156e-2) * t3396 * t1181 * t4267 * t4526 + F::new(0.10289764348336736873e0) * t16871 * t1165 * t21532 * t1096 + F::new(0.51448821741683684368e-2) * t18751 * t1165 * t5852 * t20206 + F::new(0.85748036236139473944e-3) * t17607 - F::new(0.77173232612525526552e-2) * t4450 * t1165 * t5852 * t4199 + F::new(0.80031500487063509016e-2) * t17613 + F::new(0.17149607247227894789e-2) * t17615 + F::new(0.17149607247227894789e-2) * t17617;
    t22985
}
