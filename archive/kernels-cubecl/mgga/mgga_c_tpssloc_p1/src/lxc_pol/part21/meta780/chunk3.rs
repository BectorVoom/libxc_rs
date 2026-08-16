//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2709/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2709<F: Float>(t12283: F, t19981: F, t19986: F, t16205: F, t3792: F, t19823: F, t40021: F, t12211: F, t19827: F, t19831: F, t1351: F, t6330: F) -> (F, F, F, F, F, F, F) {
    let t57143 = t12283 * t19981;
    let t57145 = t12283 * t19986;
    let t57147 = t3792 * t16205;
    let t57158 = t40021 * t19823;
    let t57160 = t12211 * t19827;
    let t57170 = t12211 * t19831;
    let t57172 = t6330 * t1351;
    (t57143, t57145, t57147, t57158, t57160, t57170, t57172)
}
