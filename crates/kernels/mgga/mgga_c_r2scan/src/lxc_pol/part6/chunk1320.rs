//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1320/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1320<F: Float>(t2183: F, t7244: F, t1616: F, t2201: F, t2719: F, t785: F, t2837: F, t5128: F, t1610: F, t2207: F, t7402: F, t6493: F, t8054: F, t20331: F, t20334: F, t2187: F, t2236: F, t24901: F, t24905: F, t24908: F, t24911: F, t24915: F, t24918: F, t7292: F) -> (F,) {
    let t24922 = t2183 * t7244;
    let t24927 = t2201 * t785 * t1616 * t2719;
    let t24928 = 0.2037639021386884617e0 * t24927;
    let t24932 = t2201 * t2837 * t5128;
    let t24935 = t2207 * t1610 * t7402;
    let t24937 = t6493 * t8054;
    let t24939 = -t24901 - t24905 - 0.41530324072742201648e-1 * t24908 - t24911 - t24915 - 0.12459097221822660495e0 * t24918 - 0.13002332610081402845e0 * t2236 * t7292 + 0.26004665220162805689e0 * t24922 * t2187 + t24928 + 0.4075278042773769234e0 * t20331 + 0.12225834128321307702e1 * t20334 - 0.17465477326173296717e-1 * t24932 - 0.1047928639570397803e0 * t24935 - 0.83214928704520978207e1 * t24937;
    (t24939,)
}
