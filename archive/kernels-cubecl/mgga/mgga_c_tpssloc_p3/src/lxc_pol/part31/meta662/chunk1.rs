//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1950/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1950<F: Float>(t25224: F, t25341: F, t6552: F, t23164: F, t7479: F, t86893: F, t16596: F, t86721: F, t1484: F, t584: F, t86753: F, t16949: F, t25014: F) -> (F, F, F, F, F) {
    let t99033 = t6552 * t25224 * t25341;
    let t99036 = t23164 * t86893 * t7479;
    let t99049 = t86721 * t16596;
    let t99053 = t86753 * t584 * t1484;
    let t99056 = t25014 * t16949;
    (t99033, t99036, t99049, t99053, t99056)
}
