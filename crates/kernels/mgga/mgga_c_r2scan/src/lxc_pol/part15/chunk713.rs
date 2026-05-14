//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 713/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk713<F: Float>(t120: F, t122: F, t135: F, t273: F, t57: F, t2096: F, t784: F, t23: F, t271: F, t6077: F, t95: F, t257: F, t260: F, t277: F, t255: F, t254: F) -> (F, F, F, F, F) {
    let t6310 = 0.92480845007273388189e0 * t120 * t122 * t273 * t57 * t135;
    let t6311 = t2096 * t784;
    let t6314 = 1.0 / t23 / t6077 / t271;
    let t6317 = t95 * t95;
    let t6319 = 1.0 / t257 / t6317;
    let t6321 = t6319 * t260 * t277;
    let t6322 = t6311 * t6314 * t255 * t6321;
    let t6324 = 0.41530324072742201648e-1 * t254 * t6322;
    (t6310, t6314, t6319, t6321, t6324)
}
