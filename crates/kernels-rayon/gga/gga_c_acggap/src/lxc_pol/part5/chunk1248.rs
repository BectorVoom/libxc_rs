//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1248/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1248(t17951: f64, t5698: f64, t16184: f64, t5694: f64, t1150: f64, t1180: f64, t1189: f64, t17540: f64, t17542: f64, t17551: f64, t17557: f64, t17567: f64, t17584: f64, t17586: f64, t20417: f64, t335: f64, t4593: f64, t5160: f64, t6288: f64, t922: f64, t960: f64) -> f64 {
    let t22947 = t17951 * t5698;
    let t22949 = t16184 * t5694;
    let t22954 = -t335 * t4593 * t5160 / 12.0_f64 - t1150 * t960 * t6288 * t922 / 16.0_f64 - 0.17149607247227894789e-2_f64 * t17540 - 0.17149607247227894789e-2_f64 * t17542 - 0.16006300097412701803e0_f64 * t17551 + 0.13719685797782315831e-1_f64 * t17557 + 0.17149607247227894789e-2_f64 * t1180 * t20417 * t1189 + 7.0_f64 / 18.0_f64 * t22947 + 7.0_f64 / 12.0_f64 * t22949 + 0.25724410870841842183e-2_f64 * t17567 + 0.42874018118069736972e-3_f64 * t17584 - 0.34299214494455789578e-2_f64 * t17586;
    t22954
}
