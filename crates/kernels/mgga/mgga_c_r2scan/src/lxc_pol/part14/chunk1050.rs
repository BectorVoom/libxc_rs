//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1050/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1050<F: Float>(t10641: F, t1102: F, t3314: F, t1615: F, t2317: F, t269: F, t3438: F, t6855: F, t874: F, t10935: F, t2068: F, t3446: F) -> (F, F, F) {
    let t37380 = t1102 * t3314 * t10641;
    let t37386 = t6855 * t1615 * t2317 * t3438 * t269 * t874;
    let t37390 = t3446 * t10935 * t2068;
    (t37380, t37386, t37390)
}
