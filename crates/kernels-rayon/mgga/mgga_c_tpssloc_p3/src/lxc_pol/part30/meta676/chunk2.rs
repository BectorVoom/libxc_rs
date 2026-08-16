//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2109/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2109(t3953: f64, t3961: f64, t3967: f64, t1437: f64, t4017: f64, t72: f64, t1433: f64, t4021: f64, t1865: f64, t22523: f64, t22554: f64, t26063: f64, t26067: f64, t26084: f64, t27966: f64, t27972: f64, t6490: f64, t6506: f64, t6510: f64, t7432: f64, t90308: f64, t90312: f64) -> f64 {
    let t96479 = t3953 * t3961;
    let t96482 = t3953 * t3967;
    let t96502 = t72 * t4017 * t1437;
    let t96506 = t72 * t1433 * t4021;
    let t96509 = 2.0_f64 / 3.0_f64 * t96479 * t1865 + 2.0_f64 / 3.0_f64 * t96482 * t1865 + 2.0_f64 / 3.0_f64 * t27966 * t6506 + 2.0_f64 / 3.0_f64 * t27966 * t6510 + 5.0_f64 / 3.0_f64 * t90308 * t7432 + 5.0_f64 / 3.0_f64 * t90312 * t7432 + 5.0_f64 / 3.0_f64 * t26084 * t26063 + 5.0_f64 / 3.0_f64 * t26084 * t26067 + 5.0_f64 / 3.0_f64 * t22554 * t27972 + 5.0_f64 / 3.0_f64 * t22523 * t27972 + 5.0_f64 / 3.0_f64 * t6490 * t96502 + 5.0_f64 / 3.0_f64 * t6490 * t96506;
    t96509
}
