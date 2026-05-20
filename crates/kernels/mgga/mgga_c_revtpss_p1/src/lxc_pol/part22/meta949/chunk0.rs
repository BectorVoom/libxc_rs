//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3189/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3189<F: Float>(t29048: F, t3362: F, t3655: F, t5258: F, t5262: F, t12976: F, t5362: F, t12963: F, t5327: F, t12995: F, t17308: F, t17283: F, t3678: F) -> (F, F, F, F, F, F, F) {
    let t59330 = t29048 * t3362;
    let t59336 = t5258 * t3655;
    let t59338 = t5262 * t3655;
    let t59349 = t12976 * t5362;
    let t59351 = t5327 * t12963;
    let t59353 = t17308 * t12995;
    let t59358 = t17283 * t3678;
    (t59330, t59336, t59338, t59349, t59351, t59353, t59358)
}
