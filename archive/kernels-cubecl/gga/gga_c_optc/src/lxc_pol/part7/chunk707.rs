//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 707/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk707<F: Float>(t6735: F, t85: F, t1998: F, t509: F, t1796: F, t4: F, t558: F) -> (F, F, F, F) {
    let t6737 = F::cast_from(0.19751789702565206229e-1_f64) * t6735 * t85;
    let t6739 = t509 * t1998;
    let t6741 = F::cast_from(0.16265371324172286321e-1_f64) * t1796 * t6739;
    let t6742 = t558 * t4;
    (t6737, t6739, t6741, t6742)
}
