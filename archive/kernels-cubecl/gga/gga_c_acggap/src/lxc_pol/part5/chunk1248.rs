//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1248/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1248<F: Float>(t17951: F, t5698: F, t16184: F, t5694: F, t1150: F, t1180: F, t1189: F, t17540: F, t17542: F, t17551: F, t17557: F, t17567: F, t17584: F, t17586: F, t20417: F, t335: F, t4593: F, t5160: F, t6288: F, t922: F, t960: F) -> F {
    let t22947 = t17951 * t5698;
    let t22949 = t16184 * t5694;
    let t22954 = -t335 * t4593 * t5160 / F::cast_from(12.0_f64) - t1150 * t960 * t6288 * t922 / F::cast_from(16.0_f64) - F::cast_from(0.17149607247227894789e-2_f64) * t17540 - F::cast_from(0.17149607247227894789e-2_f64) * t17542 - F::cast_from(0.16006300097412701803e0_f64) * t17551 + F::cast_from(0.13719685797782315831e-1_f64) * t17557 + F::cast_from(0.17149607247227894789e-2_f64) * t1180 * t20417 * t1189 + F::cast_from(7.0_f64) / F::cast_from(18.0_f64) * t22947 + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t22949 + F::cast_from(0.25724410870841842183e-2_f64) * t17567 + F::cast_from(0.42874018118069736972e-3_f64) * t17584 - F::cast_from(0.34299214494455789578e-2_f64) * t17586;
    t22954
}
