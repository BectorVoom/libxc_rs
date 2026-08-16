//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1067/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1067<F: Float>(t1: F, t33256: F, t277: F, t11831: F, t11752: F, t9703: F, t19120: F, t3765: F, t11730: F, t7553: F, t190: F, t4043: F) -> (F, F, F, F, F, F, F) {
    let t33257 = t33256 * t1;
    let t33258 = t277 * t33257;
    let t33259 = t33258 * t11831;
    let t33261 = t11752 * t9703;
    let t33263 = t19120 * t3765;
    let t33265 = t7553 * t11730;
    let t33267 = t190 * t4043;
    (t33257, t33258, t33259, t33261, t33263, t33265, t33267)
}
