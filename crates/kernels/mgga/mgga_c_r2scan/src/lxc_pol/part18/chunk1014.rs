//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1014/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1014<F: Float>(t23495: F, t3629: F, t11888: F, t8358: F, t12595: F, t19146: F, t12598: F, t6654: F, t1070: F, t1276: F, t9673: F, t11885: F, t2928: F, t3366: F, t6661: F, t2938: F) -> (F, F, F, F, F, F, F, F) {
    let t42491 = t23495 * t3629;
    let t42493 = t8358 * t11888;
    let t42495 = t19146 * t12595;
    let t42497 = t6654 * t12598;
    let t42500 = t1276 * t1070 * t9673;
    let t42502 = t8358 * t11885;
    let t42505 = t6661 * t3366 * t2928;
    let t42508 = t1276 * t3366 * t2938;
    (t42491, t42493, t42495, t42497, t42500, t42502, t42505, t42508)
}
