//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 942/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk942<F: Float>(t1862: F, t7375: F, t1073: F, t5547: F, t5522: F, t7332: F, t7336: F, t7352: F, t7361: F, t7363: F, t7366: F, t7368: F, t7371: F, t7373: F, t218: F, t2774: F, t675: F) -> (F, F, F, F, F) {
    let t7376 = t7375 * t1862;
    let t7378 = t5547 * t1073;
    let t7379 = t7378 * t1862;
    let t7382 = 0.34731666666666666667e0 * t7332 - t7336 + 0.1549425e1 * t7352 + 0.6311625e0 * t7361 + 0.3529725e1 * t7363 - 0.3529725e1 * t7366 - 0.17648625e1 * t7368 + 0.6311625e0 * t7371 + 0.31558125e0 * t7373 + 0.264729375e1 * t7376 - 0.157790625e0 * t7379 + 0.13772666666666666667e1 * t5522;
    let t7386 = t218 * t675 * t2774;
    (t7376, t7378, t7379, t7382, t7386)
}
