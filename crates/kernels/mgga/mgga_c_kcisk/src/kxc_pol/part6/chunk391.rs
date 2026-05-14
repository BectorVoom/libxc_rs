//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 391/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk391<F: Float>(t2561: F, t2565: F, t2569: F, t2573: F, t2577: F, t2581: F, t2588: F, t2592: F) -> (F,) {
    let t2666 = 0.9375e-1 * t2561 - 0.9375e-1 * t2565 - 0.25e0 * t2569 + 0.625e-1 * t2573 - 0.101171875e-1 * t2577 + 0.101171875e-1 * t2581 + 0.53958333333333333333e-1 * t2588 - 0.13489583333333333333e-1 * t2592;
    (t2666,)
}
