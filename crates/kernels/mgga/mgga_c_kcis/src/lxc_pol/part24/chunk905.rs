//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 905/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk905<F: Float>(t26420: F, t7655: F, t898: F, t2165: F, t2772: F, t874: F, t9194: F, t2157: F, t710: F, t7603: F, t86: F, t137: F, t2480: F, t2489: F, t2491: F, t125: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26421 = 4.0 * t26420;
    let t26422 = t7655 * t898;
    let t26425 = t2165 * t2772;
    let t26430 = t874 * t9194;
    let t26431 = t26430 * t2157;
    let t26434 = t86 * t710 * t7603;
    let t26437 = t86 * t2480 * t137;
    let t26439 = t2489 * t2491;
    let t26441 = t86 * t125 * t26439;
    (t26421, t26422, t26425, t26430, t26431, t26434, t26437, t26439, t26441)
}
