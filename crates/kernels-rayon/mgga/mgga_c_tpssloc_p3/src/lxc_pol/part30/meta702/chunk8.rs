//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2281/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2281(t25577: f64, t4630: f64, t25580: f64, t4571: f64, t17906: f64, t6765: f64, t17884: f64, t17655: f64, t23541: f64, t1618: f64, t17972: f64, t23433: f64, t23529: f64, t4575: f64, t5869: f64, t5900: f64, t82875: f64, t88251: f64, t88513: f64, t88591: f64) -> f64 {
    let t99495 = t25577 * t4630;
    let t99497 = t25580 * t4571;
    let t99501 = t6765 * t17906;
    let t99507 = t6765 * t17884;
    let t99509 = t23541 * t17655;
    let t99514 = -t88251 + t88513 * t4575 / 1152.0_f64 + t99495 / 1152.0_f64 + t99497 / 1728.0_f64 + t23529 * t5900 / 216.0_f64 - t99501 / 1728.0_f64 - t88591 * t1618 / 144.0_f64 + t23433 * t5869 / 1536.0_f64 + 5.0_f64 / 10368.0_f64 * t99507 - t99509 / 2304.0_f64 - t82875 / 10368.0_f64 + t6765 * t17972 / 384.0_f64;
    t99514
}
