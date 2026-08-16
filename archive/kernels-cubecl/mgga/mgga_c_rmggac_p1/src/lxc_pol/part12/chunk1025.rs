//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1025/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1025<F: Float>(t41164: F, t41199: F, t41232: F, t41259: F, t41293: F, t41334: F, t41360: F, t41383: F, t39680: F, t4669: F, t27041: F, t38564: F) -> (F, F, F) {
    let t41386 = t41164 + t41199 + t41232 + t41259 + t41293 + t41334 + t41360 + t41383;
    let t41393 = t4669 * t39680;
    let t41395 = t27041 * t38564;
    (t41386, t41393, t41395)
}
