//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 546/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk546<F: Float>(t240: F, t2681: F, t243: F, t247: F, t237: F, t124: F, t212: F, t596: F, t800: F) -> (F, F, F) {
    let t2682 = t2681 * t240;
    let t2684 = t2682 * t243 * t247;
    let t2686 = F::new(0.56688979511669985553e-2) * t237 * t2684;
    let t2689 = t800 * t124 * t596 * t212;
    (t2682, t2686, t2689)
}
