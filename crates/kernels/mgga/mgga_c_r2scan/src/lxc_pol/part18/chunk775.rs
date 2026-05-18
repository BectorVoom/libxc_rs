//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 775/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk775<F: Float>(t2526: F, t788: F, t2207: F, t785: F, t2841: F, t481: F, t6243: F, t1604: F, t625: F, t923: F, t6462: F, t2530: F, t277: F) -> (F, F, F, F, F, F, F) {
    let t7402 = t788 * t2526;
    let t7405 = F::new(0.34930954652346593434e-1) * t2207 * t785 * t7402;
    let t7406 = t2841 * t481;
    let t7407 = t6243 * t7406;
    let t7408 = t1604 * t7407;
    let t7418 = t923 * t625;
    let t7419 = t6462 * t7418;
    let t7433 = t277 * t2530;
    (t7405, t7406, t7407, t7408, t7418, t7419, t7433)
}
