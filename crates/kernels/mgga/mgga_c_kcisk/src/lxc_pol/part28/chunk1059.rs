//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1059/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1059<F: Float>(t24232: F, t5289: F, t22283: F, t7430: F, t7429: F, t2559: F, t718: F, t7304: F, t17982: F, t7317: F, t24181: F, t5290: F, t7315: F, t1949: F, t9078: F, t1948: F) -> (F, F, F, F, F, F, F, F) {
    let t24233 = t5289 * t24232;
    let t24235 = t7430 * t22283;
    let t24236 = t7429 * t24235;
    let t24238 = t2559 * t718;
    let t24239 = t24238 * t7304;
    let t24241 = t17982 * t718;
    let t24242 = t24241 * t7317;
    let t24245 = t5290 * t24181;
    let t24246 = t7315 * t24245;
    let t24248 = t9078 * t1949;
    let t24249 = t1948 * t24248;
    (t24233, t24235, t24236, t24239, t24242, t24246, t24248, t24249)
}
