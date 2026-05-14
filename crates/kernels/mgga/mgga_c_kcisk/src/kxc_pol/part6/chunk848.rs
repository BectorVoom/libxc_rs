//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 848/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk848<F: Float>(t18356: F, t18406: F, t18408: F, t2013: F, t24876: F, t24880: F, t24908: F, t24910: F, t24913: F, t24921: F, t24926: F, t2638: F, t29790: F, t7581: F, t7591: F, t9214: F, t9218: F) -> (F,) {
    let t29807 = 0.14392630972941853771e0 * t7591 * t9214 - 0.2698618307426597582e-1 * t2013 * t29790 - 0.47975436576472845903e-1 * t24908 + 0.17990788716177317214e-1 * t24910 - 0.17990788716177317214e-1 * t24913 + 0.89953943580886586067e-2 * t24921 + 0.11993859144118211476e-1 * t24926 - 0.14392630972941853771e0 * t24876 * t2638 + 0.2698618307426597582e-1 * t24880 * t2638 - 0.53972366148531951639e-1 * t7581 * t9218 - 0.59969295720591057378e-2 * t18356 - 0.17990788716177317213e-1 * t18406 + 0.47975436576472845902e-1 * t18408;
    (t29807,)
}
