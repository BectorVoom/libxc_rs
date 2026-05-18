//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 981/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk981<F: Float>(t14111: F, t416: F, t140: F, t1477: F, t430: F, t1390: F, t1402: F, t1471: F, t3278: F, t12830: F, t4272: F, t12951: F, t451: F) -> (F, F, F, F, F) {
    let t14464 = t416 * t14111;
    let t14469 = t140 * t430 * t1477;
    let t14475 = t1402 * t1390;
    let t14477 = t1471 * t14475 * t3278;
    let t14481 = t1471 * t4272 * t12830;
    let t14484 = t451 * t12951;
    (t14464, t14469, t14477, t14481, t14484)
}
