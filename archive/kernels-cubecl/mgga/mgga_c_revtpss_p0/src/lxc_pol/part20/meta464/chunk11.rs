//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1775/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1775<F: Float>(t39494: F, t3964: F, t4096: F, t40270: F, t4089: F, t1437: F, t4114: F, t46902: F, t47188: F, t47424: F, t47427: F, t47432: F, t47436: F, t47442: F, t47444: F, t47450: F, t820: F) -> F {
    let t47454 = F::cast_from(0.20561456923286030469e-1_f64) * t3964 * t4096 * t39494;
    let t47455 = t40270 * t4089;
    let t47457 = -F::cast_from(0.65854491829355115987e0_f64) * t820 * t1437 * t47188 - F::cast_from(0.11708928647259339623e0_f64) * t47424 + F::cast_from(0.65854491829355115985e-1_f64) * t47427 + F::cast_from(0.13878983423218070567e-1_f64) * t47432 + F::cast_from(0.39029762157531132076e-1_f64) * t47436 + t47442 + F::cast_from(0.12142592671231907757e0_f64) * t47444 + F::cast_from(0.39512695097613069591e1_f64) * t820 * t4114 * t46902 - F::cast_from(0.18505311230957427423e-1_f64) * t47450 + t47454 - F::cast_from(0.1040793657534163522e-1_f64) * t47455;
    t47457
}
