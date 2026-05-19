//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 188/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk188<F: Float>(t718: F, t719: F, t717: F, t415: F, t604: F, t671: F, t196: F) -> (F, F, F, F, F) {
    let t720 = t718 * t719;
    let t721 = t717 * t720;
    let t722 = t415 * t721;
    let t724 = t604 * t671 + F::cast_from(0.24872916666666666666e-2_f64) * t722;
    let t725 = t604 * t196;
    (t720, t721, t722, t724, t725)
}
