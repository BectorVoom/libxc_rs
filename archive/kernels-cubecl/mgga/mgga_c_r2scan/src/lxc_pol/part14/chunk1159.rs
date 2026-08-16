//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1159/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1159<F: Float>(t2147: F, t26307: F, t3332: F, t261: F, t3299: F, t7390: F, t10879: F, t11727: F, t3304: F, t7309: F, t10760: F, t24059: F) -> (F, F, F, F, F) {
    let t40165 = t2147 * t3332 * t26307;
    let t40175 = t3299 * t261 * t7390;
    let t40177 = t10879 * t11727;
    let t40180 = t3304 * t261 * t7309;
    let t40183 = t2147 * t10760 * t24059;
    (t40165, t40175, t40177, t40180, t40183)
}
