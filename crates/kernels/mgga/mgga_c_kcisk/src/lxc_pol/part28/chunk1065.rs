//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1065/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1065<F: Float>(t1824: F, t2487: F, t16892: F, t16805: F, t7034: F, t4265: F, t9003: F, t9007: F, t196: F, t22919: F, t1919: F, t220: F, t7389: F, t11821: F, t7715: F, t5254: F, t7718: F) -> (F, F, F, F, F, F, F, F) {
    let t24363 = t2487 * t1824;
    let t24364 = t16892 * t24363;
    let t24367 = t7034 * t16805;
    let t24374 = t4265 * t9003;
    let t24376 = t4265 * t9007;
    let t24380 = t22919 * t196;
    let t24388 = t1919 * t7389 * t220;
    let t24392 = t1919 * t11821 * t7715;
    let t24396 = t1919 * t5254 * t7718;
    (t24364, t24367, t24374, t24376, t24380, t24388, t24392, t24396)
}
