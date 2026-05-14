//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 547/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk547<F: Float>(t2742: F, t321: F, t320: F, t327: F, t301: F) -> (F, F, F, F, F) {
    let t2743 = t321 * t2742;
    let t2745 = 0.19318136643975017455e-1 * t320 * t2743;
    let t2746 = t327 * t327;
    let t2747 = 1.0 / t2746;
    let t2748 = t2747 * t301;
    (t2743, t2745, t2746, t2747, t2748)
}
