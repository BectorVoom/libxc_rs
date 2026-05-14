//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 397/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk397<F: Float>(t143: F, t2379: F, t126: F, t684: F, t15: F, t60: F, t762: F, t647: F, t130: F, t20: F, t21: F, t736: F, t97: F, t787: F, t5: F, t728: F, t88: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2542 = t2379 * t143;
    let t2545 = t684 * t126;
    let t2546 = t2545 * t15;
    let t2551 = t60 * t762;
    let t2552 = t2551 * t647;
    let t2553 = t130 * t20;
    let t2555 = t2553 * t21 * t736;
    let t2558 = t15 * t97;
    let t2559 = t787 * t2558;
    let t2561 = t5 * t88 * t728;
    (t2542, t2545, t2546, t2551, t2552, t2553, t2555, t2558, t2559, t2561)
}
