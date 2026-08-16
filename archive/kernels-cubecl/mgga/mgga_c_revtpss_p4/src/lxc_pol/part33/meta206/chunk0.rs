//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 959/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk959<F: Float>(t5332: F, t5465: F, t1269: F, t1287: F, t1794: F, t487: F, t5284: F, t3781: F, t460: F, t1248: F, t3302: F, t471: F) -> (F, F, F, F, F, F) {
    let t5466 = t5332 * t5465;
    let t5470 = t1269 * t1794 * t1287;
    let t5474 = t487 * t5284 * t1287;
    let t5477 = t3781 * t487;
    let t5478 = t460 * t5477;
    let t5479 = t3302 * t1248;
    let t5480 = t5479 * t471;
    (t5466, t5470, t5474, t5477, t5478, t5480)
}
