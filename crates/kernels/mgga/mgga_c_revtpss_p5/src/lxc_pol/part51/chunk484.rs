//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 484/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk484<F: Float>(t1207: F, t458: F, t456: F, t487: F, t1209: F, t1269: F, t3356: F, t1242: F, t472: F, t471: F, t1121: F, t414: F) -> (F, F, F, F, F, F, F) {
    let t3565 = F::cast_from(1.0_f64) / t1207 / t458;
    let t3566 = t456 * t3565;
    let t3567 = t3566 * t487;
    let t3572 = t1209 * t1269;
    let t3579 = F::cast_from(0.19755555555555555556e-1_f64) * t3356;
    let t3596 = F::cast_from(1.0_f64) / t1242 / t472;
    let t3603 = t471 * t471;
    let t3617 = F::cast_from(1.0_f64) / t414 / t1121;
    (t3566, t3567, t3572, t3579, t3596, t3603, t3617)
}
