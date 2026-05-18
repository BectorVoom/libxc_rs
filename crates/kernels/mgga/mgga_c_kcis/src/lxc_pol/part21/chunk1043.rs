//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1043/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1043<F: Float>(t137: F, t2421: F, t86: F, t695: F, t8939: F, t2157: F, t68: F) -> (F, F, F, F) {
    let t26457 = t86 * t2421 * t137;
    let t26459 = t8939 * t695;
    let t26460 = t26459 * t2157;
    let t26462 = t695 * t68;
    (t26457, t26459, t26460, t26462)
}
