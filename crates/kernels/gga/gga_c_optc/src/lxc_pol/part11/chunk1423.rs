//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1423/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1423<F: Float>(t59083: F, t59155: F, t59189: F, t59432: F, t1136: F, t55927: F, t894: F, t27189: F, t55917: F, t1114: F, t27083: F, t27037: F) -> (F, F, F, F, F, F) {
    let t59434 = t59083 + t59155 + t59189 + t59432;
    let t59448 = t894 * t1136 * t55927;
    let t59452 = t894 * t27189 * t55917;
    let t59458 = t1114 * t55927;
    let t59462 = t27083 * t55917;
    let t59468 = t27037 * t55917;
    (t59434, t59448, t59452, t59458, t59462, t59468)
}
