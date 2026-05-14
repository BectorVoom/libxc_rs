//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1019/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1019<F: Float>(t1724: F, t8698: F, t2418: F, t7134: F, t8733: F, t8730: F, t4911: F, t8729: F, t7138: F, t7099: F, t7107: F, t4864: F, t8708: F, t1709: F, t11056: F, t8701: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23516 = t8698 * t1724;
    let t23519 = t2418 * t7134;
    let t23522 = t8733 * t1724;
    let t23525 = t8730 * t1724;
    let t23528 = t8729 * t4911;
    let t23529 = t23528 * t1724;
    let t23532 = t7138 * t7134;
    let t23539 = t7099 * t7107;
    let t23541 = t4864 * t8708;
    let t23542 = t23541 * t1709;
    let t23544 = t11056 * t8701;
    (t23516, t23519, t23522, t23525, t23529, t23532, t23539, t23542, t23544)
}
