//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 475/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk475<F: Float>(t2550: F, t2551: F, t2124: F, t129: F, t524: F, t525: F, t495: F, t277: F, t938: F) -> (F, F, F, F) {
    let t2552 = t2550 * t2551;
    let t2553 = t2124 * t2552;
    let t2557 = t524 * t525 * t129;
    let t2558 = t2550 * t495;
    let t2559 = t2124 * t2558;
    let t2562 = t277 * t938;
    (t2553, t2557, t2559, t2562)
}
