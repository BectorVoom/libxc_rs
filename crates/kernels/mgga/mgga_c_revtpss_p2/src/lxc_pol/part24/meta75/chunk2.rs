//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 464/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk464<F: Float>(t2237: F, t25: F, t89: F, t90: F, t29: F, t2: F, t580: F, t47: F, t59: F, t239: F, t64: F) -> (F, F, F, F, F, F, F) {
    let t2239 = F::new(42.0) * t25 * t2237;
    let t2246 = F::new(1.0) / t90 / t89;
    let t2247 = t29 * t2246;
    let t2255 = t2 * t580;
    let t2275 = F::new(1.0) / t47;
    let t2282 = F::new(1.0) / t59;
    let t2289 = t64 * t239;
    (t2239, t2246, t2247, t2255, t2275, t2282, t2289)
}
