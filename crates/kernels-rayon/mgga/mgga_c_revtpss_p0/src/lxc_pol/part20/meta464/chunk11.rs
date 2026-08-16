//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1775/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1775(t39494: f64, t3964: f64, t4096: f64, t40270: f64, t4089: f64, t1437: f64, t4114: f64, t46902: f64, t47188: f64, t47424: f64, t47427: f64, t47432: f64, t47436: f64, t47442: f64, t47444: f64, t47450: f64, t820: f64) -> f64 {
    let t47454 = 0.20561456923286030469e-1_f64 * t3964 * t4096 * t39494;
    let t47455 = t40270 * t4089;
    let t47457 = -0.65854491829355115987e0_f64 * t820 * t1437 * t47188 - 0.11708928647259339623e0_f64 * t47424 + 0.65854491829355115985e-1_f64 * t47427 + 0.13878983423218070567e-1_f64 * t47432 + 0.39029762157531132076e-1_f64 * t47436 + t47442 + 0.12142592671231907757e0_f64 * t47444 + 0.39512695097613069591e1_f64 * t820 * t4114 * t46902 - 0.18505311230957427423e-1_f64 * t47450 + t47454 - 0.1040793657534163522e-1_f64 * t47455;
    t47457
}
