//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 894/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk894<F: Float>(t2526: F, t537: F, t2719: F, t277: F, t1567: F, t2530: F, t259: F, t6203: F, t571: F, t6240: F, t928: F, t6360: F) -> (F, F, F, F, F, F, F, F) {
    let t7994 = t537 * t2526;
    let t8001 = t277 * t2719;
    let t8012 = t1567 * t2530;
    let t8021 = t6203 * t259;
    let t8022 = t571 * t8021;
    let t8026 = t6240 * t928;
    let t8028 = t6360 * t259;
    let t8029 = t571 * t8028;
    (t7994, t8001, t8012, t8021, t8022, t8026, t8028, t8029)
}
