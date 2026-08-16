//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1263/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1263(t2822: f64, t7214: f64, t7325: f64, t10849: f64, t8272: f64, t10845: f64, t7179: f64, t10109: f64, t2433: f64, t24704: f64, t24708: f64, t24712: f64, t24715: f64, t24718: f64, t2563: f64, t2569: f64, t277: f64, t7263: f64, t8273: f64, t95: f64) -> f64 {
    let t26073 = t2822 * t2822;
    let t26080 = t7325 * t7214;
    let t26084 = t10849 * t8272;
    let t26087 = t10845 * t7179;
    let t26090 = -0.77534644304710291488e-2_f64 * t95 * t277 * t26073 * t2569 - 2.0_f64 * t7263 * t2563 - 200.0_f64 / 9.0_f64 * t26080 + 8.0_f64 / 3.0_f64 * t10109 * t8273 + 800.0_f64 / 81.0_f64 * t2433 * t26084 - 400.0_f64 / 27.0_f64 * t2433 * t26087 + t24704 + t24708 + t24712 - t24715 - t24718;
    t26090
}
