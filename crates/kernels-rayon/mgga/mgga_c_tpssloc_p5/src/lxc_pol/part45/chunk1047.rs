//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1047/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1047(t114360: f64, t115766: f64, t115771: f64, t115773: f64, t115777: f64, t115781: f64, t115809: f64, t115915: f64, t115919: f64, t115920: f64, t115922: f64, t115924: f64, t115927: f64, t115929: f64, t2040: f64, t2075: f64, t22559: f64, t24433: f64, t24442: f64, t26103: f64, t574: f64, t6517: f64, t6862: f64, t7040: f64, t7050: f64, t90044: f64) -> f64 {
    let t115934 = -2.0_f64 * t90044 * t2040 - 4.0_f64 * t26103 * t7050 + t115766 - 2.0_f64 * t7040 * t6862 - t115771 - t22559 * t2075 - t115773 + t115777 - t115781 + (t115809 + t115915) * t574 - t115919 - t115920 + t115922 + t115924 - t115927 - t115929 - 6.0_f64 * t114360 * t24433 - 2.0_f64 * t6517 * t24442;
    t115934
}
