//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 801/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk801<F: Float>(t259: F, t2649: F, t546: F, t565: F, t2526: F, t537: F, t2719: F, t277: F, t6240: F, t928: F, t6360: F, t571: F) -> (F, F, F, F, F, F, F) {
    let t7983 = t2649 * t259;
    let t7984 = t546 * t7983;
    let t7987 = t565 * t7983;
    let t7994 = t537 * t2526;
    let t8001 = t277 * t2719;
    let t8026 = t6240 * t928;
    let t8028 = t6360 * t259;
    let t8029 = t571 * t8028;
    (t7983, t7984, t7987, t7994, t8001, t8026, t8029)
}
