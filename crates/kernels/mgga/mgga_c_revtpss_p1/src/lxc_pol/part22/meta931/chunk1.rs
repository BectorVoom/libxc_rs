//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3160/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3160<F: Float>(t12886: F, t5381: F, t12627: F, t489: F, t17728: F, t13011: F, t5373: F, t1222: F, t5368: F, t697: F, t17170: F, t73: F) -> (F, F, F, F, F, F) {
    let t57258 = t5381 * t12886;
    let t57264 = t12627 * t489;
    let t57265 = t57264 * t17728;
    let t57270 = t5373 * t13011;
    let t57273 = t1222 * t697 * t5368;
    let t57275 = t17170 * t73;
    (t57258, t57264, t57265, t57270, t57273, t57275)
}
