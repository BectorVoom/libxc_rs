//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1149/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1149<F: Float>(t1005: F, t6081: F, t1856: F, t3228: F, t1008: F, t5811: F, t5816: F, t1089: F, t1095: F, t12762: F, t12770: F, t1298: F, t1524: F, t15675: F, t15710: F, t15714: F, t4099: F, t418: F, t4838: F, t495: F, t513: F) -> F {
    let t20672 = t1005 * t6081;
    let t20693 = t3228 * t1856;
    let t20695 = t1008 * t5811;
    let t20697 = t1008 * t5816;
    let t20699 = -F::cast_from(0.51448821741683684367e-2_f64) * t15675 + F::cast_from(0.85748036236139473944e-3_f64) * t20672 - F::cast_from(0.12862205435420921092e-2_f64) * t12762 - F::cast_from(0.20007875121765877254e-2_f64) * t12770 + F::cast_from(0.68598428988911579156e-2_f64) * t15710 + F::cast_from(0.34299214494455789578e-2_f64) * t15714 + F::cast_from(0.34299214494455789578e-2_f64) * t418 * t1089 * t1095 * t4099 * t513 + F::cast_from(0.68598428988911579156e-2_f64) * t418 * t1089 * t1095 * t1298 * t1524 + F::cast_from(0.34299214494455789578e-2_f64) * t418 * t1089 * t1095 * t495 * t4838 - F::cast_from(0.17149607247227894789e-2_f64) * t20693 - F::cast_from(0.34299214494455789578e-2_f64) * t20695 - F::cast_from(0.34299214494455789578e-2_f64) * t20697;
    t20699
}
