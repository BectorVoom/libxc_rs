//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 982/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk982<F: Float>(t12830: F, t14484: F, t4271: F, t4265: F, t4284: F, t1402: F, t3532: F, t3278: F, t12825: F, t41: F, t12829: F, t451: F) -> (F, F, F, F, F) {
    let t14486 = t4271 * t14484 * t12830;
    let t14489 = t4265 * t4284;
    let t14491 = t1402 * t3532;
    let t14493 = t4271 * t14491 * t3278;
    let t14496 = t41 * t12825;
    let t14497 = t451 * t12829;
    (t14486, t14489, t14493, t14496, t14497)
}
