//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 643/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk643<F: Float>(t1284: F, t487: F, t1209: F, t1287: F, t3721: F, t1269: F, t473: F, t1214: F, t1280: F, t3584: F, t3140: F, t3596: F) -> (F, F, F, F, F, F, F) {
    let t3754 = t1284 * t487;
    let t3755 = t1209 * t3754;
    let t3756 = t3721 * t1287;
    let t3759 = t473 * t1269;
    let t3760 = t3759 * t1214;
    let t3763 = t1280 * t3584;
    let t3766 = t3140 * t3596;
    (t3754, t3755, t3756, t3759, t3760, t3763, t3766)
}
