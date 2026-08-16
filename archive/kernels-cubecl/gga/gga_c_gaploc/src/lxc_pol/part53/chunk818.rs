//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 818/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk818<F: Float>(t12651: F, t2684: F, t7354: F, t12652: F, t7416: F, t161: F, t165: F, t9688: F, t2685: F, t2464: F, t2465: F, t9729: F) -> (F, F, F, F, F) {
    let t41411 = t2684 * t7354 * t12651;
    let t41413 = t7416 * t12652;
    let t41416 = t161 * t165 * t9688;
    let t41418 = t2684 * t2685 * t41416;
    let t41422 = t2684 * t2464 * t2465 * t9729;
    (t41411, t41413, t41416, t41418, t41422)
}
