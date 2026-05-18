//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 818/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk818<F: Float>(t2466: F, t838: F, t2476: F, t836: F, t2512: F, t819: F, t2520: F, t817: F, t7342: F, t837: F, t2492: F, t805: F) -> (F, F, F, F, F, F, F) {
    let t7727 = t838 * t2466;
    let t7730 = t2466 * t2476;
    let t7731 = t7730 * t836;
    let t7734 = t819 * t2512;
    let t7738 = t2512 * t2520 * t817;
    let t7741 = t7342 * t837;
    let t7744 = t805 * t2492;
    (t7727, t7730, t7731, t7734, t7738, t7741, t7744)
}
