//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1016/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1016(t114012: f64, t114025: f64, t114027: f64, t114031: f64, t114034: f64, t114038: f64, t114046: f64, t114000: f64, t114003: f64, t114007: f64, t114019: f64, t114023: f64, t114041: f64) -> f64 {
    let t115458 = 7.0_f64 / 576.0_f64 * t114012;
    let t115461 = 0.42167100809435519335e-2_f64 * t114025;
    let t115462 = 0.90434973650874475512e-1_f64 * t114027;
    let t115463 = 0.32298204875312312682e-2_f64 * t114031;
    let t115464 = 7.0_f64 / 576.0_f64 * t114034;
    let t115465 = 119.0_f64 / 3456.0_f64 * t114038;
    let t115467 = 0.5383034145885385447e-3_f64 * t114046;
    let t115468 = 0.13565246047631171327e0_f64 * t114000 - t114003 / 384.0_f64 - t114007 / 768.0_f64 + t115458 + t114019 / 384.0_f64 - t114023 / 768.0_f64 + t115461 + t115462 + t115463 - t115464 + t115465 + t114041 / 768.0_f64 + t115467;
    t115468
}
