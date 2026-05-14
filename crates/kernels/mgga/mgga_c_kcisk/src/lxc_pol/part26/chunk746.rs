//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 746/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk746<F: Float>(t1440: F, t6332: F, t9491: F, t1493: F, t485: F, t394: F) -> (F, F, F, F) {
    let t9492 = t6332 * t1440;
    let t9493 = t9491 * t9492;
    let t9495 = t485 * t1493;
    let t9497 = t485 * t394;
    (t9492, t9493, t9495, t9497)
}
