//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1195/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1195<F: Float>(t2454: F, t733: F, t9709: F, t33121: F, t7333: F, t33091: F, t9972: F, t7307: F, t9704: F, t1873: F, t7437: F, t2591: F, t4581: F, t736: F, t7400: F, t1930: F, t2568: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34345 = t733 * t2454;
    let t34346 = t34345 * t9709;
    let t34348 = t33121 * t7333;
    let t34350 = t33091 * t9972;
    let t34352 = t9704 * t7307;
    let t34354 = t1873 * t7437;
    let t34356 = t4581 * t2591;
    let t34358 = t7400 * t736;
    let t34360 = t1930 * t2568;
    (t34345, t34346, t34348, t34350, t34352, t34354, t34356, t34358, t34360)
}
