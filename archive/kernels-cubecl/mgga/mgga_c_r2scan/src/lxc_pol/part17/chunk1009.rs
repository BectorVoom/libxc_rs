//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1009/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1009<F: Float>(t12492: F, t6085: F, t10760: F, t9246: F, t6093: F, t261: F, t3217: F, t3299: F, t11748: F, t3594: F, t3223: F, t3304: F) -> (F, F, F, F, F, F, F, F) {
    let t12493 = t6085 * t12492;
    let t12495 = t10760 * t9246;
    let t12496 = t6093 * t12495;
    let t12498 = t261 * t3217;
    let t12499 = t3299 * t12498;
    let t12501 = t11748 * t3594;
    let t12503 = t261 * t3223;
    let t12504 = t3304 * t12503;
    (t12493, t12495, t12496, t12498, t12499, t12501, t12503, t12504)
}
