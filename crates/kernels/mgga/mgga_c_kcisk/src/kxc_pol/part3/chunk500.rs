//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 500/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk500<F: Float>(t4241: F, t486: F, t196: F, t3729: F, t306: F, t476: F, t140: F, t430: F, t480: F, t11: F, t139: F) -> (F, F, F, F, F) {
    let t4242 = t486 * t4241;
    let t4244 = t3729 * t196;
    let t4253 = t476 * t306;
    let t4264 = 0.88437037037037037037e-2 * t140 * t430 * t480;
    let t4265 = t139 * t11;
    (t4242, t4244, t4253, t4264, t4265)
}
