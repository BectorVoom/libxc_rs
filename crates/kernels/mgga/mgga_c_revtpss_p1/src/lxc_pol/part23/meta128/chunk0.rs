//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 836/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk836<F: Float>(t3356: F, t3140: F, t460: F, t1242: F, t472: F, t474: F, t3147: F, t479: F, t471: F) -> (F, F, F, F, F, F, F, F) {
    let t3579 = F::cast_from(0.19755555555555555556e-1_f64) * t3356;
    let t3594 = t460 * t3140;
    let t3596 = F::new(1.0) / t1242 / t472;
    let t3597 = t3596 * t474;
    let t3598 = t479 * t3147;
    let t3599 = t3597 * t3598;
    let t3600 = t3594 * t3599;
    let t3603 = t471 * t471;
    (t3579, t3594, t3596, t3597, t3598, t3599, t3600, t3603)
}
