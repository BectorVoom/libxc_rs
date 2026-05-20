//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 742/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk742<F: Float>(t3566: F, t487: F, t1209: F, t1269: F, t3356: F, t3140: F, t460: F, t1242: F, t472: F, t474: F, t3147: F, t479: F) -> (F, F, F, F, F, F, F) {
    let t3567 = t3566 * t487;
    let t3572 = t1209 * t1269;
    let t3579 = F::cast_from(0.19755555555555555556e-1_f64) * t3356;
    let t3594 = t460 * t3140;
    let t3596 = F::new(1.0) / t1242 / t472;
    let t3597 = t3596 * t474;
    let t3598 = t479 * t3147;
    (t3567, t3572, t3579, t3594, t3596, t3597, t3598)
}
