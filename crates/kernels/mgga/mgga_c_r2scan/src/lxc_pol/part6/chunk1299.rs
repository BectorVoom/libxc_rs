//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1299/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1299<F: Float>(t6118: F, t7373: F, t6527: F, t7460: F, t1632: F, t2196: F, t551: F, t7569: F, t1592: F, t19854: F, t19858: F, t19863: F, t19869: F, t19873: F, t20609: F, t20784: F, t2184: F, t2185: F, t2567: F, t2640: F, t2656: F, t2719: F, t360: F, t481: F, t5136: F, t552: F, t560: F, t6334: F, t7290: F, t7454: F, t8289: F) -> (F,) {
    let t24472 = t6118 * t7373;
    let t24475 = t6527 * t7460;
    let t24496 = t2196 * t551 * t1632 * t7569;
    let t24506 = -0.26004665220162805689e0 * t8289 * t7454 - 0.38415120233790484326e0 * t24472 + 0.11426392607441748233e0 * t19854 - 0.7801399566048841707e1 * t24475 * t360 * t2567 * t6334 + 0.39006997830244208535e0 * t20609 * t2656 + 0.15602799132097683414e1 * t20784 * t2640 + 0.26004665220162805689e0 * t2184 * t551 * t552 * t7290 * t560 + 0.39006997830244208535e0 * t1592 * t551 * t552 * t7290 * t481 - 0.41607464352260489103e1 * t24496 - 0.78013995660488417068e0 * t5136 * t551 * t552 * t2719 * t2185 + t19858 + 0.86743646395112941039e-4 * t19863 - 0.34930954652346593433e-1 * t19869 + 0.1047928639570397803e0 * t19873;
    (t24506,)
}
