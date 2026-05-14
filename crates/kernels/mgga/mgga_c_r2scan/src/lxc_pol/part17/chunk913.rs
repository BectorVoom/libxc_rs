//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 913/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk913<F: Float>(t2207: F, t3606: F, t3613: F, t1060: F, t3190: F, t1058: F, t5103: F, t2892: F, t5095: F, t3016: F, t3198: F, t3290: F, t11744: F, t3591: F, t10748: F, t3187: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12453 = t2207 * t3613 * t3606;
    let t12455 = t1060 * t3190;
    let t12457 = t5103 * t1058 * t12455;
    let t12459 = t1060 * t2892;
    let t12461 = t5095 * t1058 * t12459;
    let t12463 = t1060 * t3016;
    let t12465 = t2207 * t1058 * t12463;
    let t12468 = t3290 * t3198;
    let t12470 = t11744 * t3591;
    let t12472 = t10748 * t3187;
    (t12453, t12455, t12457, t12459, t12461, t12463, t12465, t12468, t12470, t12472)
}
